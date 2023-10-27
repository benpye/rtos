use core::{
    array::from_fn,
    num::{NonZeroU32, NonZeroU64},
    panic,
    sync::atomic::{AtomicUsize, Ordering},
};

pub use kernel_types::task::*;
use memoffset::offset_of;
use paste::paste;
use rtos_macros::{const_array_from_fn, rtos_import};
use seq_macro::seq;

use crate::{arch, syscall, time};

seq!(N in 1..=16 {
    paste! {
        #[cfg(feature = "num_tasks_" N)]
        const NUM_TASKS: usize = N;
        #[cfg(feature = "num_tasks_" N)]
        #[no_mangle]
        static mut TASK_TABLE: TaskTable = TaskTable(const_array_from_fn![Task::zeroed_at; N]);
        #[doc(hidden)]
        #[cfg(feature = "num_tasks_" N)]
        #[export_name = "rtos.feature.num_tasks_" N]
        #[link_section = ".note.rtos.feature"]
        #[used]
        static [<RTOS_FEATURE_NUM_TASKS_ N _MARKER>]: () = ();
    }
});

#[cfg(not(feature = "num_tasks_defined"))]
compile_error!("One of the num_tasks_N features must be enabled!");

#[cfg(not(feature = "num_tasks_defined"))]
pub const NUM_TASKS: usize = 0;
#[cfg(not(feature = "num_tasks_defined"))]
static mut TASK_TABLE: TaskTable = TaskTable([]);

// The task table begins locked, it is only unlocked once it is initialized.
static TASK_TABLE_LOCK: AtomicUsize = AtomicUsize::new(1);

pub struct TaskTable([Task; NUM_TASKS]);

impl core::ops::Index<TaskId> for TaskTable {
    type Output = Task;

    fn index(&self, index: TaskId) -> &Self::Output {
        // Safety: We ensure that TaskId is in bounds on it's creation.
        unsafe { self.0.get_unchecked(index.0) }
    }
}

impl core::ops::IndexMut<TaskId> for TaskTable {
    fn index_mut(&mut self, index: TaskId) -> &mut Self::Output {
        // Safety: We ensure that TaskId is in bounds on it's creation.
        unsafe { self.0.get_unchecked_mut(index.0) }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TaskState {
    // Task is in a fatal state and may only be recovered via reset.
    Fatal,
    // Task is in a runnable state.
    Ready,
    // Task is in the request phase of a SYS_CALL.
    CallRequest(TaskId),
    // Task is in the response phase of a SYS_CALL.
    CallResponse(TaskId),
    // Task is waiting to receive a SYS_CALL or notification.
    Receive,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Schedule {
    Same,
    Other,
    Exactly(TaskId),
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum InterruptControl {
    Disable,
    Enable,
    Complete,
}

pub struct InvalidInterruptControl;

impl TryFrom<syscall::abi::InterruptControl> for InterruptControl {
    type Error = InvalidInterruptControl;

    fn try_from(value: syscall::abi::InterruptControl) -> Result<Self, Self::Error> {
        match value {
            syscall::abi::InterruptControl::Disable => Ok(InterruptControl::Disable),
            syscall::abi::InterruptControl::Enable => Ok(InterruptControl::Enable),
            syscall::abi::InterruptControl::Complete => Ok(InterruptControl::Complete),
            _ => Err(InvalidInterruptControl),
        }
    }
}

pub struct Task {
    context: arch::SavedContext,
    index: u8,
    state: TaskState,
    // Priority - including any increase in priority due to dependent tasks.
    current_priority: u8,
    notifications: u32,

    timer_deadline: u64,
    timer_period: Option<NonZeroU64>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct TaskId(usize);

impl TaskId {
    #[inline]
    const unsafe fn new_unchecked(id: u8) -> Self {
        TaskId(id as usize)
    }

    #[inline]
    pub const fn new(id: u8) -> Option<Self> {
        if (id as usize) < NUM_TASKS {
            Some(TaskId(id as usize))
        } else {
            None
        }
    }
}

impl From<TaskId> for u8 {
    #[inline]
    fn from(value: TaskId) -> Self {
        value.0 as u8
    }
}

impl Task {
    // The offset of SavedContext within the Task struct, required for
    // assembler routines that need to access the context given a task pointer.
    pub const CONTEXT_OFFSET: usize = offset_of!(Self, context);

    const fn zeroed_at(_: usize) -> Self {
        // The linker script ensures that TASK_TABLE is within the .bss section
        // which requires that the result of this be all zeros.
        Self {
            context: arch::SavedContext::zeroed(),
            index: 0,
            state: TaskState::Fatal,
            current_priority: 0,
            notifications: 0,
            timer_deadline: 0,
            timer_period: None,
        }
    }

    #[inline]
    pub fn index(&self) -> TaskId {
        unsafe { TaskId::new_unchecked(self.index) }
    }

    #[inline]
    pub fn context(&self) -> &arch::SavedContext {
        &self.context
    }

    #[inline]
    pub fn context_mut(&mut self) -> &mut arch::SavedContext {
        &mut self.context
    }

    pub fn descriptor(&self) -> &'static TaskDescriptor {
        #[rtos_import]
        static TASK_DESCRIPTOR_TABLE: [TaskDescriptor; NUM_TASKS];

        // Safety: We trust that the TASK_DESC_TABLE in the image is of the
        // correct size and alignment - the linker script has assertions to
        // this effect. With that in mind it is safe to take an immutable
        // reference. An out of bounds access here will still lead to a panic.
        unsafe { &TASK_DESCRIPTOR_TABLE[self.index().0] }
    }

    #[inline]
    pub fn state(&self) -> TaskState {
        self.state
    }

    #[inline]
    fn set_state(&mut self, new_state: TaskState) {
        match (self.state, new_state) {
            // The Fatal state may be entered from any other state.
            (_, TaskState::Fatal) => self.state = new_state,

            // It is only legal to enter the CallRequest state from the Ready
            // state.
            (TaskState::Ready, TaskState::CallRequest(_)) => self.state = new_state,

            // It is only legal to enter the CallResponse state from
            // CallRequest state - with the same target.
            (TaskState::CallRequest(send_target), TaskState::CallResponse(receive_target)) => {
                assert!(send_target == receive_target);
                self.state = new_state
            }
            // It is legal to return to the Ready state from the CallResponse
            // state.
            (TaskState::CallResponse(_), TaskState::Ready) => self.state = new_state,

            // It is only legal to enter the Receive state from the Ready
            // state.
            (TaskState::Ready, TaskState::Receive) => self.state = new_state,
            // It is legal to return to the Ready state from the Receive state.
            (TaskState::Receive, TaskState::Ready) => self.state = new_state,

            // All other state transitions are illegal.
            (_, _) => panic!("attempted illegal state transition",),
        }
    }

    pub fn reset(&mut self) {
        // It is legal for a task to reset from any state, including the Fatal
        // state.
        self.state = TaskState::Ready;
        self.current_priority = self.descriptor().priority;
        self.context.task_reset(self.descriptor());
        self.set_timer(false, None);

        // Ensure that any interrupts associated with this task are in their
        // initial state.
        for (interrupt, _) in
            interrupt_descriptor_iter().filter(|(_, desc)| desc.task_id() == self.index)
        {
            arch::reset_interrupt(interrupt);
        }
    }

    #[inline]
    pub fn notifications(&self) -> u32 {
        self.notifications
    }

    #[inline]
    pub fn reset_notifications(&mut self) {
        self.notifications = 0;
    }

    pub fn set_as_current(&self) {
        assert!(self.state() == TaskState::Ready);

        arch::apply_memory_protection(self);
        // Safety: This aliases self however the aliased pointer is not used
        // outside of kernel entry/exit.
        unsafe { arch::set_current_task(self) }
    }

    fn set_timer(&mut self, periodic: bool, deadline: Option<NonZeroU32>) {
        match (periodic, deadline) {
            (_, None) => {
                // Disable the timer. The physical timer may still fire but
                // that's okay.
                self.timer_period = None;
                self.timer_deadline = u64::MAX;
            }
            (true, Some(deadline)) => {
                // Periodic timer
                let period_ticks = time::us_to_ticks(deadline.into());
                self.timer_period = Some(NonZeroU64::new(period_ticks).unwrap());
                self.timer_deadline = time::now_ticks().wrapping_add(period_ticks);

                time::update_deadline(self.timer_deadline);
            }
            (false, Some(deadline)) => {
                // Relative timer
                self.timer_period = None;
                self.timer_deadline =
                    time::now_ticks().wrapping_add(time::us_to_ticks(deadline.into()));

                time::update_deadline(self.timer_deadline);
            }
        }
    }

    fn evaluate_timer(&mut self, now_ticks: u64) -> bool {
        if now_ticks < self.timer_deadline {
            // The timer has not yet expired.
            return false;
        }

        if let Some(period_ticks) = self.timer_period {
            // This is a periodic timer that should be rearmed. The period is
            // best effort, ie. period_ticks is the minimum period at which the
            // timer will fire.
            self.timer_deadline = now_ticks.wrapping_add(period_ticks.into());
            time::update_deadline(self.timer_deadline);
        } else {
            // Otherwise this timer was a one shot that won't fire again.
            self.timer_deadline = u64::MAX;
        }

        return self.post(syscall::abi::SYS_NOTIFICATION_TIMER);
    }

    pub fn post(&mut self, notification: u32) -> bool {
        self.notifications |= notification;

        if (self.notifications != 0) && (self.state() == TaskState::Receive) {
            syscall::set_receive_result(self, None);
            self.set_state(TaskState::Ready);
            return true;
        }

        return false;
    }
}

impl TaskTable {
    pub fn get_pair_mut(&mut self, first: TaskId, second: TaskId) -> (&mut Task, &mut Task) {
        assert!(first != second);

        let array = self.0.as_mut_ptr();

        // Safety: First and second must refer to different elements, and must
        // both be in bounds. The bounds check is performed when TaskId is
        // initialized, the equality test is above.
        unsafe { (&mut *array.add(first.0), &mut *array.add(second.0)) }
    }
}

pub fn task_init() {
    static TASK_TABLE_INITIALIZED: AtomicUsize = AtomicUsize::new(0);

    // Ensure that the task table can only be initialized once, if multiple
    // initialization is attempted, panic.
    if TASK_TABLE_INITIALIZED.swap(1, Ordering::Acquire) > 0 {
        panic!()
    }

    // Take an unsafe reference to the task table, and initialize essential
    // state for each task.
    {
        // Safety: No other reference can be taken to the table as
        // TASK_TABLE_LOCK starts with the lock taken and we ensure
        // init_task_table is only called once with TASK_TABLE_INITIALIZED.
        let table = unsafe { &mut TASK_TABLE };
        for (idx, task) in table.0.iter_mut().enumerate() {
            task.index = idx as u8;

            // This task should be set to the Ready state on startup.
            if task.descriptor().flags.contains(Flags::BOOT) {
                task.reset();
            }
        }

        // Set the highest priority task as the current task.
        get_preferred_task(table).set_as_current();
    };

    // Clear the task table lock now that initialization is complete.
    TASK_TABLE_LOCK.store(0, Ordering::Release);
}

pub fn with_task_table<F, T>(f: F) -> T
where
    F: FnOnce(&mut TaskTable) -> T,
{
    if TASK_TABLE_LOCK.swap(1, Ordering::Acquire) > 0 {
        panic!();
    }

    // Create the reference and call the closure within a block, this ensures
    // that the table is not referenced after the lock is released.
    let result = {
        // Safety: We ensure that there is no concurrent reference to TASK_TABLE
        // with TASK_TABLE_LOCK, so we can safely take a mutable reference.
        let table = unsafe { &mut TASK_TABLE };
        f(table)
    };

    TASK_TABLE_LOCK.store(0, Ordering::Release);

    result
}

fn priority_scan<F, T>(task_table: &mut TaskTable, f: &mut F) -> Option<T>
where
    F: FnMut(&mut Task) -> Option<T>,
{
    // Get an array of task indices sorted by current priority.
    let mut task_idx_by_priority: [_; NUM_TASKS] = from_fn(|i| TaskId::new(i as u8).unwrap());
    task_idx_by_priority.sort_unstable_by_key(|i| task_table[*i].current_priority);

    for idx in task_idx_by_priority {
        if let Some(result) = f(&mut task_table[idx]) {
            return Some(result);
        }
    }

    None
}

pub fn get_preferred_task(task_table: &mut TaskTable) -> &mut Task {
    let next_task_idx = priority_scan(task_table, &mut |task| match task.state {
        TaskState::Ready => Some(task.index()),
        _ => None,
    })
    .expect("No runnable task.");

    &mut task_table[next_task_idx]
}

fn is_valid_target(caller_idx: TaskId, target_idx: TaskId) -> bool {
    // The target must be a different task.
    if target_idx == caller_idx {
        return false;
    }

    return true;
}

fn inherit_priority(task_table: &mut TaskTable, caller_idx: TaskId, target_idx: TaskId) {
    let mut caller_idx = caller_idx;
    let mut target_idx = target_idx;

    loop {
        let (caller, target) = task_table.get_pair_mut(caller_idx, target_idx);

        if caller.current_priority < target.current_priority {
            target.current_priority = caller.current_priority;

            if let TaskState::CallRequest(x) = target.state() {
                caller_idx = target_idx;
                target_idx = x;
            }
        } else {
            return;
        }
    }
}

fn recalculate_priority(task_table: &mut TaskTable, task_idx: TaskId) -> u8 {
    let task = &mut task_table[task_idx];
    let mut priority = task.descriptor().priority;

    for task in task_table.0.iter() {
        // If some task targeted this task with a SYS_CALL we inherit their
        // priority.
        if task.state() == TaskState::CallRequest(task_idx) {
            priority = priority.min(task.current_priority);
        }
    }

    let task = &mut task_table[task_idx];
    task.current_priority = priority;

    priority
}

fn deliver_call(caller: &mut Task, target: &mut Task) {
    debug_assert!(caller.state() == TaskState::CallRequest(target.index()));
    debug_assert!(target.state() == TaskState::Receive);

    syscall::set_receive_result(target, Some(caller));

    // The caller has sent the message to target, we must now wait for the
    // response.
    caller.set_state(TaskState::CallResponse(target.index()));

    // The target can now be unblocked.
    target.set_state(TaskState::Ready);
}

pub fn do_panic(task_table: &mut TaskTable, caller_idx: TaskId) -> Schedule {
    let caller = &mut task_table[caller_idx];

    caller.set_state(TaskState::Fatal);
    Schedule::Other
}

pub fn do_receive(task_table: &mut TaskTable, caller_idx: TaskId) -> Schedule {
    let caller = &mut task_table[caller_idx];

    caller.set_state(TaskState::Receive);

    // Scan tasks by priority to see if any task is waiting to call us.
    let target_idx = caller_idx;
    let highest_caller = priority_scan(task_table, &mut |task| {
        if task.state() == TaskState::CallRequest(target_idx) {
            Some(task.index())
        } else {
            None
        }
    });

    if let Some(caller_idx) = highest_caller {
        let (caller, target) = task_table.get_pair_mut(caller_idx, target_idx);

        deliver_call(caller, target);

        // This SYS_RECEIVE didn't block - we must still be the highest
        // priority task.
        return Schedule::Same;
    } else {
        // If there are notifications pending, then this can immediately
        // unblock.
        let target = &mut task_table[target_idx];

        if target.post(0) {
            return Schedule::Same;
        } else {
            return Schedule::Other;
        }
    }
}

pub fn do_send(task_table: &mut TaskTable, caller_idx: TaskId, target_idx: TaskId) -> Schedule {
    if !is_valid_target(caller_idx, target_idx) {
        return do_panic(task_table, caller_idx);
    }

    let target = &mut task_table[target_idx];

    if let TaskState::CallResponse(blocked_on) = target.state() {
        // The target must be waiting for this task's response.
        if blocked_on == caller_idx {
            let (caller, target) = task_table.get_pair_mut(caller_idx, target_idx);

            syscall::set_call_result(target, caller);
            target.set_state(TaskState::Ready);

            // Our priority may have been raised when the target issued the,
            // SYS_CALL - we must ensure it is reverted.
            let target_priority = target.current_priority;
            let caller_priority = recalculate_priority(task_table, caller_idx);

            if caller_priority < target_priority {
                Schedule::Same
            } else {
                Schedule::Exactly(target_idx)
            }
        } else {
            // Attempted to send to a task that was not expecting a
            // response from this task.
            return do_panic(task_table, caller_idx);
        }
    } else {
        // The target is not in the correct state.
        return do_panic(task_table, caller_idx);
    }
}

pub fn do_call(task_table: &mut TaskTable, caller_idx: TaskId, target_idx: TaskId) -> Schedule {
    if !is_valid_target(caller_idx, target_idx) {
        return do_panic(task_table, caller_idx);
    }

    let (caller, target) = task_table.get_pair_mut(caller_idx, target_idx);

    caller.set_state(TaskState::CallRequest(target_idx));

    let schedule = match target.state() {
        // We are currently the highest priority task, and the target just
        // inherited our priority so it must now be the highest priority
        // task.
        TaskState::Receive => {
            deliver_call(caller, target);
            Schedule::Exactly(target_idx)
        }
        TaskState::Ready => Schedule::Exactly(target_idx),

        _ => Schedule::Other,
    };

    // As we are now dependent on target, it should inherit our priority,
    // it must revert to it's original priority before we are unblocked.
    inherit_priority(task_table, caller_idx, target_idx);

    schedule
}

pub fn do_notify(
    task_table: &mut TaskTable,
    caller_idx: TaskId,
    target_idx: TaskId,
    notifications: u32,
) -> Schedule {
    if !is_valid_target(caller_idx, target_idx) {
        return do_panic(task_table, caller_idx);
    }

    let target = &mut task_table[target_idx];

    if target.post(notifications) {
        let (caller, target) = task_table.get_pair_mut(caller_idx, target_idx);

        if target.current_priority < caller.current_priority {
            Schedule::Exactly(target_idx)
        } else {
            Schedule::Same
        }
    } else {
        Schedule::Same
    }
}

pub fn do_set_timer(
    task_table: &mut TaskTable,
    caller_idx: TaskId,
    periodic: bool,
    deadline: u32,
) -> Schedule {
    // Periodic timers require a non-zero period.
    if periodic && (deadline == 0) {
        return do_panic(task_table, caller_idx);
    }

    let caller = &mut task_table[caller_idx];

    // A zero deadline cancels the timer, any non zero value sets it.
    caller.set_timer(periodic, NonZeroU32::new(deadline));

    // Setting a timer can't itself cause a reschedule - only a later timer
    // interrupt can.
    Schedule::Same
}

pub fn do_interrupt_control(
    task_table: &mut TaskTable,
    caller_idx: TaskId,
    interrupt: usize,
    control: InterruptControl,
) -> Schedule {
    // There must be a valid interrupt descriptor for the given interrupt.
    if let Some(descriptor) = get_interrupt_descriptor(interrupt) {
        // The interrupt must be owned by the calling task.
        if u8::from(caller_idx) != descriptor.task_id() {
            do_panic(task_table, caller_idx)
        } else {
            // Enable/disable the given interrupt.
            arch::interrupt_control(interrupt, control);

            // This call cannot cause a reschedule.
            Schedule::Same
        }
    } else {
        do_panic(task_table, caller_idx)
    }
}

pub fn evaluate_timers(task_table: &mut TaskTable, caller_idx: TaskId, now_ticks: u64) -> Schedule {
    let mut current_priority = task_table[caller_idx].current_priority;
    let mut sched = Schedule::Same;

    for task in task_table.0.iter_mut() {
        let unblocked = task.evaluate_timer(now_ticks);

        // If this task was unblocked, is not the current task and has higher
        // priority it should be scheduled.
        if unblocked && task.index() != caller_idx && task.current_priority < current_priority {
            current_priority = task.current_priority;
            sched = Schedule::Exactly(task.index());
        }
    }

    sched
}

#[rtos_import]
static INTERRUPT_DESCRIPTOR_TABLE: InterruptDescriptor;
#[rtos_import]
static INTERRUPT_MIN: usize;
#[rtos_import]
static INTERRUPT_COUNT: usize;

pub struct InterruptDescriptorIterator(usize);

impl Iterator for InterruptDescriptorIterator {
    type Item = (usize, &'static InterruptDescriptor);

    fn next(&mut self) -> Option<Self::Item> {
        // Safety: This reads two static immutable constants, this is a safe
        // operation.
        let interrupt_count = unsafe { INTERRUPT_COUNT };
        let interrupt_min = unsafe { INTERRUPT_MIN };

        // Safety: This gets a pointer to an extern static, as a pointer cannot
        // be used safely, this itself is safe.
        let table = unsafe { &INTERRUPT_DESCRIPTOR_TABLE as *const InterruptDescriptor };

        while self.0 < interrupt_count {
            let interrupt_num = self.0 + interrupt_min;

            // Safety: This gets a reference to an element within the interrupt
            // descriptor table. We know this element is in bounds and all
            // references to this table are immutable - thus this is safe.
            let descriptor = unsafe { &*(table.add(self.0)) };
            self.0 += 1;

            if descriptor.task_id() != u8::MAX {
                return Some((interrupt_num, descriptor));
            }
        }

        None
    }
}

fn interrupt_descriptor_iter() -> InterruptDescriptorIterator {
    InterruptDescriptorIterator(0)
}

fn get_interrupt_descriptor(interrupt: usize) -> Option<&'static InterruptDescriptor> {
    // Safety: This reads two static immutable constants, this is a safe
    // operation.
    let interrupt_count = unsafe { INTERRUPT_COUNT };
    let interrupt_min = unsafe { INTERRUPT_MIN };

    if interrupt < interrupt_min {
        return None;
    }

    let table_idx = interrupt - interrupt_min;
    if table_idx > interrupt_count {
        return None;
    }

    // Safety: The index must be in bounds - this is checked explicitly above.
    let descriptor = unsafe {
        let table = &INTERRUPT_DESCRIPTOR_TABLE as *const InterruptDescriptor;
        &*(table.add(table_idx))
    };

    if descriptor.task_id() == u8::MAX {
        return None;
    }

    Some(descriptor)
}

pub fn handle_interrupt(
    task_table: &mut TaskTable,
    caller_idx: TaskId,
    interrupt: usize,
) -> Schedule {
    let descriptor = get_interrupt_descriptor(interrupt).unwrap();

    let target_idx = TaskId::new(descriptor.task_id()).unwrap();
    let notification = descriptor.notification();

    if task_table[target_idx].post(notification) && target_idx != caller_idx {
        let (caller, target) = task_table.get_pair_mut(caller_idx, target_idx);

        if target.current_priority < caller.current_priority {
            Schedule::Exactly(target_idx)
        } else {
            Schedule::Same
        }
    } else {
        Schedule::Same
    }
}

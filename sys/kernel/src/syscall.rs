pub use kernel_types::syscall::*;
use zerocopy::{AsBytes, FromBytes, FromZeroes, Ref};

use crate::{arch, task};

#[repr(C)]
#[derive(AsBytes, FromBytes, FromZeroes)]
pub struct SysRegisters {
    id: abi::SysCallId,
    regs: [usize; arch::SavedContext::SYS_REGISTER_COUNT - 1],
}

impl SysRegisters {
    #[inline]
    fn id(&self) -> abi::SysCallId {
        self.id
    }

    #[inline]
    fn input<T>(&self) -> &T
    where
        T: FromBytes,
    {
        Ref::<_, T>::new_from_prefix(self.regs.as_bytes())
            .unwrap()
            .0
            .into_ref()
    }

    #[inline]
    fn output<T>(&mut self) -> &mut T
    where
        T: FromBytes + AsBytes,
    {
        Ref::<_, T>::new_from_prefix(self.regs.as_bytes_mut())
            .unwrap()
            .0
            .into_mut()
    }
}

// Panic and enter the fatal state.
// fn SYS_PANIC() -> !
fn do_sys_panic(task_table: &mut task::TaskTable, caller_idx: task::TaskId) -> task::Schedule {
    task::do_panic(task_table, caller_idx)
}

#[repr(C)]
#[derive(AsBytes, FromBytes, FromZeroes)]
struct SysReceiveInput {
    out_capacity: u8,
    _pad: [u8; 3],
}

#[repr(C)]
#[derive(AsBytes, FromBytes, FromZeroes)]
struct SysReceiveOutput {
    sender: u8,
    _pad0: u8,
    len: u8,
    _pad1: u8,
    notifications: u32,
    data: [usize; abi::MAX_MESSAGE_SIZE],
}

// Wait for a message or notification.
// fn SYS_RECEIVE() -> (sender: u8, len: u8, notifications: u32, data: [u8; len])
fn do_sys_receive(task_table: &mut task::TaskTable, caller: task::TaskId) -> task::Schedule {
    task::do_receive(task_table, caller)
}

pub fn set_receive_result(target: &mut task::Task, caller: Option<&task::Task>) {
    let notifications = target.notifications();
    target.reset_notifications();

    let target_input = target.context().sys_registers().input::<SysReceiveInput>();
    let out_capacity = target_input.out_capacity;

    let target_output = target
        .context_mut()
        .sys_registers_mut()
        .output::<SysReceiveOutput>();

    target_output.notifications = notifications;

    if let Some(sender) = caller {
        let caller_input = sender.context().sys_registers().input::<SysCallInput>();
        let in_len = caller_input.in_len;

        let len = out_capacity.min(in_len) as usize;

        target_output.sender = sender.index().into();
        target_output.len = len as u8;
        target_output.data[..len].copy_from_slice(&caller_input.data[..len]);
    } else {
        target_output.sender = u8::MAX;
        target_output.len = 0;
    }
}

#[repr(C)]
#[derive(AsBytes, FromBytes, FromZeroes)]
struct SysSendInput {
    target: u8,
    _pad0: u8,
    in_len: u8,
    _pad1: [u8; 5],
    data: [usize; abi::MAX_MESSAGE_SIZE],
}

// Send a response to a task, must be a task that sent a message with SYS_CALL.
// fn SYS_SEND(target: u8, len: u8, data: [u8; len])
fn do_sys_send(task_table: &mut task::TaskTable, caller_idx: task::TaskId) -> task::Schedule {
    let caller = &mut task_table[caller_idx];
    let input = caller.context().sys_registers().input::<SysSendInput>();

    if let Some(target_idx) = task::TaskId::new(input.target) {
        // The message length must be in bounds abi::MAX_MESSAGE_SIZE.
        if input.in_len as usize > abi::MAX_MESSAGE_SIZE {
            return task::do_panic(task_table, caller_idx);
        }

        task::do_send(task_table, caller_idx, target_idx)
    } else {
        task::do_panic(task_table, caller_idx)
    }
}

#[repr(C)]
#[derive(AsBytes, FromBytes, FromZeroes)]
struct SysCallInput {
    target: u8,
    _pad0: u8,
    in_len: u8,
    out_capacity: u8,
    _pad1: u32,
    data: [usize; abi::MAX_MESSAGE_SIZE],
}

#[repr(C)]
#[derive(AsBytes, FromBytes, FromZeroes)]
struct SysCallOutput {
    len: u8,
    _pad: [u8; 7],
    data: [usize; abi::MAX_MESSAGE_SIZE],
}

// Make a remote call to another task.
// fn SYS_CALL(target: u8, len: u8, data: [u8; len]) -> (len: u8, data [u8; len])
fn do_sys_call(task_table: &mut task::TaskTable, caller_idx: task::TaskId) -> task::Schedule {
    let caller = &mut task_table[caller_idx];
    let input = caller.context().sys_registers().input::<SysCallInput>();

    if let Some(target_idx) = task::TaskId::new(input.target) {
        // The message length must be in bounds abi::MAX_MESSAGE_SIZE.
        if input.in_len as usize > abi::MAX_MESSAGE_SIZE {
            return task::do_panic(task_table, caller_idx);
        }

        task::do_call(task_table, caller_idx, target_idx)
    } else {
        task::do_panic(task_table, caller_idx)
    }
}

pub fn set_call_result(target: &mut task::Task, caller: &task::Task) {
    let target_input = target.context().sys_registers().input::<SysCallInput>();
    let caller_input = caller.context().sys_registers().input::<SysSendInput>();

    let out_capacity = target_input.out_capacity;
    let in_len = caller_input.in_len;

    let target_output = target
        .context_mut()
        .sys_registers_mut()
        .output::<SysCallOutput>();

    let len = out_capacity.min(in_len) as usize;

    target_output.len = len as u8;
    target_output.data[..len].copy_from_slice(&caller_input.data[..len]);
}

#[repr(C)]
#[derive(AsBytes, FromBytes, FromZeroes)]
struct SysNotifyInput {
    target: u8,
    _pad: [u8; 3],
    notifications: u32,
}

// Notify another task asynchronously.
// fn SYS_NOTIFY(target: u8, notifications: u32)
fn do_sys_notify(task_table: &mut task::TaskTable, caller_idx: task::TaskId) -> task::Schedule {
    let caller = &mut task_table[caller_idx];
    let input = caller.context().sys_registers().input::<SysNotifyInput>();

    if let Some(target_idx) = task::TaskId::new(input.target) {
        let notifications = input.notifications;
        task::do_notify(task_table, caller_idx, target_idx, notifications)
    } else {
        task::do_panic(task_table, caller_idx)
    }
}

#[repr(C)]
#[derive(AsBytes, FromBytes, FromZeroes)]
struct SysRequestTimerInput {
    periodic: u8,
    _pad: [u8; 3],
    deadline: u32,
}

// Request a timer notification at the given deadline or period, will override
// any previous timer configuration.
// fn SYS_SET_TIMER(relative: bool, deadline: u32)
fn do_sys_set_timer(task_table: &mut task::TaskTable, caller_idx: task::TaskId) -> task::Schedule {
    let caller = &mut task_table[caller_idx];
    let input = caller
        .context()
        .sys_registers()
        .input::<SysRequestTimerInput>();

    let periodic = input.periodic > 0;
    let deadline = input.deadline;

    task::do_set_timer(task_table, caller_idx, periodic, deadline)
}

#[repr(C)]
#[derive(AsBytes, FromBytes, FromZeroes)]
struct SysInterruptControlInput {
    interrupt: usize,
    control: abi::InterruptControl,
}

// Enables or disables a given interrupt. After an interrupt has been received
// it must be re-enabled.
// fn SYS_INTERRUPT_CONTROL(interrupt: usize, control: u32)
fn do_sys_interrupt_control(
    task_table: &mut task::TaskTable,
    caller_idx: task::TaskId,
) -> task::Schedule {
    let caller = &mut task_table[caller_idx];
    let input = caller
        .context()
        .sys_registers()
        .input::<SysInterruptControlInput>();

    let interrupt = input.interrupt;
    if let Ok(control) = input.control.try_into() {
        task::do_interrupt_control(task_table, caller_idx, interrupt, control)
    } else {
        do_sys_panic(task_table, caller_idx)
    }
}

pub fn handle_syscall(
    task_table: &mut task::TaskTable,
    caller_idx: task::TaskId,
) -> task::Schedule {
    let caller = &mut task_table[caller_idx];
    caller.context_mut().sys_advance_pc();

    match caller.context().sys_registers().id() {
        abi::SysCallId::Panic => do_sys_panic(task_table, caller_idx),
        abi::SysCallId::Receive => do_sys_receive(task_table, caller_idx),
        abi::SysCallId::Send => do_sys_send(task_table, caller_idx),
        abi::SysCallId::Call => do_sys_call(task_table, caller_idx),
        abi::SysCallId::Notify => do_sys_notify(task_table, caller_idx),
        abi::SysCallId::SetTimer => do_sys_set_timer(task_table, caller_idx),
        abi::SysCallId::InterruptControl => do_sys_interrupt_control(task_table, caller_idx),
        _ => do_sys_panic(task_table, caller_idx),
    }
}

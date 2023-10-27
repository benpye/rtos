use core::{ffi::c_void, panic};

pub use kernel_types::arch::riscv::*;
use memoffset::offset_of;
use riscv::register;
use rtos_macros::rtos_feature;
use zerocopy::{AsBytes, FromBytes, FromZeroes, Ref};

use crate::{init, syscall, task, time};

#[cfg(feature = "riscv_aclint")]
mod aclint;
#[cfg(feature = "riscv_plic")]
mod plic;
#[cfg(feature = "riscv_wch_pfic")]
mod wch_pfic;
#[cfg(feature = "riscv_wch_systick")]
mod wch_systick;

#[cfg(feature = "riscv_aclint")]
use aclint::{clear_software_interrupt, clear_timer_interrupt, set_software_interrupt};
#[cfg(feature = "riscv_aclint")]
pub use aclint::{now_ticks, set_timer_deadline, timer_deadline};
#[cfg(feature = "riscv_plic")]
use plic::handle_interrupt;
#[cfg(feature = "riscv_plic")]
pub use plic::{interrupt_control, reset_interrupt};
#[cfg(feature = "riscv_wch_pfic")]
use wch_pfic::{
    clear_software_interrupt, clear_timer_interrupt, handle_interrupt, set_software_interrupt,
};
#[cfg(feature = "riscv_wch_pfic")]
pub use wch_pfic::{interrupt_control, reset_interrupt};
#[cfg(feature = "riscv_wch_systick")]
pub use wch_systick::{now_ticks, set_timer_deadline, timer_deadline};

rtos_feature!("family_generic");
rtos_feature!("family_wch_v4c");

#[repr(C)]
#[derive(AsBytes, FromBytes, FromZeroes)]
pub struct SavedContext {
    // Syscall in/out - 26 * sizeof(usize) = 104
    // a0-a5, s0, s1 can be accessed with compressed instructions so we use
    // those for the smallest inputs
    a0: usize,
    a1: usize,
    a2: usize,
    a3: usize,
    a4: usize,
    a5: usize,

    a6: usize,
    a7: usize,

    t0: usize,
    t1: usize,
    t2: usize,
    t3: usize,
    t4: usize,
    t5: usize,
    t6: usize,

    ra: usize,

    s2: usize,
    s3: usize,
    s4: usize,
    s5: usize,
    s6: usize,
    s7: usize,
    s8: usize,
    s9: usize,
    s10: usize,
    s11: usize,

    // These registers are not used for syscalls

    // pc and sp must be preserved.
    pc: usize,
    sp: usize,

    // s0 and s1 are used by LLVM and are disallowed in inline asm.
    s0: usize,
    s1: usize,

    // gp and tp have architectural uses but in this application are platform
    // reserved. We currently do not use them.
    gp: usize,
    tp: usize,
}

// We expect 26 syscall registers.
const _: () = assert!(SavedContext::SYS_REGISTER_COUNT == 26);

impl SavedContext {
    pub const fn zeroed() -> Self {
        Self {
            pc: 0,
            sp: 0,
            ra: 0,
            t0: 0,
            t1: 0,
            t2: 0,
            t3: 0,
            t4: 0,
            t5: 0,
            t6: 0,
            a0: 0,
            a1: 0,
            a2: 0,
            a3: 0,
            a4: 0,
            a5: 0,
            a6: 0,
            a7: 0,
            gp: 0,
            tp: 0,
            s0: 0,
            s1: 0,
            s2: 0,
            s3: 0,
            s4: 0,
            s5: 0,
            s6: 0,
            s7: 0,
            s8: 0,
            s9: 0,
            s10: 0,
            s11: 0,
        }
    }

    pub fn task_reset(&mut self, descriptor: &task::TaskDescriptor) {
        // The task is responsible for ensuring that all other register state
        // is correct, we only need to set pc.
        self.pc = descriptor.init_pc.into();
    }

    // Registers from pc in the saved context are not usable in syscalls.
    pub const SYS_REGISTER_COUNT: usize =
        offset_of!(SavedContext, pc) / core::mem::size_of::<usize>();

    #[inline]
    pub fn sys_registers(&self) -> &syscall::SysRegisters {
        Ref::<_, syscall::SysRegisters>::new_from_prefix(self.as_bytes())
            .unwrap()
            .0
            .into_ref()
    }

    #[inline]
    pub fn sys_registers_mut(&mut self) -> &mut syscall::SysRegisters {
        Ref::<_, syscall::SysRegisters>::new_from_prefix(self.as_bytes_mut())
            .unwrap()
            .0
            .into_mut()
    }

    #[inline]
    pub fn sys_advance_pc(&mut self) {
        self.pc += 4;
    }
}

#[inline]
pub fn apply_memory_protection(task: &task::Task) {
    let pmp_addr = &task.descriptor().arch.pmp_addr;
    let pmp_cfg = task.descriptor().arch.pmp_cfg;

    register::pmpcfg0::write(pmp_cfg as usize);
    register::pmpaddr0::write(pmp_addr[0] as usize);
    register::pmpaddr1::write(pmp_addr[1] as usize);
    register::pmpaddr2::write(pmp_addr[2] as usize);
    register::pmpaddr3::write(pmp_addr[3] as usize);
}

// # Safety
// This stores a pointer that aliases task, though this is not used outside of
// kernel entry/exit.
#[inline]
pub unsafe fn set_current_task(task: &task::Task) {
    register::mscratch::write(task as *const _ as usize);
}

pub fn enter_first_task() -> ! {
    // Safety: We set mpp to user and then jump to the task entry code in the
    // trap vector. This may fault and return to kernel mode, but is safe as it
    // can not overwrite kernel memory.
    unsafe {
        register::mstatus::set_mpp(register::mstatus::MPP::User);
        register::mstatus::set_mpie();

        core::arch::asm!(
            // trap_vector_full_restore expects a0 = full_restore, which must
            // be set in this case
            "li a0, 1",
            "j trap_vector_full_restore",
            options(noreturn)
        )
    }
}

#[repr(usize)]
enum RestoreContext {
    Partial = 0,
    Full = 1,
}

#[allow(dead_code)]
mod mcause {
    pub const INTERRUPT_BIT: usize = 0x80000000;

    #[cfg(not(feature = "family_wch_v4c"))]
    pub const MACHINE_SOFTWARE_INTERRUPT: usize = 0x80000003;
    #[cfg(not(feature = "family_wch_v4c"))]
    pub const MACHINE_TIMER_INTERRUPT: usize = 0x80000007;
    #[cfg(not(feature = "family_wch_v4c"))]
    pub const MACHINE_EXTERNAL_INTERRUPT: usize = 0x8000000b;

    // The WCH parts use non standard interrupt numbering.
    #[cfg(feature = "family_wch_v4c")]
    pub const MACHINE_TIMER_INTERRUPT: usize = 0x8000000c;
    #[cfg(feature = "family_wch_v4c")]
    pub const MACHINE_SOFTWARE_INTERRUPT: usize = 0x8000000e;
    #[cfg(feature = "family_wch_v4c")]
    pub const EXTERNAL_INTERRUPT_BASE: usize = 0x80000010;

    pub const INSTRUCTION_ADDRESS_MISALIGNED: usize = 0x00000000;
    pub const INSTRUCTION_ACCESS_FAULT: usize = 0x00000001;
    pub const ILLEGAL_INSTRUCTION: usize = 0x00000002;
    pub const BREAKPOINT: usize = 0x00000003;
    pub const LOAD_ADDRESS_MISALIGNED: usize = 0x00000004;
    pub const LOAD_ACCESS_FAULT: usize = 0x00000005;
    pub const STORE_AMO_ADDRESS_MISALIGNED: usize = 0x00000006;
    pub const STORE_AMO_ACCESS_FAULT: usize = 0x00000007;
    pub const ENVIRONMENT_CALL_FROM_U_MODE: usize = 0x00000008;
    pub const ENVIRONMENT_CALL_FROM_S_MODE: usize = 0x00000009;
    pub const ENVIRONMENT_CALL_FROM_M_MODE: usize = 0x0000000b;
    pub const INSTRUCTION_PAGE_FAULT: usize = 0x0000000c;
    pub const LOAD_PAGE_FAULT: usize = 0x0000000d;
    pub const STORE_AMO_PAGE_FAULT: usize = 0x0000000f;

    #[inline]
    pub const fn is_interrupt(cause: usize) -> bool {
        cause & INTERRUPT_BIT != 0
    }

    #[inline]
    pub const fn is_exception(cause: usize) -> bool {
        !is_interrupt(cause)
    }
}

// # Safety
// - This should only be called from the assembly trap vector.
unsafe fn handle_kernel_exception(cause: usize) -> ! {
    match cause {
        _ => panic!("kernel exception {:x}", cause),
    }
}

// # Safety
// - This should only be called from the assembler trap vector.
// - The task pointer must be non-null and correctly aligned.
unsafe fn handle_trap(task: *mut task::Task, cause: usize) -> RestoreContext {
    // Safety: This creates a reference to the task which aliases the global
    // table, however it used only to get the index which will not change post
    // init, and the reference is immediately discarded.
    let task_idx = unsafe { task::Task::index(&*task) };

    task::with_task_table(|task_table| {
        let schedule = match cause {
            // Syscall from a task in user mode.
            mcause::ENVIRONMENT_CALL_FROM_U_MODE => syscall::handle_syscall(task_table, task_idx),
            // Machine software interrupt - used to reschedule tasks in
            // response to other interrupts.
            mcause::MACHINE_SOFTWARE_INTERRUPT => {
                clear_software_interrupt();

                // If a machine software interrupt fires we should always
                // scan and schedule the highest priority task.
                task::Schedule::Other
            }
            mcause::MACHINE_TIMER_INTERRUPT => {
                clear_timer_interrupt();

                time::handle_timer_expiration(task_table, task_idx)
            }
            #[cfg(not(feature = "family_wch_v4c"))]
            mcause::MACHINE_EXTERNAL_INTERRUPT => handle_interrupt(cause, task_table, task_idx),
            #[cfg(feature = "family_wch_v4c")]
            mcause::EXTERNAL_INTERRUPT_BASE.. => handle_interrupt(cause, task_table, task_idx),
            mcause::INTERRUPT_BIT.. => panic!("interrupt"),
            _ => panic!("exception"),
        };

        // It is possible to schedule a new task in response to any exception,
        // or in response to a machine software interrupt.
        let can_switch_task =
            mcause::is_exception(cause) || (cause == mcause::MACHINE_SOFTWARE_INTERRUPT);

        match (schedule, can_switch_task) {
            // We can always schedule the same task.
            (task::Schedule::Same, _) => {
                task_table[task_idx].set_as_current();

                // Ecall always requires a full restore as syscalls may return
                // data in any register.
                if cause == mcause::ENVIRONMENT_CALL_FROM_U_MODE {
                    RestoreContext::Full
                } else {
                    RestoreContext::Partial
                }
            }

            // We know exactly which new task to schedule.
            (task::Schedule::Exactly(new_task_idx), true) => {
                assert!(new_task_idx != task_idx.into());
                task_table[new_task_idx].set_as_current();

                RestoreContext::Full
            }

            // We don't know which new task to schedule and need to do a
            // priority scan.
            (task::Schedule::Other, true) => {
                let new_task = task::get_preferred_task(task_table);
                new_task.set_as_current();

                RestoreContext::Full
            }

            // We need to schedule some other task and currently can't - so we
            // need to trigger a software interrupt.
            (_, false) => {
                // Schedule the same task but request a software interrupt
                // to reschedule on.
                task_table[task_idx].set_as_current();
                set_software_interrupt();

                RestoreContext::Partial
            }
        }
    })
}

#[repr(align(4))]
#[naked]
unsafe extern "C" fn trap_vector() -> ! {
    // TODO: We could skip save/restore for gp/tp as we don't use the gp
    // relaxation and so these are reserved for platform use.

    extern "C" {
        static _stack_end: c_void;
    }

    macro_rules! register_offset {
        ($reg:tt) => {{
            crate::task::Task::CONTEXT_OFFSET + ::memoffset::offset_of!(SavedContext, $reg)
        }};
    }

    // Allow named assembler labels here for the trap_vector_full_restore
    // label, we want to go through the full restore for the initial task entry
    #[allow(named_asm_labels)]
    unsafe {
        core::arch::asm!(
        // Get task pointer from mscratch
        "csrrw a0, mscratch, a0",

        // If the task pointer is zero we've taken an exception before setting
        // the next task.
        "beq a0, zero, 3f", // kernel_exception

        // Save stack pointer, this is callee saved but we necessarily trash it
        "sw sp, {task_sp}(a0)",

        // Save caller saved registers
        "sw ra, {task_ra}(a0)",

        "sw a1, {task_a1}(a0)",
        "sw a2, {task_a2}(a0)",
        "sw a3, {task_a3}(a0)",
        "sw a4, {task_a4}(a0)",
        "sw a5, {task_a5}(a0)",
        "sw a6, {task_a6}(a0)",
        "sw a7, {task_a7}(a0)",

        // a0 was saved to mscratch upon entry, save a0 and clear mscratch
        "csrrw a1, mscratch, zero",
        "sw a1, {task_a0}(a0)",

        "sw t0, {task_t0}(a0)",
        "sw t1, {task_t1}(a0)",
        "sw t2, {task_t2}(a0)",
        "sw t3, {task_t3}(a0)",
        "sw t4, {task_t4}(a0)",
        "sw t5, {task_t5}(a0)",
        "sw t6, {task_t6}(a0)",

        // Switch to the kernel stack
        "la sp, {kernel_top_of_stack}",

        // Check if we need to save the full context.
        "csrr a1, mcause",

        // First, check if the interrupt bit is zero, if so we need to perform
        // a full save as this is an exception.
        "srli a2, a1, 31",
        "beqz a2, 2f", // full_save:

        // Second, check if it's a machine software interrupt, if it is then we
        // also need to save the full context.
        "li t0, {mcause_machine_software_interrupt}",
        "beq a1, t0, 2f", // full_save:

        // handle_trap(task: a0, mcause: a1) -> (full_switch: a0)
        "jal {handle_trap}",

        // handle_trap must return zero in this case as we did not perform a
        // full save, so we don't need to check it.

        // Get the new task pointer
        "csrr a2, mscratch",

        "1:", // partial_restore:
        // Switch back to user stack
        "lw sp, {task_sp}(a2)",

        // Restore caller saved registers
        "lw ra, {task_ra}(a2)",

        "lw t0, {task_t0}(a2)",
        "lw t1, {task_t1}(a2)",
        "lw t2, {task_t2}(a2)",
        "lw t3, {task_t3}(a2)",
        "lw t4, {task_t4}(a2)",
        "lw t5, {task_t5}(a2)",
        "lw t6, {task_t6}(a2)",

        "lw a0, {task_a0}(a2)",
        "lw a1, {task_a1}(a2)",
        // Restore a2 last as it is used for the task pointer
        "lw a3, {task_a3}(a2)",
        "lw a4, {task_a4}(a2)",
        "lw a5, {task_a5}(a2)",
        "lw a6, {task_a6}(a2)",
        "lw a7, {task_a7}(a2)",

        "lw a2, {task_a2}(a2)",

        // Return from interrupt
        "mret",

        "2:", // full_save:
        // Save the remaining registers
        "sw gp, {task_gp}(a0)",
        "sw tp, {task_tp}(a0)",

        "sw s0, {task_s0}(a0)",
        "sw s1, {task_s1}(a0)",
        "sw s2, {task_s2}(a0)",
        "sw s3, {task_s3}(a0)",
        "sw s4, {task_s4}(a0)",
        "sw s5, {task_s5}(a0)",
        "sw s6, {task_s6}(a0)",
        "sw s7, {task_s7}(a0)",
        "sw s8, {task_s8}(a0)",
        "sw s9, {task_s9}(a0)",
        "sw s10, {task_s10}(a0)",
        "sw s11, {task_s11}(a0)",

        "csrr a2, mepc",
        "sw a2, {task_pc}(a0)",

        // handle_trap(task: a0, mcause: a1) -> (full_switch: a0)
        "jal {handle_trap}",

        "trap_vector_full_restore:",
        // Get the new task pointer
        "csrr a2, mscratch",

        // Always restore mepc here, we need to advance pc on an ecall even if
        // we didn't switch task.
        "lw a3, {task_pc}(a2)",
        "csrw mepc, a3",

        // Check if we can avoid the full context restore
        "beq a0, zero, 1b", // partial_restore:

        // Restore callee saved registers
        "lw gp, {task_gp}(a2)",
        "lw tp, {task_tp}(a2)",
        "lw s0, {task_s0}(a2)",
        "lw s1, {task_s1}(a2)",
        "lw s2, {task_s2}(a2)",
        "lw s3, {task_s3}(a2)",
        "lw s4, {task_s4}(a2)",
        "lw s5, {task_s5}(a2)",
        "lw s6, {task_s6}(a2)",
        "lw s7, {task_s7}(a2)",
        "lw s8, {task_s8}(a2)",
        "lw s9, {task_s9}(a2)",
        "lw s10, {task_s10}(a2)",
        "lw s11, {task_s11}(a2)",

        // Continue with rest of context restore
        "j 1b", // partial_restore:

        // Handle trap taken without a valid task set. We cannot save state as
        // there is no task. Set the stack so we can at least try to leave some
        // breadcrumbs, pass mcause to match handle_trap.
        "3:", // kernel_exception:
        "la sp, {kernel_top_of_stack}",
        "csrr a0, mcause",
        "j {handle_kernel_exception}",

        mcause_machine_software_interrupt = const mcause::MACHINE_SOFTWARE_INTERRUPT,

        task_pc = const register_offset!(pc),
        task_sp = const register_offset!(sp),

        task_ra = const register_offset!(ra),

        task_t0 = const register_offset!(t0),
        task_t1 = const register_offset!(t1),
        task_t2 = const register_offset!(t2),
        task_t3 = const register_offset!(t3),
        task_t4 = const register_offset!(t4),
        task_t5 = const register_offset!(t5),
        task_t6 = const register_offset!(t6),

        task_a0 = const register_offset!(a0),
        task_a1 = const register_offset!(a1),
        task_a2 = const register_offset!(a2),
        task_a3 = const register_offset!(a3),
        task_a4 = const register_offset!(a4),
        task_a5 = const register_offset!(a5),
        task_a6 = const register_offset!(a6),
        task_a7 = const register_offset!(a7),

        task_gp = const register_offset!(gp),
        task_tp = const register_offset!(tp),

        task_s0 = const register_offset!(s0),
        task_s1 = const register_offset!(s1),
        task_s2 = const register_offset!(s2),
        task_s3 = const register_offset!(s3),
        task_s4 = const register_offset!(s4),
        task_s5 = const register_offset!(s5),
        task_s6 = const register_offset!(s6),
        task_s7 = const register_offset!(s7),
        task_s8 = const register_offset!(s8),
        task_s9 = const register_offset!(s9),
        task_s10 = const register_offset!(s10),
        task_s11 = const register_offset!(s11),

        handle_trap = sym handle_trap,
        handle_kernel_exception = sym handle_kernel_exception,

        kernel_top_of_stack = sym _stack_end,
        options(noreturn));
    }
}

#[no_mangle]
#[naked]
#[cfg(feature = "family_wch_v4c")]
unsafe extern "C" fn _start() -> ! {
    extern "C" {
        static _data_start: c_void;
        static _data_end: c_void;
        static _data_load: c_void;

        static _bss_start: c_void;
        static _bss_end: c_void;

        static _stack_end: c_void;
    }

    unsafe {
        core::arch::asm!(
        // Set corecfgr (chicken bits?)
        "li a0, 0x1f",
        "csrw 0xbc0, a0",

        // Load .data from flash
        "la a0, {_data_start}",
        "la a1, {_data_end}",
        "la a2, {_data_load}",
        "bgeu a0, a1, 2f",
        "1:",
        "lw a3, (a2)",
        "sw a3, (a0)",
        "addi a0, a0, 4",
        "addi a2, a2, 4",
        "bltu a0, a1, 1b",
        "2:",

        // Zero .bss
        "la a0, {_bss_start}",
        "la a1, {_bss_end}",
        "bgeu a0, a1, 4f",
        "3:",
        "sw zero, (a0)",
        "addi a0, a0, 4",
        "bltu a0, a1, 3b",
        "4:",

        // Switch to the kernel stack
        "la sp, {_stack_end}",

        // Let's go!
        "j {kernel_init}",

        _data_start   = sym _data_start,
        _data_end     = sym _data_end,
        _data_load    = sym _data_load,
        _bss_start    = sym _bss_start,
        _bss_end      = sym _bss_end,
        _stack_end    = sym _stack_end,
        kernel_init   = sym init::kernel_init,
        options(noreturn));
    }
}

#[no_mangle]
#[naked]
#[cfg(not(feature = "family_wch_v4c"))]
unsafe extern "C" fn _start() -> ! {
    extern "C" {
        static _data_start: c_void;
        static _data_end: c_void;
        static _data_load: c_void;

        static _bss_start: c_void;
        static _bss_end: c_void;

        static _stack_end: c_void;
    }

    unsafe {
        core::arch::asm!(
        // Load .data from flash
        "la a0, {_data_start}",
        "la a1, {_data_end}",
        "la a2, {_data_load}",
        "bgeu a0, a1, 2f",
        "1:",
        "lw a3, (a2)",
        "sw a3, (a0)",
        "addi a0, a0, 4",
        "addi a2, a2, 4",
        "bltu a0, a1, 1b",
        "2:",

        // Zero .bss
        "la a0, {_bss_start}",
        "la a1, {_bss_end}",
        "bgeu a0, a1, 4f",
        "3:",
        "sw zero, (a0)",
        "addi a0, a0, 4",
        "bltu a0, a1, 3b",
        "4:",

        // Switch to the kernel stack
        "la sp, {_stack_end}",

        // Let's go!
        "j {kernel_init}",

        _data_start   = sym _data_start,
        _data_end     = sym _data_end,
        _data_load    = sym _data_load,
        _bss_start    = sym _bss_start,
        _bss_end      = sym _bss_end,
        _stack_end    = sym _stack_end,
        kernel_init   = sym init::kernel_init,
        options(noreturn));
    }
}

pub fn arch_init() {
    // Ensure that mscratch is zero before setting up interrupts, this ensures
    // that if we take a fault before the next task is set we don't write to
    // random memory attempting to save the context.
    register::mscratch::write(0);

    // Safety: Setting the trap vector isn't inherently unsafe, this should
    // always succeed.
    unsafe {
        register::mtvec::write(
            trap_vector as *const () as usize,
            register::utvec::TrapMode::Direct,
        );
    }

    // The WCH parts do not implement the MIE CSR.
    #[cfg(not(feature = "family_wch_v4c"))]
    {
        unsafe {
            register::mie::set_mtimer();
            register::mie::set_msoft();
            register::mie::set_mext();
        }
    }

    // Setting the deadline to u64::MAX will avoid timer interrupts firing.
    set_timer_deadline(u64::MAX);

    #[cfg(feature = "riscv_aclint")]
    aclint::aclint_init();

    #[cfg(feature = "riscv_plic")]
    plic::plic_init();

    #[cfg(feature = "riscv_wch_pfic")]
    wch_pfic::wch_pfic_init();

    #[cfg(feature = "riscv_wch_systick")]
    wch_systick::wch_systick_init();
}

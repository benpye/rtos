use pac_riscv::plic::Plic;
use rtos_macros::rtos_import;

use crate::arch::riscv::*;

#[rtos_import]
pub static mut PERIPHERAL_PLIC_BASE: u32;

pub fn plic_init() {
    // Set the threshold to zero - enables all interrupts.
    let plic = unsafe { Plic::from_ptr(&mut PERIPHERAL_PLIC_BASE as *mut _ as *mut _) };
    plic.threshold(0).write_value(0);
}

pub fn reset_interrupt(interrupt: usize) {
    disable_interrupt(interrupt);

    let plic = unsafe { Plic::from_ptr(&mut PERIPHERAL_PLIC_BASE as *mut _ as *mut _) };
    plic.priority(interrupt).write_value(1);
}

fn claim_interrupt() -> u32 {
    let plic = unsafe { Plic::from_ptr(&mut PERIPHERAL_PLIC_BASE as *mut _ as *mut _) };
    plic.claim(0).read()
}

fn complete_interrupt(interrupt: usize) {
    let plic = unsafe { Plic::from_ptr(&mut PERIPHERAL_PLIC_BASE as *mut _ as *mut _) };
    plic.complete(0).write_value(interrupt as u32);
}

fn disable_interrupt(interrupt: usize) {
    let enable_word = interrupt / 32;
    let enable_bit = interrupt % 32;

    let plic = unsafe { Plic::from_ptr(&mut PERIPHERAL_PLIC_BASE as *mut _ as *mut _) };
    plic.enable(enable_word)
        .modify(|x| x.set_enable(enable_bit, false));
}

fn enable_interrupt(interrupt: usize) {
    let enable_word = interrupt / 32;
    let enable_bit = interrupt % 32;

    let plic = unsafe { Plic::from_ptr(&mut PERIPHERAL_PLIC_BASE as *mut _ as *mut _) };
    plic.enable(enable_word)
        .modify(|x| x.set_enable(enable_bit, true));
}

pub fn handle_interrupt(
    _cause: usize,
    task_table: &mut task::TaskTable,
    caller_idx: task::TaskId,
) -> task::Schedule {
    let mut reschedule = false;

    loop {
        let interrupt = claim_interrupt();
        if interrupt == 0 {
            break;
        }

        if task::handle_interrupt(task_table, caller_idx, interrupt as usize)
            != task::Schedule::Same
        {
            reschedule = true;
        }
    }

    // The specific task to schedule doesn't matter as we can't reschedule
    // on an external interrupt anyway.
    if reschedule {
        task::Schedule::Other
    } else {
        task::Schedule::Same
    }
}

pub fn interrupt_control(interrupt: usize, control: task::InterruptControl) {
    match control {
        task::InterruptControl::Disable => disable_interrupt(interrupt),
        task::InterruptControl::Enable => enable_interrupt(interrupt),
        task::InterruptControl::Complete => complete_interrupt(interrupt),
    }
}

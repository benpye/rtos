use pac_qingke::pfic::{Interrupt, Pfic};
use rtos_macros::rtos_import;

use crate::arch::riscv::*;

#[rtos_import]
pub static mut PERIPHERAL_PFIC_BASE: usize;

pub fn wch_pfic_init() {
    // Enable the default set of interrupts. WCH parts do not use the MIE CSR.
    // Safety: This writes to a PFIC register, but does not affect current
    // kernel state.
    let pfic = unsafe { Pfic::from_ptr(&mut PERIPHERAL_PFIC_BASE as *mut _ as *mut _) };
    pfic.ienr(0).write(|x| {
        x.set_ienr(u8::from(Interrupt::NMI) as usize, true);
        x.set_ienr(u8::from(Interrupt::EXC) as usize, true);
        x.set_ienr(u8::from(Interrupt::ECALLU) as usize, true);
        x.set_ienr(u8::from(Interrupt::SYSTICK) as usize, true);
        x.set_ienr(u8::from(Interrupt::SWI) as usize, true);
    });
}

fn disable_interrupt(interrupt: usize) {
    let enable_word = interrupt / 32;
    let enable_bit = interrupt % 32;

    // Safety: Writes to PFIC registers - does not impact memory safety
    // of kernel. Bound check on interrupt above prevents writing past
    // end of register area.
    let pfic = unsafe { Pfic::from_ptr(&mut PERIPHERAL_PFIC_BASE as *mut _ as *mut _) };
    pfic.irer(enable_word).write(|x| {
        x.set_irer(enable_bit, true);
    });
}

fn enable_interrupt(interrupt: usize) {
    let enable_word = interrupt / 32;
    let enable_bit = interrupt % 32;

    // Safety: Writes to PFIC registers - does not impact memory safety
    // of kernel. Bound check on interrupt above prevents writing past
    // end of register area.
    let pfic = unsafe { Pfic::from_ptr(&mut PERIPHERAL_PFIC_BASE as *mut _ as *mut _) };
    pfic.ienr(enable_word).write(|x| {
        x.set_ienr(enable_bit, true);
    });
}

fn set_pending_interrupt(interrupt: usize) {
    let enable_word = interrupt / 32;
    let enable_bit = interrupt % 32;

    // Safety: Writes to PFIC registers - does not impact memory safety
    // of kernel. Bound check on interrupt above prevents writing past
    // end of register area.
    let pfic = unsafe { Pfic::from_ptr(&mut PERIPHERAL_PFIC_BASE as *mut _ as *mut _) };
    pfic.ipsr(enable_word).write(|x| {
        x.set_ipsr(enable_bit, true);
    });
}

fn clear_pending_interrupt(interrupt: usize) {
    let enable_word = interrupt / 32;
    let enable_bit = interrupt % 32;

    // Safety: Writes to PFIC registers - does not impact memory safety
    // of kernel. Bound check on interrupt above prevents writing past
    // end of register area.
    let pfic = unsafe { Pfic::from_ptr(&mut PERIPHERAL_PFIC_BASE as *mut _ as *mut _) };
    pfic.iprr(enable_word).write(|x| {
        x.set_iprr(enable_bit, true);
    });
}

pub fn reset_interrupt(interrupt: usize) {
    disable_interrupt(interrupt);
    clear_pending_interrupt(interrupt);
}

pub fn handle_interrupt(
    cause: usize,
    task_table: &mut task::TaskTable,
    caller_idx: task::TaskId,
) -> task::Schedule {
    let interrupt = cause & !super::mcause::INTERRUPT_BIT;

    disable_interrupt(interrupt);
    clear_pending_interrupt(interrupt);

    task::handle_interrupt(task_table, caller_idx, interrupt)
}

pub fn interrupt_control(interrupt: usize, control: task::InterruptControl) {
    match control {
        task::InterruptControl::Disable => disable_interrupt(interrupt),
        task::InterruptControl::Enable | task::InterruptControl::Complete => {
            enable_interrupt(interrupt)
        }
    }
}

pub fn set_software_interrupt() {
    set_pending_interrupt(u8::from(Interrupt::SWI) as usize);
}

pub fn clear_software_interrupt() {
    clear_pending_interrupt(u8::from(Interrupt::SWI) as usize);
}

pub fn set_timer_interrupt() {
    set_pending_interrupt(u8::from(Interrupt::SYSTICK) as usize);
}

pub fn clear_timer_interrupt() {
    super::wch_systick::set_timer_deadline(u64::MAX);
    super::wch_systick::clear_comparison_flag();

    clear_pending_interrupt(u8::from(Interrupt::SYSTICK) as usize);
}

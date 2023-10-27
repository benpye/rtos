use pac_qingke::systick::{Stclk, Systick};
use rtos_macros::rtos_import;

#[rtos_import]
pub static mut PERIPHERAL_SYSTICK_BASE: u32;

pub fn wch_systick_init() {
    // Enable the SysTick peripheral.
    let systick = unsafe { Systick::from_ptr(&mut PERIPHERAL_SYSTICK_BASE as *mut _ as *mut _) };
    systick.ctlr().write(|x| {
        x.set_ste(true);
        x.set_stie(true);
        x.set_stclk(Stclk::DIV1);
    });
}

pub fn now_ticks() -> u64 {
    // Safety: We are reading the memory mapped timer peripheral - this is
    // read as volatile as it is externally changing.
    //
    // We can't atomically read a u64, so we read the high u32, the low u32
    // and the high u32 again. If the high u32 changed we retry.

    let systick = unsafe { Systick::from_ptr(&mut PERIPHERAL_SYSTICK_BASE as *mut _ as *mut _) };

    loop {
        let time_hi = systick.cnth().read();
        let time_lo = systick.cntl().read();
        if time_hi == systick.cnth().read() {
            return ((time_hi as u64) << 32) | (time_lo as u64);
        }
    }
}

pub fn set_timer_deadline(ticks: u64) {
    let systick = unsafe { Systick::from_ptr(&mut PERIPHERAL_SYSTICK_BASE as *mut _ as *mut _) };
    systick.cmphr().write_value((ticks >> 32) as u32);
    systick.cmplr().write_value((ticks & 0xFFFFFFFF) as u32);

    // The SysTick interrupt will not fire if the timer was set in the past, so
    // set the pending interrupt explicitly.
    if ticks < now_ticks() {
        super::wch_pfic::set_timer_interrupt();
    }
}

pub fn timer_deadline() -> u64 {
    let systick = unsafe { Systick::from_ptr(&mut PERIPHERAL_SYSTICK_BASE as *mut _ as *mut _) };
    let deadline_hi = systick.cmphr().read();
    let deadline_lo = systick.cmplr().read();
    ((deadline_hi as u64) << 32) | (deadline_lo as u64)
}

pub fn clear_comparison_flag() {
    let systick = unsafe { Systick::from_ptr(&mut PERIPHERAL_SYSTICK_BASE as *mut _ as *mut _) };
    systick.sr().write(|x| x.set_cntif(false));
}

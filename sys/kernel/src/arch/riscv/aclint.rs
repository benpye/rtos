use pac_riscv::aclint::{Mswi, MtimerCompare, MtimerTime};
use rtos_macros::rtos_import;

#[rtos_import]
pub static mut PERIPHERAL_ACLINT_MSWI_BASE: u32;

#[rtos_import]
pub static mut PERIPHERAL_ACLINT_MTIMER_TIME_BASE: u32;

#[rtos_import]
pub static mut PERIPHERAL_ACLINT_MTIMER_COMPARE_BASE: u32;

pub fn aclint_init() {
    clear_software_interrupt()
}

pub fn set_software_interrupt() {
    let mswi = unsafe { Mswi::from_ptr(&mut PERIPHERAL_ACLINT_MSWI_BASE as *mut _ as *mut _) };
    mswi.msip(0).write(|x| x.set_pending(true));
}

pub fn clear_software_interrupt() {
    let mswi = unsafe { Mswi::from_ptr(&mut PERIPHERAL_ACLINT_MSWI_BASE as *mut _ as *mut _) };
    mswi.msip(0).write(|x| x.set_pending(false));
}

pub fn now_ticks() -> u64 {
    let mtimer_time = unsafe {
        MtimerTime::from_ptr(&mut PERIPHERAL_ACLINT_MTIMER_TIME_BASE as *mut _ as *mut _)
    };

    // We can't atomically read a u64, so we read the high u32, the low u32
    // and the high u32 again. If the high u32 changed we retry.
    loop {
        let time_hi = mtimer_time.mtimeh().read();
        let time_lo = mtimer_time.mtimel().read();
        if time_hi == mtimer_time.mtimeh().read() {
            return ((time_hi as u64) << 32) | (time_lo as u64);
        }
    }
}

pub fn set_timer_deadline(ticks: u64) {
    let mtimer_compare = unsafe {
        MtimerCompare::from_ptr(&mut PERIPHERAL_ACLINT_MTIMER_COMPARE_BASE as *mut _ as *mut _)
    };
    mtimer_compare.mtimecmp(0).write_value(ticks);
}

pub fn timer_deadline() -> u64 {
    let mtimer_compare = unsafe {
        MtimerCompare::from_ptr(&mut PERIPHERAL_ACLINT_MTIMER_COMPARE_BASE as *mut _ as *mut _)
    };
    mtimer_compare.mtimecmp(0).read()
}

pub fn clear_timer_interrupt() {
    set_timer_deadline(u64::MAX);
}

#![feature(naked_functions)]
#![no_std]

#[panic_handler]
#[cfg(target_os = "none")]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
#[naked]
unsafe extern "C" fn _start() -> ! {
    // Safety: This task consists only of this infinite WFI loop.
    unsafe {
        core::arch::asm!("1:", "wfi", "j 1b", options(noreturn));
    }
}

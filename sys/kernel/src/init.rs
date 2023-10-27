use crate::{app, arch, task};

#[panic_handler]
#[cfg(target_os = "none")]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    semihosting::println!("panic {}", _info);
    loop {}
}

// # Safety
// - This should only be called from the architecture specific entry point.
pub unsafe fn kernel_init() -> ! {
    // Perform critical architectural init - this should include setting up the
    // interrupt vector.
    arch::arch_init();

    app::app_init();

    // Load initial state from the task initialization table.
    task::task_init();

    arch::enter_first_task()
}

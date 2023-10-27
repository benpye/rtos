pub use abi::InterruptControl;
use kernel_types::syscall::abi;

#[inline(always)]
pub fn sys_interrupt_control(interrupt: usize, control: InterruptControl) {
    unsafe {
        core::arch::asm!(
            "ecall",
            in("a0") abi::SysCallId::InterruptControl.0,
            in("a1") interrupt,
            in("a2") control.0,
            options(nomem, nostack),
        )
    }
}

use kernel_types::syscall::abi;

#[inline(always)]
pub fn sys_panic() -> ! {
    unsafe {
        core::arch::asm!(
            "ecall",
            in("a0") abi::SysCallId::Panic.0,
            options(noreturn, nomem, nostack),
        )
    }
}

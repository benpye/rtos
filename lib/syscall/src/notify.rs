use kernel_types::syscall::abi;

#[inline(always)]
pub fn sys_notify(target: u8, notifications: u32) {
    unsafe {
        core::arch::asm!(
            "ecall",
            in("a0") abi::SysCallId::Notify.0,
            in("a1") target,
            in("a2") notifications,
            options(nomem, nostack),
        )
    }
}

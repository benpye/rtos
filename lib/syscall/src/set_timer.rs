use kernel_types::syscall::abi;

#[inline(always)]
pub fn sys_set_timer(periodic: bool, deadline: u32) {
    let periodic = if periodic { 1u8 } else { 0u8 };
    unsafe {
        core::arch::asm!(
            "ecall",
            in("a0") abi::SysCallId::SetTimer.0,
            in("a1") periodic,
            in("a2") deadline,
            options(nomem, nostack),
        )
    }
}

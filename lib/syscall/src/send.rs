use core::mem::size_of;

use kernel_types::syscall::abi;

#[inline(always)]
pub fn sys_send<const IN_SIZE: usize>(target: u8, len: u8, data: [usize; IN_SIZE]) {
    assert!((len as usize) <= size_of::<usize>() * IN_SIZE);

    let params = (target as u32) | ((len as u32) << 16);

    unsafe {
        let mut empty_out = [0usize; 0];

        crate::syscall!(
        IN_SIZE,
        0,
        data,
        empty_out,
        in("a0") abi::SysCallId::Send.0,
        in("a1") params,
        options(nomem, nostack),
        );
    }
}

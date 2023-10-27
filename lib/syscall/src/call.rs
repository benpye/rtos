use core::mem::size_of;

use kernel_types::syscall::abi;

pub struct CallResult<const OUT_SIZE: usize> {
    pub len: u8,
    pub data: [usize; OUT_SIZE],
}

#[inline(always)]
pub fn sys_call<const IN_SIZE: usize, const OUT_SIZE: usize>(
    target: u8,
    in_len: u8,
    input: [usize; IN_SIZE],
) -> CallResult<OUT_SIZE> {
    assert!((in_len as usize) <= size_of::<usize>() * IN_SIZE);

    let out_size = (OUT_SIZE * size_of::<usize>()) as u8;
    let params = (target as u32) | ((in_len as u32) << 16) | ((out_size as u32) << 24);

    let mut output: core::mem::MaybeUninit<[usize; OUT_SIZE]> = core::mem::MaybeUninit::uninit();
    let out_len: u32;

    let data = unsafe {
        let output_mut = output.assume_init_mut();

        crate::syscall!(
        IN_SIZE,
        OUT_SIZE,
        input,
        output_mut,
        in("a0") abi::SysCallId::Call.0,
        in("a1") params,
        lateout("a1") out_len,
        options(nomem, nostack),
        );

        output.assume_init()
    };

    let len = out_len as u8;
    CallResult { len, data }
}

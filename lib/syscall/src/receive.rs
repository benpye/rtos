use core::mem::size_of;

use kernel_types::syscall::abi;

pub struct ReceiveResult<const OUT_SIZE: usize> {
    pub notifications: u32,
    pub sender: u8,
    pub len: u8,
    pub data: [usize; OUT_SIZE],
}

#[inline(always)]
pub fn sys_receive<const OUT_SIZE: usize>() -> ReceiveResult<OUT_SIZE> {
    let out_size = (OUT_SIZE * size_of::<usize>()) as u8;
    let in_params = out_size as u32;

    let mut output: core::mem::MaybeUninit<[usize; OUT_SIZE]> = core::mem::MaybeUninit::uninit();
    let out_params: u32;
    let notifications: u32;

    let data = unsafe {
        let empty_in = [0usize; 0];
        let output_mut = output.assume_init_mut();

        crate::syscall!(
        0,
        OUT_SIZE,
        empty_in,
        output_mut,
        in("a0") abi::SysCallId::Receive.0,
        in("a1") in_params,
        lateout("a1") out_params,
        lateout("a2") notifications,
        options(nomem, nostack),
        );

        output.assume_init()
    };

    let sender = (out_params & 0xff) as u8;
    let len = (out_params >> 16) as u8;

    ReceiveResult {
        notifications,
        sender,
        len,
        data,
    }
}

#![feature(asm_const)]
#![no_std]

mod call;
mod interrupt_control;
mod notify;
mod panic;
mod receive;
mod send;
mod set_timer;

pub use call::{sys_call, CallResult};
pub use interrupt_control::{sys_interrupt_control, InterruptControl};
pub use kernel_types::syscall::abi;
pub use notify::sys_notify;
pub use panic::sys_panic;
pub use receive::{sys_receive, ReceiveResult};
pub use send::sys_send;
pub use set_timer::sys_set_timer;

macro_rules! syscall {
    (@asm ($($regs:tt)*)) => {
        ::core::arch::asm!(
            "ecall",
            $($regs)*
        )
        // stringify!($($regs)*)
    };
    (@accum_in (16, $output_num:tt, $input:ident, $output:ident) ($($regs:tt)*)) => {
        crate::syscall!(@accum_in (15, $output_num, $input, $output) ($($regs)* in("s4") $input[15],))
    };
    (@accum_in (15, $output_num:tt, $input:ident, $output:ident) ($($regs:tt)*)) => {
        crate::syscall!(@accum_in (14, $output_num, $input, $output) ($($regs)* in("s3") $input[14],))
    };
    (@accum_in (14, $output_num:tt, $input:ident, $output:ident) ($($regs:tt)*)) => {
        crate::syscall!(@accum_in (13, $output_num, $input, $output) ($($regs)* in("s2") $input[13],))
    };
    (@accum_in (13, $output_num:tt, $input:ident, $output:ident) ($($regs:tt)*)) => {
        crate::syscall!(@accum_in (12, $output_num, $input, $output) ($($regs)* in("ra") $input[12],))
    };
    (@accum_in (12, $output_num:tt, $input:ident, $output:ident) ($($regs:tt)*)) => {
        crate::syscall!(@accum_in (11, $output_num, $input, $output) ($($regs)* in("t6") $input[11],))
    };
    (@accum_in (11, $output_num:tt, $input:ident, $output:ident) ($($regs:tt)*)) => {
        crate::syscall!(@accum_in (10, $output_num, $input, $output) ($($regs)* in("t5") $input[10],))
    };
    (@accum_in (10, $output_num:tt, $input:ident, $output:ident) ($($regs:tt)*)) => {
        crate::syscall!(@accum_in (9, $output_num, $input, $output) ($($regs)* in("t4") $input[9],))
    };
    (@accum_in (9, $output_num:tt, $input:ident, $output:ident) ($($regs:tt)*)) => {
        crate::syscall!(@accum_in (8, $output_num, $input, $output) ($($regs)* in("t3") $input[8],))
    };
    (@accum_in (8, $output_num:tt, $input:ident, $output:ident) ($($regs:tt)*)) => {
        crate::syscall!(@accum_in (7, $output_num, $input, $output) ($($regs)* in("t2") $input[7],))
    };
    (@accum_in (7, $output_num:tt, $input:ident, $output:ident) ($($regs:tt)*)) => {
        crate::syscall!(@accum_in (6, $output_num, $input, $output) ($($regs)* in("t1") $input[6],))
    };
    (@accum_in (6, $output_num:tt, $input:ident, $output:ident) ($($regs:tt)*)) => {
        crate::syscall!(@accum_in (5, $output_num, $input, $output) ($($regs)* in("t0") $input[5],))
    };
    (@accum_in (5, $output_num:tt, $input:ident, $output:ident) ($($regs:tt)*)) => {
        crate::syscall!(@accum_in (4, $output_num, $input, $output) ($($regs)* in("a7") $input[4],))
    };
    (@accum_in (4, $output_num:tt, $input:ident, $output:ident) ($($regs:tt)*)) => {
        crate::syscall!(@accum_in (3, $output_num, $input, $output) ($($regs)* in("a6") $input[3],))
    };
    (@accum_in (3, $output_num:tt, $input:ident, $output:ident) ($($regs:tt)*)) => {
        crate::syscall!(@accum_in (2, $output_num, $input, $output) ($($regs)* in("a5") $input[2],))
    };
    (@accum_in (2, $output_num:tt, $input:ident, $output:ident) ($($regs:tt)*)) => {
        crate::syscall!(@accum_in (1, $output_num, $input, $output) ($($regs)* in("a4") $input[1],))
    };
    (@accum_in (1, $output_num:tt, $input:ident, $output:ident) ($($regs:tt)*)) => {
        crate::syscall!(@accum_in (0, $output_num, $input, $output) ($($regs)* in("a3") $input[0],))
    };
    (@accum_in (0, $output_num:tt, $input:ident, $output:ident) ($($regs:tt)*)) => {
        crate::syscall!(@accum_out ($output_num, $output) ($($regs)*))
    };
    (@accum_out (16, $output:ident) ($($regs:tt)*)) => {
        crate::syscall!(@accum_out (15, $output) ($($regs)* lateout("s4") $output[15],))
    };
    (@accum_out (15, $output:ident) ($($regs:tt)*)) => {
        crate::syscall!(@accum_out (14, $output) ($($regs)* lateout("s3") $output[14],))
    };
    (@accum_out (14, $output:ident) ($($regs:tt)*)) => {
        crate::syscall!(@accum_out (13, $output) ($($regs)* lateout("s2") $output[13],))
    };
    (@accum_out (13, $output:ident) ($($regs:tt)*)) => {
        crate::syscall!(@accum_out (12, $output) ($($regs)* lateout("ra") $output[12],))
    };
    (@accum_out (12, $output:ident) ($($regs:tt)*)) => {
        crate::syscall!(@accum_out (11, $output) ($($regs)* lateout("t6") $output[11],))
    };
    (@accum_out (11, $output:ident) ($($regs:tt)*)) => {
        crate::syscall!(@accum_out (10, $output) ($($regs)* lateout("t5") $output[10],))
    };
    (@accum_out (10, $output:ident) ($($regs:tt)*)) => {
        crate::syscall!(@accum_out (9, $output) ($($regs)* lateout("t4") $output[9],))
    };
    (@accum_out (9, $output:ident) ($($regs:tt)*)) => {
        crate::syscall!(@accum_out (8, $output) ($($regs)* lateout("t3") $output[8],))
    };
    (@accum_out (8, $output:ident) ($($regs:tt)*)) => {
        crate::syscall!(@accum_out (7, $output) ($($regs)* lateout("t2") $output[7],))
    };
    (@accum_out (7, $output:ident) ($($regs:tt)*)) => {
        crate::syscall!(@accum_out (6, $output) ($($regs)* lateout("t1") $output[6],))
    };
    (@accum_out (6, $output:ident) ($($regs:tt)*)) => {
        crate::syscall!(@accum_out (5, $output) ($($regs)* lateout("t0") $output[5],))
    };
    (@accum_out (5, $output:ident) ($($regs:tt)*)) => {
        crate::syscall!(@accum_out (4, $output) ($($regs)* lateout("a7") $output[4],))
    };
    (@accum_out (4, $output:ident) ($($regs:tt)*)) => {
        crate::syscall!(@accum_out (3, $output) ($($regs)* lateout("a6") $output[3],))
    };
    (@accum_out (3, $output:ident) ($($regs:tt)*)) => {
        crate::syscall!(@accum_out (2, $output) ($($regs)* lateout("a5") $output[2],))
    };
    (@accum_out (2, $output:ident) ($($regs:tt)*)) => {
        crate::syscall!(@accum_out (1, $output) ($($regs)* lateout("a4") $output[1],))
    };
    (@accum_out (1, $output:ident) ($($regs:tt)*)) => {
        crate::syscall!(@accum_out (0, $output) ($($regs)* lateout("a3") $output[0],))
    };
    (@accum_out (0, $output:ident) ($($regs:tt)*)) => {
        crate::syscall!(@asm ($($regs)*))
    };
    (@match_output ($max_output:expr, $input_num:tt, $input:ident, $output:ident, $($regs:tt)*)) => {
        match $max_output {
            0 => crate::syscall!(@accum_in ($input_num, 0, $input, $output) ($($regs)*)),
            1 => crate::syscall!(@accum_in ($input_num, 1, $input, $output) ($($regs)*)),
            2 => crate::syscall!(@accum_in ($input_num, 2, $input, $output) ($($regs)*)),
            3 => crate::syscall!(@accum_in ($input_num, 3, $input, $output) ($($regs)*)),
            4 => crate::syscall!(@accum_in ($input_num, 4, $input, $output) ($($regs)*)),
            5 => crate::syscall!(@accum_in ($input_num, 5, $input, $output) ($($regs)*)),
            6 => crate::syscall!(@accum_in ($input_num, 6, $input, $output) ($($regs)*)),
            7 => crate::syscall!(@accum_in ($input_num, 7, $input, $output) ($($regs)*)),
            8 => crate::syscall!(@accum_in ($input_num, 8, $input, $output) ($($regs)*)),
            9 => crate::syscall!(@accum_in ($input_num, 9, $input, $output) ($($regs)*)),
            10 => crate::syscall!(@accum_in ($input_num, 10, $input, $output) ($($regs)*)),
            11 => crate::syscall!(@accum_in ($input_num, 11, $input, $output) ($($regs)*)),
            12 => crate::syscall!(@accum_in ($input_num, 12, $input, $output) ($($regs)*)),
            13 => crate::syscall!(@accum_in ($input_num, 13, $input, $output) ($($regs)*)),
            14 => crate::syscall!(@accum_in ($input_num, 14, $input, $output) ($($regs)*)),
            15 => crate::syscall!(@accum_in ($input_num, 15, $input, $output) ($($regs)*)),
            16 => crate::syscall!(@accum_in ($input_num, 16, $input, $output) ($($regs)*)),
            _ => unimplemented!(),
        }
    };
    (@match_input ($max_input:expr, $max_output:expr, $input:ident, $output:ident, $($regs:tt)*)) => {
        match $max_input {
            0 => crate::syscall!(@match_output ($max_output, 0, $input, $output, $($regs)*)),
            1 => crate::syscall!(@match_output ($max_output, 1, $input, $output, $($regs)*)),
            2 => crate::syscall!(@match_output ($max_output, 2, $input, $output, $($regs)*)),
            3 => crate::syscall!(@match_output ($max_output, 3, $input, $output, $($regs)*)),
            4 => crate::syscall!(@match_output ($max_output, 4, $input, $output, $($regs)*)),
            5 => crate::syscall!(@match_output ($max_output, 5, $input, $output, $($regs)*)),
            6 => crate::syscall!(@match_output ($max_output, 6, $input, $output, $($regs)*)),
            7 => crate::syscall!(@match_output ($max_output, 7, $input, $output, $($regs)*)),
            8 => crate::syscall!(@match_output ($max_output, 8, $input, $output, $($regs)*)),
            9 => crate::syscall!(@match_output ($max_output, 9, $input, $output, $($regs)*)),
            10 => crate::syscall!(@match_output ($max_output, 10, $input, $output, $($regs)*)),
            11 => crate::syscall!(@match_output ($max_output, 11, $input, $output, $($regs)*)),
            12 => crate::syscall!(@match_output ($max_output, 12, $input, $output, $($regs)*)),
            13 => crate::syscall!(@match_output ($max_output, 13, $input, $output, $($regs)*)),
            14 => crate::syscall!(@match_output ($max_output, 14, $input, $output, $($regs)*)),
            15 => crate::syscall!(@match_output ($max_output, 15, $input, $output, $($regs)*)),
            16 => crate::syscall!(@match_output ($max_output, 16, $input, $output, $($regs)*)),
            _ => unimplemented!(),
        }
    };
    ($max_input:expr, $max_output:expr, $input:ident, $output:ident, $($regs:tt)*) => {
        crate::syscall!(@match_input ($max_input, $max_output, $input, $output, $($regs)*))
    }
}

pub(crate) use syscall;

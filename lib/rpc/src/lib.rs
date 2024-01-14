#![no_std]

mod client;
mod interface;
mod server;

pub use client::Client;
pub use server::{Dispatch, Notify, Server};

#[open_enum::open_enum]
#[repr(u8)]
#[derive(Debug)]
#[derive(Clone, Copy, zerocopy::AsBytes, zerocopy::FromBytes, zerocopy::FromZeroes)]
pub enum CallStatus {
    Success,
    InvalidCallCode,
    InvalidInput,
    InvalidOutput,
    InvalidParameter,
    OperationFailed,
}

#[doc(hidden)]
pub mod macro_util {
    pub use open_enum;
    pub use syscall;
    pub use zerocopy;
}

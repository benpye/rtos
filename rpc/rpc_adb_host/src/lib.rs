#![no_std]

#[repr(C)]
#[derive(Clone, Copy, zerocopy::AsBytes, zerocopy::FromBytes, zerocopy::FromZeroes)]
pub struct TalkResult {
    pub service_request: u8,
    pub len: u8,
    pub data: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy, zerocopy::AsBytes, zerocopy::FromBytes, zerocopy::FromZeroes)]
pub struct ListenResult {
    pub service_request: u8,
}

rpc::rpc_interface! {
    pub trait AdbHost {
        fn talk(address: u8, register: u8) -> crate::TalkResult;
        fn listen(address: u8, register: u8, len: u8, data: [u8; 8]) -> crate::ListenResult;
    }
}

#![no_std]

rpc::rpc_interface! {
    pub trait TestHelper {
        fn set_timer(periodic: u8, #[padding] _pad: [u8; 3], deadline: u32) -> ();
        fn notification_count(bit: usize) -> u32;
        fn swap_buffer(buffer: [u8; 36]) -> [u8; 36];
    }
}

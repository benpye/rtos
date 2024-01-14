#![no_std]

pub const VALID_CTLR_MASK: u32 = 0b00001111_00001111_00000011_11001111;

rpc::rpc_interface! {
    pub trait Afio {
        fn modify_ctlr(value: u32, mask: u32) -> ();
    }
}

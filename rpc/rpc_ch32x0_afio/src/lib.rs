#![no_std]

rpc::rpc_interface! {
    pub trait Afio {
        fn modify_ctlr(value: u32, mask: u32) -> ();
    }
}

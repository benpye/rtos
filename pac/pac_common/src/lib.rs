#![no_std]

mod internal {
    core::include!(core::concat!(core::env!("OUT_DIR"), "/lib.rs"));
}

pub use internal::common::*;

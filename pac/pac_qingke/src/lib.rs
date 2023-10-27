#![no_std]

core::include!(core::concat!(core::env!("OUT_DIR"), "/lib.rs"));

#[cfg(feature = "critical_section")]
mod critical_section;

#![no_std]

pub mod arch;
pub mod syscall;
pub mod task;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct LinkConst(*const core::ffi::c_void);

// # Safety
// - Link const cannot be mutated, it is a link time constant and so
//   implementing Sync is safe.
#[doc(hidden)]
unsafe impl Sync for LinkConst {}

impl From<LinkConst> for usize {
    #[inline]
    fn from(value: LinkConst) -> Self {
        value.0 as usize
    }
}

impl LinkConst {
    // # Safety
    // - Should only be used via the link_const! macro to reference constants
    //   from the linker.
    #[doc(hidden)]
    pub const unsafe fn new<T>(ptr: *const T) -> Self {
        Self(ptr as _)
    }
}

#[macro_export]
macro_rules! link_const {
    ($($sym:tt)*) => {{
        extern "C" {
            #[link_name = $($sym)*]
            static link_symbol: ::core::ffi::c_void;
        }
        unsafe { $crate::LinkConst::new(&link_symbol) }
    }};
}

#[macro_export]
macro_rules! task_id {
    ($task_name:literal) => {{
        extern "C" {
            #[link_name = concat!(
                                    "rtos.constant.",
                                    $task_name,
                                    ".task_id"
                                )]
            static task_id: u8;
        }
        unsafe { task_id }
    }};
}

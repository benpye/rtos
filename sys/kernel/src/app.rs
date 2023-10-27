pub fn app_init() {
    extern "C" {
        #[link_name = "rtos.app_init"]
        #[linkage = "extern_weak"]
        static app_init_impl: *const core::ffi::c_void;
    }

    type F = unsafe extern "C" fn();

    // Safety: app_init_impl is a constant determined at link time. We must
    // trust that the implementation of app_init_impl is safe as this is
    // called in kernel mode.
    unsafe {
        if !app_init_impl.is_null() {
            core::mem::transmute::<_, F>(app_init_impl)();
        }
    }
}

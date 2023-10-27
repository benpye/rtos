struct CriticalSection;
critical_section::set_impl!(CriticalSection);

unsafe impl critical_section::Impl for CriticalSection {
    unsafe fn acquire() -> critical_section::RawRestoreState {
        let mut gintenr: usize;
        unsafe {
            core::arch::asm!("csrrci {}, 0x800, 0b1000", out(reg) gintenr);
        }
        gintenr & 0b1000 != 0
    }

    unsafe fn release(restore_state: critical_section::RawRestoreState) {
        if restore_state {
            unsafe {
                core::arch::asm!("csrsi 0x800, 0b1000");
            }
        }
    }
}

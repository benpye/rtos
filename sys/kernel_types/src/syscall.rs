pub mod abi {
    use open_enum::open_enum;
    use zerocopy::{AsBytes, FromBytes, FromZeroes};

    pub const MAX_MESSAGE_SIZE: usize = 10;
    pub const MAX_MESSAGE_LENGTH: usize = MAX_MESSAGE_SIZE * ::core::mem::size_of::<usize>();

    pub const SYS_NOTIFICATION_TIMER_BIT: usize = 31;
    pub const SYS_NOTIFICATION_TIMER: u32 = 1 << SYS_NOTIFICATION_TIMER_BIT;

    #[open_enum]
    #[repr(usize)]
    #[derive(Clone, Copy, AsBytes, FromBytes, FromZeroes)]
    pub enum SysCallId {
        Panic,
        Receive,
        Send,
        Call,
        Notify,
        SetTimer,
        InterruptControl,
    }

    #[open_enum]
    #[repr(u32)]
    #[derive(Clone, Copy, AsBytes, FromBytes, FromZeroes)]
    pub enum InterruptControl {
        Disable,
        Enable,
        Complete,
    }
}

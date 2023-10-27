use bitflags::bitflags;

use crate::{arch::ArchTaskDescriptor, LinkConst};

bitflags! {
    #[repr(transparent)]
    pub struct Flags: u8 {
        // Task starts on boot.
        const BOOT = 0x01;
        // Task runs in a privileged mode (ie. machine mode on RISC-V).
        const PRIVILEGED = 0x02;
        // If this task dies it is a critical error and should result in a
        // kernel panic.
        const CRITICAL = 0x04;
    }
}

#[repr(C)]
pub struct TaskDescriptor {
    pub init_pc: LinkConst,
    pub priority: u8,
    pub flags: Flags,
    pub arch: ArchTaskDescriptor,
}

#[repr(transparent)]
pub struct InterruptDescriptor(u32);

impl InterruptDescriptor {
    pub const fn new(task_id: u8, notification: u32) -> Self {
        if (notification & 0x00FFFFFF) != notification {
            panic!("Notification for interrupt out of range.");
        }

        Self((task_id as u32) | (notification << 8))
    }

    pub const fn none() -> Self {
        Self(0xFFFFFFFF)
    }

    pub const fn task_id(&self) -> u8 {
        (self.0 & 0xFF) as u8
    }

    pub const fn notification(&self) -> u32 {
        self.0 >> 8
    }
}

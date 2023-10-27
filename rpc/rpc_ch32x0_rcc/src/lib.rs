#![no_std]

#[open_enum::open_enum]
#[repr(u8)]
#[derive(Clone, Copy)]
pub enum Bus {
    Ahb,
    Apb2,
    Apb1,
}

#[open_enum::open_enum]
#[repr(u8)]
#[derive(Clone, Copy, zerocopy::AsBytes, zerocopy::FromBytes, zerocopy::FromZeroes)]
pub enum Peripheral {
    Usart1 = Peripheral::from_parts(Bus::Apb2, 14),
    Spi1 = Peripheral::from_parts(Bus::Apb2, 12),
    Tim1 = Peripheral::from_parts(Bus::Apb2, 11),
    Adc1 = Peripheral::from_parts(Bus::Apb2, 9),
    Iopc = Peripheral::from_parts(Bus::Apb2, 4),
    Iopb = Peripheral::from_parts(Bus::Apb2, 3),
    Iopa = Peripheral::from_parts(Bus::Apb2, 2),
    Afio = Peripheral::from_parts(Bus::Apb2, 0),
    Pwr = Peripheral::from_parts(Bus::Apb1, 28),
    I2c1 = Peripheral::from_parts(Bus::Apb1, 21),
    Usart4 = Peripheral::from_parts(Bus::Apb1, 19),
    Usart3 = Peripheral::from_parts(Bus::Apb1, 18),
    Usart2 = Peripheral::from_parts(Bus::Apb1, 17),
    Wwdg = Peripheral::from_parts(Bus::Apb1, 11),
    Tim3 = Peripheral::from_parts(Bus::Apb1, 1),
    Tim2 = Peripheral::from_parts(Bus::Apb1, 0),
    UsbPd = Peripheral::from_parts(Bus::Ahb, 17),
    Pioc = Peripheral::from_parts(Bus::Ahb, 13),
    UsbFs = Peripheral::from_parts(Bus::Ahb, 12),
    Sram = Peripheral::from_parts(Bus::Ahb, 2),
    Dma1 = Peripheral::from_parts(Bus::Ahb, 0),
}

impl Peripheral {
    const fn from_parts(bus: Bus, bit: usize) -> u8 {
        assert!(bit < 32);
        assert!(bus.0 < 4);
        (bus.0 << 6) | (bit as u8)
    }

    #[inline(always)]
    pub const fn bus(&self) -> Bus {
        Bus(self.0 >> 6)
    }

    #[inline(always)]
    pub const fn bit(&self) -> u8 {
        self.0 & 0x1f
    }

    #[inline(always)]
    pub const fn mask(&self) -> u32 {
        1 << self.bit()
    }

    const VALID_AHB_RESET_MASK: u32 = Self::UsbPd.mask() | Self::Pioc.mask() | Self::UsbFs.mask();

    const VALID_APB2_RESET_MASK: u32 = Self::Usart1.mask()
        | Self::Spi1.mask()
        | Self::Tim1.mask()
        | Self::Adc1.mask()
        | Self::Iopc.mask()
        | Self::Iopb.mask()
        | Self::Iopa.mask()
        | Self::Afio.mask();

    const VALID_APB1_RESET_MASK: u32 = Self::Pwr.mask()
        | Self::I2c1.mask()
        | Self::Usart4.mask()
        | Self::Usart3.mask()
        | Self::Usart2.mask()
        | Self::Wwdg.mask()
        | Self::Tim3.mask()
        | Self::Tim2.mask();

    const VALID_AHB_CLOCK_ENABLE_MASK: u32 =
        Self::UsbPd.mask() | Self::UsbFs.mask() | Self::Sram.mask() | Self::Dma1.mask();

    const VALID_APB2_CLOCK_ENABLE_MASK: u32 = Self::Usart1.mask()
        | Self::Spi1.mask()
        | Self::Tim1.mask()
        | Self::Adc1.mask()
        | Self::Iopc.mask()
        | Self::Iopb.mask()
        | Self::Iopa.mask()
        | Self::Afio.mask();

    const VALID_APB1_CLOCK_ENABLE_MASK: u32 = Self::Pwr.mask()
        | Self::I2c1.mask()
        | Self::Usart4.mask()
        | Self::Usart3.mask()
        | Self::Usart2.mask()
        | Self::Wwdg.mask()
        | Self::Tim3.mask()
        | Self::Tim2.mask();

    pub const fn valid_clock_enable_mask(bus: Bus) -> u32 {
        match bus {
            Bus::Ahb => Self::VALID_AHB_CLOCK_ENABLE_MASK,
            Bus::Apb2 => Self::VALID_APB2_CLOCK_ENABLE_MASK,
            Bus::Apb1 => Self::VALID_APB1_CLOCK_ENABLE_MASK,
            _ => 0,
        }
    }

    pub const fn valid_reset_mask(bus: Bus) -> u32 {
        match bus {
            Bus::Ahb => Self::VALID_AHB_RESET_MASK,
            Bus::Apb2 => Self::VALID_APB2_RESET_MASK,
            Bus::Apb1 => Self::VALID_APB1_RESET_MASK,
            _ => 0,
        }
    }
}

rpc::rpc_interface! {
    /// Interface for the CH32X0 family RCC peripheral.
    pub trait Rcc {
        /// Set/clear the reset bit for the target peripheral.
        fn peripheral_reset(peripheral: crate::Peripheral, value: u8) -> ();

        /// Set/clear the clock enable bit for the target peripheral.
        fn peripheral_clock_enable(peripheral: crate::Peripheral, value: u8) -> ();
    }
}

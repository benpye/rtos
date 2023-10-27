use ch32x0::ch32x035 as device;
use critical_section::CriticalSection;

#[derive(Debug)]
pub enum Error {
    BusError,
}

pub struct Adb {
    gpioa: device::GPIOA,
}

impl Adb {
    pub fn new(gpioa: device::GPIOA) -> Self {
        // Set A0 to floating input.
        gpioa
            .cfglr
            .modify(|_, w| w.cnf0().variant(0b01).mode0().variant(0b00));

        // Ensure that A0 is set to output zero.
        gpioa.outdr.modify(|_, w| w.odr0().clear_bit());

        Self { gpioa }
    }

    #[inline(always)]
    fn data_lo(&self) {
        // Set data to output mode - will pull the line low.
        self.gpioa
            .cfglr
            .modify(|_, w| w.cnf0().variant(0b00).mode0().variant(0b01));
    }

    #[inline(always)]
    fn data_hi(&self) {
        // Set data to floating input mode - line will be pulled high by
        // external resistor.
        self.gpioa
            .cfglr
            .modify(|_, w| w.cnf0().variant(0b01).mode0().variant(0b00));
    }

    #[inline(always)]
    pub fn delay_us(_cs: CriticalSection, us: usize) {
        // Each iteration of this loop takes 4 cycles, at an F_CLK of 8MHz that
        // means each iteration takes 0.5us.
        const ITERATIONS_PER_US: usize = 2;
        let iterations = us * ITERATIONS_PER_US;

        unsafe {
            core::arch::asm!(
                "1:",
                "nop",
                "addi {i}, {i}, -1",
                "bnez {i}, 1b",
                i = inout(reg) iterations => _
            );
        }
    }

    #[inline(always)]
    fn wait_for_hi(&self, _cs: CriticalSection, us: usize) -> usize {
        // Each iteration of this loop takes 8 cycles, at an F_CLK of 8MHz that
        // means each iteration takes 1us.
        const ITERATIONS_PER_US: usize = 1;
        let mut iterations = us * ITERATIONS_PER_US;
        let addr = self.gpioa.indr.as_ptr() as usize;

        unsafe {
            core::arch::asm!(
                "1:",
                "nop",
                "lw {tmp}, ({addr})",
                "andi {tmp}, {tmp}, 1",
                "bnez {tmp}, 2f",
                "addi {i}, {i}, -1",
                "bnez {i}, 1b",
                "2:",
                addr = in(reg) addr,
                i = inout(reg) iterations => iterations,
                tmp = out(reg) _,
            );
        }

        iterations / ITERATIONS_PER_US
    }

    #[inline(always)]
    fn wait_for_lo(&self, _cs: CriticalSection, us: usize) -> usize {
        // Each iteration of this loop takes 8 cycles, at an F_CLK of 8MHz that
        // means each iteration takes 1us.
        const ITERATIONS_PER_US: usize = 1;
        let mut iterations = us * ITERATIONS_PER_US;
        let addr = self.gpioa.indr.as_ptr() as usize;

        unsafe {
            core::arch::asm!(
                "1:",
                "nop",
                "lw {tmp}, ({addr})",
                "andi {tmp}, {tmp}, 1",
                "beqz {tmp}, 2f",
                "addi {i}, {i}, -1",
                "bnez {i}, 1b",
                "2:",
                addr = in(reg) addr,
                i = inout(reg) iterations => iterations,
                tmp = out(reg) _,
            );
        }

        iterations / ITERATIONS_PER_US
    }

    #[inline(always)]
    fn bit_zero(&self, cs: CriticalSection) {
        // Zero bit low 65us, high 35us
        self.data_lo();
        Self::delay_us(cs, 65);
        self.data_hi();
        Self::delay_us(cs, 35);
    }

    #[inline(always)]
    fn bit_one(&self, cs: CriticalSection) {
        // One bit low 35us, high 65us
        self.data_lo();
        Self::delay_us(cs, 35);
        self.data_hi();
        Self::delay_us(cs, 65);
    }

    #[inline(always)]
    fn attention_and_sync(&self, cs: CriticalSection) {
        // Attention low 800us, sync high 65us
        self.data_lo();
        Self::delay_us(cs, 800);
        self.data_hi();
        Self::delay_us(cs, 65);
    }

    #[inline(always)]
    fn stop(&self, cs: CriticalSection) {
        // Stop bit low 70us
        self.data_lo();
        Self::delay_us(cs, 70);
        self.data_hi();
    }

    #[inline(always)]
    fn send_byte(&self, cs: CriticalSection, value: u8) {
        for i in 0..8 {
            if value & (0x80 >> i) != 0 {
                self.bit_one(cs);
            } else {
                self.bit_zero(cs);
            }
        }
    }

    const COMMAND_TALK: u8 = 0b11;
    const COMMAND_LISTEN: u8 = 0b10;

    fn start_command(&self, cs: CriticalSection, command: u8) -> Result<bool, Error> {
        // Attention and sync
        self.attention_and_sync(cs);

        // Command byte
        self.send_byte(cs, command);

        // Stop
        self.stop(cs);
        // Wait for the data line to go high - this may be held low by a
        // device for a service request.
        let lo = self.wait_for_hi(cs, 400);
        if lo == 0 {
            // The bus did not return high, some faulty device kept it low?
            return Err(Error::BusError);
        }

        // A service request should extend the stop bit by at least 140us.
        let service_request = lo < (400 - 100); // 140);

        Ok(service_request)
    }

    pub fn talk(&self, address: u8, register: u8, buf: &mut [u8]) -> Result<(bool, usize), Error> {
        // Zero output buffer
        buf.fill(0);

        let command = (address << 4) | (Self::COMMAND_TALK << 2) | register;

        critical_section::with(|cs| {
            let service_request = self.start_command(cs, command)?;

            // Stop to start time
            if self.wait_for_lo(cs, 260) == 0 {
                // No response within 260us, either no device or device has no
                // data.
                return Ok((service_request, 0));
            }

            // Start
            if self.wait_for_hi(cs, 40) == 0 {
                // Data held low too long for start bit.
                return Err(Error::BusError);
            }
            if self.wait_for_lo(cs, 100) == 0 {
                // Data held high too long for start bit.
                return Err(Error::BusError);
            }

            let mut i = 0;

            loop {
                // Max bit-cell time - 130us.

                let lo = self.wait_for_hi(cs, 130);
                if lo == 0 {
                    // The data line should always remain high after the final
                    // stop bit.
                    return Err(Error::BusError);
                }

                let hi = self.wait_for_lo(cs, lo);
                if hi == 0 {
                    // If data does not return to low then this must be the
                    // stop bit.
                    break;
                }

                // lo = 130 - low_time
                // hi = lo - high_time
                // lo_time = 130 - lo
                // hi_time = lo - hi
                let lo_time = 130 - lo;
                let hi_time = lo - hi;

                if let Some(byte) = buf.get_mut(i / 8) {
                    *byte = *byte << 1;

                    if lo_time < hi_time {
                        // 1 bit
                        *byte = *byte | 1;
                    } else {
                        // 0 bit
                    }
                }

                i = i + 1;
            }

            Ok((service_request, i / 8))
        })
    }

    pub fn listen(&self, address: u8, register: u8, buf: &[u8]) -> Result<bool, Error> {
        let command = (address << 4) | (Self::COMMAND_LISTEN << 2) | register;

        critical_section::with(|cs| {
            let service_request = self.start_command(cs, command)?;

            // Stop-to-start time
            Self::delay_us(cs, 200);

            // Start bit
            self.bit_one(cs);

            for byte in buf.iter() {
                self.send_byte(cs, *byte);
            }

            // Stop bit
            self.stop(cs);

            Ok(service_request)
        })
    }
}

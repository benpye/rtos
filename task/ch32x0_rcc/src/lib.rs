#![feature(naked_functions)]
#![no_std]

use ch32x0::ch32x035 as device;
use rpc_ch32x0_rcc::{Bus, Peripheral};
use rtos_macros::rtos_task_entry;

#[panic_handler]
#[cfg(target_os = "none")]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    syscall::sys_panic();
}

#[rtos_task_entry]
fn task_main() -> ! {
    let srv = RccServer {
        rcc: unsafe { &*device::RCC::ptr() },
    };
    rpc::Server::new(srv).listen()
}

struct RccServer<'a> {
    rcc: &'a device::rcc::RegisterBlock,
}

impl rpc::Notify for RccServer<'_> {
    fn receive_notifications(&mut self, _notifications: u32) {
        panic!()
    }
}

impl RccServer<'_> {
    unsafe fn modify_reg<T>(reg: &ch32x0::Reg<T>, or_mask: u32, and_mask: u32)
    where
        T: ch32x0::RegisterSpec<Ux = u32> + ch32x0::Readable + ch32x0::Writable,
    {
        reg.modify(|r, w| unsafe { w.bits((r.bits() | or_mask) & and_mask) })
    }
}

impl rpc_ch32x0_rcc::Rcc<rpc::CallStatus> for RccServer<'_> {
    fn peripheral_reset(
        &mut self,
        peripheral: Peripheral,
        value: u8,
    ) -> Result<(), rpc::CallStatus> {
        let valid_mask = Peripheral::valid_reset_mask(peripheral.bus());
        if (peripheral.mask() & !valid_mask) != 0 {
            return Err(rpc::CallStatus::InvalidParameter);
        }

        let or_mask = if value != 0 { peripheral.mask() } else { 0 };
        let and_mask = if value != 0 { !0 } else { !peripheral.mask() };

        // Safety: The per-bus valid mask ensures only safe bits are set/reset.
        // The valid mask for an invalid bus is always zero.
        unsafe {
            match peripheral.bus() {
                Bus::Apb2 => Self::modify_reg(&self.rcc.apb2prstr, or_mask, and_mask),
                Bus::Apb1 => Self::modify_reg(&self.rcc.apb1prstr, or_mask, and_mask),
                Bus::Ahb => Self::modify_reg(&self.rcc.ahbrstr, or_mask, and_mask),
                _ => {}
            }
        }
        Ok(())
    }

    fn peripheral_clock_enable(
        &mut self,
        peripheral: Peripheral,
        value: u8,
    ) -> Result<(), rpc::CallStatus> {
        let valid_mask = Peripheral::valid_clock_enable_mask(peripheral.bus());
        if (peripheral.mask() & !valid_mask) != 0 {
            return Err(rpc::CallStatus::InvalidParameter);
        }

        let or_mask = if value != 0 { peripheral.mask() } else { 0 };
        let and_mask = if value != 0 { !0 } else { !peripheral.mask() };

        // Safety: The per-bus valid mask ensures only safe bits are set/reset.
        // The valid mask for an invalid bus is always zero.
        unsafe {
            match peripheral.bus() {
                Bus::Apb2 => Self::modify_reg(&self.rcc.apb2pcenr, or_mask, and_mask),
                Bus::Apb1 => Self::modify_reg(&self.rcc.apb1pcenr, or_mask, and_mask),
                Bus::Ahb => Self::modify_reg(&self.rcc.ahbpcenr, or_mask, and_mask),
                _ => {}
            }
        }
        Ok(())
    }
}

rpc::rpc_impl_dispatch_for!(RccServer<'_> as rpc_ch32x0_rcc::DispatchImpl);

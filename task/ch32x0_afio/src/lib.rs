#![feature(naked_functions)]
#![no_std]

use ch32x0::ch32x035 as device;
use kernel_types::task_id;
use rpc_ch32x0_rcc::{Peripheral, Rcc};
use rtos_macros::rtos_task_entry;

#[panic_handler]
#[cfg(target_os = "none")]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[rtos_task_entry]
fn task_main() -> ! {
    let mut rcc_client = rpc_ch32x0_rcc::Client::new(task_id!("ch32x0_rcc"));
    rcc_client
        .peripheral_clock_enable(Peripheral::Afio, 1)
        .unwrap();

    let srv = AfioServer {
        afio: unsafe { &*device::AFIO::ptr() },
    };
    rpc::Server::new(srv).listen()
}

struct AfioServer<'a> {
    afio: &'a device::afio::RegisterBlock,
}

impl rpc::Notify for AfioServer<'_> {
    fn receive_notifications(&mut self, _notifications: u32) {
        panic!()
    }
}

impl rpc_ch32x0_afio::Afio<rpc::CallStatus> for AfioServer<'_> {
    fn modify_ctlr(&mut self, value: u32, mask: u32) -> Result<(), rpc::CallStatus> {
        assert_eq!(value, value & mask);
        self.afio
            .ctlr
            .modify(|r, w| unsafe { w.bits((r.bits() & !mask) | value) });
        Ok(())
    }
}

rpc::rpc_impl_dispatch_for!(AfioServer<'_> as rpc_ch32x0_afio::DispatchImpl);

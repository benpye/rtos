#![feature(naked_functions)]
#![no_std]

use ch32x0::ch32x035 as device;
use kernel_types::task_id;
use pac_qingke as _;
use rpc_adb_host::{ListenResult, TalkResult};
use rpc_ch32x0_rcc::{Peripheral, Rcc};
use rtos_macros::rtos_task_entry;

mod adb;

#[panic_handler]
#[cfg(target_os = "none")]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[rtos_task_entry]
fn task_main() -> ! {
    let mut rcc_client = rpc_ch32x0_rcc::Client::new(task_id!("ch32x0_rcc"));
    rcc_client
        .peripheral_clock_enable(Peripheral::Iopa, 1)
        .unwrap();

    let dp = unsafe { device::Peripherals::steal() };
    let adb = adb::Adb::new(dp.GPIOA);

    rpc::Server::new(AdbHostServer { adb }).listen()

    // dp.GPIOA
    //     .cfglr
    //     .modify(|_, w| w.cnf2().variant(0b00).mode2().variant(0b01));
    // let dp = unsafe { device::Peripherals::steal() };

    // for addr in 1..16 {
    //     let mut reg = [0, 0];
    //     let (service_request, bytes) = adb.talk(addr, 3, &mut reg).unwrap();

    // use core::fmt::Write;
    // let mut w = DebugWriter {};
    // let _ = writeln!(
    //     w,
    //     "device {}, reg[3] = {:x} {:x} (len {}) srq {}",
    //     addr, reg[0], reg[1], bytes, service_request
    // );
    // }

    // let _ = adb.listen(2, 3, &[0x02, 0x03]).unwrap();

    // let mut reg = [0, 0];
    // let (service_request, bytes) = adb.talk(2, 3, &mut reg).unwrap();

    // loop {}

    // use core::fmt::Write;
    // let mut w = DebugWriter {};
    // let _ = writeln!(
    //     w,
    //     "reg[3] = {:x} {:x} (len {}) srq {}",
    //     reg[0], reg[1], bytes, service_request
    // );

    // syscall::sys_set_timer(true, 12000);

    // let mut on = false;
    // let mut count = 0;

    // loop {
    //     let mut notifications = 0;
    //     let mut sender = 0;
    //     let mut len = 0;
    //     let mut data = [0; 16];
    //     // uart_puts("pre sys_receive");
    //     syscall::sys_receive(&mut notifications, &mut sender, &mut len, &mut data);
    //     // uart_puts("post sys_receive");

    //     if count % 20 == 0 {
    //         on = !on;
    //         dp.GPIOA.outdr.modify(|r, w| w.odr2().bit(!r.odr2().bit()));
    //     }

    //     let service_request = if on {
    //         adb.listen(2, 2, &[0xff, 0xff]).unwrap()
    //     } else {
    //         adb.listen(2, 2, &[0x00, 0x00]).unwrap()
    //     };

    //     let mut reg = [0, 0];
    //     let (service_request1, bytes) = adb.talk(2, 0, &mut reg).unwrap();

    //     if service_request || service_request1 {
    //         use core::fmt::Write;
    //         let mut w = DebugWriter {};
    //         let _ = writeln!(
    //             w,
    //             "reg[0] = {:x} {:x} (len {}) srq {} {}",
    //             reg[0], reg[1], bytes, service_request, service_request1
    //         );
    //     }

    //     count = count + 1;
    // }

    // loop {
    //     // delay_us(1_000_000);
    //     // uart_puts("hi");

    //     // wait_for_lo(10_000_000, &gpioa.indr);
    //     // gpioa.outdr.modify(|r, w| w.odr1().bit(!r.odr1().bit()));

    //     // uart_puts("hi");

    //     // wait_for_lo(1_000_000, &gpioa.indr);
    //     // uart_puts("hi");
    // }

    // syscall::sys_set_timer(true, 1000000);

    // loop {
    //     let mut notifications = 0;
    //     let mut sender = 0;
    //     let mut len = 0;
    //     let mut data = [0; 16];
    //     syscall::sys_receive(&mut notifications, &mut sender, &mut len, &mut data);

    //     if notifications & 0x80000000 != 0 {
    //         uart_putc('a');
    //     }
    // }
}

struct AdbHostServer {
    adb: adb::Adb,
}

impl rpc::Notify for AdbHostServer {
    fn receive_notifications(&mut self, _notifications: u32) {
        panic!()
    }
}

impl rpc_adb_host::AdbHost<rpc::CallStatus> for AdbHostServer {
    fn talk(
        &mut self,
        address: u8,
        register: u8,
    ) -> Result<rpc_adb_host::TalkResult, rpc::CallStatus> {
        let mut result = TalkResult {
            service_request: 0,
            len: 0,
            data: [0; 8],
        };

        let (service_request, len) = self.adb.talk(address, register, &mut result.data).unwrap();
        result.len = len as u8;
        result.service_request = service_request as u8;
        Ok(result)
    }

    fn listen(
        &mut self,
        address: u8,
        register: u8,
        len: u8,
        data: [u8; 8],
    ) -> Result<rpc_adb_host::ListenResult, rpc::CallStatus> {
        let mut result = ListenResult { service_request: 0 };

        let service_request = self
            .adb
            .listen(address, register, &data[..(len as usize)])
            .unwrap();
        result.service_request = service_request as u8;
        Ok(result)
    }
}

rpc::rpc_impl_dispatch_for!(AdbHostServer as rpc_adb_host::DispatchImpl);

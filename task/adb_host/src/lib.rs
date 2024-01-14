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

        let (service_request, len) = self
            .adb
            .talk(address, register, &mut result.data)
            .map_err(|_| rpc::CallStatus::OperationFailed)?;
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

        let len = len as usize;
        if len > data.len() {
            return Err(rpc::CallStatus::InvalidParameter);
        }

        let service_request = self
            .adb
            .listen(address, register, &data[..len as usize])
            .map_err(|_| rpc::CallStatus::OperationFailed)?;

        result.service_request = service_request as u8;
        Ok(result)
    }
}

rpc::rpc_impl_dispatch_for!(AdbHostServer as rpc_adb_host::DispatchImpl);

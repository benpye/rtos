#![feature(naked_functions)]
#![no_std]

use core::mem;

use rtos_macros::rtos_task_entry;

#[panic_handler]
#[cfg(target_os = "none")]
fn panic(info: &core::panic::PanicInfo) -> ! {
    semihosting::eprintln!("test_helper: {}", info);
    loop {}
}

#[rtos_task_entry]
fn task_main() -> ! {
    semihosting::println!("test_helper: start");

    let srv = TestHelperServer {
        notification_count: [0; 32],
        buffer: [0; 36],
    };
    rpc::Server::new(srv).listen()
}

struct TestHelperServer {
    notification_count: [u32; 32],
    buffer: [u8; 36],
}

impl rpc::Notify for TestHelperServer {
    fn receive_notifications(&mut self, notifications: u32) {
        let mut notifications = notifications;

        for bit in 0..32 {
            self.notification_count[bit] += notifications & 1;
            notifications >>= 1;
        }
    }
}

impl rpc_test_helper::TestHelper<rpc::CallStatus> for TestHelperServer {
    fn set_timer(&mut self, periodic: u8, deadline: u32) -> Result<(), rpc::CallStatus> {
        syscall::sys_set_timer(periodic > 0, deadline);
        Ok(())
    }

    fn notification_count(&mut self, bit: usize) -> Result<u32, rpc::CallStatus> {
        let element = self
            .notification_count
            .get_mut(bit)
            .ok_or(rpc::CallStatus::InvalidParameter)?;
        let count = mem::replace(element, 0);
        Ok(count)
    }

    fn swap_buffer(&mut self, buffer: [u8; 36]) -> Result<[u8; 36], rpc::CallStatus> {
        let old_buffer = self.buffer;
        self.buffer = buffer;
        Ok(old_buffer)
    }
}

rpc::rpc_impl_dispatch_for!(TestHelperServer as rpc_test_helper::DispatchImpl);

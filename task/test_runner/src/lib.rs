#![feature(naked_functions)]
#![no_std]

use kernel_types::task_id;
use rpc_test_helper::TestHelper;
use rtos_macros::rtos_task_entry;

#[panic_handler]
#[cfg(target_os = "none")]
fn panic(info: &core::panic::PanicInfo) -> ! {
    semihosting::eprintln!("test_runner: {}", info);
    loop {}
}

#[rtos_task_entry]
fn task_main() -> ! {
    semihosting::println!("test_runner: start");

    let mut client = rpc_test_helper::Client::new(task_id!("test_helper"));

    {
        for i in 0..32 {
            let expiration_count = client.notification_count(i).unwrap();
            assert_eq!(0, expiration_count);
        }
    }

    {
        let err = client.notification_count(32).unwrap_err();
        assert_eq!(rpc_test_helper::CallStatus::InvalidParameter, err);
    }

    {
        client.set_timer(0, 20_000).unwrap();

        syscall::sys_set_timer(false, 10_000);
        let _: syscall::ReceiveResult<0> = syscall::sys_receive();

        let expiration_count = client
            .notification_count(syscall::abi::SYS_NOTIFICATION_TIMER_BIT)
            .unwrap();
        assert_eq!(0, expiration_count);

        syscall::sys_set_timer(false, 10_000);
        let _: syscall::ReceiveResult<0> = syscall::sys_receive();

        let expiration_count = client
            .notification_count(syscall::abi::SYS_NOTIFICATION_TIMER_BIT)
            .unwrap();
        assert_eq!(1, expiration_count);

        syscall::sys_set_timer(false, 20_000);
        let _: syscall::ReceiveResult<0> = syscall::sys_receive();

        let expiration_count = client
            .notification_count(syscall::abi::SYS_NOTIFICATION_TIMER_BIT)
            .unwrap();
        assert_eq!(0, expiration_count);
    }

    {
        client.set_timer(1, 10_000).unwrap();

        syscall::sys_set_timer(false, 55_000);
        let _: syscall::ReceiveResult<0> = syscall::sys_receive();

        let expiration_count = client
            .notification_count(syscall::abi::SYS_NOTIFICATION_TIMER_BIT)
            .unwrap();
        assert_eq!(5, expiration_count);

        client.set_timer(0, 0).unwrap();

        syscall::sys_set_timer(false, 10_000);
        let _: syscall::ReceiveResult<0> = syscall::sys_receive();

        let expiration_count = client
            .notification_count(syscall::abi::SYS_NOTIFICATION_TIMER_BIT)
            .unwrap();
        assert_eq!(0, expiration_count);
    }

    {
        let notification_count = client.notification_count(0).unwrap();
        assert_eq!(0, notification_count);

        syscall::sys_notify(task_id!("test_helper"), 1);

        let notification_count = client.notification_count(0).unwrap();
        assert_eq!(0, notification_count);

        let notification_count = client.notification_count(0).unwrap();
        assert_eq!(1, notification_count);
    }

    {
        let notification_count = client.notification_count(0).unwrap();
        assert_eq!(0, notification_count);

        syscall::sys_notify(task_id!("test_helper"), 1);

        syscall::sys_set_timer(false, 10_000);
        let _: syscall::ReceiveResult<0> = syscall::sys_receive();

        let notification_count = client.notification_count(0).unwrap();
        assert_eq!(1, notification_count);
    }

    {
        let buffer = client
            .swap_buffer(core::array::from_fn(|i| (i + 1) as u8))
            .unwrap();
        for reg in buffer {
            assert_eq!(0, reg);
        }

        let buffer = client
            .swap_buffer(core::array::from_fn(|i| u8::MAX - (i as u8)))
            .unwrap();
        for (i, reg) in buffer.iter().enumerate() {
            assert_eq!((i + 1) as u8, *reg);
        }

        let buffer = client.swap_buffer([0; 36]).unwrap();
        for (i, reg) in buffer.iter().enumerate() {
            assert_eq!(u8::MAX - (i as u8), *reg);
        }
    }

    {
        for i in 0..32 {
            let expiration_count = client.notification_count(i).unwrap();
            assert_eq!(0, expiration_count);
        }
    }

    semihosting::println!("test_runner: finish");
    panic!();
}

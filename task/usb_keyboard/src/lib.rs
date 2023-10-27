#![feature(naked_functions)]
#![no_std]

use ch32x0::ch32x035 as device;
use kernel_types::task_id;
use rpc_adb_host::{AdbHost, ListenResult, TalkResult};
use rpc_ch32x0_afio::Afio;
use rpc_ch32x0_rcc::{Peripheral, Rcc};
use rtos_macros::rtos_task_entry;
use usb_device::{
    class_prelude::UsbBusAllocator,
    prelude::{UsbDeviceBuilder, UsbVidPid},
};
use usbd_human_interface_device::usb_class::UsbHidClassBuilder;

fn uart_putc(c: char) {
    let usart1 = unsafe { &*device::USART1::ptr() };
    while usart1.statr.read().tc().bit_is_clear() {}
    usart1.datar.write(|w| w.dr().variant(c as _));
}

fn uart_puts(s: &str) {
    for c in s.chars() {
        uart_putc(c);
    }
}

pub struct DebugWriter;

impl core::fmt::Write for DebugWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        uart_puts(s);
        Ok(())
    }
}

#[panic_handler]
#[cfg(target_os = "none")]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    use core::fmt::Write;
    let mut w = DebugWriter {};
    let _ = write!(w, "{}", _info);
    loop {}
}

static EP_MEM: ch32x035_usb::EndpointMemory<256> = ch32x035_usb::EndpointMemory::new();

#[rtos_task_entry]
fn task_main() -> ! {
    let usart1 = unsafe { &*device::USART1::ptr() };

    let mut rcc_client = rpc_ch32x0_rcc::Client::new(task_id!("ch32x0_rcc"));
    rcc_client
        .peripheral_clock_enable(Peripheral::Usart1, 1)
        .unwrap();

    usart1
        .ctlr1
        .modify(|_, w| w.te().set_bit().rxneie().set_bit().re().set_bit());

    let apbclk = 8_000_000;
    let baudrate = 115_200;
    let integer_divider = (25 * apbclk) / (4 * baudrate);
    let mut tmpreg = (integer_divider / 100) << 4;
    let fractional_divider = integer_divider - (100 * (tmpreg >> 4));
    tmpreg |= (((fractional_divider * 16) + 50) / 100) & 0x0F;

    usart1.brr.write(|w| unsafe { w.bits(tmpreg) });
    usart1.ctlr1.modify(|_, w| w.ue().set_bit());

    uart_puts("test_task start\n");

    rcc_client
        .peripheral_clock_enable(Peripheral::UsbFs, 1)
        .unwrap();

    // gpioc.cfgxr.modify(|_, w| {
    //     w.cnf16()
    //         .variant(0b10)
    //         .mode16()
    //         .variant(0b00)
    //         .cnf17()
    //         .variant(0b10)
    //         .mode17()
    //         .variant(0b00)
    // });
    // gpioc.bsxr.write(|w| w.bs17().set_bit().bs16().set_bit());
    // 3.3V VDD
    // afio.ctlr.modify(|_, w| {
    //     w.usb_ioen()
    //         .set_bit()
    //         .udp_pue()
    //         .variant(0b11)
    //         .udm_pue()
    //         .variant(0b00)
    // });

    let mut afio_client = rpc_ch32x0_afio::Client::new(task_id!("ch32x0_afio"));
    afio_client.modify_ctlr(0b11001000, 0b11001111).unwrap();

    // afio.ctlr.modify(|_, w| {
    //     w.usb_ioen()
    //         .set_bit()
    //         .usb_phy_v33()
    //         .clear_bit()
    //         .udp_pue()
    //         .variant(0b10)
    //         .udm_pue()
    //         .variant(0b00)
    // });

    let usbfs = unsafe { device::Peripherals::steal().USBFS };

    let usb_bus = UsbBusAllocator::new(ch32x035_usb::UsbBus::new(usbfs, &EP_MEM));
    let mut keyboard = UsbHidClassBuilder::new()
        .add_device(usbd_human_interface_device::device::keyboard::BootKeyboardConfig::default())
        .build(&usb_bus);
    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x1209, 0x0001))
        .manufacturer("usbd-human-interface-device")
        .product("Boot Keyboard")
        .serial_number("TEST")
        .build();

    uart_puts("usb keyboard constructed\n");

    let mut adb_host = rpc_adb_host::Client::new(task_id!("adb_host"));

    for addr in 1..16 {
        let TalkResult {
            service_request,
            len,
            data,
        } = adb_host.talk(addr, 3).unwrap();

        use core::fmt::Write;
        let mut w = DebugWriter {};
        let _ = writeln!(
            w,
            "device {}, reg[3] = {:x} {:x} (len {}) srq {}",
            addr, data[0], data[1], len, service_request
        );
    }

    syscall::sys_set_timer(true, 1000);

    syscall::sys_interrupt_control(
        device::Interrupt::USART1 as usize,
        syscall::InterruptControl::Enable,
    );
    syscall::sys_interrupt_control(
        device::Interrupt::USBFS as usize,
        syscall::InterruptControl::Enable,
    );
    syscall::sys_interrupt_control(
        device::Interrupt::USBFS_WKUP as usize,
        syscall::InterruptControl::Enable,
    );

    let mut count = 0;

    let mut key = usbd_human_interface_device::page::Keyboard::NoEventIndicated;

    loop {
        let syscall::ReceiveResult { notifications, .. } = syscall::sys_receive::<0>();

        if notifications & 0x80000000 != 0 {
            if count % 500 == 0 {
                if key == usbd_human_interface_device::page::Keyboard::CapsLock {
                    key = usbd_human_interface_device::page::Keyboard::NoEventIndicated;
                } else {
                    key = usbd_human_interface_device::page::Keyboard::CapsLock;
                }
            }

            if count % 10 == 0 {
                match keyboard.device().write_report([key]) {
                    Err(usbd_human_interface_device::UsbHidError::WouldBlock) => {}
                    Err(usbd_human_interface_device::UsbHidError::Duplicate) => {}
                    Ok(_) => {}
                    Err(e) => {
                        core::panic!("Failed to write keyboard report: {:?}", e)
                    }
                };
            }

            match keyboard.tick() {
                Ok(_) => {}
                Err(usbd_human_interface_device::UsbHidError::WouldBlock) => {}
                Err(e) => {
                    core::panic!("Failed to process keyboard tick: {:?}", e)
                }
            }

            if count % 1000 == 0 {
                uart_putc('t');
            }

            count = count + 1;
        }

        if notifications & 4 != 0 {
            if usart1.statr.read().rxne().bit_is_set() {
                uart_putc(usart1.datar.read().bits() as u8 as char);
            }

            syscall::sys_interrupt_control(
                device::Interrupt::USART1 as usize,
                syscall::InterruptControl::Complete,
            );
        }

        if notifications & 1 != 0 {
            if usb_dev.poll(&mut [&mut keyboard]) {
                match keyboard.device().read_report() {
                    Err(usb_device::UsbError::WouldBlock) => {
                        //do nothing
                    }
                    Err(e) => {
                        core::panic!("Failed to read keyboard report: {:?}", e)
                    }
                    Ok(leds) => {
                        if leds.caps_lock {
                            uart_putc('C');
                        } else {
                            uart_putc('c');
                        }
                    }
                }
            }

            syscall::sys_interrupt_control(
                device::Interrupt::USBFS as usize,
                syscall::InterruptControl::Complete,
            );
        }

        if notifications & 2 != 0 {
            syscall::sys_interrupt_control(
                device::Interrupt::USBFS_WKUP as usize,
                syscall::InterruptControl::Complete,
            );
        }
    }
}

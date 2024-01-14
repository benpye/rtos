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
use usbd_human_interface_device::{
    device::keyboard::KeyboardLedsReport, page::Keyboard, usb_class::UsbHidClassBuilder,
};

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

    // 3.3V VDD
    // afio.ctlr.modify(|_, w| {
    //     w.usb_ioen()
    //         .set_bit()
    //         .udp_pue()
    //         .variant(0b11)
    //         .udm_pue()
    //         .variant(0b00)
    // });
    // 5V VDD
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

    let mut afio_client = rpc_ch32x0_afio::Client::new(task_id!("ch32x0_afio"));
    afio_client.modify_ctlr(0b11001000, 0b11001111).unwrap();

    let usbfs = unsafe { device::Peripherals::steal().USBFS };
    let usb_bus = UsbBusAllocator::new(ch32x035_usb::UsbBus::new(usbfs, &EP_MEM));

    let mut keyboard = UsbHidClassBuilder::new()
        .add_device(
            usbd_human_interface_device::device::keyboard::NKROBootKeyboardConfig::default(),
        )
        .build(&usb_bus);

    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x1209, 0x0001))
        .manufacturer("hresult.dev")
        .product("ADB to USB-C")
        .serial_number("00000000")
        .build();

    uart_puts("usb keyboard constructed\n");

    // Wait 500ms before attempting to interact with keyboard
    syscall::sys_set_timer(false, 500 * 1000);
    syscall::sys_receive::<0>();

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

    // Enable extended mode
    adb_host
        .listen(2, 3, 2, [0x02, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00])
        .unwrap();

    {
        let rpc_adb_host::TalkResult { len, data, .. } = adb_host.talk(2, 3).unwrap();

        use core::fmt::Write;
        let mut w = DebugWriter {};
        let _ = writeln!(w, "len {}, data = [{:x}, {:x}, ..]", len, data[0], data[1],);
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

    let mut count = 0u32;

    // The NKRO report has 17 keys
    let mut down_keys = [Keyboard::NoEventIndicated; 17];
    let mut adb_leds = 0xffu8;

    fn update_down_keys(scan: u8, down_keys: &mut [Keyboard; 17]) {
        let released = scan & 0x80 != 0;
        let scan = scan & 0x7f;

        let lut = [
            Keyboard::A,                     //0x04,	// 0
            Keyboard::S,                     //0x16,	// 1
            Keyboard::D,                     //0x07,	// 2
            Keyboard::F,                     //0x09,	// 3
            Keyboard::H,                     //0x0b,	// 4
            Keyboard::G,                     //0x0a,	// 5
            Keyboard::Z,                     //0x1d,	// 6
            Keyboard::X,                     //0x1b,	// 7
            Keyboard::C,                     //0x06,	// 8
            Keyboard::V,                     //0x19,	// 9
            Keyboard::NonUSBackslash,        //0x64,	// a
            Keyboard::B,                     //0x05,	// b
            Keyboard::Q,                     //0x14,	// c
            Keyboard::W,                     //0x1a,	// d
            Keyboard::E,                     //0x08,	// e
            Keyboard::R,                     //0x15,	// f
            Keyboard::Y,                     //0x1c,	// 10
            Keyboard::T,                     //0x17,	// 11
            Keyboard::Keyboard1,             //0x1e,	// 12
            Keyboard::Keyboard2,             //0x1f,	// 13
            Keyboard::Keyboard3,             //0x20,	// 14
            Keyboard::Keyboard4,             //0x21,	// 15
            Keyboard::Keyboard6,             //0x23,	// 16
            Keyboard::Keyboard5,             //0x22,	// 17
            Keyboard::Equal,                 //0x2e,	// 18
            Keyboard::Keyboard9,             //0x26,	// 19
            Keyboard::Keyboard7,             //0x24,	// 1a
            Keyboard::Minus,                 //0x2d,	// 1b
            Keyboard::Keyboard8,             //0x25,	// 1c
            Keyboard::Keyboard0,             //0x27,	// 1d
            Keyboard::RightBrace,            //0x30,	// 1e
            Keyboard::O,                     //0x12,	// 1f
            Keyboard::U,                     //0x18,	// 20
            Keyboard::LeftBrace,             //0x2f,	// 21
            Keyboard::I,                     //0x0c,	// 22
            Keyboard::P,                     //0x13,	// 23
            Keyboard::ReturnEnter,           //0x28,	// 24
            Keyboard::L,                     //0x0f,	// 25
            Keyboard::J,                     //0x0d,	// 26
            Keyboard::Apostrophe,            //0x34,	// 27
            Keyboard::K,                     //0x0e,	// 28
            Keyboard::Semicolon,             //0x33,	// 29
            Keyboard::Backslash,             //0x31,	// 2a
            Keyboard::Comma,                 //0x36,	// 2b
            Keyboard::ForwardSlash,          //0x38,	// 2c
            Keyboard::N,                     //0x11,	// 2d
            Keyboard::M,                     //0x10,	// 2e
            Keyboard::Dot,                   //0x37,	// 2f
            Keyboard::Tab,                   //0x2b,	// 30
            Keyboard::Space,                 //0x2c,	// 31
            Keyboard::Grave,                 //0x35,	// 32
            Keyboard::DeleteBackspace,       //0x2a,	// 33
            Keyboard::KeypadEnter,           //0x58,	// 34
            Keyboard::Escape,                //0x29,	// 35
            Keyboard::LeftControl,           //0xe7,	// 36
            Keyboard::LeftGUI,               //0xe3,	// 37
            Keyboard::LeftShift,             //0xe1,	// 38
            Keyboard::CapsLock,              //0x39,	// 39
            Keyboard::LeftAlt,               //0xe2,	// 3a
            Keyboard::LeftArrow,             //0xe0,	// 3b
            Keyboard::RightArrow,            //0xe5,	// 3c
            Keyboard::DownArrow,             //0xe6,	// 3d
            Keyboard::UpArrow,               //0xe4,	// 3e
            Keyboard::NoEventIndicated,      //0x00,	// 3f
            Keyboard::F17,                   //0x6c,	// 40
            Keyboard::KeypadDot,             //0x63,	// 41
            Keyboard::NoEventIndicated,      //0x00,	// 42
            Keyboard::KeypadMultiply,        //0x55,	// 43
            Keyboard::F18,                   //0x6d,	// 44
            Keyboard::KeypadAdd,             //0x57,	// 45
            Keyboard::NoEventIndicated,      //0x00,	// 46
            Keyboard::KeypadNumLockAndClear, //0x53,	// 47
            Keyboard::NoEventIndicated,      //0x00,	// 48
            Keyboard::NoEventIndicated,      //0x00,	// 49
            Keyboard::NoEventIndicated,      //0x00,	// 4a
            Keyboard::KeypadDivide,          //0x54,	// 4b
            Keyboard::KeypadEnter,           //0x58,	// 4c
            Keyboard::NoEventIndicated,      //0x00,	// 4d
            Keyboard::KeypadSubtract,        //0x56,	// 4e
            Keyboard::F18,                   //0x6d,	// 4f
            Keyboard::F19,                   //0x6e,	// 50
            Keyboard::KeypadEqual,           //0x67,	// 51
            Keyboard::Keypad0,               //0x62,	// 52
            Keyboard::Keypad1,               //0x59,	// 53
            Keyboard::Keypad2,               //0x5a,	// 54
            Keyboard::Keypad3,               //0x5b,	// 55
            Keyboard::Keypad4,               //0x5c,	// 56
            Keyboard::Keypad5,               //0x5d,	// 57
            Keyboard::Keypad6,               //0x5e,	// 58
            Keyboard::Keypad7,               //0x5f,	// 59
            Keyboard::F20,                   //0x6f,	// 5a
            Keyboard::Keypad8,               //0x60,	// 5b
            Keyboard::Keypad9,               //0x61,	// 5c
            Keyboard::Kanji3,                //0x89,	// 5d
            Keyboard::Kanji1,                //0x87,	// 5e
            Keyboard::KeypadComma,           //0x85,	// 5f
            Keyboard::F5,                    //0x3e,	// 60
            Keyboard::F6,                    //0x3f,	// 61
            Keyboard::F7,                    //0x40,	// 62
            Keyboard::F3,                    //0x3c,	// 63
            Keyboard::F8,                    //0x41,	// 64
            Keyboard::F9,                    //0x42,	// 65
            Keyboard::LANG2,                 //0x91,	// 66
            Keyboard::F11,                   //0x44,	// 67
            Keyboard::LANG1,                 //0x90,	// 68
            Keyboard::F13,                   //0x68,	// 69
            Keyboard::F16,                   //0x6b,	// 6a
            Keyboard::F14,                   //0x69,	// 6b
            Keyboard::NoEventIndicated,      //0x0,	// 6c
            Keyboard::F10,                   //0x43,	// 6d
            Keyboard::Application,           //0x65,	// 6e  Microsoft Winodows95 key
            Keyboard::F12,                   //0x45,	// 6f
            Keyboard::NoEventIndicated,      //0x0,	// 70
            Keyboard::F15,                   //0x6a,	// 71
            Keyboard::Help,                  //0x75,	// 72
            Keyboard::Home,                  //0x4a,	// 73
            Keyboard::PageUp,                //0x4b,	// 74
            Keyboard::DeleteForward,         //0x4c,	// 75
            Keyboard::F4,                    //0x3d,	// 76
            Keyboard::End,                   //0x4d,	// 77
            Keyboard::F2,                    //0x3b,	// 78
            Keyboard::PageDown,              //0x4e,	// 79
            Keyboard::F1,                    //0x3a,	// 7a
            Keyboard::RightShift,            //0x50,	// 7b
            Keyboard::RightAlt,              //0x4f,	// 7c
            Keyboard::RightControl,          //0x51,	// 7d
            Keyboard::UpArrow,               //0x52,	// 7e
            Keyboard::Power,                 //0x66,	// 7f
        ];

        let keycode = lut[scan as usize];

        if released {
            for code in down_keys {
                if *code == keycode {
                    *code = Keyboard::NoEventIndicated;
                }
            }
        } else {
            for code in down_keys {
                if *code == Keyboard::NoEventIndicated {
                    *code = keycode;
                    break;
                }
            }
        }
    }

    loop {
        let syscall::ReceiveResult { notifications, .. } = syscall::sys_receive::<0>();

        // Timer interrupt
        if notifications & 0x80000000 != 0 {
            if count == 0 {
                // Update ADB keyboard leds
                adb_host
                    .listen(
                        2,
                        2,
                        2,
                        [0x00, adb_leds, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
                    )
                    .unwrap();

                // Query ADB keyboard
                let rpc_adb_host::TalkResult { len, data, .. } = adb_host.talk(2, 0).unwrap();

                if (data[0] | data[1]) != 0 {
                    update_down_keys(data[0], &mut down_keys);

                    if data[1] != 0xff {
                        update_down_keys(data[1], &mut down_keys);
                    }
                }

                match keyboard.device().write_report(down_keys) {
                    Err(usbd_human_interface_device::UsbHidError::WouldBlock) => {}
                    Err(usbd_human_interface_device::UsbHidError::Duplicate) => {}
                    Ok(_) => {}
                    Err(e) => {
                        core::panic!("Failed to write keyboard report: {:?}", e)
                    }
                };

                let data = ((data[0] as u16) << 8) | (data[1] as u16);
                if data != 0 {
                    use core::fmt::Write;
                    let mut w = DebugWriter {};
                    let _ = writeln!(
                        w,
                        "adb_leds = {:x}, len {}, data = {:x}, keys = [{:?}, {:?}, {:?}, {:?}, ..]",
                        adb_leds, len, data, down_keys[0], down_keys[1], down_keys[2], down_keys[3],
                    );
                }
            }

            count = (count + 1) % 10;

            match keyboard.tick() {
                Ok(_) => {}
                Err(usbd_human_interface_device::UsbHidError::WouldBlock) => {}
                Err(e) => {
                    core::panic!("Failed to process keyboard tick: {:?}", e)
                }
            }
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
                        adb_leds = 0xff;

                        if leds.num_lock {
                            adb_leds &= !0x01;
                        }

                        if leds.caps_lock {
                            adb_leds &= !0x02;
                        }

                        if leds.scroll_lock {
                            adb_leds &= !0x04;
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

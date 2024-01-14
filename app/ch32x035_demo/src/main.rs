#![no_std]
#![no_main]

use ch32x0::ch32x035 as device;

include!(concat!(env!("OUT_DIR"), "/app.rs"));

#[export_name = "rtos.app_init"]
extern "C" fn app_init() {
    let rcc = unsafe { &*device::RCC::ptr() };
    let gpiob = unsafe { &*device::GPIOB::ptr() };
    let gpioc = unsafe { &*device::GPIOC::ptr() };

    rcc.apb2pcenr
        .modify(|_, w| w.iopben().set_bit().iopcen().set_bit());

    gpiob.cfghr.modify(|_, w| {
        w.cnf10()
            .variant(0b10)
            .mode10()
            .variant(0b11)
            .cnf11()
            .variant(0b01)
            .mode11()
            .variant(0b00)
    });

    gpioc.cfgxr.modify(|_, w| {
        w.cnf16()
            .variant(0b10)
            .mode16()
            .variant(0b00)
            .cnf17()
            .variant(0b10)
            .mode17()
            .variant(0b00)
    });

    gpioc.bsxr.write(|w| w.bs17().set_bit().bs16().set_bit());
}

//! Serial loopback via USART1

#![deny(warnings)]
#![feature(proc_macro)]
#![no_std]

extern crate blue_pill;
#[macro_use(task)]
extern crate cortex_m_rtfm as rtfm;

use blue_pill::Serial;
use blue_pill::prelude::*;
use blue_pill::serial::Event;
use blue_pill::time::Hertz;
use rtfm::{Threshold, app};

// CONFIGURATION
pub const BAUD_RATE: Hertz = Hertz(115_200);

app! {
    device: blue_pill::stm32f103xx,

    tasks: {
        USART1: {
            enabled: true,
            priority: 1,
            resources: [USART1],
        },
    },
}

fn init(p: init::Peripherals) {
    let serial = Serial(p.USART1);

    serial.init(BAUD_RATE.invert(), p.AFIO, None, p.GPIOA, p.RCC);
    serial.listen(Event::Rxne);
}

fn idle() -> ! {
    loop {
        rtfm::wfi();
    }
}

task!(USART1, loopback);

fn loopback(_t: Threshold, r: USART1::Resources) {
    let serial = Serial(r.USART1);

    let byte = serial.read().unwrap();
    serial.write(byte).unwrap();
}

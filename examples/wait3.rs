//! Periodic timeouts with TIM3

#![deny(warnings)]
#![feature(proc_macro)]
#![no_std]

extern crate blue_pill;
extern crate cortex_m_rtfm as rtfm;

use blue_pill::Timer;
use blue_pill::led::{self, Green};
use blue_pill::prelude::*;
use blue_pill::time::Hertz;
use rtfm::app;

const FREQUENCY: Hertz = Hertz(1);

app! {
    device: blue_pill::stm32f103xx,

    idle: {
        resources: [TIM3],
    },
}

fn init(p: init::Peripherals) {
    let timer = Timer(p.TIM3);

    led::init(p.GPIOC, p.RCC);

    timer.init(FREQUENCY.invert(), p.RCC);
    timer.resume();
}

fn idle(r: idle::Resources) -> ! {
    let timer = Timer(r.TIM3);

    let mut state = false;
    loop {
        while timer.wait().is_err() {}

        state = !state;

        if state {
            Green.on();
        } else {
            Green.off();
        }
    }
}

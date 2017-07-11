//! Periodic timeouts with TIM1

#![deny(warnings)]
#![feature(plugin)]
#![no_std]
#![plugin(cortex_m_rtfm_macros)]

extern crate blue_pill;
extern crate cortex_m_rtfm as rtfm;

use blue_pill::Timer;
use blue_pill::led::{self, Green};
use blue_pill::prelude::*;
use blue_pill::time::Hertz;

// CONFIGURATION
const FREQUENCY: Hertz = Hertz(1);

rtfm! {
    device: blue_pill::stm32f103xx,

    init: {
        path: init,
    },

    idle: {
        path: idle,
        resources: [TIM1],
    },
}

fn init(p: init::Peripherals) {
    let timer = Timer(p.TIM1);

    led::init(p.GPIOC, p.RCC);

    timer.init(FREQUENCY.invert(), p.RCC);
    timer.resume();
}

fn idle(r: idle::Resources) -> ! {
    let timer = Timer(r.TIM1);

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

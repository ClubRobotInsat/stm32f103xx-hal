//! Input capture using TIM3

#![deny(warnings)]
#![feature(const_fn)]
#![feature(used)]
#![no_std]

extern crate blue_pill;

#[macro_use]
extern crate cortex_m;

// version = "0.2.3"
extern crate cortex_m_rt;

// version = "0.1.0"
#[macro_use]
extern crate cortex_m_rtfm as rtfm;

use blue_pill::stm32f103xx;
use blue_pill::{Capture, Channel};
use rtfm::{P0, T0, TMax};

// CONFIGURATION
const FREQUENCY: u32 = 1_000;

// RESOURCES
peripherals!(stm32f103xx, {
    AFIO: Peripheral {
        ceiling: C0,
    },
    GPIOA: Peripheral {
        ceiling: C0,
    },
    ITM: Peripheral {
        ceiling: C0,
    },
    RCC: Peripheral {
        ceiling: C0,
    },
    TIM3: Peripheral {
        ceiling: C0,
    },
});

// INITIALIZATION PHASE
fn init(ref prio: P0, thr: &TMax) {
    let afio = &AFIO.access(prio, thr);
    let gpioa = &GPIOA.access(prio, thr);
    let rcc = &RCC.access(prio, thr);
    let tim3 = TIM3.access(prio, thr);

    let capture = Capture(&*tim3);

    capture.init(FREQUENCY, afio, gpioa, rcc);
}

// IDLE LOOP
fn idle(ref prio: P0, ref thr: T0) -> ! {
    const CHANNELS: [Channel; 2] = [Channel::_1, Channel::_2];

    let itm = ITM.access(prio, thr);
    let tim3 = TIM3.access(prio, thr);

    let capture = Capture(&*tim3);

    for c in &CHANNELS {
        capture.enable(*c);
    }

    loop {
        for c in &CHANNELS {
            if let Ok(n) = capture.capture(*c) {
                iprintln!(&itm.stim[0], "{:?}: {}", c, n);
            }
        }
    }
}

// TASKS
tasks!(stm32f103xx, {});

//! Quadrature Encoder Interface using TIM3
//!
//! Periodically reports the readings of the QEI
#![deny(unsafe_code)]
#![deny(warnings)]
#![feature(proc_macro)]
#![no_std]

extern crate blue_pill;
#[macro_use(iprint, iprintln)]
extern crate cortex_m;
extern crate cortex_m_rtfm as rtfm;

use blue_pill::time::Hertz;
use blue_pill::{Qei, Timer};
use blue_pill::prelude::*;
use rtfm::{app, Threshold};

const FREQUENCY: Hertz = Hertz(1);

app! {
    device: blue_pill::stm32f103xx,

    resources: {
        static PREVIOUS: Option<u16> = None;
    },

    tasks: {
        TIM4: {
            path: periodic,
            resources: [ITM, PREVIOUS, TIM3, TIM4],
        },
    },
}

fn init(p: init::Peripherals, _r: init::Resources) {
    let qei = Qei(p.TIM3);
    let timer = Timer(p.TIM4);

    qei.init(p.AFIO, p.GPIOA, p.RCC);

    timer.init(FREQUENCY.invert(), p.RCC);
    timer.resume();
}

fn idle() -> ! {
    loop {
        rtfm::wfi();
    }
}

fn periodic(_t: &mut Threshold, r: TIM4::Resources) {
    let qei = Qei(&**r.TIM3);
    let timer = Timer(&**r.TIM4);

    timer.wait().unwrap();

    let curr = qei.count();
    let dir = qei.direction();

    if let Some(prev) = r.PREVIOUS.take() {
        let speed = (curr as i16).wrapping_sub(prev as i16);

        iprintln!(&r.ITM.stim[0], "{} - {} - {:?}", curr, speed, dir);
    }

    **r.PREVIOUS = Some(curr);
}

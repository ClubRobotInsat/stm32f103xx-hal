//! Quadrature Encoder Interface using TIM4
//!
//! Periodically reports the readings of the QEI

#![deny(unsafe_code)]
#![deny(warnings)]
#![feature(const_fn)]
#![feature(proc_macro)]
#![no_std]

extern crate blue_pill;
#[macro_use(iprint, iprintln)]
extern crate cortex_m;
#[macro_use(task)]
extern crate cortex_m_rtfm as rtfm;

use blue_pill::time::Hertz;
use blue_pill::{Qei, Timer};
use blue_pill::prelude::*;
use rtfm::{app, Threshold};

const FREQUENCY: Hertz = Hertz(1);

app! {
    device: blue_pill::stm32f103xx,

    tasks: {
        TIM1_UP_TIM10: {
            enabled: true,
            priority: 1,
            resources: [ITM, TIM1, TIM4],
        },
    },
}

fn init(p: init::Peripherals) {
    let qei = Qei(p.TIM4);
    let timer = Timer(p.TIM1);

    qei.init(p.AFIO, p.GPIOB, p.RCC);

    timer.init(FREQUENCY.invert(), p.RCC);
    timer.resume();
}

fn idle() -> ! {
    loop {
        rtfm::wfi();
    }
}

task!(TIM1_UP_TIM10, periodic, Locals {
    static PREVIOUS: Option<u16> = None;
});

fn periodic(_t: &mut Threshold, l: &mut Locals, r: TIM1_UP_TIM10::Resources) {
    let qei = Qei(&**r.TIM4);
    let timer = Timer(&**r.TIM1);

    timer.wait().unwrap();

    let curr = qei.count();
    let dir = qei.direction();

    if let Some(prev) = l.PREVIOUS.take() {
        let speed = (curr as i16).wrapping_sub(prev as i16);

        iprintln!(&r.ITM.stim[0], "{} - {} - {:?}", curr, speed, dir);
    }

    *l.PREVIOUS = Some(curr);
}

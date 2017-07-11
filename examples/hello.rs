//! Prints "Hello" and then "World" on the OpenOCD console

#![deny(warnings)]
#![feature(const_fn)]
#![feature(plugin)]
#![no_std]
#![plugin(cortex_m_rtfm_macros)]

extern crate blue_pill;
extern crate cortex_m;
extern crate cortex_m_rtfm as rtfm;
extern crate cortex_m_semihosting;

use core::fmt::Write;

use cortex_m_semihosting::hio::{self, HStdout};

rtfm! {
    device: blue_pill::stm32f103xx,

    resources: {
        HSTDOUT: Option<HStdout> = None;
    },

    init: {
        path: init,
    },

    idle: {
        path: idle,
        resources: [HSTDOUT],
    },
}

// INITIALIZATION PHASE
fn init(_p: init::Peripherals, r: init::Resources) {
    let mut hstdout = hio::hstdout().unwrap();

    writeln!(hstdout, "Hello").unwrap();

    **r.HSTDOUT = Some(hstdout);
}

// IDLE LOOP
fn idle(r: idle::Resources) -> ! {
    if let Some(mut hstdout) = r.HSTDOUT.take() {
        writeln!(hstdout, "World").unwrap();
    }

    // Sleep
    loop {
        rtfm::wfi();
    }
}

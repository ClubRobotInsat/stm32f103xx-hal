//! Test the USART2 instance
//!
//! Connect the TX and RX pins to run this test

#![deny(warnings)]
#![feature(plugin)]
#![plugin(cortex_m_rtfm_macros)]
#![no_std]

extern crate blue_pill;
extern crate cortex_m_rtfm as rtfm;
extern crate nb;

use blue_pill::Serial;
use blue_pill::time::Hertz;
use blue_pill::prelude::*;
use nb::Error;

// CONFIGURATION
pub const BAUD_RATE: Hertz = Hertz(115_200);

rtfm! {
    device: blue_pill::stm32f103xx,

    init: {
        path: init,
    },

    idle: {
        path: idle,
    },
}

fn init(p: init::Peripherals) {
    let serial = Serial(p.USART2);

    serial.init(BAUD_RATE.invert(), p.AFIO, None, p.GPIOA, p.RCC);

    const BYTE: u8 = b'A';

    assert!(serial.write(BYTE).is_ok());

    for _ in 0..1_000 {
        match serial.read() {
            Ok(byte) => {
                assert_eq!(byte, BYTE);
                return;
            }
            Err(Error::Other(e)) => panic!("{:?}", e),
            Err(Error::WouldBlock) => continue,
        }
    }

    panic!("Timeout")
}

fn idle() -> ! {
    // OK
    rtfm::bkpt();

    // Sleep
    loop {
        rtfm::wfi();
    }
}

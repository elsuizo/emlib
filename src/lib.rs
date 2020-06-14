#![no_std]
#![crate_type = "lib"]
#![crate_name = "emlib"]
#![allow(warnings)]
#[macro_use]

// emlib bindings
pub mod acmp;
pub mod adc;
pub mod chip;
pub mod cmu;
pub mod dma;
pub mod ebi;
pub mod emu;
pub mod gpio;
pub mod i2c;
pub mod irq;
pub mod lesense;
pub mod leuart;
pub mod prs;
pub mod rtc;
pub mod timer;
pub mod usart;

mod std {
    pub use core::*;
}

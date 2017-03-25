//! Example application
//!
//! This shows all the parts that are required to build an application

// We don't link to the `std` crate because it's not available for Cortex-M
// devices.
#![no_std]

// Necessary to pull in `bkpt!()`
#![feature(asm)]

// We have to link our crate, obviously
#[macro_use]
extern crate demopill;

extern crate stm32f103xx;

// Instead of `std` we use the `core` crate, which provides the subset of
// `std`'s functionality that works on bare metal environments
use core::u32;

use demopill::exceptions::{self, Exceptions};
use demopill::interrupts::{self, Interrupts};

// We need a `main` function, just like every other Rust program
fn main() {
    // "break main" doesn't seem to trigger?
    // unsafe {
    //     bkpt!();
    // }

    init_led();

    let mut gpioc = stm32f103xx::GPIOC.get();
    loop {
        unsafe {
            for _ in 0..10_000 {};
            (*gpioc).bsrr.write(|w| w.bits(0x2000_0000));  // .br13();
            for _ in 0..10_000 {};
            (*gpioc).bsrr.write(|w| w.bits(0x0000_2000));  // .bs13();
        }
    }

}

fn init_led() {
    unsafe {
        let mut gpioc = stm32f103xx::GPIOC.get();
        let mut rcc = stm32f103xx::RCC.get();

        // Enable peripheral clock
        (*rcc).apb2enr.modify(|_, w| w.bits(0xFF)); // FF? or 01? looks like the masking happens anyway

        // gpioc: Configure pin 13 as output
        (*gpioc).crh.modify(|_, w| {
            w.bits(0x4454_4444) // todo, don't set all?
        });
    }

}

// The program must specify how exceptions will be handled
// Here we just use the default handler to handle all the exceptions
#[no_mangle]
pub static _EXCEPTIONS: Exceptions =
    Exceptions { ..exceptions::DEFAULT_HANDLERS };

// Likewise with interrupts
#[no_mangle]
pub static _INTERRUPTS: Interrupts =
    Interrupts { ..interrupts::DEFAULT_HANDLERS };

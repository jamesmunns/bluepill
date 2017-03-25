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
        for _ in 0..10_000 {};
        led_off();

        for _ in 0..10_000 {};
        led_on();
    }

}

fn led_on() {
    let mut gpioc = stm32f103xx::GPIOC.get();
    unsafe {
        (*gpioc).bsrr.write(|w| {
            w.br13();
            w
        });
    }
}

fn led_off() {
    let mut gpioc = stm32f103xx::GPIOC.get();
    unsafe {
        (*gpioc).bsrr.write(|w| {
            w.bs13();
            w
        });
    }

}

fn init_led() {
    unsafe {
        let mut gpioc = stm32f103xx::GPIOC.get();
        let mut rcc = stm32f103xx::RCC.get();

        // Enable peripheral clock
        (*rcc).apb2enr.write(|w| {
            w.iopcen().bits(0b1)
        });

        // gpioc: Configure pin 13 as output
        (*gpioc).crh.write(|w| {
            w.mode13().bits(0b10)
        })
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

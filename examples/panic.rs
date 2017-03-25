//! What happens when `panic!` is invoked?
//!
//! Find out with this app

#![no_std]

extern crate demopill;

use demopill::exceptions::{self, Exceptions};
use demopill::interrupts::{self, Interrupts};

fn main() {
    panic!()
}

#[no_mangle]
pub static _EXCEPTIONS: Exceptions =
    Exceptions { ..exceptions::DEFAULT_HANDLERS };

#[no_mangle]
pub static _INTERRUPTS: Interrupts =
    Interrupts { ..interrupts::DEFAULT_HANDLERS };

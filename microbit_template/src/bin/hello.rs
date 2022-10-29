// We don't have access to the Rust standard library
#![no_std]
// There is no standard main(), we cook it up ourselves
#![no_main]

// Implements a panic handler for us.
// The handler conveniently logs the error message to RTT before entering an infinite loop.
use panic_rtt_target as _;
// Enables to capture the output from Micro:bit while debugging, using the so-called
// Real-Time Transfer (RTT). Look at https://docs.rs/rtt-target/ for more details.
use rtt_target::{rprintln, rtt_init_print};
// The import is unused, but otherwise the linker
// somehow cannot find the interrupt table
use microbit::hal::prelude::*;

use cortex_m_rt::entry;

// That is the actual entry point of our program
#[entry]
// '!' means that the function never returns
fn main() -> ! {
    // This should be called once (and only once) in order to set up RTT
    rtt_init_print!();
    rprintln!("Hello Embedded world, do you mind if I join?");
    loop {}
}

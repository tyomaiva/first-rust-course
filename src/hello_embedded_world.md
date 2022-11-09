# Hello, Embedded world!

## Setting up the environment
Apart from the general Rust environment (including the Rust compiler and Cargo) that we have since [this Section](./setting_up.md), we need embedded-specific tools. Their setup on different OS platforms is described in the [Discovery book](https://docs.rust-embedded.org/discovery/microbit/03-setup/index.html). Make sure you look at the instructions for Micro:bit V2 (not V1)!

```shell
rustup target add thumbv7em-none-eabihf
cargo install cargo-embed
```

## First code example
Get template that is tailored to our board from [GitHub](https://github.com/tyomaiva/first-rust-course/blob/master/microbit_template). For more details, look at the [original template](https://github.com/rust-embedded/cortex-m-quickstart).

Attach the Micro:bit using USB, go to the root of the template project, and run it:
```shell
cargo embed --target thumbv7em-none-eabihf --bin hello
```

The contents of `src/bin/hello.rs`:
```rust,noplayground
// We don't have access to the Rust standard library
#![no_std]
// There is no standard main(), we cook it up ourselves
#![no_main]

// Implements a panic handler for us.
// The handler conveniently logs the error message to RTT
// before entering an infinite loop.
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
```
+ `cortex_m_rt` does linking (based on the layout in `memory.x` file) and RAM initialization for ARM Cortex-M chips

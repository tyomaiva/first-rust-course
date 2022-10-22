# Hello, Embedded world!

## Setting up the environment
Apart from the general Rust environment (including the Rust compiler and Cargo) that we have since [this Section](./setting_up.md), we need embedded-specific tools. Their setup on different OS platforms is described in the [Discovery book](https://docs.rust-embedded.org/discovery/microbit/03-setup/index.html).

## First code example
Get template from ??????

Run it so: ???

```rust,noplayground
#![no_std]
#![no_main]

use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};
// The import is unused, but otherwise the linker
// somehow cannot find the interrupt table
use microbit::hal::prelude::*;

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Hello Embedded world, do you mind if I join?");
    loop {}
}
```
+ `#![no_std]` tells that we don't have access to the Rust standard library
+ `#![no_main]` ???
+ `use panic_rtt_target as _;` provides us with some default implementation of the panic handler, so we don't spend time on that.
+ ???

Layers:
+ Peripheral-Access Crate (PAC) is the lowest level, you access here individual MCU registers. [nrf52833-pac](https://crates.io/crates/nrf52833-pac/) is the PAC that we actually use under the hood for Micro:bit V2. This crate is generated automatically from an [XML file](https://raw.githubusercontent.com/nrf-rs/nrf-pacs/master/svds/nrf52833.svd) provided by the manufacturer.
  + Normally you want to stay away from this level, unless higher-level layers do no provide all the functionality you need (or maybe you need to optimize the existing functionality).
+ Hardware-Abstraction Layer (HAL) is a way to abstract the /MCU/ peripherals into structs and methods. We will use [nrf52833-hal](https://crates.io/crates/nrf52833-hal).
 + As an example, at this level you can create and start a built-in hardware timer or establish a serial (UART) communication.
 + HALs in the Rust ecosystem normally build on top of the [embedded-hal](https://docs.rs/embedded-hal/) crate, which is basically a collection of universal traits like [digital::v2::OutputPin](https://docs.rs/embedded-hal/latest/embedded_hal/digital/v2/trait.OutputPin.html) for an output pin.
+ Board Support Package (BSP) is the highest level, and provides access to all the peripherals of the board (not only the built into MCU) like the accelerometer and the LED.
+ More explanation on different levels is [here](https://docs.rust-embedded.org/discovery/microbit/04-meet-your-hardware/terminology.html), together with a link to video.

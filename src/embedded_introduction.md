# What makes Embedded Rust special

## Embedded vs. OS-based application development
Things that are easy to use from ordinary (non-embedded) applications:
+ Heap allocation
+ Run a new process (you just specify the executable file, OS loads it automatically)
+ Spawn new threads and communicate between them
+ Security
  + You don't even notice other processes unless you really want to (or unless shared resources like RAM or disk space are over)
  + OS takes care of emergency situations like the [stack overflow](https://en.wikipedia.org/wiki/Stack_overflow)
+ Read/write/create a file
+ Networking (TCP/IP)

In general, it is relatively easy to write OS-agnostic code (apart from some things are not portable in principle).

In bare-metal embedded systems, you normally instead
+ Avoid heap whatsoever (RAM is quite limited)
+ Make your own custom scheduler to provide multi-tasking (i.e., frequent switching between multiple tasks to make illusion of concurrent execution)
  + You often need a real-time response (e.g. every 1 millisecond, you need to finish processing new chunk of sensor data)
  + Data races between different tasks are easy to introduce, hard to detect
+ Are responsible for <!-- writing your application into permanent (flash) memory and then --> initializing the RAM layout at runtime
+ Have access to all the peripherals - easy to ruin everything
+ Write/tailor drivers for the I/O (like accessing an SD card or an Ethernet controller)
+ Have to understand board-specific hardware aspects even simply to make sure your software doesn't burn the board
  + Logic interfacing, motor control, power supplies, etc.

There is one reward in embedded development: you have the _full_ overview of your code (including all the dependencies).
+ You design the _whole_ application. Not only the user-space part of it.
+ You can inspect any function you use, in principle. No "hidden layer" (like the OS kernel) beneath what you write. No external library of unknown/uncontrolled version.

Anyways, embedded development sounds scary... Can Rust help here?

## Embedded Rust
Embedded Rust is different from normal Rust:
+ You don't have access to the Rust [standard library](https://doc.rust-lang.org/std/) (no off-the-shelf variable-size data structures and I/O).
  + Nevertheless all of the [`core`](https://doc.rust-lang.org/core/) (i.e., almost all that we've covered in previous Sections) is still available.
  + All the safety features of Rust are also in place.
+ No ordinary `main()` as the default entry point <!-- : you have to do the steps preceding `main()` as well. -->
  + The standard `main()` expects command-line arguments as input, which don't make sense in the embedded context. And you never return from `main()` either.
+ Panic behaviour is undefined by default: you can (and should) tailor it to your needs.

You can work at 3 different abstraction layers:
+ Peripheral-Access Crate (PAC) is the lowest layer, you access here individual MCU (MicroController Unit) registers. [nrf52833-pac](https://crates.io/crates/nrf52833-pac/) is the PAC that we actually use under the hood for Micro:bit V2. This crate is generated automatically from an [XML file](https://raw.githubusercontent.com/nrf-rs/nrf-pacs/master/svds/nrf52833.svd) provided by the manufacturer.
  + Normally you want to stay away from this layer, unless either higher-level layers do no provide all the features you need or you need to optimize the _existing_ functionality.
+ Hardware-Abstraction Layer (HAL) is a way to abstract the _MCU peripherals_ into structs and methods. We will use [nrf52833-hal](https://crates.io/crates/nrf52833-hal) for that.
  + As an example, at this layer you can create and start a built-in hardware timer or establish a serial (UART) communication.
  + HALs in the Rust ecosystem normally build on top of the [`embedded-hal`](https://docs.rs/embedded-hal/) crate, which is basically a collection of universal traits (e.g., [`digital::v2::OutputPin`](https://docs.rs/embedded-hal/latest/embedded_hal/digital/v2/trait.OutputPin.html) for an output pin).
  + A lot of platform-agnostic (and thus portable) drivers are written based on `embedded-hal`.
+ Board Support Package (BSP) is the highest layer, and provides access to all the peripherals of the board (not only the ones built into MCU) like the accelerometer and the LEDs.
+ More explanation on different layers is [here](https://docs.rust-embedded.org/discovery/microbit/04-meet-your-hardware/terminology.html), together with a useful videolink.

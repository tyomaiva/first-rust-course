# Hello, Embedded world!

## Setting up the environment
Apart from the general Rust environment (including the Rust compiler and Cargo) that we have since [this Section](./setting_up.md), we need embedded-specific tools. Their setup on different OS platforms is described in the [Discovery book](https://docs.rust-embedded.org/discovery/microbit/03-setup/index.html). Make sure you look at the instructions for Micro:bit V2 (not V1)! The most important steps (apart from installing the ARM GNU toolchain and, in case of Linux, changing some permissions) are:
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
{{#include ../microbit_template/src/bin/hello.rs}}
```
+ `cortex_m_rt` provides linking (based on the layout in `memory.x` file) and RAM initialization for ARM Cortex-M chips

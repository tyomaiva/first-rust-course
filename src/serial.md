# Serial communication with UART

Universal Asynchronous Receiver-Transmitter (UART)
+ Is a serial-communication protocol (data is sent serially, usually byte-by-byte)
+ Full duplex (reception and transmission do not interfere with each other)
+ More details are [here](https://docs.rust-embedded.org/discovery/microbit/06-serial-communication/index.html)

Code for Micro:bit:
```rust,noplayground
use microbit::hal::uarte;

// Add your code here ...

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = microbit::Board::take().unwrap();

    let mut serial = uarte::Uarte::new(
        board.UARTE0,
        board.uart.into(),
        uarte::Parity::EXCLUDED,
        uarte::Baudrate::BAUD115200,
    );

    display_message(&mut serial, "Hello, do you see me?");

// Add your code here ...
}
```

Apart from Micro:bit, we need to send and receive messages on the _host computer_ somehow. One way is to use a standard tool (PuTTY on Windows, minicom on Linux), but it's more fun to use more Rust and less external applications. So, take code from [GitHub](https://github.com/tyomaiva/first-rust-course/blob/master/serial_port_processing) and run it in a separate window using `cargo run`. (Make sure to run it a couple of seconds earlier than the Micro:bit application, in order to capture the incoming message.)

[`microbit::hal::uarte::Uarte`](https://docs.rs/microbit-v2/latest/microbit/hal/uarte/struct.Uarte.html) implements
+ `read()` and `write()` methods, which receive and send serial messages, respectively
+ `core::fmt::Write` trait (see [Section](./traits_generics.md))

> #### Exercise
> + Implement a standalone function `display_message()` that receives instance of `Uarte` and a message string (more precisely, a string slice, `&str`) and does two things at the same time:
>   + Sends the message via `Uarte`
>   + Prints the message using `rprintln!`
> + Make `display_message()` generic over any type that implements `core::fmt::Write` so that mocking of the UART peripheral becomes possible, in principle
> + Echo the incoming messages back in an infinite loop using `display_message()`
>   + Hint: use [`core::str::from_utf8()`](https://doc.rust-lang.org/beta/core/str/fn.from_utf8.html) to convert result of `read()` to `&str`

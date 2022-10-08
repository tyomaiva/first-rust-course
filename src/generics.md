# Generics

```rust,editable
use std::io::Write;

fn display_message<T>(serial: &mut uarte::Uarte<T>, string: &str)
where
    T: MySerial,
{
    // We need to add \r here in order for the serial terminal to properly start new line
    write!(serial, "{}\r\n", string).unwrap();
    // Side effect: increase a counter (number of output characters) for statistics
    ????
}

fn main() {
    display_message(???, "How much ");
    display_message(???, "do I talk?");
    println!("Counter: {}", serial.counter);
}
```

# Traits and generics

## Motivation
> #### Exercises 2, 3, and 4 from [Rust By Practice](https://practice.rs/generics-traits/generics.html)

## Traits
Traits:
+ Are the Rust approach to interfaces (contracts between different parts of the code)
+ Are powered by generics, though generics do not necessarily involve traits
+ Allow to write polymorphic code
  + Compile-time polymorphism (runtime polymorphism is also possible, but using a bit different mechanism)
+ Can be combined (composed)
```rust,editable
??????
```
  + Compare this approach with the idiomatic hierarchy-based solution in pre-C++20 (multiple inheritance) which leads to the infamous [diamond problem](https://en.wikipedia.org/wiki/Multiple_inheritance). See more about the Rust and the diamond problem [here](https://locka99.gitbooks.io/a-guide-to-porting-c-to-rust/content/porting_from_cpp/multiple_inheritance.html).
+ Are good for dependency injection and mocking while testing
  + For example, you may emulate a hardware peripheral:
```rust,editable
trait SerialPort {
    fn read_byte(&self) -> u8;
}

struct SerialPortMock;

impl SerialPort for SerialPortMock {
    fn read_byte(&self) -> u8 {
        0
    }
}

fn process_serial_input<T: SerialPort>(port: &T) {
    println!("Serial port gives: {}", port.read_byte());
}

fn main() {
    let mock = SerialPortMock;
    process_serial_input(&mock);
    process_serial_input(&mock);
    process_serial_input(&mock);
}
```

## Standard traits
### `Copy` and `Clone`
+ Some objects can be neither copied nor cloned (makes sense for a unique resource like an open file or a hardware peripheral)
+ `Copy` is an example of a _marker_ trait: it does not introduce any new methods or types, it only indicates whether a certain operation is possible with the associated type
+ Some traits can be automatically derived

### `Debug`
It's useful to dump contents of a variable, e.g. during the debugging
```rust
struct MyPoint {
    x: f64,
    y: f64,
}
# impl MyPoint {
#     fn new(x: f64, y: f64) -> Self {
#         Self{x: x, y: y}
#     }
# }

fn main() {
    let point = MyPoint::new(3., 4.);
    println!("The point is {:?}", point);
```
Compiler tells that we need [`Debug`](https://doc.rust-lang.org/beta/core/fmt/trait.Debug.html) trait and even suggests two possible solutions
> note: add `#[derive(Debug)]` to `MyPoint` or manually `impl Debug for MyPoint`
+ Bad thing: we don't have `Debug` trait by default
+ Good thing: we can write our custom implementation or derive a default implementation for free
<!-- > #### Question why does the line below need `Debug`? --- we also need partialEq here,  -->
<!-- ```rust,editable -->
<!-- assert_eq!() -->

## `core::fmt::Write`
Documentation for [core::fmt::Write](https://doc.rust-lang.org/core/fmt/trait.Write.html) says:
```rust
trait Write {
    fn write_str(&mut self, s: &str) -> Result;

    fn write_char(&mut self, c: char) -> Result { ... }
    fn write_fmt(&mut self, args: Arguments<'_>) -> Result { ... }
}
```
+ This trait is normally used for unbuffered writes (like appending characters to a string)
+ Good thing: you only need to implement `write_str()`, the other two methods are provided automatically
+ Allows to use `println!`-like formatted printing
```rust,noplayground
    // w implements core::fmt::Write
    // ...
    write!(w, "Formatted {}", "printing").unwrap();
```
+ We will use this feature later for [serial communication](./serial.md)

> #### Exercise
> Design a struct that implements `core::fmt::Write`, so that writing to that struct does several things simultaneously:
> - The message contents is printed on screen
> - An internal statistical counter is updated (side effect). The counter keeps track of total number of characters being ever written
> - (*) Every second time you write to the struct, the counter value is printed on screen
> - (**) You print only whenever end-of-line is encountered in your message, but buffer the message otherwise (similar to `printf()` in C). Yes, `core::fmt::Write` is used for _unbuffered_ writes, so such an implementation is not recommended for production use :-)

## Resources for deeper understanding
+ [Chapter 10](https://doc.rust-lang.org/book/ch10-00-generics.html) of the Rust book

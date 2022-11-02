# Traits and generics

## Motivation
> #### Exercises: 3, 4, and 2 from [Section on Generics](https://practice.rs/generics-traits/generics.html#exercises) in Rust By Practice (yes, do them in this particular order) and then Ex. 1 from [Section on Traits](https://practice.rs/generics-traits/traits.html#exercises).

## Traits
Traits:
+ Are the Rust approach to interfaces (contracts between different parts of the code)
  + Traits only declare methods (not data members)
+ Are powered by generics, though generics do not necessarily involve traits
+ Allow to write polymorphic code
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
+ Single type can have (and usually does have) multiple traits, traits are easily composed
  + Compare this approach with the idiomatic hierarchy-based solution in pre-C++20 (multiple inheritance) which leads to the infamous [diamond problem](https://en.wikipedia.org/wiki/Multiple_inheritance). See more about the Rust and why it does not have the diamond problem [here](https://locka99.gitbooks.io/a-guide-to-porting-c-to-rust/content/porting_from_cpp/multiple_inheritance.html).
  + Inheritance in OOP languages involves runtime polymorphism while traits provide compile-time polymorphism

## Traits from `core`
### `Copy` and `Clone`
[`core::marker::Copy`](https://doc.rust-lang.org/core/marker/trait.Copy.html) is a trait used to implicitly copy values bit-by-bit, instead of moving the value
```rust
pub trait Copy: Clone { }
```
+ `Copy` is an example of a _marker_ trait: it does not introduce any new methods or types, it only indicates whether a certain operation is possible with the type.
+ Syntax with `:` means a _trait bound_: any type implementing `Copy` should implement `Clone` as well.
+ [`core::clone::Clone`](https://doc.rust-lang.org/core/clone/trait.Clone.html) is a trait that allows _cloning_ the values:
```rust
pub trait Clone {
    fn clone(&self) -> Self;

    fn clone_from(&mut self, source: &Self) { ... }
}
```
+ Differences between cloning and copying:
  + If you want to clone, you always should do this explicitly, `a = b.clone();` No implicit cloning! (In this particular case, we get two values and two owners, `a` and `b`, _similar_ to copying.)
  + Cloning can be equivalent to copying, but can be totally different. On contrary, copying cannot be overriden.
+ Good thing: you only need to implement `clone()` (it is called a "required method"). Once that is done, the other method (`clone_from()`) is provided automatically for you ("provided method").
+ Both `Copy` and `Clone` can be automatically derived (with some default implementation), we saw that in [Section on Ownership](./ownership.md)
```rust
#[derive(Copy,Clone)]
struct MyPoint { x: f64, y: f64 }
```
+ Some values can be neither copied nor cloned (only moved): for example for a unique resource like an open file or a hardware peripheral

### `Debug`
It's useful to dump contents of a variable, e.g. during the debugging or error logging
```rust
#[derive(Debug)]
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
}
```
<!-- > #### Question why does the line below need `Debug`? --- we also need partialEq here,  -->
<!-- ```rust,editable -->
<!-- assert_eq!() -->

## `core::fmt::Write`
Documentation for [core::fmt::Write](https://doc.rust-lang.org/core/fmt/trait.Write.html) says:
```rust,noplayground
trait Write {
    // Required method:
    fn write_str(&mut self, s: &str) -> Result;

    // Provided methods:
    fn write_char(&mut self, c: char) -> Result { ... }
    fn write_fmt(&mut self, args: Arguments<'_>) -> Result { ... }
}
```
+ This trait is normally used for unbuffered writes (like appending characters to a string)
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

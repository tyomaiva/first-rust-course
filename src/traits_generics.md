# Traits and generics

## TODO
+ Good for dependency injection and mocking while testing
+ Some objects can be neither copied nor cloned (makes sense for a unique resource like an open file)

## Traits
Real-world trait: [core::fmt::Write](https://doc.rust-lang.org/core/fmt/trait.Write.html)
```rust
trait Write {
    fn write_str(&mut self, s: &str) -> Result;

    fn write_char(&mut self, c: char) -> Result { ... }
    fn write_fmt(&mut self, args: Arguments<'_>) -> Result { ... }
}
```
+ The trait is normally used for unbuffered writes (like appending characters to a string)
+ Good thing: you only need to implement `write_str()`, the other two methods are provided automatically
+ Allows to use `println!`-like formatted printing
```rust,noplayground
    // w implements core::fmt::Write
    // ...
    write!(w, "Formatted {}", "printing").unwrap();
```
+ We will use this feature later for [serial communication](./serial.md)

> #### Exercise:
> Implement a struct that implements `core::fmt::Write`, so that writing to that struct does several things:
> - The message contents is printed on screen
> - An internal statistical counter is updated (side effect). The counter keeps track of total number of characters being ever written
> - (*) Every second time you write to the struct, the counter value is printed on screen
> - (**) You print only whenever end-of-line is encountered in your message, but buffer the message otherwise (similar to `printf()` in C). Yes, `core::fmt::Write` is used for _unbuffered_ writes, so such an implementation is not recommended for production use :-)

## Resources for deeper understanding
+ [Chapter 10](https://doc.rust-lang.org/book/ch10-00-generics.html) of the Rust book

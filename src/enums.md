# Enums

## Motivation
Often we have a limited number of possible values, for example on our Micro:bit we have two buttons, A and B:
```rust,editable
enum KeyPress {
    A,
    B,
}

fn main() {
    let key = KeyPress::B;
    match key {
        KeyPress::A => println!("Start/stop collecting the data"),
        KeyPress::B => println!("Send collected data to cloud"),
    }
}
```
Pattern matching is exhaustive: if you forget to include an enum variant, code won't compile.
+ Good safety feature if you add a new variant and forget to process it somewhere.

## `Option`
How to implement a function that either returns either some value or _no value at all_?
+ In C and in the old-style C++, we use a null pointer for the latter case
+ In Rust, it is idiomatic to use [`core::option::Option`](https://doc.rust-lang.org/core/option/enum.Option.html):
```rust
enum Option<T> {
    Some(T),
    None
}
```
+ Two enum variants store _different_ values: `None` does not store anything, `Some` stores a value of `T`.
+ `Option` is similar to C++ [`std::optional`](https://en.cppreference.com/w/cpp/utility/optional), which was introduced only in C++17.
+ For types `T` that can never be 0 (in binary representation), Rust optimizes `None` to a null pointer, so no performance loss at all!

`Option` has many batteries included:
+ `#[derive(Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]`
+ Plenty of helper functions
```rust,editable
fn main() {
    let x: Option<u8> = Some(2);
    assert_eq!(x.is_some(), true);
    assert_eq!(x.unwrap(), 2);
    assert_eq!(x.expect("Oops!"), 2);
}
```
```rust,editable
fn main() {
    let x: Option<u8> = None;
    x.expect("Oops!");
}
```

## `Result`
Whenever an operation may _fail_, it is idiomatic to use [`core::result::Result`](https://doc.rust-lang.org/core/result/enum.Result.html) instead of `Option`
```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```
+ It has a set of batteries very similar to `Option`
+ If a function returns a `Result` and the caller ignores the returned value, the compiler gives a warning
+ It is very popular to use the `?` syntactic sugar to propagate errors in an elegant way:
```rust,editable
fn problematic_function() -> Result<u8, ()> {
    let x: Result<u8, ()> = Err(());
    println!("{:?}", x?);
    Ok(4)
}

fn main() {
    println!("{:?}", problematic_function());
}
```
+ Compare this approach with C++/Java/Python exceptions, where error propagation is _non-local_ and you may easily get an exception at runtime that you were not even aware of and which crashes the application! (Or even worse, bloat the code with catching all potential exceptions in a paranoic fashion, just to be on the safe side.)

## Resources for deeper understanding
+ [Chapter 6](https://doc.rust-lang.org/book/ch06-00-enums.html), [also here](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html#handling-potential-failure-with-the-result-type) and [here](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html) in the Rust book

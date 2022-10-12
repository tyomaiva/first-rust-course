# Enums

## TODO
+ Pattern matching

Whenever an operation may return either some value or _no value at all_, it is idiomatic to use [`core::option::Option`](https://doc.rust-lang.org/core/option/enum.Option.html)
```rust
enum Option<T> {
    Some(T),
    None
}
```
`Option` has many batteries included:
+ `#[derive(Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]`
+ Plenty of functions
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

Whenever an operation may _fail_, it is idiomatic to use [`core::result::Result`](https://doc.rust-lang.org/core/result/enum.Result.html) instead
```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```
+ It has a set of batteries very similar to `Option`
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
+ Compare this approach with C++/Java/Python exceptions, where error propagation is _non-local_ and you may easily get an exception at runtime that you were not even aware of and which crashes the application! (Or even worse, catch all exceptions in a paranoic fashion, just to be on the safe side.)

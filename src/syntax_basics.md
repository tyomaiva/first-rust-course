# Syntax basics

## TODO

+ =use=, =loop=, =array= (example of nested 5x5 micro:bit LED array from =blink.rs=), comments
  + `use` creates shorthand aliases, so instead of typing `std::io::stdin()` we type `io::stdin()`
+ do we really need structs and tuples (for understanding docs maybe)?
  + functions associated to a struct

## First code snippet

```rust,editable
fn main() {
    let mut index = 10; // Define a variable
    while index != 0 {
        println!("Counting down: {}", index);
        /* Decrease the value on each iteration */
        index -= 1;
    }
}
```
Several things to note here:
+ `fn` defines a new function, in this case it is called `main`, it does not have any input arguments and does not return anything either
+ `let mut` binds a value `10` to a _mutable_ variable called `index`
  + If you omit `mut`, you get instead an _immutable_ variable, which does not work in this case, try it!
  + So variables in Rust are immutable by default
+ Syntax is similar to C and languages that (partially) inherited the C syntax (like C++, Java, and JavaScript): brackets, braces, semicolons, comparison and increment/decrement operators, comments
+ `println!()` does formatted printing, similar to `printf()` in C, but more powerful
+ You can edit this code snippet and re-run it without leaving this webpage. Alternatively, you can copy it and run in the [Rust Playground](https://play.rust-lang.org/)

## Basic numerical types

In the example above, the type of the variable `index` is inferred automatically
+ In that case it is `i32`, which is a 32-bit signed integer
+ You can get this information by hovering over `index` in your IDE with Rust Language Server enabled (e.g. in Visual Studio Code)
+ If we change `10` to `10.`, we get instead the double-precision floating-point type, `f64`
+ If you want to use a different type, you need to specify it explicitly
```rust,editable
fn main() {
    let index: u8 = 10;
    // OR
    let index2 = 10_u8;
}
```
+ `u8` is 8-bit unsigned integer type, which is very popular in Embedded Rust and in general in low-level programming, where we have to shuffle individual bytes
+ More types here: [Scalar Types](https://doc.rust-lang.org/book/ch03-02-data-types.html#scalar-types)

## User-defined types

Apart from the basic types, we often want to combine them in a _compound_ user-defined type

### Tuples
<!-- tuples go before arrays as an example for the latter needs an iterator that produces a tuple -->

### Arrays
Array is a fixed-size collection of elements of the _same_ type
...

What happens if we access an index out of range?
```rust,editable
fn main() {
    let array = [7, 21, 42];
    let mut index = 0;
    loop {
        println!("The element #{} is {}", index, array[index]);
        index += 1;
    }
}
```
+ It is safe to do (i.e., no undefined behaviour), the Rust program just panics and writes a diagnostic post-mortem message
+ `loop` is an infinite loop (a dedicated language construct, unlike the hacky `while(1) {}` and `for(;;) {}` in C)

More idiomatic (and less error-prone) alternative is to use Rust _iterators_
```rust,editable
fn main() {
    let array = [7, 21, 42];
    for (index, element) in array.iter().enumerate() {
        println!("The element #{} is {}", index, element);
    }
}
```

Nested arrays
```rust
    let light_pattern = [
        [1, 1, 1, 1, 1],
        [1, 1, 1, 1, 1],
        [1, 1, 0, 1, 1],
        [1, 1, 1, 1, 1],
        [1, 1, 1, 1, 1],
    ];
    println!("{}", light_pattern[0][0]);
    println!("{}", light_pattern[2][2]);
```
We will use it for blinking a 5x5 LED matrix in the [Embedded Rust part](./blink.md)

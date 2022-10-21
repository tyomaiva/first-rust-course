# Ownership and borrow checker

## Warm-up example
The code below looks good, we only call the struct method twice. Why is compiler not happy?
```rust
# struct MyPoint {
#     x: f64,
#     y: f64,
# }
impl MyPoint {
#     fn new(x: f64, y: f64) -> Self {
#         Self{x: x, y: y}
#     }
#
    fn is_close_to_origin(self) -> bool {
        (self.x * self.x + self.y * self.y) < f64::EPSILON
    }
}

fn main() {
    let point = MyPoint::new(3., 4.);
    assert_eq!(point.is_close_to_origin(), false);
    assert_eq!(point.is_close_to_origin(), false);
}
```

At the same time, the following example is compiling without issues:
```rust,editable
fn print_me(x: i32) {
    println!("The value is {}", x);
}

fn main() {
    let x = 5;
    let y = x;
    print_me(x);
    print_me(x);
    print_me(y);
}
```
What's the difference?

## Ownership
The ownership and access system in Rust is quite restrictive and also non-intuitive at first for developers with C++/Java/C# background. As we will see, the reason is to guarantee safety, at compile time.
<!-- In this Section, we will use the [RustViz](https://github.com/rustviz/rustviz) tool to visualize the ownership and access concepts. -->

Here are the basic rules for ownership:
+ Values in Rust have only _one_ owner at a time, meaning there is _one and only one_ responsible for deallocating the value when it's not needed anymore.
  + No memory leaks or [double frees](https://stackoverflow.com/a/21057524) anymore
  + Owner can change over time by _moving_ the value, but at any given time there is only one owner. The old owner is said to be _consumed_ in this case.
    + Move does not touch the value (neither the contents nor the memory location), no non-trivial move constructors (a la C++) are allowed
+ Deallocation is similar to RAII in C++: the value is droppped when the last owner goes out of scope
 + All the potential code locations for deallocation can be determined at compile-time, so no need for garbage collection (unlike Java and C#)

It explains the problem with the double call,
```rust
    assert_eq!(point.is_close_to_origin(), false);
    assert_eq!(point.is_close_to_origin(), false);
```
since `point` is consumed at the first call of `is_close_to_origin()`.

But why then `x` is not consumed in the second example above?
```rust
    let x = 5;
    let y = x;
```
+ `i32`, as well as many other stack-allocated types, can be efficiently copied by directly copying the bits
+ These types have `Copy` trait, which we cover in more detail in the [next Section](./traits_generics.md)
+ Rust _always copies_ types with `Copy` instead of moving them (on every assignment and passing to function)
+ `MyPoint` has only stack-allocated values inside, can't we make it `Copy` as well? Yes, we can:
```rust
#[derive(Copy,Clone)]
struct MyPoint {
    x: f64,
    y: f64,
}
# impl MyPoint {
#     fn new(x: f64, y: f64) -> Self {
#         Self{x: x, y: y}
#     }
#     fn is_close_to_origin(self) -> bool {
#         (self.x * self.x + self.y * self.y) < f64::EPSILON
#     }
# }

fn main() {
    let point = MyPoint::new(3., 4.);
    assert_eq!(point.is_close_to_origin(), false);
    assert_eq!(point.is_close_to_origin(), false);
}
```
+ No accidental implicit copies for non-`Copy` types are possible! (in contrast with C++)

<!-- > #### Exercise -->
<!-- > ... -->

## Borrow checker
We don't actually need an independent copy of `MyPoint` on every method call. (We get now in total 3 different copies of the same value!) What we really need is to read the value, how to do that? _Borrowing_ is the way to go.

Borrow rules:
+ You can access a value without owning it. You are then _borrowing_ it, which basically means you have a reference (or a pointer, if we dig at lower level) to the value
+ You either:
  + only read the value contents (_immutable borrow_), the syntax is `&T` OR
  + both read and write (_mutable borrow_), the syntax is `&mut T`
+ You can have _multiple immutable_ borrows OR a _single mutable_ borrow (not both at the same time)
  + Immutable borrow means not only that you can't change the value, but nobody else can do that either!

Compiler makes sure that
+ References are always valid
+ In particular, they are not null pointers

So what we need for the previous example is an immutable borrow:
```rust
struct MyPoint {
    x: f64,
    y: f64,
}
impl MyPoint {
#     fn new(x: f64, y: f64) -> Self {
#         Self{x: x, y: y}
#     }
    fn is_close_to_origin(&self) -> bool {
        (self.x * self.x + self.y * self.y) < f64::EPSILON
    }
}

fn main() {
    let point = MyPoint::new(3., 4.);
    assert_eq!(point.is_close_to_origin(), false);
    assert_eq!(point.is_close_to_origin(), false);
}
```
+ Note that we've removed `#[derive(...)]` as we don't need to copy the values anymore

## Resources for deeper understanding
+ [Chapter 4](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html) of the Rust book

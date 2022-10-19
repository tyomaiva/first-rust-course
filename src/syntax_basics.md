# Syntax basics

<!-- + `use` creates shorthand aliases, so instead of typing `std::io::stdin()` we type `io::stdin()` -->

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

## Basic built-in types
In the example above, the type of the variable `index` is inferred automatically
+ In that case it is `i32`, which is a 32-bit signed integer
+ You can get this information by hovering over `index` in your IDE with Rust Language Server enabled (e.g. in Visual Studio Code)
+ If we change `10` to `10.`, we get instead the double-precision floating-point type, `f64`
+ If you want to use a different type, you need to specify it explicitly
```rust,editable
fn main() {
    let index: u8 = 10;
    // OR
    let index = 10_u8;
    // OR
    let index = 10 as u8;
}
```
+ `u8` is 8-bit unsigned integer type, which is very popular in Embedded Rust and in general in low-level programming, where we have to shuffle individual bytes
+ `usize` is another extremely popular type, which is an unsigned integer with native size (32-bit for 32-bit architectures like ARM Cortex-M, 64-bit for a desktop/laptop CPU)
+ `bool` is a type with two possible values: `true` and `false`
+ More types here: [Scalar Types](https://doc.rust-lang.org/book/ch03-02-data-types.html#scalar-types)

## User-defined types
Apart from the basic types, we often want to combine them in a _compound_ user-defined type

### Tuples
<!-- tuples go before arrays as an example for the latter needs an iterator that produces a tuple -->

A tuple combines values of different types
```rust
    let tuple = ("Something", 2);
    println!("{}, {}", tuple.0, tuple.1);
```

### Structs
Structs are similar to tuples in that they combine potentially different types, but each field is _named_:
```rust
struct MyPoint {
    x: f64,
    y: f64,
}

    let point = MyPoint{x: 3., y: 4.};
    println!("The point is: (x: {}, y: {})", point.x, point.y);
```
+ Note that if you pass `{x: 3, y: 4}` instead of `{x: 3., y:4.}`, you get a compiler error (try it!): you need to convert the types explicitly, e.g. `{x: 3 as f64, y: 4 as f64)` will do the trick. It is a safety feature, to avoid accidental type castings

You can associate methods acting on a given struct type (encapsulation):
```rust
# struct MyPoint {
#     x: f64,
#     y: f64,
# }
impl MyPoint {
    fn new(x: f64, y: f64) -> Self {
        Self{x: x, y: y}
    }

    fn is_close_to_origin(self) -> bool {
        (self.x * self.x + self.y * self.y) < f64::EPSILON
    }
}

fn main() {
    let point = MyPoint::new(3., 4.);
    assert_eq!(point.is_close_to_origin(), false);
}
```
+ Implementing `new()` is a customary way of initializing structs. It's like a class constructor in C++, with two major differences:
  + Call of `new()` should be always _explicit_ (no implicit construction)
  + There can be only one implementation of `new()`, since Rust intentionally does not have function overloading. If you need multiple ways to construct an object (e.g. construct a default struct value and a custom one), use the [Builder pattern](https://rust-unofficial.github.io/patterns/patterns/creational/builder.html) instead
+ `Self` is an alias to the current struct _type_, so `Self` \\( \iff \\) `MyPoint`
+ `self` is an alias to the current struct _instance_, so `self` \\( \iff \\) `point`

### Arrays
Array is a fixed-size collection of elements of the _same_ type (similar to C arrays)
```rust
    let mut array = [7, 21, 42];
    array[0] += 20;
    println!("{:?}", array);
```
+ Note the `{:?}` syntax which normally tells to dump the contents of the variable. We will elaborate on this [later](./traits.md), while discussing the `Debug` trait.

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
+ `loop` denotes an infinite loop (a dedicated language construct, unlike the hacky `while(1)` and `for(;;)` in C)

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
We will use this code for blinking a 5x5 LED matrix in the [Embedded Rust part](./blink.md)

> #### Exercise
> Implement a 2D straight-line type `Line`
> - `Line` can be constructed by supplying two endpoints
> - `Line` has a method `is_point_on_line()` which checks whether a given point belongs to the line or not
```rust
# struct MyPoint {
#     x: f64,
#     y: f64
# }
# impl MyPoint {
#     fn new(x: f64, y: f64) -> Self {
#         Self{x: x, y: y}
#     }
# }
fn main() {
    let point1 = MyPoint::new(2., 1.);
    let point2 = MyPoint::new(-2., -1.);
    let segment1 = Line::new(point1, point2);
    let point3 = MyPoint::new(4., 2.);
    assert_eq!(segment1.is_point_on_line(point3), true);
    // let point4 = MyPoint::new(1., 1.);
    // assert_eq!(segment2.is_point_on_line(point4), false);
}
```
<!-- struct Line(MyPoint, MyPoint); -->
<!-- impl Line { -->
<!--     fn new(point1: MyPoint, point2: MyPoint) -> Self { -->
<!--         Self(point1, point2) -->
<!--     } -->
<!--     fn is_point_on_line(self, point: MyPoint) -> bool { -->
<!--         let vector1 = MyPoint::new(self.0.x - self.1.x, self.0.y - self.1.y); -->
<!--         let vector2 = MyPoint::new(point.x - self.1.x, point.y - self.1.y); -->
<!--         (vector1.x * vector2.y - vector1.y * vector2.x).abs() < 10. * f64::EPSILON -->
<!--     } -->
<!-- } -->
> (*) Implement a type `Triangle` (you may choose between 2D and 3D triangles)
> - `Triangle` can be constructed based on three vertices
> - `Triangle` has a method to check whether a given point lies within it or not


## Resources for deeper understanding
+ [Chapter 3](https://doc.rust-lang.org/book/ch03-00-common-programming-concepts.html), [Chapter 5](https://doc.rust-lang.org/book/ch05-00-structs.html) and [here](https://doc.rust-lang.org/book/ch13-02-iterators.html) in the Rust book

# Ownership and borrow checker

## TODO
+ Case with fn length(self) -> f64 : move occurs, the number is not usable anymore

## Warm up example

## Ownership

The ownership and access system in Rust is quite restrictive and also non-intuitive at first for developers with C++/Java/C# background. As we will see, the reason is to facilitate safety, at compile time. In this Section, we will use the [RustViz](https://github.com/rustviz/rustviz) tool to visualize these challenging concepts.

Here are the basic rules:
+ Objects in Rust have only _one_ owner at a time, meaning there is _one and only one_ responsible for deallocating the object when it's not needed anymore.
  + No memory leaks or [double frees](https://stackoverflow.com/a/21057524) anymore
  + Owner can change over time by _moving_ the object, but at any given time there is only one owner
+ Deallocation is similar to RAII in C++: the object is droppped when the last owner goes out of scope
 + All the potential code locations for deallocation can be determined at compile-time, so no need for garbage collection (unlike Java and C#)

## Borrow checker
+ You can access an object without owning it. You are then _borrowing_ it, which basically means you have a reference (or a pointer, if we dig at lower level) to an object
+ Either you only read the object contents (_immutable borrow_) or both read and write (_mutable borrow_)
+ You can have _multiple immutable_ borrows OR a _single mutable_ borrow (not both at the same time)
  + Immutable borrow means not only that you can't change the object, but nobody else can do that either!
+ By default, objects are moved, not copied, when assigned or passed as function arguments (in contrast with C++)
  + No implicit copies apart from some basic types (like `u8`) and special user-defined types (we will cover it in [the traits Section](./traits_generics.md))

<div class="flex-container vis_block" style="position:relative; margin-left:-75px; margin-right:-75px; display">
    <object type="image/svg+xml" class="copy code_panel" data="rustviz_assets/example1/vis_code.svg"></object>
    <object type="image/svg+xml" class="copy tl_panel" data="rustviz_assets/example1/vis_timeline.svg" style="width: auto;" onmouseenter="helpers('copy')"></object>
</div>

## Resources for deeper understanding
+ [Chapter 4](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html) of the Rust book

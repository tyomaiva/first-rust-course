# Introduction

Welcome to a Rust programming language course!

The goals of the course:
+ Learn to write basic Rust code and read API docs like [this](https://doc.rust-lang.org/core/option/enum.Option.html)
+ Get experience with Embedded Rust on a real hardware platform

_Out_ of scope:
+ Motivation to learn Rust (it is a whole separate topic).
+ Data collections like [Vec](https://doc.rust-lang.org/std/vec/struct.Vec.html) or [HashMap](https://doc.rust-lang.org/stable/std/collections/struct.HashMap.html), since they involve heap allocation, which is often undesirable in embedded systems. Also no file access and no other forms of I/O (printing to standard output is the only exception). In general, we omit the Rust standard library.
+ Multithreading and asynchronous programming, though both are quite easy with Embedded Rust (see [here](https://embassy.dev/) and [here](https://rtic.rs/1/book/en/)), and almost unavoidable when implementing an RTOS.
+ Unit testing, modularization, and other good development practices, since all of our projects will be very small.
+ PAC layer of Embedded Rust. We start directly with the higher abstraction levels which are HAL and BSP. So no direct register manipulation.
<!-- + Smart pointers like [Box](https://doc.rust-lang.org/std/boxed/struct.Box.html), for the same reason -->

<!-- What knowledge is expected: -->
<!-- + how stack/heap works -->
<!-- + UNIX shell (e.g. bash) ?????????? -->
<!-- + C++ experience is useful, as we will draw analogies and make comparisons from time to time -->

## Resources
+ [A half-hour to learn Rust](https://fasterthanli.me/articles/a-half-hour-to-learn-rust) is a nice crash course without much theory. Note: it may actually take more than 30 minutes to read and digest 😊
+ [Teaching material](https://ferrous-systems.github.io/teaching-material/) developed by Ferrous Systems, a company which specializes in Rust development (including Embedded Rust) and in providing Rust trainings.
+ [Rust by practice](https://practice.rs/) is a large set of exercises structured by topics (e.g., Basic Types, Ownership and Borrowing) where you normally have to fix several lines of code to make an example working.
+ The official [Rust book](https://doc.rust-lang.org/book/) is written extremely clear and is quite comprehensive. Absolute recommendation if you are serious about Rust!
+ [The Discovery book](https://docs.rust-embedded.org/discovery/microbit/) is an introductory resource on Embedded Rust using the same hardware platform that we cover here.

<p style="text-align: center;"><img src="https://visitor-badge.glitch.me/badge?page_id=https://tyomaiva.github.io/first-rust-course/&left_color=green&right_color=red" alt="Visitors" /></p>

# Introduction

Welcome to a course for the Rust programming language!

The goals of the course:
+ Learn to write basic Rust code and read API docs like [this](https://doc.rust-lang.org/core/option/enum.Option.html)
+ Get experience with Embedded Rust on a real hardware platform

_Out_ of scope:
+ Motivation to learn Rust (we tried to accomplish that in the previous sessions)
+ Data collections like [Vec](https://doc.rust-lang.org/std/vec/struct.Vec.html) or [HashMap](https://doc.rust-lang.org/stable/std/collections/struct.HashMap.html), since they involve heap allocation, which is often undesirable in embedded systems. In general, we omit the Rust standard library completely
+ Multithreading and asynchronous programming, though both are quite easy with Embedded Rust (see [here](https://embassy.dev/) and [here](https://rtic.rs/1/book/en/)), and almost unavoidable when implementing an RTOS
+ Unit testing, modularization, and other good development practices, since all of our projects will be very small
<!-- + Smart pointers like [Box](https://doc.rust-lang.org/std/boxed/struct.Box.html), for the same reason -->

<!-- What knowledge is expected: -->
<!-- + how stack/heap works -->
<!-- + UNIX shell (e.g. bash) ?????????? -->
<!-- + C++ experience is useful, as we will draw analogies and make comparisons from time to time -->

## Resources
+ [A half-hour to learn Rust](https://fasterthanli.me/articles/a-half-hour-to-learn-rust) is a nice crash course without much theory. Note: it may actually take more than 30 minutes to read and digest 😊
+ [Teaching material](https://ferrous-systems.github.io/teaching-material/) developed by Ferrous Systems, a company which specializes in Rust development (including Embedded Rust) and in providing Rust trainings
+ The official [Rust book](https://doc.rust-lang.org/book/) is written extremely clear and is quite comprehensive. Absolute recommendation if you are serious about Rust!
+ [The Discovery book](https://docs.rust-embedded.org/discovery/microbit/) is an introductory resource on Embedded Rust using the same hardware platform that we cover here

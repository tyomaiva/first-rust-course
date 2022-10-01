# Introduction

The goals of the course:
+ Learn to write and read basic Rust code
+ Get experience with Embedded Rust with a real hardware system

_Out_ of scope:
+ Motivate to learn Rust (we tried to accomplish that in the previous sessions)
+ Data collections like [Vec](https://doc.rust-lang.org/std/vec/struct.Vec.html) or [HashMap](https://doc.rust-lang.org/stable/std/collections/struct.HashMap.html), since they involve heap allocation, which is often undesirable on embedded systems
+ Multithreading and asynchronous programming, though both are [relatively easy](https://docs.rust-embedded.org/book/concurrency/) with Embedded Rust, and almost unavoidable when implementing an RTOS
+ Unit testing, modularization, and other good development practices, since all of our projects will be very small
<!-- + Smart pointers like [Box](https://doc.rust-lang.org/std/boxed/struct.Box.html), for the same reason -->

<!-- What knowledge is expected: -->
<!-- + how stack/heap works -->
<!-- + UNIX shell (e.g. bash) ?????????? -->
<!-- + C++ experience is useful, as we will draw analogies and make comparisons from time to time -->

# Parallel and concurrent programming

## Parallel vs. concurrent
Two types of execution (in addition to the "regular" serial execution):
+ _Parallel_ means that two different pieces of code run at the same time (for example, on different CPU cores)
+ _Concurrent_ means that two different pieces of code run at different moments, but provide an _illusion_ of parallel execution (first piece starts, then it suspends, second piece kicks in, then it suspends, the first one resumes, and so on...).

These two types:
+ are known to be extremely hard to implement properly and even harder to debug (execution order is non-deterministic and hence non-reproducible):
  + concurrent problems were traditionally turned into serial ones using callbacks, but any more-less complex logic leads to so-called [callback hell](https://en.wiktionary.org/wiki/callback_hell) where it's hard to reason who calls what.
  + in embedded systems (where you typically have a single CPU core), it is common to solve concurrent problems (like the real-time OS scheduling) via a super-loop that goes over all the scheduled tasks and polls them one-by-one. The disadvantage is that you may have 100 tasks and only 3 of them are really active at the moment, but you keep polling each and every task all the time.
  + co-routines like in [C++20](https://en.cppreference.com/w/cpp/language/coroutines) is another approach for concurrency but it gives you only a limited control over the program execution.
+ are typically combined, so we won't distinguish them unless we really need to.
+ are supported by Rust via thread and async mechanisms.

## Threads
+ [`std::thread`](https://doc.rust-lang.org/std/thread/) is an abstraction on top of the native OS threads (1-to-1 correspondence: 1 OS thread corresponds to 1 Rust thread).
  + The preferred way to communicate data between threads is message passing via [`std::sync::mpsc::channel`](https://doc.rust-lang.org/std/sync/mpsc/fn.channel.html). You can have multiple "producers" of a data and a single "consumer". Rust channels are lock-free (at the API level).
    + (Absence of) flow control is a potential issue: producers may generate data at higher rate than consumers are able to process that, so RAM gets overloaded with the buffered values. NOTE: there exists [`sync_channel`](https://doc.rust-lang.org/std/sync/mpsc/fn.sync_channel.html) that has a bounded capacity, but it blocks _all the producers_ when that capacity is reached.
  + The less preferred way is locking shared data by means of [`std::sync::Mutex`](https://doc.rust-lang.org/std/sync/struct.Mutex.html) (MUTual EXclusion). Welcome to the world of deadlocks and threads blocking each other! (How long is your critical section?)
  + Both ways of communication are guaranteed to be thread-safe (no data races) at the compile level by means of special marker traits [`core::marker::Send`](https://doc.rust-lang.org/core/marker/trait.Send.html) and [`core::marker::Sync`](https://doc.rust-lang.org/core/marker/trait.Sync.html). The idea is that you cannot accidentally share some value to another thread if it's type does not allow it. (Of course, you are welcome to spoil everything by using `unsafe` code.)
  + The level of parallelism is limited physically by the number of CPU cores. (If you have 8 cores and you spawned 80 threads that are all awake at the same time, you will have 10 concurrent tasks per core, on average.)
  + Spawning a thread is relatively slow and resource-heavy since it is done eventually by the OS (it involves a context switch and plenty of internal kernel administration and setup like allocating a dedicated stack). Keeping a thread pool improves the situation, but now you become responsible for scheduling the tasks for that pool.
  + Threads are good for solving _CPU-bound_ problems like matrix multiplication or other mathematical algorithms that do not involve I/O. (Though you should think then seriosly about offloading these problems to a GPU/FPGA instead.)
  + See the link to the Rust book below for more details.

## Async
+ Note that typically each language (like C++, Javascript) has it's own way of doing async, Rust is not an exception.
+ Rust language has a built-in keyword [`async`](https://doc.rust-lang.org/std/keyword.async.html) that first appeared in Rust 2018.
```rust
async fn first_async_trial() -> u8 {
  println!("Hello, Rust async!");
  1 + 2
}

fn main() {
  first_async_trial();
}
```
+ The code compiles but it does not print anything! The reason is that async in Rust is _lazy_: no progress is ever done if you never poll your async code.
+ `async` is basically a syntactic sugar that actually turns your function into a [`Future`](https://doc.rust-lang.org/core/future/trait.Future.html)
```rust,noplayground
pub trait Future {
  type Output;

  // Required method
  fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
```
+ In our specific case, `Output = u8`
+ [`Poll`](https://doc.rust-lang.org/core/task/enum.Poll.html) is an enum that describes two possible states of the Future:
```rust,noplayground
pub enum Poll<T> {
  Ready(T),
  Pending,
}
```
+ `Context` is basically a wrapper around the callback function which notifies the async runtime that `Future` is finished, but the catch is that you don't need to pass it explicitly. Instead of calling `poll()` directly, you have another syntactic sugar, [`await`](https://doc.rust-lang.org/std/keyword.await.html) keyword:
```rust
# async fn first_async_trial() -> u8 {
#   println!("Hello, Rust async!");
#   1 + 2
# }
#[tokio::main]
async fn main() {
  let value = first_async_trial().await;
  println!("The computed value is {}", value);
}
```
+ Note that we made `main()` async as well (`await` can be called exclusively from an `async` function). But then we need an executor (`main()` does not have any `Context` by default), so we used Tokio for that.

Features of Rust async
+ Typical async runtime (like Tokio) combines concurrent execution with parallel, automatically. (But you are free to make it purely concurrent by pinning everything to a single thread, if you really wish.)
+ Async does not rely on `std` library (it does not need either OS or heap allocation) and hence can run even on a bare-metal micro-controller. Take a look at the [Embassy](https://embassy.dev/) framework which implements async runtime for STM32, Micro:bit, and Raspberry Pi Pico.
+ Async itself is lightweight, runtimes (like Tokio) give more overhead but still very performant.
+ Tokio in particular uses OS-provided async facilities like [`epoll`](https://man7.org/linux/man-pages/man7/epoll.7.html) in case of Linux. It means that if you are watching 1000 files, you don't have to poll them again and again, but OS will tell you directly which ones have changed. (Since in UNIX "everything is a file", even timers like [`timerfd`](https://man7.org/linux/man-pages/man2/timerfd_create.2.html), the approach is very scalable.)
+ Async is good for solving _I/O-bound_ problems (e.g., accessing multiple files and listening for mutliple TCP/UDP connections)

## Resources for deeper understanding
+ [Chapter 16](https://doc.rust-lang.org/book/ch16-00-concurrency.html) of the Rust book
+ [Async book](https://rust-lang.github.io/async-book/) is a good starting point to understand how async Rust works under the hood, without tying yourself to specifics of Tokio or other async runtime.
+ [Tokio tutorial](https://tokio.rs/tokio/tutorial)
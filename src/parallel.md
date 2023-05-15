# Parallel and concurrent programming

## Parallel vs. concurrent
Two types of execution (in addition to the "regular" serial execution):
+ _Parallel_ means that two different pieces of code run at the same time (for example, on different CPU cores)
+ _Concurrent_ means that two different pieces of code run at different moments, but provide an _illusion_ of parallel execution (first piece starts, then it suspends, second piece kicks in, then it suspends, the first one resumes, and so on...).

These two types:
+ are known to be extremely hard to implement properly and even harder to debug (execution order is non-deterministic and hence non-reproducible):
  + concurrent problems were traditionally turned into serial ones using callbacks, but any more-less complex logic leads to so-called [callback hell](https://en.wiktionary.org/wiki/callback_hell) where it's hard to reason who calls what.
+ are typically combined, so we won't distinguish them unless we really need to.
+ are supported by Rust via thread and async mechanisms.

## Threads
+ [`std::thread`](https://doc.rust-lang.org/std/thread/) is an abstraction on top of the native OS threads (1-to-1 correspondence: 1 OS thread corresponds to 1 Rust thread).
  + The preferred way to communicate data between threads is message passing via [`std::sync::mpsc::channel`](https://doc.rust-lang.org/std/sync/mpsc/fn.channel.html). You can have multiple "producers" of a data and a single "consumer". Rust channels are lock-free (at the API level).
  + The less preferred way is locking shared data by means of [`std::sync::Mutex`](https://doc.rust-lang.org/std/sync/struct.Mutex.html) (MUTual EXclusion). Welcome to the world of deadlocks and threads blocking each other! (How long is your critical section?)
  + Both ways of communication are guaranteed to be thread-safe (no data races) at the compile level by means of special marker traits [`core::marker::Send`](https://doc.rust-lang.org/core/marker/trait.Send.html) and [`core::marker::Sync`](https://doc.rust-lang.org/core/marker/trait.Sync.html). The idea is that you cannot accidentally share some value to another thread if it's type does not allow it. (Of course, you are welcome to spoil everything by using `unsafe` code.)
  + The level of parallelism is limited physically by the number of CPU cores. (If you have 8 cores and you spawned 80 threads that are all awake at the same time, you will have 10 concurrent tasks per core, on average.)
  + Spawning a thread is relatively slow and resource-heavy since it is done eventually by the OS (it involves a context switch and plenty of internal kernel administration and setup like allocating a dedicated stack).
  + See the link to the Rust book below for more details.

## Async
+ Combines parallel with concurrent (but you are free to make it purely concurrent by running everything on a single thread, if you really wish)
+ Is lazy: your future does not get polled if you never used it

## Resources for deeper understanding
+ [Chapter 16](https://doc.rust-lang.org/book/ch16-00-concurrency.html) of the Rust book
+ [Async book](https://rust-lang.github.io/async-book/) is a good starting point to understand how async Rust works under the hood, without tying yourself to specifics of Tokio or other async runtime.
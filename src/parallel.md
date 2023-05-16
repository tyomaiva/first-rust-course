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
+ Combines parallel with concurrent (but you are free to make it purely concurrent by running everything on a single thread, if you really wish)
+ Is lazy: your future does not get polled if you never used it
+ Async is good for solving _I/O-bound_ problems (e.g., accessing files or listening for a TCP/UDP connection)

## Resources for deeper understanding
+ [Chapter 16](https://doc.rust-lang.org/book/ch16-00-concurrency.html) of the Rust book
+ [Async book](https://rust-lang.github.io/async-book/) is a good starting point to understand how async Rust works under the hood, without tying yourself to specifics of Tokio or other async runtime.
+ [Tokio tutorial](https://tokio.rs/tokio/tutorial)
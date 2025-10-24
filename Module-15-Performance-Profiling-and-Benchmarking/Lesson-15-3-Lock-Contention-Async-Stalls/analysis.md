# Lesson 15.3: Lock Contention & Async Stalls

## 🧠 Concept Summary

This lesson explores common performance pitfalls in concurrent and asynchronous Rust applications: **lock contention** and **async stalls**. Understanding and avoiding these issues is crucial for building high-performance systems.

- **Lock Contention:**
    - Occurs when multiple threads or `async` tasks frequently try to acquire the same lock (e.g., `tokio::sync::Mutex`, `std::sync::Mutex`).
    - If a lock is held for a significant duration, other tasks attempting to acquire it will block, waiting for its release. This serialization of access can severely degrade performance, especially in highly concurrent scenarios.
    - Symptoms include reduced throughput, increased latency, and CPU usage that doesn't scale with the number of cores.

- **Async Stalls:**
    - An async stall (or blocking the async runtime) occurs when an `async` task performs a *blocking* operation on the main `async` runtime thread.
    - Blocking operations include `std::thread::sleep()`, synchronous file I/O (`std::fs::File::open`), or acquiring a `std::sync::Mutex` lock (which blocks the thread until acquired).
    - This is problematic because the `async` runtime relies on its threads being able to quickly switch between tasks. If a thread is blocked, all other `async` tasks scheduled on that thread cannot make progress, leading to a complete halt of the `async` runtime's progress.

## 🧩 Code Walkthrough

Let's analyze the code in `main.rs`.

### Lock Contention Example

```rust
async fn contended_mutex_example() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..100 {
        let counter_clone = Arc::clone(&counter);
        handles.push(tokio::spawn(async move {
            time::sleep(Duration::from_millis(1)).await; // Work before lock
            let mut num = counter_clone.lock().await;
            time::sleep(Duration::from_micros(10)).await; // Hold lock for a bit
            *num += 1;
        }));
    }
    // ... await handles ...
}
```

This example simulates high contention on a single `tokio::sync::Mutex`. 100 tasks are spawned, each trying to acquire the lock, hold it briefly, and then release it. The `time::sleep` calls before and during the lock acquisition exaggerate the contention. If you run this, you'll notice that even though tasks are `async`, the total execution time will be significantly longer than the sum of individual `sleep`s, due to tasks waiting for the mutex.

### Async Stall Example

```rust
async fn async_stall_example() {
    println!("Async stall example: Starting non-blocking task.");
    let non_blocking_task = tokio::spawn(async {
        for i in 0..5 {
            println!("Non-blocking task: {}", i);
            time::sleep(Duration::from_millis(50)).await;
        }
    });

    println!("Async stall example: Starting blocking operation on main thread.");
    std::thread::sleep(Duration::from_millis(300)); // THIS BLOCKS THE RUNTIME
    println!("Async stall example: Blocking operation finished.");

    non_blocking_task.await.unwrap();
}
```

Here, a `non_blocking_task` is spawned that prints messages periodically. However, the `main` `async` function then calls `std::thread::sleep()`. This is a *blocking* call that halts the entire thread on which the Tokio runtime is running. You will observe that the `non_blocking_task`'s output is paused for the duration of `std::thread::sleep()`, demonstrating the async stall.

## ⚔️ Cross-Language Insights

- **Golang:** Go's `sync.Mutex` can also lead to contention. Blocking operations in `goroutine`s can also cause issues, though the Go runtime is designed to handle many blocking calls by scheduling other `goroutine`s. However, excessive blocking can still degrade performance.

- **TypeScript (Node.js):** Node.js is single-threaded, so blocking the event loop is a critical performance issue. Any synchronous, long-running operation will cause an async stall. This is why non-blocking I/O and offloading CPU-bound tasks to worker threads are paramount in Node.js.

- **C++:** Lock contention is a classic problem in multi-threaded C++ applications. Async stalls are less of a direct concern in C++'s `std::async` model, as it typically uses thread pools, but blocking operations on those threads can still lead to resource exhaustion.

## 🚀 Practical Reflection

- **Minimizing Lock Scope:** When using locks, hold them for the shortest possible duration. Only protect the critical section of code that absolutely requires exclusive access.

- **Choosing the Right Lock:** `RwLock` can reduce contention for read-heavy workloads by allowing multiple readers concurrently.

- **Avoiding Blocking Calls in Async:** Never use blocking I/O or synchronous synchronization primitives (`std::sync::Mutex`) directly within `async` functions that run on the main `async` runtime threads. Instead, use `tokio::task::spawn_blocking` to offload such operations to a dedicated blocking thread pool.

- **Profiling for Concurrency Issues:** Tools like `perf` and `flamegraph` (especially with `perf`'s `lock` events) can help identify lock contention. Specialized `async` profilers (e.g., `tokio-console`) can help diagnose async stalls.

## 🧩 Self-Review Prompts

- Modify the `contended_mutex_example` to use `tokio::sync::RwLock` instead of `Mutex`. How would you expect the performance to change if you had many more readers than writers?
- Rewrite the `async_stall_example` to correctly handle the blocking operation using `tokio::task::spawn_blocking`.
- Research `tokio-console`. How can it help visualize and debug async stalls and contention?

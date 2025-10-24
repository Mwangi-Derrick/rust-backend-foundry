# Lesson 14.5: Parallel Processing with join! / rayon

## 🧠 Concept Summary

This lesson explores how to achieve **parallel processing** in Rust, distinguishing between asynchronous concurrency and true parallelism. We'll look at two primary ways to execute code across multiple CPU cores:

- **Concurrency vs. Parallelism:**
    - **Concurrency:** Dealing with many things at once (e.g., `async/await` with Tokio, where a single thread can manage multiple tasks waiting for I/O).
    - **Parallelism:** Doing many things at once (e.g., using multiple CPU cores to execute computations simultaneously).

- **`tokio::task::spawn_blocking`:** For CPU-bound tasks within an `async` application, this function offloads work to a dedicated thread pool managed by Tokio. This prevents CPU-intensive operations from blocking the main `async` runtime thread, allowing I/O-bound tasks to continue making progress.

- **`rayon`:** A powerful data-parallelism library for Rust. It makes it easy to convert sequential iterators into parallel iterators, automatically distributing the work across available CPU cores. `rayon` is ideal for CPU-bound computations on collections.

## ⚙️ Setup

To use `rayon`, you need to add it to your `Cargo.toml` file:

```toml
[dependencies]
rayon = "1.5"
```

## 🧩 Code Walkthrough

Let's analyze the code in `main.rs`.

### Parallelism with `tokio::task::spawn_blocking`

```rust
fn cpu_intensive_task(id: u32) -> u32 {
    let mut sum = 0;
    for i in 0..1_000_000 {
        sum += i;
    }
    println!("CPU intensive task {} finished.", id);
    sum
}

async fn spawn_blocking_example() {
    let mut handles = vec![];

    for i in 0..3 {
        handles.push(task::spawn_blocking(move || cpu_intensive_task(i)));
    }

    for handle in handles {
        handle.await.unwrap();
    }
}
```

The `cpu_intensive_task` function simulates a CPU-bound operation. In `spawn_blocking_example`, we use `tokio::task::spawn_blocking` to run multiple instances of this task. Each `spawn_blocking` call executes its closure on a separate thread from Tokio's dedicated blocking thread pool. This ensures that these CPU-intensive tasks do not starve the main `async` event loop.

### Parallelism with `rayon`

```rust
use rayon::prelude::*;

fn rayon_example() {
    let mut vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // Sequential processing
    let sum_seq: u32 = vec.iter().map(|x| x * 2).sum();

    // Parallel processing
    let sum_par: u32 = vec.par_iter().map(|x| x * 2).sum();

    // Parallel modification
    vec.par_iter_mut().for_each(|x| *x += 1);
}
```

`rayon` provides parallel iterators (e.g., `par_iter()`, `par_iter_mut()`) that mirror the standard library's sequential iterators. By simply changing `iter()` to `par_iter()`, `rayon` automatically distributes the work of `map` and `sum` across multiple threads, leveraging all available CPU cores. This is a very ergonomic way to achieve data parallelism.

## ⚔️ Cross-Language Insights

- **Golang:** Go achieves parallelism through `goroutine`s. The Go runtime schedules `goroutine`s across available CPU cores. For data parallelism, you would typically manually divide the work and launch `goroutine`s for each chunk, then use `sync.WaitGroup` to wait for them.

- **TypeScript (Node.js):** Node.js is single-threaded by default. To achieve parallelism, you would use `worker_threads` to offload CPU-bound tasks to separate threads. Data parallelism would involve manually splitting data and sending chunks to different workers.

- **C++:** C++ offers various ways to achieve parallelism, including `std::thread`, OpenMP, TBB (Threading Building Blocks), and `std::for_each` with execution policies (C++17).

## 🚀 Practical Reflection

- **Choosing the Right Tool:**
    - Use `tokio::task::spawn_blocking` when you have a CPU-bound task that needs to run within an `async` application without blocking the `async` runtime.
    - Use `rayon` when you have a collection of data and you want to process it in parallel across multiple CPU cores.

- **Performance Gains:** Parallel processing can lead to significant performance improvements for CPU-bound workloads, especially on multi-core machines.

- **Overhead:** Be mindful of the overhead associated with creating threads and coordinating parallel work. For very small tasks, the overhead might outweigh the benefits.

## 🧩 Self-Review Prompts

- Modify the `cpu_intensive_task` to take a `Vec<u32>` and return the sum of its elements. Then use `spawn_blocking` to process multiple such vectors in parallel.
- Use `rayon` to find the maximum value in a large `Vec<u32>`.
- What are the potential challenges of debugging parallel code compared to sequential code?

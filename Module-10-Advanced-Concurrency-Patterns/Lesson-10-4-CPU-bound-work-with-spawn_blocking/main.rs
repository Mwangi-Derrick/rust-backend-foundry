// Lesson 10.4: CPU-bound work with spawn_blocking

// This lesson explains how to handle CPU-bound work in an async application.

// --- The Problem with CPU-bound Work ---

// Async runtimes like Tokio are designed for I/O-bound work, where tasks spend
// most of their time waiting for I/O to complete. If you have a CPU-bound task
// (a task that does a lot of computation), it can block the thread and prevent
// other tasks from running. This can lead to poor performance and high latency.

// --- `spawn_blocking` ---

// The `tokio::task::spawn_blocking` function is a way to run a blocking function
// on a separate thread pool that is specifically designed for blocking
// operations. This allows you to run CPU-bound work without blocking the main
// async runtime.

use tokio::task;
use tokio::time::{self, Duration};

fn cpu_bound_work() -> u64 {
    let mut sum = 0;
    for i in 0..1_000_000_000 {
        sum += i;
    }
    sum
}

#[tokio::main]
async fn main() {
    println!("Starting CPU-bound work...");

    let handle = task::spawn_blocking(cpu_bound_work);

    // We can do other work while the CPU-bound work is running.
    for i in 0..5 {
        println!("Doing other work...");
        time::sleep(Duration::from_millis(100)).await;
    }

    let result = handle.await.unwrap();
    println!("CPU-bound work finished. Result: {}", result);
}

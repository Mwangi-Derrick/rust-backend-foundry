// Lesson 14.5: Parallel Processing with join! / rayon

// This lesson explores how to achieve parallel processing in Rust, distinguishing
// between asynchronous concurrency and true parallelism.

// --- Concurrency vs. Parallelism ---

// - **Concurrency:** Dealing with many things at once. Async Rust (Tokio) provides
//   concurrency by allowing a single thread to manage multiple tasks that are
//   waiting for I/O.

// - **Parallelism:** Doing many things at once. This typically involves using
//   multiple CPU cores to execute computations simultaneously.

// --- Parallelism with `tokio::task::spawn_blocking` ---

// For CPU-bound tasks within an async application, `tokio::task::spawn_blocking`
// can offload work to a dedicated thread pool, allowing the main async runtime
// to remain unblocked.

use tokio::task;
use tokio::time::{self, Duration};

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

// --- Parallelism with `rayon` ---

// `rayon` is a data-parallelism library for Rust. It makes it easy to convert
// sequential iterators into parallel iterators, automatically distributing the
// work across available CPU cores.

// Note: To use `rayon`, you need to add it to your `Cargo.toml`:
//
// [dependencies]
// rayon = "1.5"

use rayon::prelude::*;

fn rayon_example() {
    let mut vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // Sequential processing
    let sum_seq: u32 = vec.iter().map(|x| x * 2).sum();
    println!("Sequential sum: {}", sum_seq);

    // Parallel processing
    let sum_par: u32 = vec.par_iter().map(|x| x * 2).sum();
    println!("Parallel sum: {}", sum_par);

    // Parallel modification
    vec.par_iter_mut().for_each(|x| *x += 1);
    println!("Parallel modified vec: {:?}", vec);
}

#[tokio::main]
async fn main() {
    println!("--- spawn_blocking Example ---");
    spawn_blocking_example().await;

    println!("\n--- Rayon Example ---");
    rayon_example();
}

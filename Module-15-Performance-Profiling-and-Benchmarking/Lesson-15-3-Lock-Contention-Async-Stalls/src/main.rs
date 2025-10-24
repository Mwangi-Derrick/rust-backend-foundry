// Lesson 15.3: Lock Contention & Async Stalls

// This lesson explores common performance pitfalls in concurrent and asynchronous
// Rust applications: lock contention and async stalls.

// --- Lock Contention ---

// Lock contention occurs when multiple threads or async tasks frequently try to
// acquire the same lock (e.g., `Mutex`, `RwLock`). If one task holds the lock
// for a long time, other tasks will be blocked, waiting for the lock to be
// released. This can severely degrade performance, especially in highly
// concurrent applications.

use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{self, Duration};

async fn contended_mutex_example() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..100 {
        let counter_clone = Arc::clone(&counter);
        handles.push(tokio::spawn(async move {
            // Simulate some work before acquiring the lock
            time::sleep(Duration::from_millis(1)).await;

            let mut num = counter_clone.lock().await;
            // Simulate holding the lock for a while
            time::sleep(Duration::from_micros(10)).await;
            *num += 1;
        }));
    }

    for handle in handles {
        handle.await.unwrap();
    }

    println!("Final counter value (contended): {}", *counter.lock().await);
}

// --- Async Stalls ---

// An async stall occurs when an `async` task performs a blocking operation
// (e.g., `std::thread::sleep`, `std::fs::File::open`, or a synchronous `Mutex`
// lock) on the main async runtime thread. This blocks the entire thread, preventing
// other `async` tasks from making progress, even if they are ready to run.

async fn async_stall_example() {
    println!("Async stall example: Starting non-blocking task.");
    let non_blocking_task = tokio::spawn(async {
        for i in 0..5 {
            println!("Non-blocking task: {}", i);
            time::sleep(Duration::from_millis(50)).await;
        }
    });

    println!("Async stall example: Starting blocking operation on main thread.");
    // This will block the entire Tokio runtime thread!
    std::thread::sleep(Duration::from_millis(300));
    println!("Async stall example: Blocking operation finished.");

    non_blocking_task.await.unwrap();
}

#[tokio::main]
async fn main() {
    println!("--- Lock Contention Example ---");
    contended_mutex_example().await;

    println!("\n--- Async Stall Example ---");
    async_stall_example().await;
}

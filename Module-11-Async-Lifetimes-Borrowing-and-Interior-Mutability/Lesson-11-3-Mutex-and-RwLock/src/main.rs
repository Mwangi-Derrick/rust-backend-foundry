// Lesson 11.3: Mutex and RwLock (Thread-safe Patterns)

// This lesson introduces `Mutex` and `RwLock`, which are synchronization
// primitives that allow safe shared mutable state across multiple threads or
// async tasks.

// --- The Problem with `RefCell` in Multi-threaded Contexts ---

// `RefCell` provides interior mutability but is not thread-safe. If you try to
// use `RefCell` across multiple threads, the compiler will prevent it because
// `RefCell` does not implement `Sync`.

// --- `Mutex<T>` ---

// A `Mutex` (mutual exclusion) is a synchronization primitive that allows only
// one thread to access a shared resource at a time. It provides exclusive
// access to the data it protects.

// In Rust, `Mutex` is typically used in conjunction with `Arc<T>` to allow
// multiple threads to share ownership of the `Mutex`.

use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::task;

async fn mutex_example() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for i in 0..10 {
        let counter_clone = Arc::clone(&counter);
        let handle = task::spawn(async move {
            let mut num = counter_clone.lock().await;
            *num += 1;
            println!("Task {} incremented counter to {}", i, *num);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap();
    }

    println!("Final counter value: {}", *counter.lock().await);
}

// --- `RwLock<T>` ---

// An `RwLock` (read-write lock) is a synchronization primitive that allows
// multiple readers or one writer to access a shared resource at a time.
// It is more permissive than a `Mutex` because it allows multiple readers to
// access the data concurrently.

use tokio::sync::RwLock;

async fn rwlock_example() {
    let data = Arc::new(RwLock::new(String::from("hello")));
    let mut handles = vec![];

    // Multiple readers
    for i in 0..3 {
        let data_clone = Arc::clone(&data);
        let handle = task::spawn(async move {
            let r = data_clone.read().await;
            println!("Reader task {} read: {}", i, *r);
        });
        handles.push(handle);
    }

    // One writer
    let data_clone = Arc::clone(&data);
    let writer_handle = task::spawn(async move {
        let mut w = data_clone.write().await;
        w.push_str(", world");
        println!("Writer task wrote: {}", *w);
    });
    handles.push(writer_handle);

    // Another reader after the writer
    let data_clone = Arc::clone(&data);
    let handle = task::spawn(async move {
        let r = data_clone.read().await;
        println!("Reader task after writer read: {}", *r);
    });
    handles.push(handle);

    for handle in handles {
        handle.await.unwrap();
    }
}

#[tokio::main]
async fn main() {
    println!("--- Mutex Example ---");
    mutex_example().await;

    println!("\n--- RwLock Example ---");
    rwlock_example().await;
}

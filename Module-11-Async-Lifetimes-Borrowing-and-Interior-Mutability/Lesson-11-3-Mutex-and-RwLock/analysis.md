# Lesson 11.3: Mutex and RwLock (Thread-safe Patterns)

## 🧠 Concept Summary

This lesson introduces **`Mutex<T>`** and **`RwLock<T>`** (Read-Write Lock), which are synchronization primitives that allow safe shared mutable state across multiple threads or `async` tasks. These are the multi-threaded equivalents of `RefCell`.

- **The Problem:** `RefCell` provides interior mutability but is *not* thread-safe. When you need to share mutable data between multiple threads or `async` tasks, you need stronger guarantees.

- **`Mutex<T>` (Mutual Exclusion):**
    - A synchronization primitive that allows only *one* thread or task to access a shared resource at a time.
    - It provides exclusive access to the data it protects. When a thread wants to access the data, it must first acquire a lock. If another thread already holds the lock, the current thread will block until the lock is released.
    - In Rust, `Mutex` is typically used in conjunction with `Arc<T>` to allow multiple threads to share ownership of the `Mutex` itself.

- **`RwLock<T>` (Read-Write Lock):**
    - A synchronization primitive that allows *multiple readers* or *one writer* to access a shared resource at a time.
    - It is more permissive than a `Mutex` because it allows multiple readers to access the data concurrently. However, if a writer wants to access the data, it must wait until all readers have released their locks.
    - `RwLock` is generally preferred over `Mutex` when you have many readers and few writers, as it can provide better performance.

## 🧩 Code Walkthrough

Let's analyze the code in `main.rs`.

### `Mutex<T>` Example

```rust
use std::sync::Arc;
use tokio::sync::Mutex;

let counter = Arc::new(Mutex::new(0));
// ... spawn tasks ...
let mut num = counter_clone.lock().await;
*num += 1;
```

Here, we use `Arc<Mutex<i32>>` to share a mutable integer `counter` across multiple `async` tasks. Each task clones the `Arc`, then calls `lock().await` on the `Mutex` to acquire a lock. This `await` is crucial in `async` contexts; it allows the task to yield if the lock is held by another task, rather than blocking the entire thread. Once the lock is acquired, the task gets a mutable reference to the inner `i32` and can increment it. The lock is automatically released when `num` goes out of scope.

### `RwLock<T>` Example

```rust
use tokio::sync::RwLock;

let data = Arc::new(RwLock::new(String::from("hello")));
// ... reader tasks ...
let r = data_clone.read().await;
// ... writer task ...
let mut w = data_clone.write().await;
```

This example uses `Arc<RwLock<String>>` to share a mutable `String`. Multiple reader tasks can acquire a read lock concurrently using `read().await`. A writer task acquires a write lock using `write().await`. While a write lock is held, no other read or write locks can be acquired. This allows for more concurrency than a `Mutex` when reads are frequent and writes are rare.

## ⚔️ Cross-Language Insights

- **vs. Golang:** Go's `sync.Mutex` and `sync.RWMutex` are very similar to Rust's `Mutex` and `RwLock`. They provide the same functionality for protecting shared mutable state.

- **vs. TypeScript:** In JavaScript/TypeScript, shared mutable state across threads is generally avoided due to the single-threaded nature of the event loop. For web workers, communication is typically done via message passing. In Node.js, `Worker` threads can share `SharedArrayBuffer`s, but access to them needs to be synchronized using `Atomics`.

- **vs. C++:** `std::mutex` and `std::shared_mutex` in C++ provide similar functionality. `std::unique_lock` and `std::shared_lock` are used to manage the locks.

## 🚀 Practical Reflection

- **Thread Safety:** `Mutex` and `RwLock` are fundamental for writing thread-safe code in Rust. They ensure that only one thread can modify shared data at a time, preventing data races.

- **Performance Trade-offs:** `Mutex` is simpler but can limit concurrency. `RwLock` offers more concurrency for read-heavy workloads but has slightly more overhead. Choose the right tool for your specific access patterns.

- **Deadlocks:** Be careful when using `Mutex` and `RwLock` to avoid deadlocks. A deadlock occurs when two or more threads are blocked forever, waiting for each other to release a lock. This is a common problem in concurrent programming.

## 🧩 Self-Review Prompts

- Modify the `mutex_example` to demonstrate a deadlock scenario. How would you fix it?
- When would you prefer `RwLock` over `Mutex`?
- What happens if you try to acquire a `Mutex` lock twice in the same thread without releasing it?

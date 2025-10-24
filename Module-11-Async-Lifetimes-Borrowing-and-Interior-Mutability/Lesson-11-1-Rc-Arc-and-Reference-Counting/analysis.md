# Lesson 11.1: Rc, Arc, and Reference Counting

## 🧠 Concept Summary

This lesson introduces **`Rc<T>`** (Reference Counted) and **`Arc<T>`** (Atomic Reference Counted), which are smart pointers that enable **multiple ownership** of data. They are crucial when Rust's strict ownership rules (one owner at a time) don't quite fit your data model.

- **The Problem:** Rust's ownership rules prevent data races and ensure memory safety by enforcing a single owner. However, sometimes you need multiple parts of your program to logically "own" the same piece of data (e.g., in graph data structures, UI components sharing state).

- **`Rc<T>` (Reference Counted):**
    - A single-threaded smart pointer that allows multiple immutable references to a value.
    - It keeps a count of how many `Rc` pointers are pointing to the data. When the count drops to zero, the data is dropped.
    - It's efficient because it doesn't need atomic operations for its reference count.

- **`Arc<T>` (Atomic Reference Counted):**
    - A multi-threaded smart pointer, safe to share across threads (and thus across `async` tasks).
    - Similar to `Rc<T>`, but its reference count is updated using atomic operations, which are slower but necessary for thread safety.
    - Use `Arc<T>` when you need to share data between `async` tasks or threads.

## 🧩 Code Walkthrough

Let's analyze the code in `main.rs`.

### `Rc<T>` Example

```rust
use std::rc::Rc;

let a = Rc::new(String::from("hello"));
println!("Count after creating a: {}", Rc::strong_count(&a)); // 1

let b = Rc::clone(&a);
println!("Count after creating b: {}", Rc::strong_count(&a)); // 2

{ // new scope
    let c = Rc::clone(&a);
    println!("Count after creating c: {}", Rc::strong_count(&a)); // 3
} // c goes out of scope, count becomes 2

println!("Count after c goes out of scope: {}", Rc::strong_count(&a)); // 2
```

This demonstrates how `Rc::clone(&a)` increments the reference count. When `c` goes out of scope, its `Rc` is dropped, and the count decrements. The data is only deallocated when the last `Rc` (in this case, `a` and `b`) goes out of scope.

### `Arc<T>` Example

```rust
use std::sync::Arc;
use tokio::task;

async fn arc_example() {
    let a = Arc::new(String::from("hello"));
    // ...
    for i in 0..3 {
        let a_clone = Arc::clone(&a);
        let handle = task::spawn(async move {
            println!("Task {} has a: {}", i, a_clone);
            // ...
        });
        // ...
    }
    // ...
}
```

Here, we use `Arc<T>` to share a `String` across multiple `async` tasks. `Arc::clone(&a)` creates a new `Arc` pointer, incrementing the atomic reference count. Each task receives its own `Arc` clone, allowing it to access the shared data. The data will only be dropped when all `Arc`s go out of scope.

## ⚔️ Cross-Language Insights

- **vs. Golang:** Go's garbage collector handles memory management automatically, so you don't typically deal with explicit reference counting. However, `sync.WaitGroup` or channels are used for coordinating access to shared resources.

- **vs. TypeScript:** JavaScript/TypeScript also use garbage collection. The concept of reference counting is handled internally by the runtime. When you pass objects around, you're passing references, and the GC determines when an object is no longer reachable.

- **vs. C++:** `std::shared_ptr` in C++ is very similar to `Rc<T>` and `Arc<T>`. `std::shared_ptr` also uses reference counting to manage shared ownership, and it has a thread-safe variant for concurrent access.

## 🚀 Practical Reflection

- **Multiple Ownership:** `Rc` and `Arc` are your go-to tools when you need multiple parts of your program to own the same data. This is common in scenarios like shared configuration, caches, or complex data structures.

- **Thread Safety:** Always use `Arc` when sharing data across threads or `async` tasks. Using `Rc` in a multi-threaded context will lead to compile-time errors because `Rc` is not `Send` or `Sync`.

- **Immutability:** By default, `Rc` and `Arc` provide shared *immutable* access to the inner value. If you need shared *mutable* access, you'll need to combine them with interior mutability patterns (like `RefCell` or `Mutex`), which we'll cover in the next lesson.

## 🧩 Self-Review Prompts

- What happens if you try to use `Rc<T>` in a `tokio::spawn` task? Why?
- Create a simple graph data structure where nodes can have multiple parents, using `Rc<T>`.
- How would you implement a simple cache that stores `Arc<String>` values?

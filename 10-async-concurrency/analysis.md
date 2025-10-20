# Lesson 10: Async & Concurrency with Tokio

## 🧠 Concept Summary
This lesson introduces **asynchronous programming** in Rust, a model for writing concurrent code that is highly efficient for I/O-bound workloads (like network services and database interactions). The key concepts are:

-   **`async` / `.await` Syntax**: The `async` keyword transforms a function into one that returns a `Future`. A `Future` is a computation that can be run and will produce a value at some later point. The `.await` keyword is used inside an `async` function to pause its execution until a `Future` is complete, without blocking the entire OS thread. This allows the thread to work on other tasks.

-   **Asynchronous Runtime**: `async` code needs a runtime to execute it. The runtime has a *scheduler* that manages the `Future`s, polling them to see if they can make progress and pausing them when they are waiting for I/O. **Tokio** is the most popular and widely used async runtime in the Rust ecosystem.

-   **Tasks**: A task is an independent, concurrent unit of execution. You can create a task using `tokio::spawn`, which takes a `Future` and runs it in the background, allowing the rest of your code to continue.

This model allows a small number of OS threads to handle tens of thousands of concurrent operations, making it perfect for high-performance network services.

## 🧩 Code Walkthrough

```rust
// Import the 'sleep' function and 'Duration' type from tokio.
use tokio::time::{sleep, Duration};

// This is an async function. It simulates a task that takes 2 seconds to complete.
async fn upload_service() {
    println!("📤 Upload service started...");
    // sleep() returns a Future that completes after the duration.
    // .await pauses this function until the sleep Future is done.
    // While paused, the thread can run other tasks.
    sleep(Duration::from_secs(2)).await;
    println!("✅ Upload completed");
}

// Another async function, simulating a 3-second task.
async fn payment_service() {
    println!("💳 Payment service started...");
    sleep(Duration::from_secs(3)).await;
    println!("✅ Payment processed");
}

// The #[tokio::main] attribute is a macro that sets up the Tokio runtime
// and makes our main function the entry point.
#[tokio::main]
async fn main() {
    println!("🚀 Starting async tasks...");

    // tokio::spawn starts a new concurrent task. It takes a Future (the result of
    // calling our async functions) and runs it in the background.
    // It returns a JoinHandle, which is itself a Future that resolves when the task is done.
    let upload_handle = tokio::spawn(upload_service());
    let payment_handle = tokio::spawn(payment_service());

    // The tokio::join! macro waits for multiple Futures to complete.
    // We wait for both the upload and payment tasks to finish here.
    // If we didn't join, main might exit before the spawned tasks are done.
    let _ = tokio::join!(upload_handle, payment_handle);

    println!("🎉 All async tasks completed!");
}
```

### Expected Output & Timing
The output will show that both services start immediately, and then they complete based on their sleep duration. The total execution time will be around 3 seconds (the duration of the longest task), not 5 seconds, because they run concurrently.

```
🚀 Starting async tasks...
📤 Upload service started...
💳 Payment service started...
(after ~2 seconds)
✅ Upload completed
(after ~3 seconds)
✅ Payment processed
🎉 All async tasks completed!
```

## ⚔️ Cross-Language Insights
-   **Golang Equivalent:**
    -   Rust's `tokio::spawn(foo())` is the direct equivalent of Go's `go foo()`. Both start a concurrent task.
    -   Rust's `async/await` provides *explicit* suspension points. In Go, any I/O call can implicitly yield execution to the Go runtime scheduler. Rust's explicitness can make control flow easier to reason about.
    -   Waiting for tasks in Rust with `join!` is similar to using a `sync.WaitGroup` in Go to wait for a collection of goroutines to finish.
-   **TypeScript (Node.js) Equivalent:**
    -   The `async/await` syntax is virtually identical. Rust's `async fn` is TypeScript's `async function`.
    -   `tokio::spawn` is conceptually similar to creating a `Promise` and not immediately awaiting it. `tokio::join!` is a direct parallel to `Promise.all()`, which takes an array of Promises and resolves when all ofthem have resolved.
-   **C Reference:**
    -   C has no native `async/await`. Concurrency is traditionally done with OS threads (e.g., `pthreads`), which are much heavier than async tasks. A single thread can run thousands of Rust tasks, but would struggle to run thousands of OS threads.
    -   Modern asynchronous C involves complex, low-level, platform-specific APIs like `epoll` (Linux) or `iocp` (Windows). Tokio provides a safe, high-level, cross-platform abstraction over these APIs.

## 🚀 Practical Reflection
Async is not just an option—it's a necessity for building the high-throughput backend systems you're aiming for. Your outbox-relay service is a perfect use case. The main loop will likely look something like this:

```rust
// Pseudo-code
#[tokio::main]
async fn main() {
    loop {
        // 1. Poll the database for new messages (an async DB call)
        let messages = db::get_new_messages().await;

        // 2. For each message, spawn a task to publish it
        for message in messages {
            tokio::spawn(async move {
                // 3. Publish to message broker (an async network call)
                message_queue::publish(message).await;
                // 4. Mark as sent in DB (another async DB call)
                db::mark_as_sent(message.id).await;
            });
        }

        // 5. Wait for a bit before polling again
        sleep(Duration::from_secs(5)).await;
    }
}
```
This structure allows the service to handle publishing many messages concurrently without blocking.

## 🧩 Self-Review Prompts
-   What is the difference between concurrency and parallelism? How does `async` relate to each?
-   What is a `Future` in your own words? What does it mean for a `Future` to be "polled"?
-   What would happen if you removed the `tokio::join!` line from `main`? Why?
-   `tokio::spawn` requires the `Future` to be `'static`. What does this mean and why is it necessary?

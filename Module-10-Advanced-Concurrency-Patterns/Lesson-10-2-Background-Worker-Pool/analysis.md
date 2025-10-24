# Lesson 10.2: Background Worker Pool (Tokio Tasks)

## 🧠 Concept Summary

This lesson demonstrates how to create a **background worker pool** using Tokio tasks. A worker pool is a common pattern for processing a large number of jobs concurrently.

- **Worker Pool:** A set of worker tasks that all pull jobs from a common source and process them in parallel.

- **Shared Receiver:** To have multiple workers pulling from the same MPSC channel, we need to share the receiver. Since `mpsc::Receiver` is not `Copy`, we can't just clone it. Instead, we wrap it in an `Arc<Mutex<...>>`. `Arc` allows for shared ownership across threads, and `Mutex` provides safe concurrent access to the receiver.

- **Graceful Shutdown:** We can gracefully shut down the worker pool by dropping the sender of the channel. This will cause the `recv` calls in the workers to return `None`, which will cause them to exit their loops and shut down.

## 🧩 Code Walkthrough

Let's analyze the code in `main.rs`.

### The Worker

```rust
async fn worker(id: u32, rx: Arc<Mutex<mpsc::Receiver<Job>>>) {
    loop {
        let job = {
            let mut lock = rx.lock().await;
            lock.recv().await
        };

        if let Some(job) = job {
            // ... process the job ...
        } else {
            break;
        }
    }
}
```

The `worker` function takes a shared reference to the receiver. Inside the loop, it locks the mutex, receives a job, and then processes it. If `recv` returns `None`, it means the channel has been closed, so the worker breaks out of the loop and shuts down.

### The Worker Pool

```rust
struct WorkerPool {
    sender: mpsc::Sender<Job>,
}

impl WorkerPool {
    fn new(num_workers: u32) -> Self {
        let (tx, rx) = mpsc::channel(100);
        let rx = Arc::new(Mutex::new(rx));

        for i in 0..num_workers {
            let rx_clone = Arc::clone(&rx);
            tokio::spawn(worker(i, rx_clone));
        }

        WorkerPool { sender: tx }
    }

    // ...
}
```

The `WorkerPool` creates the channel and the workers. It wraps the receiver in an `Arc<Mutex<...>>` and then clones it for each worker. It returns a `WorkerPool` struct that contains the sender, which can be used to send jobs to the workers.

### Running the Pool

```rust
let pool = WorkerPool::new(4);

for i in 0..10 {
    pool.send_job(Job { id: i }).await;
}

drop(pool.sender);
```

In `main`, we create a worker pool with 4 workers. We then send 10 jobs to the pool. Finally, we `drop` the sender. This will close the channel, which will cause the workers to shut down after they have finished processing their current jobs.

## ⚔️ Cross-Language Insights

- **vs. Golang:** You can implement a worker pool in Go using `goroutine`s and a channel. You can have multiple `goroutine`s all receiving from the same channel.

- **vs. TypeScript (Node.js):** In Node.js, you can use the `worker_threads` module to create a pool of worker threads. You would then use a message queue (like RabbitMQ or Redis) to distribute jobs to the workers.

- **vs. C:** You would have to implement a worker pool from scratch using threads, a queue, and mutexes.

## 🚀 Practical Reflection

- **CPU-bound vs. I/O-bound:** This worker pool is good for I/O-bound jobs, where the workers spend most of their time waiting for I/O to complete. For CPU-bound jobs, you would want to use a thread pool with a number of threads equal to the number of CPU cores. We will cover this in a later lesson.

- **Backpressure:** The MPSC channel provides backpressure. If the workers are busy and the channel is full, the `send_job` method will block until there is space in the channel. This prevents the sender from overwhelming the workers.

- **The `async-channel` Crate:** The `async-channel` crate provides a multi-producer, multi-consumer channel that can be a simpler alternative to `Arc<Mutex<mpsc::Receiver<...>>>` for some use cases.

## 🧩 Self-Review Prompts

- Modify the `WorkerPool` to allow for a graceful shutdown signal (e.g., from a broadcast channel).
- What are the trade-offs of using a `Mutex` to share the receiver?
- How would you implement a worker pool for CPU-bound jobs?

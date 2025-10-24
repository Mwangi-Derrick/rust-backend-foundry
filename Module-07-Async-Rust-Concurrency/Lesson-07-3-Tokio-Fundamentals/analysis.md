# Lesson 07.3: Tokio Fundamentals

## 🧠 Concept Summary

This lesson provides a more in-depth look at **Tokio**, the most popular runtime for writing asynchronous applications in Rust.

- **The Tokio Runtime:** Tokio is a library that provides the components needed to run asynchronous code, including an executor, a scheduler, a timer, and an I/O reactor.

- **The `#[tokio::main]` Macro:** This is the easiest way to get started with Tokio. It sets up a default runtime and runs the `async main` function on it. You can also configure the runtime (e.g., to be single-threaded or multi-threaded).

- **Tokio I/O:** Tokio provides asynchronous versions of the standard library's I/O types, such as `tokio::fs::File` and `tokio::net::TcpStream`. These types are non-blocking, which means they don't block the thread when they are waiting for I/O to complete.

- **Tokio Timers:** Tokio provides a timer that allows you to schedule tasks to run at a later time.

## 🧩 Code Walkthrough

Let's analyze the code in `main.rs`.

### Tokio I/O

```rust
use tokio::fs::File;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};

async fn read_and_write_file() -> io::Result<()> {
    let mut file = File::open("foo.txt").await?;
    let mut contents = vec![];
    file.read_to_end(&mut contents).await?;

    let mut file = File::create("bar.txt").await?;
    file.write_all(&contents).await?;

    Ok(())
}
```

This function uses Tokio's asynchronous file I/O. `File::open` and `File::create` are `async` functions that return `Future`s. The `AsyncReadExt` and `AsyncWriteExt` traits provide `async` methods for reading from and writing to the file.

### Tokio Timers

```rust
use tokio::time::{self, Duration};

async fn timer_example() {
    println!("Waiting for 1 second...");
    time::sleep(Duration::from_secs(1)).await;
    println!("Done.");
}
```

The `tokio::time::sleep` function returns a `Future` that completes after a specified duration. This is a useful way to introduce delays or to schedule tasks to run at a later time.

## ⚔️ Cross-Language Insights

- **vs. Golang:** The Tokio runtime is similar to the Go runtime. Both provide a scheduler for running lightweight threads (`goroutine`s or tasks) on a thread pool.

- **vs. TypeScript (Node.js):** Tokio is similar to the Node.js runtime. Both are based on an event loop and provide non-blocking I/O APIs.

- **vs. C:** C does not have a built-in runtime for asynchronous programming. You would have to use a library like `libuv` or `libevent`.

## 🚀 Practical Reflection

- **The Power of Tokio:** Tokio is a very powerful and flexible library. It provides all the tools you need to build high-performance network services, from TCP and UDP sockets to timers and synchronization primitives.

- **Single-threaded vs. Multi-threaded:** The choice between a single-threaded and a multi-threaded runtime depends on your application. A single-threaded runtime can be faster for some workloads because it avoids the overhead of thread synchronization. A multi-threaded runtime can take advantage of multiple CPU cores to run tasks in parallel.

- **The `Async` Ecosystem:** The Rust `async` ecosystem is still evolving, but it is already very powerful. There are many libraries that are built on top of Tokio and `async/await`, such as `hyper` for HTTP, `tonic` for gRPC, and `axum` for web applications.

## 🧩 Self-Review Prompts

- Write a program that uses `tokio::net::TcpListener` to create a simple TCP echo server.
- What is the difference between `tokio::time::sleep` and `std::thread::sleep`?
- Look at the documentation for the `Builder` struct in the `tokio::runtime` module. How can you use it to create a custom runtime?

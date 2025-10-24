# Lesson 07.1: Intro to async/await

## 🧠 Concept Summary

This lesson introduces **`async/await`**, which is Rust's framework for writing asynchronous, non-blocking code. This is a fundamental concept for building high-performance network services.

- **Asynchronous Programming:** A way of writing non-blocking code. Instead of waiting for a long-running operation to complete (like a network request), you can continue to do other work. When the operation is complete, you are notified and can process the result.

- **`async` Keyword:** The `async` keyword transforms a function or a block of code into a state machine that implements the `Future` trait. A `Future` is a value that may not be ready yet.

- **`await` Keyword:** The `await` keyword is used to wait for a `Future` to complete. When you `await` a `Future`, you yield control back to the runtime, which can then run other tasks.

- **Runtime:** `async` code needs a runtime to run it. The runtime is responsible for polling the `Future`s and scheduling them to run on a thread pool. **Tokio** is a popular and powerful runtime for asynchronous Rust.

## ⚙️ Setup

To use `async/await` with Tokio, you need to add the `tokio` and `futures` crates to your `Cargo.toml` file:

```toml
[dependencies]
futures = "0.3"
tokio = { version = "1", features = ["full"] }
```

## 🧩 Code Walkthrough

Let's analyze the code in `main.rs`.

### `async` and `await`

```rust
async fn hello_world() {
    println!("hello, world!");
}

#[tokio::main]
async fn main() {
    hello_world().await;
}
```

The `async fn hello_world()` defines an asynchronous function. When you call it, it returns a `Future`. The `#[tokio::main]` macro sets up the Tokio runtime and makes our `main` function asynchronous. Inside `main`, we call `hello_world()` and then `.await` the returned `Future` to run it.

### A More Complex Example

```rust
async fn download_file(url: &str) -> String {
    // ...
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    String::from("File contents")
}

let file_contents = download_file("https://example.com").await;
```

This example simulates a long-running I/O operation with `tokio::time::sleep`. When we `await` the sleep, we are telling the runtime that we are waiting for something, and it can go and do other work. Once the sleep is over, the runtime will resume our task.

### Running Futures Concurrently

```rust
let future1 = download_file("https://example.com/1");
let future2 = download_file("https://example.com/2");

let (contents1, contents2) = tokio::join!(future1, future2);
```

This shows how to run multiple `Future`s concurrently. `tokio::join!` takes multiple `Future`s and runs them at the same time. It will wait for all of them to complete and then return a tuple of their results.

## ⚔️ Cross-Language Insights

- **vs. Golang:** Go has `goroutine`s, which are lightweight threads. You can spawn a `goroutine` with the `go` keyword. This is similar to spawning a task in Tokio. Go's channels are used for communication between `goroutine`s, which we will cover in a later lesson.

- **vs. TypeScript:** `async/await` in Rust is very similar to `async/await` in TypeScript. Both are based on the concept of promises/futures.

- **vs. C:** C does not have built-in support for `async/await`. You would have to use a library like `libuv` (which Node.js is built on) to write asynchronous code.

## 🚀 Practical Reflection

- **Non-blocking I/O:** `async/await` is essential for writing high-performance network services that can handle many connections at once. Because the I/O operations are non-blocking, a single thread can handle thousands of concurrent connections.

- **The `Future` Trait:** The `Future` trait is the core of `async/await` in Rust. It is a trait that represents a value that will be available in the future. The `poll` method on the `Future` trait is what the runtime calls to see if the `Future` is ready.

- **Runtimes are Libraries:** It's important to remember that the `async/await` syntax is part of the Rust language, but the runtime is a library. Tokio is the most popular runtime, but there are others, like `async-std` and `smol`.

## 🧩 Self-Review Prompts

- Write an `async` function that downloads two files concurrently and then prints the contents of both.
- What is the difference between `tokio::join!` and `tokio::spawn`?
- Look at the documentation for the `Future` trait. What is the `poll` method and what does it do?

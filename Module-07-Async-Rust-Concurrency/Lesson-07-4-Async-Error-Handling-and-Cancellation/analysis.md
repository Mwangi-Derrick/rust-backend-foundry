# Lesson 07.4: Async Error Handling and Cancellation

## 🧠 Concept Summary

This lesson covers two important topics in asynchronous programming: **error handling** and **cancellation**.

- **Error Handling in Async Code:** Error handling in `async` code is very similar to error handling in synchronous code. You can use `Result` and the `?` operator to propagate errors. The main difference is that you have to `.await` the `Future`s that can return errors.

- **Cancellation:** Cancellation is the process of stopping a `Future` before it has completed. In Rust, cancellation is handled by **dropping the `Future`**. When a `Future` is dropped, it will stop executing. This is a key feature of Rust's `async` system and is different from how cancellation is handled in many other languages.

- **The `select!` Macro:** The `select!` macro from Tokio is a powerful tool for handling cancellation. It allows you to wait on multiple `Future`s at the same time and returns when the first one completes. The other `Future`s are then dropped, which cancels them.

## 🧩 Code Walkthrough

Let's analyze the code in `main.rs`.

### Error Handling

```rust
async fn read_file_and_parse() -> Result<i32> {
    let mut file = File::open("number.txt").await?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;
    let number = contents.trim().parse()?;
    Ok(number)
}
```

This function is very similar to the synchronous error handling code we have seen before. The only difference is that we have to `.await` the `async` functions (`File::open` and `read_to_string`). The `?` operator works just the same, propagating any errors that occur.

### Cancellation with `select!`

```rust
tokio::select! {
    _ = long_running_task() => {
        println!("The long-running task finished first.");
    }
    _ = time::sleep(Duration::from_secs(2)) => {
        println!("The timer finished first. The long-running task was cancelled.");
    }
}
```

This `select!` block waits for either `long_running_task` or `time::sleep` to complete. In this case, the `sleep` will complete first because it has a shorter duration. When the `sleep` completes, the `long_running_task` `Future` is dropped, which cancels it.

## ⚔️ Cross-Language Insights

- **vs. Golang:** Go does not have a direct equivalent to `select!`. You can use a `select` statement with channels to achieve a similar effect, but it is not as general. Cancellation in Go is typically handled with `context`s.

- **vs. TypeScript:** In TypeScript, you can use `Promise.race` to wait for the first of several promises to complete. Cancellation is typically handled with `AbortController`s.

- **vs. C:** C does not have a built-in mechanism for cancellation.

## 🚀 Practical Reflection

- **Drop is Cancellation:** The fact that dropping a `Future` cancels it is a very powerful and elegant feature of Rust. It means that you don't need a separate cancellation mechanism; you can just use the ownership system that you are already familiar with.

- **Graceful Shutdown:** The `select!` macro is a key tool for implementing graceful shutdown in servers. You can have one branch of the `select!` that listens for a shutdown signal (e.g., from a channel), and another branch that runs the main server loop. When the shutdown signal is received, the server loop `Future` will be dropped, and you can perform any necessary cleanup.

- **Timeouts:** `select!` is also a great way to implement timeouts. You can have one branch that runs your `Future` and another branch that is a `time::sleep`. If the `sleep` completes first, you know that your `Future` has timed out.

## 🧩 Self-Review Prompts

- Write a program that spawns a long-running task and then cancels it after 1 second.
- What happens if multiple branches of a `select!` block are ready at the same time?
- Look at the documentation for `tokio::signal`. How can you use it to handle shutdown signals (like Ctrl-C)?

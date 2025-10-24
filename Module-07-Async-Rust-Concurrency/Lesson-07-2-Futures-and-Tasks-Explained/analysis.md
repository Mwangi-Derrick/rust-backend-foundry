# Lesson 07.2: Futures and Tasks Explained

## 🧠 Concept Summary

This lesson dives deeper into the core concepts of asynchronous programming in Rust: **`Future`s** and **tasks**.

- **The `Future` Trait:** A `Future` is a trait that represents a value that may not be ready yet. It's similar to a promise in other languages. The `Future` trait has a `poll` method that the runtime calls to see if the `Future` is ready. You don't usually need to implement this trait yourself; the `async` keyword does it for you.

- **Tasks:** A task is a lightweight, non-blocking unit of execution. You can think of it as an asynchronous green thread. Tasks are what allow you to run multiple operations concurrently.

- **`tokio::spawn`:** The `tokio::spawn` function is used to create a new task. It takes a `Future` and runs it in the background, without blocking the current thread. It returns a `JoinHandle`, which is a `Future` that resolves to the return value of the task.

## 🧩 Code Walkthrough

Let's analyze the code in `main.rs`.

### Spawning a Task

```rust
async fn my_task(id: u32) {
    println!("Task {} started", id);
    time::sleep(Duration::from_secs(1)).await;
    println!("Task {} finished", id);
}

let handle = tokio::spawn(my_task(1));
```

We define an `async` function `my_task`. We then use `tokio::spawn` to create a new task that runs this function. `tokio::spawn` immediately returns a `JoinHandle`, and the task runs in the background.

### Running Multiple Tasks Concurrently

```rust
let mut handles = vec![];

for i in 2..=5 {
    handles.push(tokio::spawn(my_task(i)));
}

for handle in handles {
    handle.await.unwrap();
}
```

This code creates multiple tasks in a loop and stores their `JoinHandle`s in a vector. We then iterate over the handles and `.await` each one. This ensures that we wait for all of the tasks to complete before the program exits.

## ⚔️ Cross-Language Insights

- **vs. Golang:** `tokio::spawn` is very similar to Go's `go` keyword. Both are used to start a new lightweight thread of execution. A `JoinHandle` is similar to a channel that you can use to get the return value of a `goroutine`.

- **vs. TypeScript:** `tokio::spawn` is similar to `Promise.all` or `Promise.race` in that it allows you to manage multiple asynchronous operations. However, tasks are more powerful because they can be managed independently and can communicate with each other.

- **vs. C:** C does not have a built-in concept of tasks. You would have to use a library like `pthreads` to create new threads, which are much heavier than tasks.

## 🚀 Practical Reflection

- **Concurrency vs. Parallelism:** It's important to understand the difference between concurrency and parallelism. Concurrency is the ability to have multiple tasks in progress at the same time. Parallelism is the ability to run multiple tasks at the same time on multiple CPU cores. `tokio::spawn` gives you concurrency. To get parallelism, you need a multi-threaded runtime, which Tokio provides by default.

- **`JoinHandle`:** The `JoinHandle` is a powerful tool for managing tasks. You can use it to wait for a task to complete, to get its return value, or to abort it.

- **Structured Concurrency:** The pattern of spawning multiple tasks and then joining them at the end is an example of structured concurrency. This is a powerful pattern for writing concurrent code that is easy to reason about.

## 🧩 Self-Review Prompts

- What happens if you don't `await` the `JoinHandle` returned by `tokio::spawn`?
- Write a program that spawns two tasks. One task should send a message to the other task over a channel (we will cover channels in a later lesson).
- What is the difference between `tokio::spawn` and `tokio::join!`?

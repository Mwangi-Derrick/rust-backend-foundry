// Lesson 07.2: Futures and Tasks Explained

// In the last lesson, we were introduced to `async/await`. In this lesson, we
// will dive deeper into the concepts of `Future`s and tasks.

// --- The `Future` Trait ---

// A `Future` is a trait that represents a value that may not be ready yet. It's
// a bit like a promise in JavaScript. The `Future` trait has one required
// method, `poll`, which the runtime calls to see if the `Future` is ready.

// The `poll` method returns a `Poll` enum, which has two variants:
// - `Poll::Ready(T)`: The `Future` is complete, and a value of type `T` is ready.
// - `Poll::Pending`: The `Future` is not ready yet.

// You don't usually need to implement the `Future` trait yourself. The `async`
// keyword does it for you.

// --- Tasks ---

// A task is a lightweight, non-blocking unit of execution. You can think of it
// as an asynchronous green thread. Tasks are run by an executor, like Tokio.

// The `tokio::spawn` function is used to create a new task. It takes a `Future`
// and runs it in the background, without blocking the current thread.

use tokio::time::{self, Duration};

async fn my_task(id: u32) {
    println!("Task {} started", id);
    time::sleep(Duration::from_secs(1)).await;
    println!("Task {} finished", id);
}

#[tokio::main]
async fn main() {
    // --- Spawning a Task ---

    let handle = tokio::spawn(my_task(1));

    // `tokio::spawn` returns a `JoinHandle`, which is a `Future` that resolves to
    // the return value of the task. We can `.await` the handle to wait for the
    // task to complete.

    // --- Running Multiple Tasks Concurrently ---

    let mut handles = vec![];

    for i in 2..=5 {
        handles.push(tokio::spawn(my_task(i)));
    }

    // We can wait for all of the tasks to complete by awaiting their handles.
    for handle in handles {
        handle.await.unwrap();
    }

    // We need to await the first handle as well.
    handle.await.unwrap();

    println!("All tasks finished!");
}

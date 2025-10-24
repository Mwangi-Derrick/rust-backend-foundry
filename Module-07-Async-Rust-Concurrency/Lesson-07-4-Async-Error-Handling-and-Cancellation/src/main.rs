// Lesson 07.4: Async Error Handling and Cancellation

// This lesson covers how to handle errors in asynchronous code and how to
// handle cancellation.

// --- Error Handling in Async Code ---

// Error handling in `async` code is very similar to error handling in synchronous
// code. You can use `Result` and the `?` operator to propagate errors.

use anyhow::Result;
use tokio::fs::File;
use tokio::io::{self, AsyncReadExt};

async fn read_file_and_parse() -> Result<i32> {
    let mut file = File::open("number.txt").await?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;
    let number = contents.trim().parse()?;
    Ok(number)
}

// --- Cancellation ---

// Cancellation is the process of stopping a `Future` before it has completed.
// In Rust, cancellation is handled by dropping the `Future`. When a `Future` is
// dropped, it will stop executing.

// This is a key difference from other languages, where cancellation is often an
// explicit operation.

// Tokio provides some tools for handling cancellation, such as `select!` and
// cancellation tokens.

use tokio::time::{self, Duration};

async fn long_running_task() {
    println!("Long-running task started");
    time::sleep(Duration::from_secs(5)).await;
    println!("Long-running task finished");
}

#[tokio::main]
async fn main() {
    // --- Error Handling ---

    // Note: To run this code, you will need to create a file named `number.txt`
    // in the root of your project with a number in it.

    match read_file_and_parse().await {
        Ok(n) => println!("The number is: {}", n),
        Err(e) => println!("Error: {:?}", e),
    }

    // --- Cancellation with `select!` ---

    // The `select!` macro allows you to wait on multiple `Future`s at the same
    // time and returns when the first one completes. The other `Future`s are
    // then dropped, which cancels them.

    println!("\n--- Cancellation with select! ---");

    tokio::select! {
        _ = long_running_task() => {
            println!("The long-running task finished first.");
        }
        _ = time::sleep(Duration::from_secs(2)) => {
            println!("The timer finished first. The long-running task was cancelled.");
        }
    }
}

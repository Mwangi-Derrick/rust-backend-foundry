// Lesson 07.3: Tokio Fundamentals

// This lesson provides a more in-depth look at Tokio, the most popular runtime
// for writing asynchronous applications in Rust.

// --- The Tokio Runtime ---

// The Tokio runtime is a library that provides the components needed to run
// asynchronous code, including:
// - An executor for running tasks.
// - A scheduler for scheduling tasks to run on a thread pool.
// - A timer for scheduling tasks to run at a later time.
// - An I/O reactor for handling asynchronous I/O operations.

// --- The `#[tokio::main]` Macro ---

// The `#[tokio::main]` macro is the easiest way to get started with Tokio. It
// sets up a default runtime and runs the `async` `main` function on it.

// You can also configure the runtime by using `#[tokio::main(flavor = "...")]`.
// The two main flavors are:
// - `"current_thread"`: A single-threaded runtime.
// - `"multi_thread"`: A multi-threaded runtime (the default).

// --- Tokio I/O ---

// Tokio provides asynchronous versions of the standard library's I/O types,
// such as `tokio::fs::File` and `tokio::net::TcpStream`.

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

// --- Tokio Timers ---

// Tokio provides a timer that allows you to schedule tasks to run at a later
// time.

use tokio::time::{self, Duration};

async fn timer_example() {
    println!("Waiting for 1 second...");
    time::sleep(Duration::from_secs(1)).await;
    println!("Done.");
}

#[tokio::main]
async fn main() {
    // Note: To run this code, you will need to create a file named `foo.txt`
    // in the root of your project.

    if let Err(e) = read_and_write_file().await {
        eprintln!("Error reading/writing file: {}", e);
    }

    timer_example().await;
}

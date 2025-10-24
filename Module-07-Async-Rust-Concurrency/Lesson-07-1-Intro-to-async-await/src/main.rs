// Lesson 07.1: Intro to async/await

// This lesson introduces async/await, which is Rust's built-in framework for
// writing asynchronous code.

// Note: To run this code, you will need to add the `futures` and `tokio` crates
// to your `Cargo.toml` file:
//
// [dependencies]
// futures = "0.3"
// tokio = { version = "1", features = ["full"] }

// --- What is Async? ---

// Asynchronous programming is a way of writing non-blocking code. Instead of
// waiting for a long-running operation to complete, you can submit the operation
// and then continue to do other work. When the operation is complete, you will
// be notified and you can process the result.

// This is especially important for I/O-bound operations, like reading from a
// file or making a network request.

// --- `async` and `await` ---

// The `async` keyword transforms a block of code into a state machine that
// implements a trait called `Future`. A `Future` is a value that may not be
// ready yet. It represents a value that will be computed at some point in the
// future.

// The `await` keyword is used to wait for a `Future` to complete. When you
// `await` a `Future`, you are yielding control back to the runtime, which can
// then run other tasks.

async fn hello_world() {
    println!("hello, world!");
}

// --- The Tokio Runtime ---

// `async` functions are lazy. They don't do anything until you run them on an
// executor. An executor is a library that is responsible for running `Future`s.

// Tokio is a popular runtime for writing asynchronous applications in Rust.

#[tokio::main]
async fn main() {
    // `hello_world` returns a `Future`. We can `.await` it to run it.
    hello_world().await;

    // --- A More Complex Example ---

    async fn download_file(url: &str) -> String {
        // In a real application, we would make a network request here.
        // For now, we will just simulate it with a delay.
        println!("Downloading {}...", url);
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        String::from("File contents")
    }

    async fn process_file(contents: &str) {
        println!("Processing file: {}", contents);
    }

    let file_contents = download_file("https://example.com").await;
    process_file(&file_contents).await;

    // --- Running Futures Concurrently ---

    // We can use `join!` to run multiple futures concurrently.

    let future1 = download_file("https://example.com/1");
    let future2 = download_file("https://example.com/2");

    let (contents1, contents2) = tokio::join!(future1, future2);

    println!("\n--- Concurrent Downloads ---");
    println!("File 1: {}", contents1);
    println!("File 2: {}", contents2);
}

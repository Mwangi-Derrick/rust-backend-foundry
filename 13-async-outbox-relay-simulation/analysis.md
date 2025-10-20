# Lesson 13: Sync Outbox Relay Simulation

## 🧠 Concept Summary
This lesson is a practical integration of everything learned so far. It builds a **synchronous, single-threaded simulation** of the core logic for an outbox relay service. This is a crucial step before adding the complexity of asynchronous code.

This simulation demonstrates:
-   **Domain Modeling with Enums**: Using an `enum` (`OutboxEvent`) to represent the different types of messages the service will handle.
-   **Business Logic with `Result`**: Implementing a processing function (`process_event`) that enforces business rules (e.g., validating payment amounts) and returns a `Result` to clearly separate success from failure.
-   **Advanced File I/O**: Using `std::fs::OpenOptions` to robustly open a file for appending, which is ideal for logging.
-   **Using External Crates**: Adding and using an external dependency (`chrono`) to get timestamps, a common requirement in real-world applications.

This lesson builds the logical core of the application, which can be developed and tested before being placed into a concurrent, asynchronous environment.

## 🧩 Code Walkthrough

```rust
// Import necessary modules from the standard library and the 'chrono' crate.
use std::fs::OpenOptions;
use std::io::{self, Write};
use chrono::Local;

// An enum to model the different kinds of events our service can process.
#[derive(Debug)]
enum OutboxEvent {
    Upload(String),
    Payment(f64),
    Notification(String),
}

// This function contains the core business logic for handling each event type.
// It takes a reference to an event and returns a Result.
fn process_event(event: &OutboxEvent) -> Result<String, String> {
    match event {
        OutboxEvent::Upload(file) => Ok(format!("📤 Relaying upload: {}", file)),
        OutboxEvent::Payment(amount) => {
            // Enforce a business rule: payments must be positive.
            if *amount <= 0.0 {
                Err("❌ Invalid payment amount".into())
            } else {
                Ok(format!("💳 Payment of ${} completed", amount))
            }
        }
        OutboxEvent::Notification(msg) => Ok(format!("🔔 Notification sent: {}", msg)),
    }
}

// This function handles writing log messages to a file.
fn log_to_file(entry: &str) -> io::Result<()> {
    // OpenOptions provides a builder for customizing how a file is opened.
    // Here, we create it if it doesn't exist and always append to the end.
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("relay_log.txt")?;

    // Get the current timestamp from the chrono crate.
    let timestamp = Local::now();
    // writeln! is a macro that writes a formatted string, followed by a newline, to a writer.
    writeln!(file, "[{}] {}", timestamp.format("%Y-%m-%d %H:%M:%S"), entry)?;
    Ok(())
}

fn main() {
    // A hardcoded vector of events to simulate a batch of work.
    let events = vec![
        OutboxEvent::Upload("video123.mp4".into()),
        OutboxEvent::Payment(49.99),
        OutboxEvent::Payment(0.0), // An event that will fail
        OutboxEvent::Notification("Summary ready!".into()),
    ];

    // Loop through each event and process it.
    for event in &events {
        // Match on the result of the business logic.
        match process_event(event) {
            Ok(msg) => {
                println!("{}", msg);
                // .unwrap() is used for simplicity. In a real app, you'd need a strategy
                // for what to do if logging fails (e.g., print to stderr and exit).
                log_to_file(&msg).unwrap();
            }
            Err(err) => {
                // eprintln! prints to the standard error stream.
                eprintln!("{}", err);
                log_to_file(&format!("Error: {}", err)).unwrap();
            }
        }
    }

    println!("✅ All events processed and logged!");
}
```

## ⚔️ Cross-Language Insights
-   **Golang Equivalent:**
    -   The overall structure is very similar to a simple Go program that iterates over a slice. The `process_event` function is analogous to a Go function returning `(string, error)`. The logging function is similar to using `os.OpenFile` with the `O_APPEND|O_CREATE` flags. The single-threaded, synchronous execution model is identical.
-   **TypeScript (Node.js) Equivalent:**
    -   This is like a standard Node.js script. You'd have an array of objects, a `for...of` loop, a function that processes each object (possibly throwing an error), and a `try/catch` block inside the loop to handle failures. Logging would use `fs.appendFileSync`. The synchronous nature makes it a direct comparison.
-   **C Reference:**
    -   This would be significantly more complex in C. You would need a `struct` with a `union` and `enum` tag for the event type, manual file management with `fopen("relay_log.txt", "a")`, and manual checking of return codes for all I/O operations. The Rust version is far more concise, readable, and safer, especially regarding resource management (file handles) and memory.

## 🚀 Practical Reflection
This synchronous prototype is an invaluable development step. It allows you to perfect the core data structures and business logic of your application in a simple, easy-to-test environment. You can write unit tests for `process_event` without needing a complex `async` runtime.

The major limitation, however, is that it's synchronous and single-threaded. If `process_event` or `log_to_file` involved a slow operation (like a real network call), the entire loop would block, and no other events could be processed. This code processes events **sequentially**. The next step is to take this proven logic and execute it **concurrently** using an async runtime like Tokio, which is the focus of the final lesson.

## 🧩 Self-Review Prompts
-   What are the limitations of this synchronous design for a high-throughput microservice?
-   The `log_to_file` call uses `.unwrap()`. What could cause logging to fail, and what would be a more robust way to handle that failure?
-   How would you restructure this code to fetch events from a (simulated) database function instead of a hardcoded `Vec`?
-   This code processes one event at a time. How would you modify it to use `std::thread` to process multiple events in parallel? What challenges would you face? (This provides a good contrast to the `async` approach).

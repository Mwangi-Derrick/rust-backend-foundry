# Lesson 07.5: Real Example: Async Outbox Relay

## 🧠 Concept Summary

This lesson is a practical example that combines what we have learned about `async` Rust to create a simple simulation of an **outbox relay**. This is an `async` version of the example from Module 6, and it demonstrates how to use `async/await` to write concurrent I/O-bound code.

- **Async Outbox Pattern:** We are implementing the same outbox pattern as before, but this time we are using `async` I/O operations to make it non-blocking. This means that the writer and the processor can run concurrently without blocking each other.

- **Tokio for I/O and Concurrency:** We use Tokio for asynchronous file I/O (`tokio::fs`, `tokio::io`) and for running our writer and processor tasks concurrently (`tokio::spawn`).

## 🧩 Code Walkthrough

Let's analyze the code in `main.rs`.

### Async `Outbox` and `EventProcessor`

```rust
struct Outbox {
    // ...
}

impl Outbox {
    async fn write_event(&self, event: &Event) -> io::Result<()> {
        let mut file = OpenOptions::new()
            // ...
            .open(&self.file_path)
            .await?;

        file.write_all(event.to_string().as_bytes()).await?;
        // ...
    }
}

struct EventProcessor {
    // ...
}

impl EventProcessor {
    async fn process_events(&self) -> Result<()> {
        let file = File::open(&self.file_path).await?;
        let reader = BufReader::new(file);
        let mut lines = reader.lines();

        while let Some(line) = lines.next_line().await? {
            // ...
        }

        fs::remove_file(&self.file_path).await?;

        Ok(())
    }
}
```

The `write_event` and `process_events` methods are now `async` functions. They use `.await` to perform non-blocking file I/O operations.

### Concurrent Writer and Processor

```rust
#[tokio::main]
async fn main() -> Result<()> {
    let outbox_file = "async_outbox.txt";

    // --- Writer Task ---
    let outbox = Outbox::new(outbox_file);
    let writer_task = tokio::spawn(async move {
        // ...
    });

    // --- Processor Task ---
    let processor = EventProcessor::new(outbox_file);
    let processor_task = tokio::spawn(async move {
        // ...
    });

    // Wait for both tasks to complete
    writer_task.await?;
    processor_task.await?;

    Ok(())
}
```

In the `main` function, we create two tasks: one for the writer and one for the processor. We use `tokio::spawn` to run these tasks concurrently. The `writer_task` writes events to the outbox file, and the `processor_task` reads and processes them. We then `.await` the handles of both tasks to wait for them to complete.

## ⚔️ Cross-Language Insights

- This example is a good demonstration of the power of `async/await` for writing concurrent I/O-bound code. You could implement a similar pattern in other languages with `async/await` support, like TypeScript or C#.

## 🚀 Practical Reflection

- **Concurrency without Complexity:** `async/await` allows you to write concurrent code that looks very similar to synchronous code. This makes it much easier to reason about than traditional callback-based or thread-based concurrency models.

- **The `move` Keyword in `async` Blocks:** The `async move` block in `tokio::spawn` is important. It moves the `outbox` and `processor` variables into the `async` block, which is necessary because the tasks might outlive the `main` function.

- **Real-world Applications:** This outbox relay pattern is a simplified version of what you might find in a real-world microservices application. In a real application, the outbox would likely be a database table, and the events would be sent to a message broker like RabbitMQ or Kafka.

## 🧩 Self-Review Prompts

- How could you make this example more robust? What happens if the processor crashes before it has processed all the events?
- Modify the `EventProcessor` to run in a loop, periodically checking for new events in the outbox file.
- How would you use channels to communicate between the writer and the processor?

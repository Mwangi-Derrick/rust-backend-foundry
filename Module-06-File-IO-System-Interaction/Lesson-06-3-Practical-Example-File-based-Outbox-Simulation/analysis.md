# Lesson 06.3: Practical Example: File-based Outbox Simulation

## 🧠 Concept Summary

This lesson is a practical example that combines everything we have learned in this module about file I/O and error handling. We will simulate the **outbox pattern**, a common pattern in microservices architecture for reliable messaging.

- **Outbox Pattern:** In this pattern, a service that wants to send an event doesn't send it directly to a message broker. Instead, it writes the event to an "outbox" table in its own database. A separate process then reads from the outbox table and sends the events to the message broker. This ensures that the event is eventually sent, even if the message broker is temporarily unavailable.

- **Our Simulation:** In our simulation, the "outbox" will be a file. We will have an `Outbox` service that writes events to the file, and an `EventProcessor` service that reads events from the file and "processes" them (in our case, just prints them to the console).

## 🧩 Code Walkthrough

Let's analyze the code in `main.rs`.

### The `Event` Struct

```rust
#[derive(Debug)]
struct Event {
    id: u64,
    payload: String,
}

impl Event {
    // ...
    fn to_string(&self) -> String { ... }
    fn from_string(s: &str) -> Result<Self, &'static str> { ... }
}
```

We define an `Event` struct to represent the events that we want to process. We also provide methods for serializing an `Event` to a string and deserializing it from a string.

### The `Outbox` Struct

```rust
struct Outbox {
    file_path: String,
}

impl Outbox {
    // ...
    fn write_event(&self, event: &Event) -> io::Result<()> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.file_path)?;

        writeln!(file, "{}", event.to_string())
    }
}
```

The `Outbox` struct is responsible for writing events to the outbox file. The `write_event` method opens the file in append mode (so that it doesn't overwrite existing events) and writes the event to the file.

### The `EventProcessor` Struct

```rust
struct EventProcessor {
    file_path: String,
}

impl EventProcessor {
    // ...
    fn process_events(&self) -> io::Result<()> {
        let file = File::open(&self.file_path)?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            // ...
        }

        fs::remove_file(&self.file_path)?;

        Ok(())
    }
}
```

The `EventProcessor` struct is responsible for reading events from the outbox file and processing them. The `process_events` method reads the file line by line, deserializes each line into an `Event`, and then "processes" it. After processing all the events, it deletes the file.

## ⚔️ Cross-Language Insights

- This is a general architectural pattern that can be implemented in any language. The implementation details would be different, but the overall concept is the same.

## 🚀 Practical Reflection

- **Reliability:** The outbox pattern is a great way to build reliable systems. By persisting events before sending them, you can ensure that they are not lost if there is a temporary failure.

- **Separation of Concerns:** This example also demonstrates the principle of separation of concerns. The `Outbox` is only responsible for writing events, and the `EventProcessor` is only responsible for reading and processing them. This makes the code easier to understand and maintain.

- **Error Handling:** This example shows how to handle different kinds of errors. The `write_event` and `process_events` methods can return I/O errors, and the `Event::from_string` method can return a parsing error. We use the `?` operator and `match` to handle these errors in a clean and robust way.

## 🧩 Self-Review Prompts

- How could you make the `EventProcessor` more robust? What happens if it crashes in the middle of processing the events?
- Modify the `EventProcessor` to move the processed file to an archive directory instead of deleting it.
- How would you implement this pattern using a database instead of a file?

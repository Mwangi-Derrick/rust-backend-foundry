# Lesson 09.4: Real-world Modular Outbox Bridge Layout

## 🧠 Concept Summary

This lesson is a practical example of how to structure a real-world Rust application using the concepts we have learned in this module. We will lay out the structure for our **outbox bridge** project using a multi-crate workspace.

- **Workspace Structure:** We will use a workspace to manage our project. The workspace will have three crates:
    - `outbox_bridge`: The main binary crate that runs the application.
    - `outbox_core`: A library crate that contains the core logic of the outbox bridge, such as the `Event` struct and the `Outbox` and `EventProcessor` traits.
    - `outbox_db`: A library crate that provides a database implementation of the `Outbox` trait.

- **Separation of Concerns:** This structure separates the application into three distinct parts: the main application logic, the core business logic, and the database logic. This makes the code easier to understand, test, and maintain.

## 🧩 Code Walkthrough

The code for this lesson is conceptual and is meant to be run by creating a workspace with the described structure.

### The `outbox_core` Crate

This crate is the heart of our application. It defines the core data structures and traits that are used by the other crates.

```rust
// outbox_core/src/lib.rs

// ...

pub struct Event { ... }

#[async_trait::async_trait]
pub trait Outbox: Send + Sync { ... }

#[async_trait::async_trait]
pub trait EventProcessor: Send + Sync { ... }
```

By defining the traits in a separate crate, we can have multiple implementations of them. For example, we could have a `DbOutbox` and a `FileOutbox`.

### The `outbox_db` Crate

This crate provides a database implementation of the `Outbox` trait.

```rust
// outbox_db/src/lib.rs

use outbox_core::{Event, Outbox};

pub struct DbOutbox;

#[async_trait::async_trait]
impl Outbox for DbOutbox {
    // ...
}
```

This crate depends on `outbox_core` and provides a concrete implementation of the `Outbox` trait.

### The `outbox_bridge` Crate

This is the main binary crate that runs the application.

```rust
// outbox_bridge/src/main.rs

use outbox_core::{Event, Outbox};
use outbox_db::DbOutbox;

#[tokio::main]
async fn main() -> Result<()> {
    let outbox = DbOutbox;
    // ...
}
```

This crate depends on both `outbox_core` and `outbox_db`. It creates an instance of `DbOutbox` and uses it to write an event.

## ⚔️ Cross-Language Insights

- This kind of modular structure is common in many languages. For example, in Java you might have a multi-module Maven project, and in C# you might have a solution with multiple projects.

## 🚀 Practical Reflection

- **Testability:** This structure makes the code much easier to test. We can write unit tests for the `outbox_core` and `outbox_db` crates in isolation. We can also write integration tests in the `outbox_bridge` crate that test the interaction between the different crates.

- **Flexibility:** This structure is very flexible. If we want to switch from a database outbox to a file-based outbox, we can just create a new `outbox_file` crate that implements the `Outbox` trait and then change one line of code in `outbox_bridge` to use the new implementation.

- **The `async_trait` Crate:** The `async_trait` crate is a third-party crate that allows you to use `async` functions in traits. This is a feature that is not yet built into the Rust language, but it is planned for a future version.

## 🧩 Self-Review Prompts

- Create the workspace and the three crates described in this lesson.
- Add a `FileOutbox` implementation of the `Outbox` trait in a new `outbox_file` crate.
- How would you add configuration to this application (e.g., the path to the database)?

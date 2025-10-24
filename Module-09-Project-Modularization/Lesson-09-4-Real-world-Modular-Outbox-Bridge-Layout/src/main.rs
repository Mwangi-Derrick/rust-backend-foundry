// Lesson 09.4: Real-world Modular Outbox Bridge Layout

// This lesson is a practical example of how to structure a real-world Rust
// application using the concepts we have learned in this module. We will lay
// out the structure for our outbox bridge project.

// --- The Project Structure ---

// We will use a workspace to manage our project. The workspace will have three
// crates:
//
// - `outbox_bridge`: The main binary crate that runs the application.
// - `outbox_core`: A library crate that contains the core logic of the outbox
//   bridge, such as the `Event` struct and the `Outbox` and `EventProcessor`
//   traits.
// - `outbox_db`: A library crate that provides a database implementation of the
//   `Outbox` trait.

// --- The `outbox_core` Crate ---

// This crate will define the core traits and types that are used throughout the
// application.

// ```rust
// // outbox_core/src/lib.rs
//
// use anyhow::Result;
//
// #[derive(Debug, Clone)]
// pub struct Event {
//     pub id: u64,
//     pub payload: String,
// }
//
// #[async_trait::async_trait]
// pub trait Outbox: Send + Sync {
//     async fn write_event(&self, event: &Event) -> Result<()>;
// }
//
// #[async_trait::async_trait]
// pub trait EventProcessor: Send + Sync {
//     async fn process_events(&self) -> Result<()>;
// }
// ```

// --- The `outbox_db` Crate ---

// This crate will provide a database implementation of the `Outbox` trait. For
// now, we will just have a dummy implementation.

// ```rust
// // outbox_db/src/lib.rs
//
// use anyhow::Result;
// use outbox_core::{Event, Outbox};
//
// pub struct DbOutbox;
//
// #[async_trait::async_trait]
// impl Outbox for DbOutbox {
//     async fn write_event(&self, event: &Event) -> Result<()> {
//         println!("Writing event to the database: {:?}", event);
//         Ok(())
//     }
// }
// ```

// --- The `outbox_bridge` Crate ---

// This is the main binary crate that runs the application. It will use the
// `outbox_core` and `outbox_db` crates.

// ```rust
// // outbox_bridge/src/main.rs
//
// use anyhow::Result;
// use outbox_core::{Event, Outbox};
// use outbox_db::DbOutbox;
//
// #[tokio::main]
// async fn main() -> Result<()> {
//     let outbox = DbOutbox;
//     let event = Event { id: 1, payload: "hello".to_string() };
//     outbox.write_event(&event).await?;
//     Ok(())
// }
// ```

fn main() {
    println!("This lesson is about laying out a real-world modular project.");
    println!("The code for this lesson is conceptual and is meant to be run");
    println!("by creating a workspace with the described structure.");
}

# Lesson 14.2: Outbox Store (tokio::fs or sqlx)

## 🧠 Concept Summary

This lesson focuses on implementing the **Outbox Store** component of our outbox bridge. The Outbox Store is responsible for reliably persisting events before they are sent to an external message broker. We explore a concrete file-based implementation and a conceptual database-backed approach.

- **`OutboxStore` Trait:** We define a trait that abstracts the storage mechanism. This allows us to swap out different implementations (e.g., file, database) without changing the core logic of the message relayer.

- **File-based Implementation (`FileOutboxStore`):** A simple implementation using `tokio::fs` for asynchronous file I/O. Events are stored as lines in a text file, with a simple format (`id|payload|processed`). This is suitable for demonstration but not for production.

- **Database-backed Implementation (Conceptual `SqlxOutboxStore`):** In a real-world scenario, a database (like PostgreSQL) is the preferred choice for an outbox. We outline how `sqlx` (an asynchronous, compile-time checked ORM) would be used to interact with a database table.

## 🧩 Code Walkthrough

Let's analyze the code in `main.rs`.

### `Event` Struct and `OutboxStore` Trait

```rust
#[derive(Debug, Clone)]
pub struct Event {
    pub id: String,
    pub payload: String,
    pub processed: bool,
}

#[async_trait]
pub trait OutboxStore: Send + Sync {
    async fn save_event(&self, event: Event) -> Result<()>;
    async fn get_unprocessed_events(&self) -> Result<Vec<Event>>;
    async fn mark_event_processed(&self, event_id: &str) -> Result<()>;
}
```

We define the `Event` struct, which now includes an `id` (for unique identification), `payload`, and a `processed` flag. The `OutboxStore` trait defines the core operations: saving an event, retrieving unprocessed events, and marking an event as processed. The `#[async_trait]` macro is used because these methods are `async`.

### `FileOutboxStore` Implementation

```rust
pub struct FileOutboxStore {
    file_path: String,
}

// ... impl methods ...

#[async_trait]
impl OutboxStore for FileOutboxStore {
    async fn save_event(&self, event: Event) -> Result<()> { ... }
    async fn get_unprocessed_events(&self) -> Result<Vec<Event>> { ... }
    async fn mark_event_processed(&self, event_id: &str) -> Result<()> { ... }
}
```

This implementation uses `tokio::fs` for file operations. Events are serialized to a simple string format (`id|payload|processed`) when written and parsed back when read. The `read_all_events` and `write_all_events` helper methods handle the file I/O. Note that for `mark_event_processed`, we read all events, update the relevant one in memory, and then write all events back to the file. This is inefficient for large files but demonstrates the concept.

### Conceptual `SqlxOutboxStore`

```rust
// pub struct SqlxOutboxStore {
//     pool: sqlx::PgPool,
// }
//
// #[async_trait]
// impl OutboxStore for SqlxOutboxStore {
//     async fn save_event(&self, event: Event) -> Result<()> { ... }
//     async fn get_unprocessed_events(&self) -> Result<Vec<Event>> { ... }
//     async fn mark_event_processed(&self, event_id: &str) -> Result<()> { ... }
// }
```

This commented-out section shows how a database-backed implementation would look using `sqlx`. It would interact with a PostgreSQL database to store, retrieve, and update events. `sqlx::query!` and `sqlx::query_as!` provide compile-time checked SQL queries, which is a powerful feature of `sqlx`.

## ⚔️ Cross-Language Insights

- **Database as Outbox:** The concept of using a database table as an outbox is common across many languages and frameworks (e.g., Java with Spring, Go with GORM/SQLX, Python with SQLAlchemy).

- **ORM/Query Builders:** `sqlx` in Rust is similar to ORMs or query builders in other languages (e.g., GORM in Go, SQLAlchemy in Python, TypeORM in TypeScript) that provide an abstraction layer over database interactions.

## 🚀 Practical Reflection

- **Persistence Guarantees:** Using a database for the outbox store provides strong persistence guarantees. Events are durable even if the application crashes.

- **Atomicity:** Writing an event to the outbox and committing the business transaction in the same database transaction ensures atomicity. This is a key aspect of the outbox pattern's reliability.

- **Abstraction with Traits:** The `OutboxStore` trait is a great example of how Rust's traits enable flexible and testable architectures. We can easily swap between different storage implementations.

## 🧩 Self-Review Prompts

- Implement a simple `InMemoryOutboxStore` that stores events in a `Vec<Event>` protected by an `Arc<Mutex<...>>`.
- What are the advantages and disadvantages of the file-based outbox store compared to a database-backed one?
- Research `sqlx` and try to set up a local PostgreSQL database to implement the `SqlxOutboxStore`.

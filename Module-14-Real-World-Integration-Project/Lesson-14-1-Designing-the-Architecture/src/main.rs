// Lesson 14.1: Designing the Architecture

// This lesson focuses on designing the architecture for our real-world outbox
// bridge project. We will outline the components and their interactions.

// --- Project Goal ---

// The goal is to build a robust and scalable outbox bridge that reliably
// transfers events from a source (e.g., a database) to a destination (e.g., a
// message broker).

// --- Architectural Principles ---

// 1.  **Modularity:** Break down the system into small, independent components.
// 2.  **Decoupling:** Components should have minimal dependencies on each other.
// 3.  **Reliability:** Ensure events are processed exactly once (or at least once
//     with idempotency).
// 4.  **Scalability:** Design for horizontal scaling.
// 5.  **Observability:** Provide metrics, logging, and tracing.

// --- Core Components ---

// 1.  **Event Model:** Defines the structure of the events being processed.
// 2.  **Outbox Store:** Responsible for persisting events before they are sent.
//     This could be a database table, a file, or a dedicated queue.
// 3.  **Message Relayer:** Reads events from the Outbox Store and sends them to
//     the Message Broker.
// 4.  **Message Broker:** The external system (e.g., Kafka, RabbitMQ, NATS) that
//     receives the events.
// 5.  **Configuration:** Manages application settings.
// 6.  **Metrics/Logging:** For monitoring and debugging.

// --- High-Level Flow ---

// 1.  An application writes an event to the **Outbox Store**.
// 2.  The **Message Relayer** periodically queries the Outbox Store for new events.
// 3.  For each new event, the Message Relayer attempts to send it to the
//     **Message Broker**.
// 4.  If successful, the Message Relayer marks the event as sent in the
//     Outbox Store.
// 5.  If unsuccessful, it retries (with backoff) or moves the event to a dead-letter queue.

// --- Rust Implementation Strategy ---

// - **Workspace:** Use a Cargo workspace to organize the project into multiple
//   crates (e.g., `outbox_core`, `outbox_db`, `outbox_broker`, `outbox_bridge`).
// - **Traits:** Define traits for `OutboxStore` and `MessageRelayer` to allow
//   for different implementations (e.g., `PostgresOutboxStore`, `KafkaRelayer`).
// - **Async/Await:** Use Tokio for all asynchronous operations.
// - **Error Handling:** Use `anyhow` for application-level errors and `thiserror`
//   for library-level errors.
// - **Configuration:** Use a crate like `config` or `clap`.
// - **Logging:** Use `tracing`.

fn main() {
    println!("This lesson focuses on the architectural design of the outbox bridge.");
    println!("There is no executable code to run for this lesson, as it's a design phase.");
}

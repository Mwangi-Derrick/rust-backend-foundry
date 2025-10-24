// Lesson 14.2: Outbox Store (tokio::fs or sqlx)

// This lesson focuses on implementing the `Outbox Store` component of our
// outbox bridge. We will explore two potential implementations: a simple
// file-based store using `tokio::fs` and a conceptual database-backed store
// using `sqlx`.

// --- The `OutboxStore` Trait ---

// First, let's define the trait that our outbox store implementations will
// adhere to. This trait will be part of our `outbox_core` crate.

use anyhow::Result;
use async_trait::async_trait;

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

// --- File-based Outbox Store Implementation ---

// This is a simple implementation for demonstration purposes. In a real
// application, you would likely use a more robust storage solution.

use tokio::fs::{self, OpenOptions};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

pub struct FileOutboxStore {
    file_path: String,
}

impl FileOutboxStore {
    pub fn new(file_path: &str) -> Self {
        FileOutboxStore { file_path: file_path.to_string() }
    }

    async fn read_all_events(&self) -> Result<Vec<Event>> {
        let mut events = Vec::new();
        if !fs::metadata(&self.file_path).await.is_ok() {
            return Ok(events); // File doesn't exist yet
        }

        let file = fs::File::open(&self.file_path).await?;
        let reader = BufReader::new(file);
        let mut lines = reader.lines();

        while let Some(line) = lines.next_line().await? {
            let parts: Vec<&str> = line.splitn(3, '|').collect();
            if parts.len() == 3 {
                events.push(Event {
                    id: parts[0].to_string(),
                    payload: parts[1].to_string(),
                    processed: parts[2].parse().unwrap_or(false),
                });
            }
        }
        Ok(events)
    }

    async fn write_all_events(&self, events: &[Event]) -> Result<()> {
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(&self.file_path)
            .await?;

        for event in events {
            file.write_all(format!("{}|{}|{}
", event.id, event.payload, event.processed).as_bytes()).await?;
        }
        Ok(())
    }
}

#[async_trait]
impl OutboxStore for FileOutboxStore {
    async fn save_event(&self, event: Event) -> Result<()> {
        let mut events = self.read_all_events().await?;
        events.push(event);
        self.write_all_events(&events).await?;
        Ok(())
    }

    async fn get_unprocessed_events(&self) -> Result<Vec<Event>> {
        let events = self.read_all_events().await?;
        Ok(events.into_iter().filter(|e| !e.processed).collect())
    }

    async fn mark_event_processed(&self, event_id: &str) -> Result<()> {
        let mut events = self.read_all_events().await?;
        for event in &mut events {
            if event.id == event_id {
                event.processed = true;
                break;
            }
        }
        self.write_all_events(&events).await?;
        Ok(())
    }
}

// --- Conceptual Database-backed Outbox Store (using sqlx) ---

// In a real application, you would use a database like PostgreSQL.
// The `sqlx` crate provides an asynchronous, compile-time checked ORM.

// pub struct SqlxOutboxStore {
//     pool: sqlx::PgPool,
// }
//
// impl SqlxOutboxStore {
//     pub fn new(pool: sqlx::PgPool) -> Self {
//         SqlxOutboxStore { pool }
//     }
// }
//
// #[async_trait]
// impl OutboxStore for SqlxOutboxStore {
//     async fn save_event(&self, event: Event) -> Result<()> {
//         sqlx::query!(
//             "INSERT INTO outbox (id, payload, processed) VALUES ($1, $2, $3)",
//             event.id,
//             event.payload,
//             event.processed
//         )
//         .execute(&self.pool)
//         .await?;
//         Ok(())
//     }
//
//     async fn get_unprocessed_events(&self) -> Result<Vec<Event>> {
//         let records = sqlx::query_as!(Event,
//             "SELECT id, payload, processed FROM outbox WHERE processed = FALSE"
//         )
//         .fetch_all(&self.pool)
//         .await?;
//         Ok(records)
//     }
//
//     async fn mark_event_processed(&self, event_id: &str) -> Result<()> {
//         sqlx::query!(
//             "UPDATE outbox SET processed = TRUE WHERE id = $1",
//             event_id
//         )
//         .execute(&self.pool)
//         .await?;
//         Ok(())
//     }
// }

#[tokio::main]
async fn main() -> Result<()> {
    let file_store = FileOutboxStore::new("outbox_events.txt");

    // Clean up previous run
    let _ = fs::remove_file("outbox_events.txt").await;

    // Save some events
    file_store.save_event(Event::new("1", "UserCreated")).await?;
    file_store.save_event(Event::new("2", "OrderPlaced")).await?;
    file_store.save_event(Event::new("3", "ProductUpdated")).await?;

    println!("Saved events.");

    // Get unprocessed events
    let unprocessed = file_store.get_unprocessed_events().await?;
    println!("Unprocessed events: {:?}", unprocessed);

    // Mark an event as processed
    file_store.mark_event_processed("2").await?;

    println!("Marked event 2 as processed.");

    // Get unprocessed events again
    let unprocessed_after_mark = file_store.get_unprocessed_events().await?;
    println!("Unprocessed events after marking: {:?}", unprocessed_after_mark);

    Ok(())
}

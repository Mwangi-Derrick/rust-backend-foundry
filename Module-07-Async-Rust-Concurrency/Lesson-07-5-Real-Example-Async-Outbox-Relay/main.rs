// Lesson 07.5: Real Example: Async Outbox Relay

// This lesson is a practical example that combines what we have learned about
// async Rust to create a simple simulation of an outbox relay. This is an async
// version of the example from Module 6.

use anyhow::Result;
use tokio::fs::{self, File, OpenOptions};
use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::time::{self, Duration};

// --- The Event ---

#[derive(Debug, Clone)]
struct Event {
    id: u64,
    payload: String,
}

impl Event {
    fn new(id: u64, payload: &str) -> Self {
        Event { id, payload: payload.to_string() }
    }

    fn to_string(&self) -> String {
        format!("{}:வுகளை", self.id, self.payload)
    }

    fn from_string(s: &str) -> Result<Self> {
        let mut parts = s.splitn(2, ':');
        let id_str = parts.next().ok_or_else(|| anyhow::anyhow!("Missing id"))?;
        let payload = parts.next().ok_or_else(|| anyhow::anyhow!("Missing payload"))?;

        let id = id_str.parse()?;

        Ok(Event { id, payload: payload.to_string() })
    }
}

// --- The Outbox ---

struct Outbox {
    file_path: String,
}

impl Outbox {
    fn new(file_path: &str) -> Self {
        Outbox { file_path: file_path.to_string() }
    }

    async fn write_event(&self, event: &Event) -> io::Result<()> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.file_path)
            .await?;

        file.write_all(event.to_string().as_bytes()).await?;
        file.write_all(b"\n").await?;
        Ok(())
    }
}

// --- The Event Processor ---

struct EventProcessor {
    file_path: String,
}

impl EventProcessor {
    fn new(file_path: &str) -> Self {
        EventProcessor { file_path: file_path.to_string() }
    }

    async fn process_events(&self) -> Result<()> {
        let file = File::open(&self.file_path).await?;
        let reader = BufReader::new(file);
        let mut lines = reader.lines();

        while let Some(line) = lines.next_line().await? {
            match Event::from_string(&line) {
                Ok(event) => {
                    println!("Processing event: {:?}", event);
                    // Simulate some work
                    time::sleep(Duration::from_millis(100)).await;
                }
                Err(e) => {
                    eprintln!("Error parsing event: {}", e);
                }
            }
        }

        fs::remove_file(&self.file_path).await?;

        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let outbox_file = "async_outbox.txt";

    // --- Writer Task ---
    let outbox = Outbox::new(outbox_file);
    let writer_task = tokio::spawn(async move {
        for i in 1..=10 {
            let event = Event::new(i, &format!("Event {}", i));
            if let Err(e) = outbox.write_event(&event).await {
                eprintln!("Error writing event: {}", e);
            }
            time::sleep(Duration::from_millis(50)).await;
        }
    });

    // --- Processor Task ---
    let processor = EventProcessor::new(outbox_file);
    let processor_task = tokio::spawn(async move {
        // Wait for the writer to finish
        time::sleep(Duration::from_secs(1)).await;
        if let Err(e) = processor.process_events().await {
            eprintln!("Error processing events: {}", e);
        }
    });

    // Wait for both tasks to complete
    writer_task.await?;
    processor_task.await?;

    Ok(())
}

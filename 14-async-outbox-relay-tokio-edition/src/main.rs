use std::fs::OpenOptions;
use std::io::{self, Write};
use chrono::Local;
use tokio::time::{sleep, Duration};

#[derive(Debug, Clone)]
enum OutboxEvent {
    Upload(String),
    Payment(f64),
    Notification(String),
}

async fn process_event(event: OutboxEvent) -> Result<String, String> {
    match event {
        OutboxEvent::Upload(file) => {
            sleep(Duration::from_millis(500)).await;
            Ok(format!("📤 Relayed upload: {}", file))
        }
        OutboxEvent::Payment(amount) => {
            sleep(Duration::from_millis(800)).await;
            if amount <= 0.0 {
                Err("❌ Invalid payment amount".into())
            } else {
                Ok(format!("💳 Payment of ${:.2} processed", amount))
            }
        }
        OutboxEvent::Notification(msg) => {
            sleep(Duration::from_millis(300)).await;
            Ok(format!("🔔 Notification delivered: {}", msg))
        }
    }
}

fn log_to_file(entry: &str) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("async_log.txt")?;
    let timestamp = Local::now();
    writeln!(file, "[{}] {}", timestamp.format("%Y-%m-%d %H:%M:%S"), entry)?;
    Ok(())
}

#[tokio::main]
async fn main() {
    let events = vec![
        OutboxEvent::Upload("video987.mp4".into()),
        OutboxEvent::Payment(75.5),
        OutboxEvent::Notification("Summary Ready".into()),
        OutboxEvent::Payment(0.0),
    ];

    println!("🚀 Starting async outbox relay...\n");

    // Process events concurrently
    let handles = events.into_iter().map(|event| {
        tokio::spawn(async move {
            match process_event(event.clone()).await {
                Ok(msg) => {
                    println!("{}", msg);
                    log_to_file(&msg).unwrap();
                }
                Err(err) => {
                    eprintln!("{}", err);
                    log_to_file(&format!("Error: {}", err)).unwrap();
                }
            }
        })
    });

    for h in handles {
        h.await.unwrap();
    }

    println!("\n✅ All async events processed and logged!");
}


// Example output:

// 🚀 Starting async outbox relay...

// 📤 Relayed upload: video987.mp4
// 💳 Payment of $75.50 processed
// ❌ Invalid payment amount
// 🔔 Notification delivered: Summary Ready

// ✅ All async events processed and logged!


// And in async_log.txt:

// [2025-10-20 15:12:34] 📤 Relayed upload: video987.mp4
// [2025-10-20 15:12:35] 💳 Payment of $75.50 processed
// [2025-10-20 15:12:35] ❌ Invalid payment amount
// [2025-10-20 15:12:35] 🔔 Notification delivered: Summary Ready

// 🧠 You’ve Now Learned:

// ✅ Ownership, Structs & Enums
// ✅ Traits, Generics, and Pattern Matching
// ✅ Error Handling & the ? Operator
// ✅ File I/O
// ✅ Async Concurrency (Tokio)
// ✅ Simulating a Real-world Outbox Relay microservice 🎯
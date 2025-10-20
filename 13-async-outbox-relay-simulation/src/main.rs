use std::fs::OpenOptions;
use std::io::{self, Write};
use chrono::Local;

#[derive(Debug)]
enum OutboxEvent {
    Upload(String),
    Payment(f64),
    Notification(String),
}

fn process_event(event: &OutboxEvent) -> Result<String, String> {
    match event {
        OutboxEvent::Upload(file) => Ok(format!("📤 Relaying upload: {}", file)),
        OutboxEvent::Payment(amount) => {
            if *amount <= 0.0 {
                Err("❌ Invalid payment amount".into())
            } else {
                Ok(format!("💳 Payment of ${} completed", amount))
            }
        }
        OutboxEvent::Notification(msg) => Ok(format!("🔔 Notification sent: {}", msg)),
    }
}

fn log_to_file(entry: &str) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("relay_log.txt")?;

    let timestamp = Local::now();
    writeln!(file, "[{}] {}", timestamp.format("%Y-%m-%d %H:%M:%S"), entry)?;
    Ok(())
}

fn main() {
    let events = vec![
        OutboxEvent::Upload("video123.mp4".into()),
        OutboxEvent::Payment(49.99),
        OutboxEvent::Payment(0.0),
        OutboxEvent::Notification("Summary ready!".into()),
    ];

    for event in &events {
        match process_event(event) {
            Ok(msg) => {
                println!("{}", msg);
                log_to_file(&msg).unwrap();
            }
            Err(err) => {
                eprintln!("{}", err);
                log_to_file(&format!("Error: {}", err)).unwrap();
            }
        }
    }

    println!("✅ All events processed and logged!");
}


// Step 3 — Run It
// cargo run


// You’ll see console output like:

// 📤 Relaying upload: video123.mp4
// 💳 Payment of $49.99 completed
// ❌ Invalid payment amount
// 🔔 Notification sent: Summary ready!
// ✅ All events processed and logged!


// And your relay_log.txt will contain timestamped logs.
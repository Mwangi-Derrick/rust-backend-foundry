// 🧩 Step 1: Add Tokio (already in your Cargo.toml)

// We’ll use its built-in message passing (multi-producer, single-consumer).

// ⚙️ Step 2: Example — Simple Message Channel
use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    // Create a channel with buffer size 3
    let (tx, mut rx) = mpsc::channel(3);

    // Clone transmitter for multiple producers
    let tx1 = tx.clone();
    let tx2 = tx.clone();

    // Producer 1
    tokio::spawn(async move {
        tx1.send("📤 Upload completed").await.unwrap();
        sleep(Duration::from_secs(1)).await;
        tx1.send("📦 File synced to bucket").await.unwrap();
    });

    // Producer 2
    tokio::spawn(async move {
        tx2.send("💳 Payment processed").await.unwrap();
    });

    // Consumer
    while let Some(msg) = rx.recv().await {
        println!("📨 Received: {}", msg);
    }
}


// ✅ Try This:
// Run:

// cargo run


// You’ll see messages arriving asynchronously in the order they finish — not the order they started.
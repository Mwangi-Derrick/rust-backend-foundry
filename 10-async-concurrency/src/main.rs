use tokio::time::{sleep, Duration};

async fn upload_service() {
    println!("📤 Upload service started...");
    sleep(Duration::from_secs(2)).await;
    println!("✅ Upload completed");
}

async fn payment_service() {
    println!("💳 Payment service started...");
    sleep(Duration::from_secs(3)).await;
    println!("✅ Payment processed");
}

#[tokio::main]
async fn main() {
    println!("🚀 Starting async tasks...");

    let upload = tokio::spawn(upload_service());
    let payment = tokio::spawn(payment_service());

    // Wait for both tasks to complete
    let _ = tokio::join!(upload, payment);

    println!("🎉 All async tasks completed!");
}
// 🧩 Breakdown
// #[tokio::main] → turns main() into an async runtime.

// tokio::spawn() → spawns concurrent async tasks.

// tokio::join!() → waits for multiple tasks to finish.

// sleep() → simulates async work (like API calls, DB writes, etc.)

// ✅ Try This
// Run:

// bash
// Copy code
// cargo run
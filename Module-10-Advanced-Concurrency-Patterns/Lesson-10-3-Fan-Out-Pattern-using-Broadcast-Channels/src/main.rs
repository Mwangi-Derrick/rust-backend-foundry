// Lesson 10.3: Fan-Out Pattern using Broadcast Channels

// This lesson demonstrates the fan-out pattern, where a single message is sent
// to multiple consumers. We will use a broadcast channel to implement this
// pattern.

use tokio::sync::broadcast;
use tokio::time::{self, Duration};

// --- The Message ---

#[derive(Debug, Clone)]
struct Message {
    content: String,
}

// --- The Consumer ---

// The consumer is a task that receives messages from a broadcast channel and
// processes them.

async fn consumer(id: u32, mut rx: broadcast::Receiver<Message>) {
    while let Ok(msg) = rx.recv().await {
        println!("Consumer {} received message: {:?}", id, msg);
    }
}

// --- The Producer ---

// The producer is a task that sends messages to a broadcast channel.

async fn producer(tx: broadcast::Sender<Message>) {
    for i in 0..5 {
        let msg = Message { content: format!("Message {}", i) };
        if tx.send(msg).is_err() {
            break;
        }
        time::sleep(Duration::from_millis(500)).await;
    }
}

#[tokio::main]
async fn main() {
    let (tx, _) = broadcast::channel(16);

    let mut handles = vec![];

    // Create multiple consumers
    for i in 0..3 {
        let rx = tx.subscribe();
        handles.push(tokio::spawn(consumer(i, rx)));
    }

    // Create a producer
    handles.push(tokio::spawn(producer(tx)));

    // Wait for all tasks to complete
    for handle in handles {
        handle.await.unwrap();
    }
}

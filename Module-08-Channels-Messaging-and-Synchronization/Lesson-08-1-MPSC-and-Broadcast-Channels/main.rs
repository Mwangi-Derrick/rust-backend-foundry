// Lesson 08.1: MPSC and Broadcast Channels

// This lesson introduces channels, which are a way for tasks to communicate
// with each other. We will look at two kinds of channels provided by Tokio:
// MPSC (multi-producer, single-consumer) and broadcast.

// --- MPSC Channels ---

// An MPSC channel is a channel where there can be multiple producers (senders)
// but only one consumer (receiver).

use tokio::sync::mpsc;

async fn mpsc_example() {
    // `mpsc::channel` creates a new MPSC channel. It returns a sender and a
    // receiver. The `100` is the capacity of the channel.
    let (tx, mut rx) = mpsc::channel(100);

    // We can clone the sender to have multiple producers.
    let tx2 = tx.clone();

    // Spawn a task for the first producer.
    tokio::spawn(async move {
        for i in 0..5 {
            if let Err(_) = tx.send(i).await {
                println!("Receiver dropped");
                return;
            }
        }
    });

    // Spawn a task for the second producer.
    tokio::spawn(async move {
        for i in 5..10 {
            if let Err(_) = tx2.send(i).await {
                println!("Receiver dropped");
                return;
            }
        }
    });

    // The receiver can receive messages from both producers.
    while let Some(i) = rx.recv().await {
        println!("Got = {}", i);
    }
}

// --- Broadcast Channels ---

// A broadcast channel is a channel where there can be multiple producers and
// multiple consumers. Each consumer will receive every message.

use tokio::sync::broadcast;

async fn broadcast_example() {
    let (tx, mut rx1) = broadcast::channel(16);
    let mut rx2 = tx.subscribe();

    tokio::spawn(async move {
        tx.send(10).unwrap();
        tx.send(20).unwrap();
    });

    // The first receiver will receive both messages.
    let val1 = rx1.recv().await.unwrap();
    println!("rx1 got = {}", val1);
    let val2 = rx1.recv().await.unwrap();
    println!("rx1 got = {}", val2);

    // The second receiver will also receive both messages.
    let val3 = rx2.recv().await.unwrap();
    println!("rx2 got = {}", val3);
    let val4 = rx2.recv().await.unwrap();
    println!("rx2 got = {}", val4);
}

#[tokio::main]
async fn main() {
    println!("--- MPSC Example ---");
    mpsc_example().await;

    println!("\n--- Broadcast Example ---");
    broadcast_example().await;
}

// Lesson 08.2: Select! and Graceful Shutdowns

// This lesson covers the `select!` macro, which is a powerful tool for waiting
// on multiple futures at the same time. We will also see how to use `select!`
// to implement graceful shutdown.

// --- The `select!` Macro ---

// The `select!` macro allows you to wait on multiple futures and returns when
// the first one completes. It is similar to the `select` statement in Go.

use tokio::sync::mpsc;
use tokio::time::{self, Duration};

async fn select_example() {
    let (tx, mut rx) = mpsc::channel(100);

    tokio::spawn(async move {
        for i in 0..10 {
            if tx.send(i).await.is_err() {
                break;
            }
            time::sleep(Duration::from_millis(100)).await;
        }
    });

    loop {
        tokio::select! {
            Some(i) = rx.recv() => {
                println!("Got = {}", i);
            }
            _ = time::sleep(Duration::from_secs(1)) => {
                println!("Timeout");
                break;
            }
        }
    }
}

// --- Graceful Shutdown ---

// `select!` is a key tool for implementing graceful shutdown. You can have one
// branch of the `select!` that listens for a shutdown signal, and another
// branch that runs the main server loop.

use tokio::sync::broadcast;

async fn graceful_shutdown_example() {
    let (shutdown_tx, mut shutdown_rx) = broadcast::channel(1);

    let mut handles = vec![];

    for i in 0..5 {
        let mut shutdown_rx_clone = shutdown_tx.subscribe();
        let handle = tokio::spawn(async move {
            loop {
                tokio::select! {
                    _ = time::sleep(Duration::from_millis(500)) => {
                        println!("Task {} is running", i);
                    }
                    _ = shutdown_rx_clone.recv() => {
                        println!("Task {} received shutdown signal", i);
                        break;
                    }
                }
            }
        });
        handles.push(handle);
    }

    // Wait for a Ctrl-C signal
    tokio::signal::ctrl_c().await.unwrap();

    println!("\nSending shutdown signal...");
    shutdown_tx.send(()).unwrap();

    // Wait for all tasks to complete
    for handle in handles {
        handle.await.unwrap();
    }

    println!("All tasks finished!");
}

#[tokio::main]
async fn main() {
    println!("--- Select Example ---");
    select_example().await;

    println!("\n--- Graceful Shutdown Example ---");
    graceful_shutdown_example().await;
}

// Lesson 10.5: Graceful Shutdowns with select!

// This lesson revisits graceful shutdowns, focusing on how to use `select!`
// effectively to manage the shutdown process in a more complex scenario.

use tokio::sync::broadcast;
use tokio::time::{self, Duration};

async fn worker_task(id: u32, mut shutdown_rx: broadcast::Receiver<()>) {
    println!("Worker {} started.", id);
    loop {
        tokio::select! {
            _ = time::sleep(Duration::from_millis(500)) => {
                println!("Worker {} doing work...", id);
            }
            _ = shutdown_rx.recv() => {
                println!("Worker {} received shutdown signal. Exiting.", id);
                break;
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let (shutdown_tx, _) = broadcast::channel(1);
    let mut worker_handles = vec![];

    // Spawn multiple worker tasks
    for i in 0..3 {
        let shutdown_rx = shutdown_tx.subscribe();
        worker_handles.push(tokio::spawn(worker_task(i, shutdown_rx)));
    }

    println!("Main: Workers spawned. Press Ctrl-C to initiate shutdown.");

    // Wait for a Ctrl-C signal
    tokio::signal::ctrl_c().await.unwrap();

    println!("Main: Ctrl-C received. Sending shutdown signal to workers.");

    // Send shutdown signal to all workers
    // `send` returns an error if there are no active receivers, which is fine here.
    let _ = shutdown_tx.send(());

    // Wait for all worker tasks to complete their shutdown
    for handle in worker_handles {
        handle.await.unwrap();
    }

    println!("Main: All workers shut down gracefully. Exiting.");
}

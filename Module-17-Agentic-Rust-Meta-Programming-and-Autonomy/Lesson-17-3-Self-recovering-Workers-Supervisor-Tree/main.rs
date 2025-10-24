// Lesson 17.3: Self-recovering Workers (Supervisor Tree)

// This lesson delves into building more resilient systems by implementing
// self-recovering workers, often based on the **supervisor tree** pattern.
// This pattern allows parts of a system to fail independently without
// bringing down the entire application.

// --- Why Self-recovering Workers? ---

// In complex applications, individual tasks or workers can encounter unexpected
// errors or panics. Instead of propagating these failures and crashing the whole
// application, a supervisor can detect worker failures and attempt to restart
// them, improving the overall fault tolerance and uptime.

// --- The Supervisor Tree Pattern ---

// Inspired by Erlang/Elixir's OTP, the supervisor tree is a hierarchical system
// where supervisor processes (or tasks in our async Rust context) monitor
// worker processes.
// - If a worker fails, its supervisor takes action (e.g., restarts the worker).
// - If a supervisor fails, its parent supervisor takes action.
// This creates a resilient fault-tolerant structure.

use tokio::sync::mpsc;
use tokio::time::{self, Duration};
use anyhow::Result;

// --- Worker Messages ---

#[derive(Debug)]
enum WorkerMessage {
    StartWork(u32),
    Stop,
}

// --- Worker Task (can fail) ---

// This worker can deliberately panic to simulate unexpected failures.
async fn worker_job(id: u32, mut rx: mpsc::Receiver<WorkerMessage>) -> Result<()> {
    println!("Worker {} started.", id);
    let mut work_factor = 0;
    loop {
        tokio::select! {
            Some(msg) = rx.recv() => {
                match msg {
                    WorkerMessage::StartWork(factor) => {
                        work_factor = factor;
                        println!("Worker {} starting work with factor {}.", id, factor);
                        if factor % 3 == 0 {
                            eprintln!("Worker {} is purposefully panicking with factor {}.", id, factor);
                            panic!("Simulated worker panic!");
                        }
                        time::sleep(Duration::from_millis(100 * work_factor as u64)).await;
                        println!("Worker {} finished work with factor {}.", id, factor);
                    }
                    WorkerMessage::Stop => {
                        println!("Worker {} received stop signal. Exiting.", id);
                        break;
                    }
                }
            }
            _ = time::sleep(Duration::from_secs(1)) => {
                // Keep worker alive if no messages
            }
        }
    }
    Ok(())
}

// --- Supervisor Task ---

// The supervisor monitors its children workers and restarts them if they fail.
async fn supervisor(worker_id: u32, mut main_tx: mpsc::Sender<WorkerMessage>) {
    println!("Supervisor for Worker {} started.", worker_id);
    loop {
        let (worker_tx, worker_rx) = mpsc::channel(1);
        let handle = tokio::spawn(worker_job(worker_id, worker_rx));

        // Forward messages from main to the worker
        let main_tx_clone = main_tx.send(WorkerMessage::StartWork(worker_id)).await.unwrap();

        // Wait for the worker to finish or panic
        if let Err(e) = handle.await {
            eprintln!("Supervisor: Worker {} failed: {:?}. Restarting...", worker_id, e);
            // In a real system, you might implement backoff or retry limits.
            time::sleep(Duration::from_secs(1)).await; // Delay before restarting
        } else {
            // Worker exited gracefully (e.g., after receiving a Stop message)
            println!("Supervisor: Worker {} exited gracefully.", worker_id);
            break; // Supervisor can exit if worker exited gracefully
        }
    }
    println!("Supervisor for Worker {} stopped.", worker_id);
}

#[tokio::main]
async fn main() -> Result<()> {
    // This is a simplified example. In a real supervisor tree, the main task
    // would be a top-level supervisor for multiple supervisors.

    let (main_tx, mut main_rx) = mpsc::channel(10);

    for i in 0..3 {
        let tx_clone = main_tx.clone();
        tokio::spawn(supervisor(i, tx_clone));
    }

    // Send some work messages to the workers.
    main_tx.send(WorkerMessage::StartWork(1)).await?;
    main_tx.send(WorkerMessage::StartWork(2)).await?;
    main_tx.send(WorkerMessage::StartWork(3)).await?;
    main_tx.send(WorkerMessage::StartWork(4)).await?;
    main_tx.send(WorkerMessage::StartWork(5)).await?;

    // Wait for some time to see restarts
    time::sleep(Duration::from_secs(10)).await;

    // Stop all workers (conceptual - in a real system, supervisor would manage this)
    // The current main_tx is being cloned in the supervisor, so it's not a direct channel to workers.
    // This example focuses on worker self-recovery, not main-initiated graceful shutdown of supervisors.

    println!("Main application finished.");

    Ok(())
}

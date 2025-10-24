// Lesson 14.6: Graceful Shutdown & Cleanup

// This lesson focuses on implementing a comprehensive graceful shutdown strategy
// for our outbox bridge, ensuring all resources are properly released and
// ongoing operations are completed before the application exits.

// --- Why Graceful Shutdown? ---

// Abrupt termination of an application can lead to:
// - Data corruption or inconsistency.
// - Resource leaks (e.g., open files, database connections).
// - Incomplete operations, leading to lost data or incorrect state.
// - Poor user experience (if it's a user-facing service).

// Graceful shutdown ensures that the application can:
// 1. Stop accepting new work.
// 2. Finish processing existing work.
// 3. Release all acquired resources.
// 4. Exit cleanly.

// --- Components of Graceful Shutdown ---

// 1.  **Signal Handling:** Detecting external signals (e.g., Ctrl-C, SIGTERM).
// 2.  **Shutdown Signal Propagation:** Notifying all active tasks/components to shut down.
// 3.  **Task Coordination:** Waiting for tasks to complete their work and exit.
// 4.  **Resource Cleanup:** Closing database connections, file handles, etc.

use tokio::sync::broadcast;
use tokio::time::{self, Duration};
use anyhow::Result;

// --- Worker Task (Simulated) ---

async fn worker_task(id: u32, mut shutdown_rx: broadcast::Receiver<()>) {
    println!("Worker {} started.", id);
    let mut work_count = 0;
    loop {
        tokio::select! {
            _ = time::sleep(Duration::from_millis(200)) => {
                work_count += 1;
                println!("Worker {} doing work ({}).", id, work_count);
                // Simulate some resource usage that needs cleanup
                if work_count % 5 == 0 {
                    println!("Worker {} acquired a resource.", id);
                }
            }
            _ = shutdown_rx.recv() => {
                println!("Worker {} received shutdown signal. Finishing current work.", id);
                // Simulate finishing current work
                time::sleep(Duration::from_millis(500)).await;
                println!("Worker {} finished current work. Releasing resources.", id);
                // In a real app, release database connections, file handles, etc.
                break;
            }
        }
    }
    println!("Worker {} gracefully shut down.", id);
}

// --- Main Application Loop (Simulated) ---

async fn app_main_loop(mut shutdown_rx: broadcast::Receiver<()>) {
    println!("Application main loop started.");
    loop {
        tokio::select! {
            _ = time::sleep(Duration::from_secs(1)) => {
                println!("App: Running background tasks...");
            }
            _ = shutdown_rx.recv() => {
                println!("App: Received shutdown signal. Stopping new work.");
                break;
            }
        }
    }
    println!("Application main loop stopped.");
}

#[tokio::main]
async fn main() -> Result<()> {
    let (shutdown_tx, _) = broadcast::channel(1);
    let mut worker_handles = vec![];

    // Spawn worker tasks
    for i in 0..3 {
        let shutdown_rx = shutdown_tx.subscribe();
        worker_handles.push(tokio::spawn(worker_task(i, shutdown_rx)));
    }

    // Spawn the main application loop task
    let app_handle = tokio::spawn(app_main_loop(shutdown_tx.subscribe()));

    println!("Main: Application and workers started. Press Ctrl-C to initiate shutdown.");

    // Wait for a Ctrl-C signal
    tokio::signal::ctrl_c().await?;

    println!("Main: Ctrl-C received. Sending shutdown signal to all components.");

    // Send shutdown signal to all components
    // `send` returns an error if there are no active receivers, which is fine here.
    let _ = shutdown_tx.send(());

    // Wait for all worker tasks to complete their shutdown
    for handle in worker_handles {
        handle.await?;
    }

    // Wait for the main application loop to stop
    app_handle.await?;

    println!("Main: All components shut down gracefully. Exiting.");

    Ok(())
}

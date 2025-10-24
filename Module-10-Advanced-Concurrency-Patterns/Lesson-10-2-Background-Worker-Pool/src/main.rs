// Lesson 10.2: Background Worker Pool (Tokio Tasks)

// This lesson demonstrates how to create a background worker pool using Tokio
// tasks. A worker pool is a common pattern for processing a large number of
// jobs concurrently.

use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};
use tokio::time::{self, Duration};

// --- The Job ---

// First, let's define the job that we want to process.

#[derive(Debug)]
struct Job {
    id: u32,
}

// --- The Worker ---

// The worker is a task that receives jobs from a channel and processes them.
// We use an `Arc<Mutex<...>>` to allow multiple workers to share the same
// receiver.

async fn worker(id: u32, rx: Arc<Mutex<mpsc::Receiver<Job>>>) {
    loop {
        let job = {
            let mut lock = rx.lock().await;
            lock.recv().await
        };

        if let Some(job) = job {
            println!("Worker {} processing job {:?}", id, job);
            // Simulate some work
            time::sleep(Duration::from_millis(500)).await;
        } else {
            // The channel has been closed, so we can exit.
            println!("Worker {} shutting down.", id);
            break;
        }
    }
}

// --- The Worker Pool ---

// The worker pool is responsible for creating the workers and providing a way
// to send jobs to them.

struct WorkerPool {
    sender: mpsc::Sender<Job>,
}

impl WorkerPool {
    fn new(num_workers: u32) -> Self {
        let (tx, rx) = mpsc::channel(100);
        let rx = Arc::new(Mutex::new(rx));

        for i in 0..num_workers {
            let rx_clone = Arc::clone(&rx);
            tokio::spawn(worker(i, rx_clone));
        }

        WorkerPool { sender: tx }
    }

    async fn send_job(&self, job: Job) {
        self.sender.send(job).await.unwrap();
    }
}

#[tokio::main]
async fn main() {
    let pool = WorkerPool::new(4);

    for i in 0..10 {
        pool.send_job(Job { id: i }).await;
    }

    // To gracefully shut down the workers, we can drop the sender. This will
    // cause the `recv` calls in the workers to return `None`.
    drop(pool.sender);

    // Wait for a bit to allow the workers to finish processing their current jobs.
    time::sleep(Duration::from_secs(3)).await;
}
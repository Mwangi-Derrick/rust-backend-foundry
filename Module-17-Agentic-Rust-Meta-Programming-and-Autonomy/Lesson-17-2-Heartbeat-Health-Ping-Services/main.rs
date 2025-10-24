// Lesson 17.2: Heartbeat & Health-Ping Services

// This lesson focuses on implementing heartbeat and health-ping mechanisms,
// crucial for monitoring the liveness and readiness of services in a distributed
// system.

// --- Why Heartbeats and Health Pings? ---

// In a distributed system, services can fail for various reasons (crashes,
// network issues, resource exhaustion). Heartbeats and health pings allow other
// services or monitoring systems to:
// - **Detect Liveness:** Determine if a service is still running.
// - **Detect Readiness:** Determine if a service is ready to accept requests.
// - **Enable Self-Healing:** Allow orchestrators (like Kubernetes) to restart
//   unhealthy services.

// --- Heartbeat Mechanism ---

// A heartbeat is typically a periodic message sent by a service to a central
// monitoring system, indicating that it's alive and well.

use tokio::sync::mpsc;
use tokio::time::{self, Duration};
use anyhow::Result;

// --- Agent Message (extended) ---

#[derive(Debug, Clone)]
enum AgentMessage {
    PerformTask(String),
    Heartbeat,
    Shutdown,
}

// --- Worker Agent (modified to send heartbeats) ---

pub struct WorkerAgent {
    id: u32,
    state: String,
    heartbeat_sender: mpsc::Sender<u32>,
}

impl WorkerAgent {
    pub fn new(id: u32, heartbeat_sender: mpsc::Sender<u32>) -> Self {
        WorkerAgent { id, state: format!("Worker {} idle", id), heartbeat_sender }
    }

    async fn handle_message(&mut self, message: AgentMessage) -> Result<()> {
        match message {
            AgentMessage::PerformTask(task) => {
                self.state = format!("Worker {} processing: {}", self.id, task);
                println!("{} received: {}", self.state, task);
                time::sleep(Duration::from_millis(500)).await;
                self.state = format!("Worker {} idle", self.id);
                println!("Worker {} finished task.", self.id);
            }
            AgentMessage::Heartbeat => {
                println!("Worker {} sending heartbeat.", self.id);
                self.heartbeat_sender.send(self.id).await?;
            }
            AgentMessage::Shutdown => {
                println!("Worker {} shutting down.", self.id);
            }
        }
        Ok(())
    }

    async fn run(mut self, mut receiver: mpsc::Receiver<AgentMessage>) {
        println!("Worker {} started.", self.id);
        let mut heartbeat_interval = time::interval(Duration::from_secs(1));
        loop {
            tokio::select! {
                _ = heartbeat_interval.tick() => {
                    if let Err(e) = self.handle_message(AgentMessage::Heartbeat).await {
                        eprintln!("Worker {} heartbeat error: {:?}", self.id, e);
                    }
                }
                Some(message) = receiver.recv() => {
                    if let AgentMessage::Shutdown = message {
                        let _ = self.handle_message(message).await;
                        break;
                    }
                    if let Err(e) = self.handle_message(message).await {
                        eprintln!("Worker {} error handling message: {:?}", self.id, e);
                    }
                }
            }
        }
        println!("Worker {} stopped.", self.id);
    }
}

// --- Health Monitor Service ---

// A service that receives heartbeats and monitors the health of workers.

async fn health_monitor(mut heartbeat_receiver: mpsc::Receiver<u32>) {
    let mut last_heartbeat = std::collections::HashMap::new();
    let mut monitor_interval = time::interval(Duration::from_secs(3));

    println!("Health monitor started.");

    loop {
        tokio::select! {
            _ = monitor_interval.tick() => {
                println!("Monitor: Checking worker health...");
                let now = time::Instant::now();
                for (worker_id, last_time) in last_heartbeat.iter() {
                    if now.duration_since(*last_time) > Duration::from_secs(2) {
                        warn!("Monitor: Worker {} is unhealthy (last heartbeat {:?}).", worker_id, last_time);
                    }
                }
            }
            Some(worker_id) = heartbeat_receiver.recv() => {
                println!("Monitor: Received heartbeat from Worker {}.", worker_id);
                last_heartbeat.insert(worker_id, time::Instant::now());
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let (heartbeat_tx, heartbeat_rx) = mpsc::channel(32);

    let worker1 = WorkerAgent::new(1, heartbeat_tx.clone());
    let worker2 = WorkerAgent::new(2, heartbeat_tx.clone());

    let worker1_handle = tokio::spawn(worker1.run(mpsc::channel(32).1)); // Dummy receiver
    let worker2_handle = tokio::spawn(worker2.run(mpsc::channel(32).1)); // Dummy receiver

    let monitor_handle = tokio::spawn(health_monitor(heartbeat_rx));

    println!("Main: Agents and monitor started.");

    // Simulate some runtime
    time::sleep(Duration::from_secs(10)).await;

    // In a real app, you'd send shutdown signals here.

    println!("Main: Shutting down.");

    // For this example, we'll just let the program exit.
    // In a real app, you'd gracefully shut down workers and monitor.

    Ok(())
}

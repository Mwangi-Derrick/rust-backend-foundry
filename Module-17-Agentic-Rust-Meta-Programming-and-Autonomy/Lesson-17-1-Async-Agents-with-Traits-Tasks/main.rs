// Lesson 17.1: Async Agents with Traits + Tasks

// This lesson introduces the concept of "async agents" in Rust, which are
// autonomous, concurrent entities that perform specific tasks and communicate
// via message passing. We'll build a simple agent using traits and Tokio tasks.

// --- What is an Agent? ---

// In the context of agentic programming, an agent is an autonomous software
// entity that can perceive its environment, make decisions, and act upon them.
// Key characteristics:
// - **Autonomy:** Operates independently.
// - **Reactivity:** Responds to changes in its environment.
// - **Pro-activeness:** Initiates actions to achieve goals.
// - **Social ability:** Communicates with other agents.

// --- Async Agents in Rust ---

// We can model async agents in Rust using:
// - **Tokio Tasks:** For concurrent execution.
// - **Channels:** For message-based communication.
// - **Traits:** To define common agent behaviors and interfaces.

use anyhow::Result;
use async_trait::async_trait;
use tokio::sync::mpsc;
use tokio::time::{self, Duration};

// --- Agent Message ---

#[derive(Debug)]
enum AgentMessage {
    PerformTask(String),
    Shutdown,
}

// --- Agent Trait ---

#[async_trait]
pub trait Agent: Send + Sync {
    fn name(&self) -> &str;
    async fn handle_message(&mut self, message: AgentMessage) -> Result<()>;
    async fn run(mut self, mut receiver: mpsc::Receiver<AgentMessage>);
}

// --- Simple Worker Agent Implementation ---

pub struct WorkerAgent {
    id: u32,
    state: String,
}

impl WorkerAgent {
    pub fn new(id: u32) -> Self {
        WorkerAgent { id, state: format!("Worker {} idle", id) }
    }
}

#[async_trait]
impl Agent for WorkerAgent {
    fn name(&self) -> &str {
        "WorkerAgent"
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
            AgentMessage::Shutdown => {
                println!("Worker {} shutting down.", self.id);
            }
        }
        Ok(())
    }

    async fn run(mut self, mut receiver: mpsc::Receiver<AgentMessage>) {
        println!("Worker {} started.", self.id);
        while let Some(message) = receiver.recv().await {
            if let AgentMessage::Shutdown = message {
                let _ = self.handle_message(message).await;
                break;
            }
            if let Err(e) = self.handle_message(message).await {
                eprintln!("Worker {} error handling message: {:?}", self.id, e);
            }
        }
        println!("Worker {} stopped.", self.id);
    }
}

// --- Agent Manager (for sending messages) ---

struct AgentManager {
    senders: Vec<mpsc::Sender<AgentMessage>>,
}

impl AgentManager {
    fn new() -> Self {
        AgentManager { senders: vec![] }
    }

    fn add_agent<A: Agent + 'static>(&mut self, agent: A) -> mpsc::Sender<AgentMessage> {
        let (sender, receiver) = mpsc::channel(32);
        tokio::spawn(agent.run(receiver));
        self.senders.push(sender.clone());
        sender
    }

    async fn broadcast_message(&self, message: AgentMessage) {
        for sender in &self.senders {
            if let Err(e) = sender.send(message.clone()).await {
                eprintln!("Failed to send message to agent: {:?}", e);
            }
        }
    }

    async fn shutdown_all(&self) {
        self.broadcast_message(AgentMessage::Shutdown).await;
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut manager = AgentManager::new();

    // Add agents
    let worker1_sender = manager.add_agent(WorkerAgent::new(1));
    let worker2_sender = manager.add_agent(WorkerAgent::new(2));

    // Send specific tasks
    worker1_sender.send(AgentMessage::PerformTask(String::from("Task A"))).await?;
    worker2_sender.send(AgentMessage::PerformTask(String::from("Task B"))).await?;

    // Broadcast a message
    manager.broadcast_message(AgentMessage::PerformTask(String::from("Broadcast Task"))).await;

    // Simulate some runtime
    time::sleep(Duration::from_secs(2)).await;

    // Initiate shutdown
    manager.shutdown_all().await;

    // Give agents time to shut down
    time::sleep(Duration::from_secs(1)).await;

    println!("Main: All agents managed.");

    Ok(())
}

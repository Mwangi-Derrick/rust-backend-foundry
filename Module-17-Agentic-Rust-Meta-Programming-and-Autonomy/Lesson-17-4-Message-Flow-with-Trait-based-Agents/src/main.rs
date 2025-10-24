// Lesson 17.4: Message Flow with Trait-based Agents

// This lesson builds upon our agent model by focusing on more sophisticated
// message flow patterns between trait-based agents. We'll explore how agents
// can send messages to specific other agents and handle replies.

// --- Agent-to-Agent Communication ---

// In a multi-agent system, agents often need to communicate directly with each
// other, not just broadcast messages. This requires agents to know the address
// (or sender channel) of other agents.

// --- Request-Response Pattern ---

// A common communication pattern is request-response, where one agent sends a
// request and expects a reply. This can be implemented using `oneshot` channels.

use anyhow::Result;
use async_trait::async_trait;
use tokio::sync::{mpsc, oneshot};
use tokio::time::{self, Duration};

// --- Agent Message (extended) ---

#[derive(Debug)]
enum AgentMessage {
    PerformTask(String),
    RequestValue { reply_to: oneshot::Sender<u32> },
    Shutdown,
}

// --- Agent Trait (extended) ---

#[async_trait]
pub trait Agent: Send + Sync + Sized + 'static {
    fn name(&self) -> &str;
    async fn handle_message(&mut self, message: AgentMessage) -> Result<()>;
    async fn run(mut self, mut receiver: mpsc::Receiver<AgentMessage>);
}

// --- Worker Agent (modified for request-response) ---

pub struct WorkerAgent {
    id: u32,
    state: String,
    value: u32,
}

impl WorkerAgent {
    pub fn new(id: u32) -> Self {
        WorkerAgent { id, state: format!("Worker {} idle", id), value: 0 }
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
                self.value += 1;
                self.state = format!("Worker {} idle", self.id);
                println!("Worker {} finished task. Value: {}", self.id, self.value);
            }
            AgentMessage::RequestValue { reply_to } => {
                println!("Worker {} received value request. Replying with {}.", self.id, self.value);
                reply_to.send(self.value).map_err(|_| anyhow::anyhow!("Failed to send reply"))?;
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

// --- Agent Manager (modified to get agent senders) ---

struct AgentManager {
    agent_senders: HashMap<u32, mpsc::Sender<AgentMessage>>,
}

impl AgentManager {
    fn new() -> Self {
        AgentManager { agent_senders: HashMap::new() }
    }

    fn add_agent<A: Agent + 'static>(&mut self, agent: A) -> mpsc::Sender<AgentMessage> {
        let (sender, receiver) = mpsc::channel(32);
        let agent_id = agent.id;
        tokio::spawn(agent.run(receiver));
        self.agent_senders.insert(agent_id, sender.clone());
        sender
    }

    async fn send_message_to_agent(&self, agent_id: u32, message: AgentMessage) -> Result<()> {
        if let Some(sender) = self.agent_senders.get(&agent_id) {
            sender.send(message).await?;
            Ok(())
        } else {
            Err(anyhow::anyhow!("Agent {} not found", agent_id))
        }
    }

    async fn request_value_from_agent(&self, agent_id: u32) -> Result<u32> {
        let (tx, rx) = oneshot::channel();
        self.send_message_to_agent(agent_id, AgentMessage::RequestValue { reply_to: tx }).await?;
        Ok(rx.await?)
    }

    async fn shutdown_all(&self) {
        for sender in self.agent_senders.values() {
            if let Err(e) = sender.send(AgentMessage::Shutdown).await {
                eprintln!("Failed to send shutdown to agent: {:?}", e);
            }
        }
    }
}

use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<()> {
    let mut manager = AgentManager::new();

    // Add agents
    manager.add_agent(WorkerAgent::new(1));
    manager.add_agent(WorkerAgent::new(2));

    // Send specific tasks
    manager.send_message_to_agent(1, AgentMessage::PerformTask(String::from("Task A"))).await?;
    manager.send_message_to_agent(2, AgentMessage::PerformTask(String::from("Task B"))).await?;

    // Request value from agent 1
    let value1 = manager.request_value_from_agent(1).await?;
    println!("Main: Agent 1's value: {}", value1);

    // Simulate some runtime
    time::sleep(Duration::from_secs(1)).await;

    // Request value again after more work
    manager.send_message_to_agent(1, AgentMessage::PerformTask(String::from("Task C"))).await?;
    let value1_after_c = manager.request_value_from_agent(1).await?;
    println!("Main: Agent 1's value after Task C: {}", value1_after_c);

    // Initiate shutdown
    manager.shutdown_all().await;

    // Give agents time to shut down
    time::sleep(Duration::from_secs(1)).await;

    println!("Main: All agents managed.");

    Ok(())
}

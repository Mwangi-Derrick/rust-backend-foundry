# Lesson 17.1: Async Agents with Traits + Tasks

## 🧠 Concept Summary

This lesson introduces the concept of **"async agents"** in Rust, which are autonomous, concurrent entities that perform specific tasks and communicate via message passing. We'll build a simple agent system using traits and Tokio tasks, laying the groundwork for more complex autonomous systems.

- **What is an Agent?** In agent-oriented programming, an agent is an autonomous software entity characterized by:
    - **Autonomy:** Operates independently, often with its own internal state.
    - **Reactivity:** Responds to changes or messages from its environment.
    - **Pro-activeness:** Initiates actions to achieve its goals.
    - **Social ability:** Communicates with other agents, typically via message passing.

- **Async Agents in Rust:** We can effectively model async agents in Rust by combining:
    - **Tokio Tasks:** For concurrent, non-blocking execution of each agent's logic.
    - **Channels (`tokio::sync::mpsc`):** For message-based communication between agents, serving as their "mailboxes."
    - **Traits (`async_trait`):** To define common agent behaviors and interfaces, allowing for polymorphism and extensibility.

## 🧩 Code Walkthrough

Let's analyze the code in `main.rs`.

### Agent Message

```rust
#[derive(Debug)]
enum AgentMessage {
    PerformTask(String),
    Shutdown,
}
```

This enum defines the types of messages our agents can understand. `PerformTask` carries a `String` payload, and `Shutdown` is a signal to terminate.

### Agent Trait

```rust
#[async_trait]
pub trait Agent: Send + Sync {
    fn name(&self) -> &str;
    async fn handle_message(&mut self, message: AgentMessage) -> Result<()>;
    async fn run(mut self, mut receiver: mpsc::Receiver<AgentMessage>);
}
```

The `Agent` trait defines the interface for any agent. It requires a `name`, an `async handle_message` method to process incoming messages, and an `async run` method that contains the agent's main event loop. `Send + Sync` bounds are necessary for `async_trait` and for spawning agents as Tokio tasks.

### `WorkerAgent` Implementation

```rust
pub struct WorkerAgent { /* ... */ }

#[async_trait]
impl Agent for WorkerAgent {
    // ...
    async fn handle_message(&mut self, message: AgentMessage) -> Result<()> { /* ... */ }

    async fn run(mut self, mut receiver: mpsc::Receiver<AgentMessage>) {
        println!("Worker {} started.", self.id);
        while let Some(message) = receiver.recv().await {
            if let AgentMessage::Shutdown = message {
                let _ = self.handle_message(message).await;
                break;
            }
            // ... handle other messages ...
        }
        println!("Worker {} stopped.", self.id);
    }
}
```

`WorkerAgent` is a concrete implementation of our `Agent` trait. Its `run` method contains the agent's event loop: it continuously waits for messages on its `mpsc::Receiver`. When a message arrives, it calls `handle_message`. A special check for `AgentMessage::Shutdown` allows the agent to gracefully exit its loop.

### `AgentManager` (Orchestrator)

```rust
struct AgentManager {
    senders: Vec<mpsc::Sender<AgentMessage>>,
}

impl AgentManager {
    // ...
    fn add_agent<A: Agent + 'static>(&mut self, agent: A) -> mpsc::Sender<AgentMessage> {
        let (sender, receiver) = mpsc::channel(32);
        tokio::spawn(agent.run(receiver));
        self.senders.push(sender.clone());
        sender
    }

    async fn broadcast_message(&self, message: AgentMessage) { /* ... */ }
    async fn shutdown_all(&self) { /* ... */ }
}
```

The `AgentManager` is responsible for creating and managing agents. The `add_agent` method creates a channel for the new agent, spawns the agent's `run` method as a Tokio task, and stores a sender to that agent's mailbox. It also provides methods to broadcast messages to all agents or initiate a shutdown.

## ⚔️ Cross-Language Insights

- **Golang:** The actor model can be implemented in Go using `goroutine`s for agents and channels for message passing. Go's `select` statement is often used to handle multiple incoming messages or events.

- **Erlang/Elixir:** These languages have first-class support for the actor model, where actors are lightweight processes that communicate via message passing. This is a core paradigm in their design.

- **Akka (Scala/Java):** Akka is a popular framework for building actor-based concurrent applications on the JVM.

## 🚀 Practical Reflection

- **Concurrency Model:** This agent-based approach provides a clear and robust concurrency model. Each agent manages its own state, and communication happens explicitly via messages, avoiding shared mutable state issues.

- **Scalability:** Agents can be easily scaled by spawning more tasks. The message-passing paradigm naturally supports distribution across multiple machines.

- **Fault Tolerance:** By isolating state within agents, a failure in one agent is less likely to bring down the entire system. Supervisor agents can be implemented to monitor and restart failed agents.

## 🧩 Self-Review Prompts

- Modify the `WorkerAgent` to maintain a count of processed tasks. Add a message type to query this count.
- Implement a simple "Router Agent" that receives messages and forwards them to specific `WorkerAgent`s based on some logic.
- How would you implement a mechanism for agents to register themselves with the `AgentManager` dynamically?

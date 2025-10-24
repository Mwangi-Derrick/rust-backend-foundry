# Lesson 08.3: Actor Model and Message Passing Simulation

## 🧠 Concept Summary

This lesson introduces the **actor model**, a popular concurrency model for building complex, stateful systems. We will simulate a simple actor system using tasks and channels.

- **The Actor Model:** A concurrency model where "actors" are the fundamental units of computation. An actor is a lightweight process that has its own state and communicates with other actors by sending and receiving messages.

- **Key Properties of Actors:**
    - They do not share state.
    - They communicate with each other by sending messages.
    - Each actor has a "mailbox" (a channel) where it receives messages.
    - An actor processes one message at a time.

- **Actors in Rust:** We can model an actor in Rust as a task that has its own state and a channel for receiving messages. The state is owned by the task and is not shared with any other task. This avoids the need for locks and other synchronization primitives.

## 🧩 Code Walkthrough

Let's analyze the code in `main.rs`.

### The Actor

```rust
enum Message {
    Increment,
    GetValue(tokio::sync::oneshot::Sender<u64>),
}

struct MyActor {
    receiver: mpsc::Receiver<Message>,
    value: u64,
}

impl MyActor {
    // ...
    async fn run(&mut self) {
        while let Some(msg) = self.receiver.recv().await {
            match msg {
                Message::Increment => {
                    self.value += 1;
                }
                Message::GetValue(sender) => {
                    sender.send(self.value).unwrap();
                }
            }
        }
    }
}
```

The `MyActor` struct holds the actor's state (`value`) and a receiver for its mailbox. The `run` method is the main loop of the actor. It receives messages from the mailbox and processes them one at a time.

### The Handle

```rust
struct MyActorHandle {
    sender: mpsc::Sender<Message>,
}

impl MyActorHandle {
    async fn increment(&self) {
        self.sender.send(Message::Increment).await.unwrap();
    }

    async fn get_value(&self) -> u64 {
        let (sender, receiver) = tokio::sync::oneshot::channel();
        self.sender.send(Message::GetValue(sender)).await.unwrap();
        receiver.await.unwrap()
    }
}
```

The `MyActorHandle` is a handle for communicating with the actor. It holds a sender for the actor's mailbox. The `increment` method sends an `Increment` message to the actor. The `get_value` method sends a `GetValue` message that includes a `oneshot` channel for the actor to send the value back on.

### Running the Actor

```rust
let (sender, receiver) = mpsc::channel(100);
let mut actor = MyActor::new(receiver);

let actor_task = tokio::spawn(async move {
    actor.run().await;
});

let handle = MyActorHandle { sender };

// ... use the handle to interact with the actor ...
```

In `main`, we create a channel for the actor's mailbox, create the actor, and then spawn a task to run the actor. We then create a handle and use it to interact with the actor.

## ⚔️ Cross-Language Insights

- **vs. Erlang/Elixir:** The actor model is the core concurrency model in Erlang and Elixir. These languages have first-class support for actors (called "processes").

- **vs. Akka (Scala/Java):** Akka is a popular actor framework for the JVM.

- **vs. Go:** Go's `goroutine`s and channels can be used to implement the actor model, but it is not as central to the language as it is in Erlang/Elixir.

## 🚀 Practical Reflection

- **State Management:** The actor model is a great way to manage state in a concurrent system. Because each actor has its own state and does not share it, you don't have to worry about race conditions and other synchronization issues.

- **Fault Tolerance:** The actor model can also be used to build fault-tolerant systems. If an actor crashes, you can have a "supervisor" actor that restarts it.

- **Frameworks:** While it's good to know how to implement an actor system from scratch, in practice you will often use a framework like `actix` or `bastion` to do it for you.

## 🧩 Self-Review Prompts

- Add a `Decrement` message to the `MyActor`.
- How would you gracefully shut down the `MyActor`?
- Look at the documentation for the `actix` crate. How does it compare to the simple actor system we have built here?

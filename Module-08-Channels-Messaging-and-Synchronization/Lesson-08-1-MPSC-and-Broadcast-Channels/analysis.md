# Lesson 08.1: MPSC and Broadcast Channels

## 🧠 Concept Summary

This lesson introduces **channels**, which are a way for tasks to communicate with each other by sending messages. We will look at two kinds of channels provided by Tokio: **MPSC** (multi-producer, single-consumer) and **broadcast**.

- **MPSC Channels:** An MPSC channel is a channel where there can be multiple producers (senders) but only one consumer (receiver). This is a common pattern for distributing work to a single worker task.

- **Broadcast Channels:** A broadcast channel is a channel where there can be multiple producers and multiple consumers. Each consumer will receive every message. This is useful for when you want to broadcast a message to multiple listeners.

- **Backpressure:** Channels have a fixed capacity. If the channel is full, the sender will have to wait until there is space in the channel. This is a form of backpressure that prevents the sender from overwhelming the receiver.

## 🧩 Code Walkthrough

Let's analyze the code in `main.rs`.

### MPSC Channels

```rust
use tokio::sync::mpsc;

let (tx, mut rx) = mpsc::channel(100);

let tx2 = tx.clone();

tokio::spawn(async move {
    // ... send messages with tx ...
});

tokio::spawn(async move {
    // ... send messages with tx2 ...
});

while let Some(i) = rx.recv().await {
    println!("Got = {}", i);
}
```

`mpsc::channel` creates a new MPSC channel with a capacity of 100. It returns a sender (`tx`) and a receiver (`rx`). We can clone the sender to have multiple producers. The receiver can then receive messages from all of the producers.

### Broadcast Channels

```rust
use tokio::sync::broadcast;

let (tx, mut rx1) = broadcast::channel(16);
let mut rx2 = tx.subscribe();

tokio::spawn(async move {
    tx.send(10).unwrap();
    tx.send(20).unwrap();
});

// ... receive messages on rx1 and rx2 ...
```

`broadcast::channel` creates a new broadcast channel. It returns a sender and an initial receiver. We can create new receivers by calling `tx.subscribe()`. When a message is sent on the channel, all of the receivers will receive it.

## ⚔️ Cross-Language Insights

- **vs. Golang:** Channels are a core feature of Go. Go's channels are similar to Rust's MPSC channels, but they can also be unbuffered. Go does not have a built-in broadcast channel, but you can implement one using a combination of channels and `select`.

- **vs. TypeScript:** TypeScript does not have built-in channels. You would typically use a library like `rxjs` to implement message passing between different parts of your application.

- **vs. C:** C does not have channels. You would have to implement them yourself using mutexes and condition variables.

## 🚀 Practical Reflection

- **Message Passing:** Channels are a key part of the message passing concurrency model, which is a popular alternative to the shared memory concurrency model. In the message passing model, tasks communicate by sending messages to each other, rather than by sharing memory. This can make it easier to write correct concurrent code.

- **Choosing the Right Channel:** The choice between an MPSC channel and a broadcast channel depends on your use case. If you have one worker task that is processing work from multiple sources, an MPSC channel is a good choice. If you have multiple listeners that all need to receive the same message, a broadcast channel is a good choice.

- **Other Channel Types:** Tokio provides other kinds of channels as well, such as `oneshot` channels (for sending a single value) and `watch` channels (for watching for changes to a value). We will cover these in later lessons.

## 🧩 Self-Review Prompts

- Write a program that has one producer and multiple consumers using an MPSC channel. What happens?
- What happens if a receiver on a broadcast channel is too slow? Does it block the sender?
- Look at the documentation for `tokio::sync::mpsc`. What is the difference between a bounded and an unbounded channel?

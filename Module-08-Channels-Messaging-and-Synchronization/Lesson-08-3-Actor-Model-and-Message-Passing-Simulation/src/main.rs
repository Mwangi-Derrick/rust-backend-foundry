// Lesson 08.3: Actor Model and Message Passing Simulation

// This lesson introduces the actor model, which is a popular concurrency model
// for building complex, stateful systems. We will simulate a simple actor
// system using tasks and channels.

// --- The Actor Model ---

// The actor model is a concurrency model where "actors" are the fundamental
// units of computation. An actor is a lightweight process that has its own
// state and communicates with other actors by sending and receiving messages.

// Key properties of actors:
// - They do not share state.
// - They communicate with each other by sending messages.
// - Each actor has a "mailbox" (a channel) where it receives messages.
// - An actor processes one message at a time.

// --- A Simple Actor ---

// We can model an actor in Rust as a task that has its own state and a channel
// for receiving messages.

use tokio::sync::mpsc;

// The messages that our actor can receive
enum Message {
    Increment,
    GetValue(tokio::sync::oneshot::Sender<u64>),
}

// The actor itself
struct MyActor {
    receiver: mpsc::Receiver<Message>,
    value: u64,
}

impl MyActor {
    fn new(receiver: mpsc::Receiver<Message>) -> Self {
        MyActor { receiver, value: 0 }
    }

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

// A handle for communicating with the actor
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

#[tokio::main]
async fn main() {
    let (sender, receiver) = mpsc::channel(100);
    let mut actor = MyActor::new(receiver);

    let actor_task = tokio::spawn(async move {
        actor.run().await;
    });

    let handle = MyActorHandle { sender };

    handle.increment().await;
    handle.increment().await;

    let value = handle.get_value().await;
    println!("The value is: {}", value);

    // The actor task will run forever. In a real application, you would want to
    // have a way to gracefully shut it down.
}

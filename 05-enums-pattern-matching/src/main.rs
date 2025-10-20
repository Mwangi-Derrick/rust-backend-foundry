

// 🪜 Next rung — Lesson 5: Enums + Pattern Matching (The Expressive Powerhouse)

// Enums are how Rust represents multiple possible states of a value — like Result, Option, or message types in your outbox service later.
// 🧩 Example 1 — Basic Enum

// enum MessageType {
//     Upload,
//     Payment,
//     Notification,
// }

// fn main() {
//     let msg = MessageType::Upload;

//     match msg {
//         MessageType::Upload => println!("Processing an upload message..."),
//         MessageType::Payment => println!("Handling a payment event..."),
//         MessageType::Notification => println!("Sending notification..."),
//     }
// }

// 🧠 Concept:

// enum = “one of many”
// match = super-powered switch that enforces exhaustiveness (no missing cases allowed!)

// 🧩 Example 2 — Enum with Data

// Now let’s make it realistic like your RabbitMQ messages:

// 🎯 Your Challenge:
// Add a new enum variant:

// Retry { attempt: u8, reason: String }


// Then handle it in the match statement.
enum OutboxEvent {
    Upload { file_id: String, user_id: String },
    Payment { amount: f64, status: String },
    Retry { attempt: u8, reason: String },
    Notification(String),
}

fn process_event(event: OutboxEvent) {
    match event {
        OutboxEvent::Upload { file_id, user_id } => {
            println!("Relaying upload {} from user {}", file_id, user_id);
        }
        OutboxEvent::Payment { amount, status } => {
            println!("Payment of ${} is {}", amount, status);
        }
        OutboxEvent::Notification(msg) => {
            println!("Notification: {}", msg);
        }
        OutboxEvent::Retry {attempt , reason} => {
            println!("Retry attempt: {} due to {}", attempt, reason)
        }
    }
}

fn main() {
    let e1 = OutboxEvent::Upload {
        file_id: "file123".to_string(),
        user_id: "user456".to_string(),
    };

    let e2 = OutboxEvent::Payment {
        amount: 49.99,
        status: "completed".to_string(),
    };

    let e3 = OutboxEvent::Retry {
        attempt: 3,
        reason: "failed database transaction".to_string(),
    };

    process_event(e1);
    process_event(e2);
    process_event(e3);
}
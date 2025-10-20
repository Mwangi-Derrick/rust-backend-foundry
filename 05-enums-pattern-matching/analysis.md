# Lesson 5: Enums & Pattern Matching

## 🧠 Concept Summary
This lesson introduces two of Rust's most expressive features:

-   **`enum` (Enumeration):** An `enum` is a type that can have one of several possible values, known as *variants*. Unlike enums in many other languages that are just named integers, Rust enums can also hold data. This makes them incredibly powerful for modeling a type that can be in one of several different states.

-   **`match`:** The `match` keyword lets you compare a value against a series of patterns and then execute code based on which pattern matches. It's like a `switch` statement on steroids. The Rust compiler enforces *exhaustiveness*, meaning you must handle every possible case, which eliminates a whole category of bugs.

Together, `enum` and `match` allow you to create rich, type-safe state machines and data models.

## 🧩 Code Walkthrough

### Example 1: Basic Enum (Commented Out)
This example shows a simple `enum` where the variants are just tags.
```rust
// enum MessageType {
//     Upload,
//     Payment,
//     Notification,
// }
//
// fn main() {
//     let msg = MessageType::Upload;
//
//     match msg {
//         MessageType::Upload => println!("Processing an upload message..."),
//         MessageType::Payment => println!("Handling a payment event..."),
//         MessageType::Notification => println!("Sending notification..."),
//     }
// }
```
-   `MessageType` is an `enum` with three possible variants: `Upload`, `Payment`, or `Notification`.
-   The `match` statement checks the value of `msg` and executes the code for the matching variant.
-   If you were to add a new variant to `MessageType` but forget to update the `match` block, the compiler would give you an error. This is exhaustiveness checking in action.

### Example 2: Enum with Data
This is where Rust's enums really shine. Each variant can carry different data.
```rust
enum OutboxEvent {
    // A variant that looks like a struct
    Upload { file_id: String, user_id: String },
    // Another struct-like variant
    Payment { amount: f64, status: String },
    // A third struct-like variant (from the challenge)
    Retry { attempt: u8, reason: String },
    // A variant that holds a single, unnamed String (like a tuple)
    Notification(String),
}

fn process_event(event: OutboxEvent) {
    match event {
        // If 'event' is an Upload variant, destructure it to get file_id and user_id.
        OutboxEvent::Upload { file_id, user_id } => {
            println!("Relaying upload {} from user {}", file_id, user_id);
        }
        // If it's a Payment, get the amount and status.
        OutboxEvent::Payment { amount, status } => {
            println!("Payment of ${} is {}", amount, status);
        }
        // If it's a Notification, bind the inner String to the variable 'msg'.
        OutboxEvent::Notification(msg) => {
            println!("Notification: {}", msg);
        }
        // Handle the new Retry variant from the challenge.
        OutboxEvent::Retry { attempt, reason } => {
            println!("Retry attempt: {} due to {}", attempt, reason)
        }
    }
}

fn main() {
    // Create instances of the different variants.
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
```
-   **Variants with Data:** `OutboxEvent` variants hold different payloads. `Upload` has two `String`s, `Payment` has an `f64` and a `String`, and `Notification` has a single `String`.
-   **Pattern Matching with Destructuring:** The `match` arms do more than just check the variant type; they also *destructure* the data inside the variant, binding it to new variables (`file_id`, `user_id`, `amount`, `status`, `msg`, etc.) that you can use in the arm's code block.

## ⚔️ Cross-Language Insights
-   **Golang Equivalent:**
    -   Go does not have Rust-style enums. The idiomatic way to achieve a similar result is much more verbose, typically involving an `interface` and multiple `struct` types that implement it. You would then use a type switch (`switch v := v.(type)`) to handle the different concrete types. This works, but it doesn't have the same level of compile-time safety and conciseness as Rust's `enum` and `match`.
-   **TypeScript Equivalent:**
    -   The closest equivalent in TypeScript is a **discriminated union**.
        ```typescript
        type OutboxEvent =
          | { type: 'Upload', fileId: string, userId: string }
          | { type: 'Payment', amount: number, status: string }
          | { type: 'Notification', message: string };

        function processEvent(event: OutboxEvent) {
            switch (event.type) {
                case 'Upload':
                    console.log(`Relaying upload ${event.fileId}...`);
                    break;
                // ... etc
            }
        }
        ```
    -   This is a very powerful pattern in TypeScript, and it's conceptually very similar to Rust's enums.
-   **C Reference:**
    -   The classic C approach is to use a `struct` containing a `union` and an `enum` tag field to specify which member of the `union` is currently active. This is powerful but notoriously unsafe. It's easy to write to one field of the union and read from another, leading to undefined behavior. Rust's `enum` is essentially a safe, compiler-verified implementation of this C pattern.

## 🚀 Practical Reflection
Enums are the *perfect* tool for modeling the different kinds of messages your outbox-relay service will handle. You can define an `enum` where each variant represents a different event type (`UserCreated`, `OrderPlaced`, `PaymentProcessed`). Each variant will hold exactly the data needed for that event. The core logic of your service will likely be a loop that fetches an event from the database, and then uses a `match` statement to determine how to process it. This creates code that is not only efficient but also highly readable and easy to maintain.

Two of the most important enums in the Rust standard library are:
-   `Option<T>`: Represents an optional value (`Some(T)` or `None`).
-   `Result<T, E>`: Represents a value that could be a success (`Ok(T)`) or an error (`Err(E)`).

Understanding enums is key to understanding idiomatic Rust, especially for error handling.

## 🧩 Self-Review Prompts
-   What happens if you remove one of the arms from the `match` statement in `process_event`? Why is this a good thing?
-   The `Option<T>` enum is defined as `enum Option<T> { Some(T), None }`. How would you write a `match` statement to get the value out of an `Option<i32>`?
-   What is the `if let` syntax, and when might you use it instead of a `match` statement?
-   How could you use an `enum` to represent the state of a network connection (e.g., `Connecting`, `Connected`, `Disconnected`, `Error`)?

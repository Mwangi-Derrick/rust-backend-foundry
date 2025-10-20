# Lesson 8: Custom Errors & Propagation

## 🧠 Concept Summary
While `Result<T, String>` is useful for simple cases, real-world applications need more structured error information. This lesson teaches you how to create your own custom error types using `enum`. This allows you to categorize different failure modes, carry specific error data, and handle different errors in different ways.

The key steps are:
1.  **Define a custom error `enum`**: This `enum` represents all the things that can go wrong in a particular part of your application.
2.  **Return `Result<T, YourCustomError>`**: Functions now return your specific error type, making failure states explicit and part of the function's signature.
3.  **Propagate with `?`**: The `?` operator works seamlessly with custom error types, allowing for clean and concise error propagation.
4.  **Handle specific errors with `match`**: The calling code can `match` on the specific variant of your error enum and decide what to do for each case (e.g., retry on a network error, abort on an invalid input error).

## 🧩 Code Walkthrough
The code is laid out in four clear steps, which we will follow.

```rust
// This attribute allows us to print the enum for debugging purposes (e.g., with println!("{:?}", e)).
#[derive(Debug)]
enum OutboxError {
    NetworkError, // A simple error variant
    DatabaseError(String), // An error variant that carries data
    InvalidInput(String), // Another variant with data
}

// This function simulates sending data over a network.
// It can fail with an InvalidInput or NetworkError.
fn send_to_network(data: &str) -> Result<(), OutboxError> {
    if data.is_empty() {
        Err(OutboxError::InvalidInput("Empty data".into()))
    } else if data == "fail_network" {
        Err(OutboxError::NetworkError)
    } else {
        println!("Data '{}' sent to network successfully!", data);
        Ok(())
    }
}

// This function simulates saving data to a database.
// It can fail with a DatabaseError.
fn save_to_db(data: &str) -> Result<(), OutboxError> {
    if data == "fail_db" {
        Err(OutboxError::DatabaseError("DB connection lost".into()))
    } else {
        println!("Data '{}' saved to DB successfully!", data);
        Ok(())
    }
}

// This function orchestrates the process, chaining the failable operations.
fn process_event(data: &str) -> Result<(), OutboxError> {
    // The '?' operator is used here. If send_to_network() returns an Err,
    // process_event will immediately return that same Err.
    send_to_network(data)?;

    // If send_to_network() succeeded, execution continues. If save_to_db() fails,
    // its Err is returned from process_event.
    save_to_db(data)?;

    // If both functions succeed, process_event returns Ok.
    Ok(())
}

fn main() {
    // The 'process_event' function is called. Change the input string
    // to "fail_db", "fail_network", or "" to test the different error paths.
    let result = process_event("fail_db");

    // The outer match handles the Result (Ok or Err).
    match result {
        Ok(_) => println!("Event processed successfully!"),
        // If there was an error, the inner match determines its specific type.
        Err(e) => match e {
            OutboxError::NetworkError => println!("Network issue! Retry later."),
            OutboxError::DatabaseError(msg) => println!("Database issue: {}", msg),
            OutboxError::InvalidInput(msg) => println!("Invalid input: {}", msg),
        },
    }
}
```

## ⚔️ Cross-Language Insights
-   **Golang Equivalent:**
    -   In Go, you can define custom error types by creating structs that satisfy the built-in `error` interface. You then use `errors.Is` or `errors.As` to check for specific error types. While effective, Rust's `enum` and `match` combination provides a more structured and exhaustive compile-time check that you have handled all possible error variants.
        ```go
        // Go equivalent pattern
        if err != nil {
            var dbErr *DatabaseError
            if errors.As(err, &dbErr) {
                // handle database error
            } else {
                // handle other errors
            }
        }
        ```
-   **TypeScript Equivalent:**
    -   The common pattern is to create custom error classes that extend the base `Error` class.
        ```typescript
        class DatabaseError extends Error { ... }
        // in a catch block:
        catch (e) {
            if (e instanceof DatabaseError) {
                // handle database error
            }
        }
        ```
    -   This relies on runtime checking with `instanceof`, whereas Rust's `match` is a compile-time construct.
-   **C Reference:**
    -   C has no direct support for this. A common pattern is to return integer error codes, where each integer maps to a specific error. The caller must then use a `switch` or `if/else` chain to check the code and act accordingly. This is not type-safe and relies on documentation and convention.

## 🚀 Practical Reflection
This pattern is **essential** for your outbox-relay microservice. Your service will have many potential points of failure:

1.  Could not connect to the database.
2.  Could not deserialize a message payload.
3.  Could not connect to the message broker (e.g., RabbitMQ/Kafka).
4.  The message broker rejected the message.

A custom `enum AppError` is the perfect way to model these distinct failure modes. Your main processing loop can then `match` on the error and implement business logic: a `DatabaseError` might be fatal and cause the service to shut down, while a `NetworkError` might trigger a retry mechanism with exponential backoff.

This level of granular, type-safe error handling is what makes it possible to build truly resilient and reliable systems in Rust.

## 🧩 Self-Review Prompts
-   For your error enum to be used with other libraries and for better ergonomics, it's good practice to implement the `std::error::Error` trait. How would you do that for `OutboxError`? (Hint: You'll also need to implement `std::fmt::Display`).
-   Libraries like `thiserror` and `anyhow` are extremely popular for reducing the boilerplate of error handling. Research `thiserror`. How would it simplify the definition of `OutboxError`?
-   The `?` operator can convert between error types if the `From` trait is implemented. How could you implement `From<DatabaseError> for AppError` to make composing different error types easier?
-   What is the difference between the `Debug` and `Display` traits? Why are both important for error types?

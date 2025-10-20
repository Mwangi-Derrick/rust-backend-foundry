// Perfect ⚙️ let’s move straight into Lesson 8 — Custom Errors & Propagation in Rust 🧠

// 🧩 Step 1: Define Your Own Error Type

// You can use an enum to represent multiple kinds of possible errors your app might encounter.

#[derive(Debug)]
enum OutboxError {
    NetworkError,
    DatabaseError(String),
    InvalidInput(String),
}


// This is like defining your own Result error type — Result<T, OutboxError>.

// ⚙️ Step 2: Functions That Return These Errors
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

fn save_to_db(data: &str) -> Result<(), OutboxError> {
    if data == "fail_db" {
        Err(OutboxError::DatabaseError("DB connection lost".into()))
    } else {
        println!("Data '{}' saved to DB successfully!", data);
        Ok(())
    }
}

🚀 Step 3: Propagate Errors Gracefully Using ?
fn process_event(data: &str) -> Result<(), OutboxError> {
    send_to_network(data)?; // if this fails, return error immediately
    save_to_db(data)?;      // otherwise, continue
    Ok(())
}

🧠 Step 4: Handle All Error Types in main()
fn main() {
    match process_event("fail_db") {
        Ok(_) => println!("Event processed successfully!"),
        Err(e) => match e {
            OutboxError::NetworkError => println!("Network issue! Retry later."),
            OutboxError::DatabaseError(msg) => println!("Database issue: {}", msg),
            OutboxError::InvalidInput(msg) => println!("Invalid input: {}", msg),
        },
    }
}

// 🧩 Key Concept Recap:

// enum OutboxError lets you categorize multiple failure modes.

// ? cleanly bubbles up any error.

// Result<T, OutboxError> gives strong compile-time guarantees.

// ✅ Try This:
// Run that example as a new Rust project (e.g., 08-custom-errors)
// Then replace "fail_db" with "fail_network" or "" to see different error paths.
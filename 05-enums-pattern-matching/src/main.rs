
// 🎯 Lesson 1: Variables, Mutability, and Types

// In Rust, variables are immutable by default — unlike Go or JS.

// 👉 Create a new playground:

// fn main() {
//     let x = 5;
//     println!("x = {}", x);

//     //Uncomment this and run — it will throw an error
//     //x = 6;^ cannot assign twice to immutable variable

//     let mut y = 10;
//     println!("y before = {}", y);
//     y = 15;
//     println!("y after = {}", y);

//     let s1 = String::from("hello");
//     // let s2 = s1; // ownership moves to s2

//     //  //println!("{}", s1); // ❌ error! s1 no longer valid ... let s2 = s1.clone(); // ownership moves to s2
//     // println!("{}", s2);

//      let s2 = &s1; // borrow, not move
//     println!("s1 = {}, s2 = {}", s1, s2);
// }

// 🧠 Concept:

// let = immutable

// let mut = mutable

// println!() is a macro, not a function — that’s why it has !

// 🧩 Lesson 2: Ownership — The Secret Sauce of Rust

// This is what makes Rust special. No GC (Garbage Collector) like Go or JS — Rust enforces memory safety through rules.

// Rule 1: Each value has a single owner
// Rule 2: When the owner goes out of scope → value is dropped (freed)
// Rule 3: You can borrow, but not own twice

// Try this:
// fn main() {
//     let mut name = String::from("summafy");
//     print_length(&name); // borrow here
//     println!("Back in main: {}", name);
// }

// fn print_length(s: &String) {
//     let len = s.len();
//     println!("The length of '{}' is {}", s, len);
// }

// “Notice the &name when calling the function? That’s a borrow.
// We’re saying: hey, print_length, you can look at this data, but don’t take ownership of it.
// This lets us use name again in main() — and Rust’s compiler guarantees we can’t accidentally mess with memory safety. No segfaults, no leaks, no GC needed.”


// 🎬 Lesson 3 — Mutable Borrowing & References

// In Lesson 2, we borrowed immutably — the function could read but not change the data.
// Now let’s learn to borrow mutably so a function can modify the value while keeping ownership safe.
// fn main() {
//     let mut name = String::from("summafy");
//     add_io(&mut name);
//     // let mut_ref1 = &mut name;
//     // let mut_ref2 = &mut name;
// println!("{} {}", mut_ref1, mut_ref2);
//     println!("After function call: {}", name);
// }

// fn add_io(s: &mut String) {
//     s.push_str(".io");
//     println!("Inside function: {}", s);
// }
// 💡 Instructor Commentary:
// “So what happened here?
// We passed &mut name — a mutable reference — into add_io().
// That means add_io can mutate the original string without taking ownership.
// Rust allows only one mutable reference at a time — no shared mut access!
// This rule prevents race conditions and data corruption at compile time 💥”

// 🧠 Mini Challenge

// Write a function that:

// Takes a mutable String

// Appends " - powered by Rust"

// Returns nothing, just modifies the input
// Then print the final string in main().

// fn main(){
//     let mut mutable = String::from("Summafy");
//      append(&mut mutable);
//     println!("concatenated string is {}", mutable);
// }

// fn append(string: &mut String){
// string.push_str("- powered by Rust");
// }

// Next rung 🪜 coming up:

// on 4 — Structs + Impl Blocks (BLessuilding Your Own Data Types)

// Structs are the blueprint for creating custom data types (like Go structs or JS objects).

// Let’s go step-by-step:

// 🧩 Example 1: Basic Struct


// struct Startup {
//     name: String,
//     category: String,
//     monthly_revenue: f64,
// }

// fn main() {
//     let summafy = Startup {
//         name: String::from("Summafy.io"),
//         category: String::from("AI & Productivity"),
//         monthly_revenue: 1200.50,
//     };

//     println!(
//         "{} is in {} category and earns ${} monthly",
//         summafy.name, summafy.category, summafy.monthly_revenue
//     );
// }
// // 🧠 Concept:

// // struct defines the shape of your data

// // You use String::from() for owned strings

// // Fields are accessed with dot notation (summafy.name)



// 🧩 Example 2: Add Behavior (Impl Block)

// Now let’s give our struct some methods, like in OOP:

// 🎯 Your Challenge:

// Add a new field called employees: u32

// Add a method avg_revenue_per_employee() → divide annual revenue by employees

// Print it in the description


// struct Startup {
//     name: String,
//     category: String,
//     monthly_revenue: f64,
//     employees: u32
// }

// impl Startup {
//     fn annual_revenue(&self) -> f64 {
//         self.monthly_revenue * 12.0
//     }

//     fn describe(&self) {
//         println!(
//             "{} operates in {} and makes ${} per year.We have {} empolyees each with an average revenue of ${} per year.",
//             self.name,
//             self.category,
//             self.annual_revenue(),
//             self.employees,
//             self.avg_revenue_per_employee()
//         );
//     }

//     fn avg_revenue_per_employee(&self) -> f64 {
//         self.annual_revenue() / (self.employees as f64)
//     }
// }

// fn main() {
//     let summafy = Startup {
//         name: String::from("Summafy.io"),
//         category: String::from("AI & Productivity"),
//         monthly_revenue: 1200.50,
//         employees: 10,
//     };

//     summafy.describe();
// }


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
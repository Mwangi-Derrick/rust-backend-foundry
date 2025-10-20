// 🪜 Lesson 7 — Error Handling (Result, Option & the ? Operator)

// Think of it like this:

// Option<T> = something may or may not exist

// Result<T, E> = something may succeed or fail
// Rust doesn’t throw exceptions — you handle everything gracefully at compile time.

// 🧩 Example 1 — Option<T>
// fn find_user(id: u32) -> Option<&'static str> {
//     if id == 1 {
//         Some("derrick")
//     } else {
//         None
//     }
// }

// fn main() {
//     match find_user(1) {
//         Some(name) => println!("User found: {}", name),
//         None => println!("User not found"),
//     }
// }


// ✅ Output:

// User found: derrick


// Change find_user(2) and you’ll see:

// User not found

// 🧠 Concept

// Option forces you to think about missing values at compile time.

// No null pointer crashes — ever.

// 🧩 Example 2 — Result<T, E>

// Now let’s simulate reading a file (or DB call):

// use std::fs::File;
// use std::io::{self, Read};

// fn read_config() -> Result<String, io::Error> {
//     let mut file = File::open("config.txt")?; // the '?' propagates errors
//     let mut contents = String::new();
//     file.read_to_string(&mut contents)?;
//     Ok(contents)
// }

// fn main() {
//     match read_config() {
//         Ok(c) => println!("Config loaded:\n{}", c),
//         Err(e) => println!("Error reading config: {}", e),
//     }
// }


// ✅ Explanation:

// Result<T, E> has two variants:

// Ok(value) → success

// Err(error) → failure

// The ? operator automatically returns the error if it occurs — super clean!

// 🧠 Instructor Tip

// Think of ? as saying:
// “If this fails, bubble it up to the caller — don’t panic.”

// This makes it safe and elegant — no exceptions, no runtime cost.

// 🧩 Mini Challenge

// Create a function divide(a: f64, b: f64) -> Result<f64, String>

// If b == 0.0, return Err("Cannot divide by zero".into())

// Otherwise, return Ok(a / b)

// In main(), handle both cases with a match
 
fn divide(a: f64, b: f64) -> Result<f64, String>{
if b == 0.0 {
Err("Cannot divide by zero".into())
} else {
    Ok(a / b)
}
}
fn main(){
    match divide(20.0,2.0) {
      Ok(e) => println!("Error: {}", e),
      Err(result) => println!("cannot divide by zero"),
    }
}
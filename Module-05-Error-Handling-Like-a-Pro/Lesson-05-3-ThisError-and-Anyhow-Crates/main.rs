// Lesson 05.3: ThisError and Anyhow Crates

// In the last lesson, we learned how to create custom error types from scratch.
// This can be a lot of boilerplate code. In this lesson, we will learn about
// two popular crates that make error handling much easier: `thiserror` and `anyhow`.

// Note: To run this code, you will need to add `thiserror` and `anyhow` to your
// `Cargo.toml` file:
//
// [dependencies]
// thiserror = "1.0"
// anyhow = "1.0"

// --- `thiserror`: For Libraries ---

// `thiserror` is a crate for creating custom error types in libraries. It uses a
// procedural macro to automatically generate the `Display`, `Error`, and `From`
// implementations for your error enum.

use thiserror::Error;

#[derive(Error, Debug)]
pub enum MyError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Parse error: {0}")]
    Parse(#[from] std::num::ParseIntError),
}

fn read_and_parse_thiserror() -> Result<i32, MyError> {
    let mut file = std::fs::File::open("number.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let number = contents.trim().parse()?;
    Ok(number)
}

// --- `anyhow`: For Applications ---

// `anyhow` is a crate for creating a single, general-purpose error type in
// applications. It is designed to be very easy to use. It provides a `Result`
// type alias and an `Error` type that can hold any kind of error.

use anyhow::{Context, Result};

fn read_and_parse_anyhow() -> Result<i32> {
    let mut file = std::fs::File::open("number.txt")
        .with_context(|| "Failed to open number.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let number = contents.trim().parse()
        .with_context(|| format!("Failed to parse '{}'", contents.trim()))?;
    Ok(number)
}

fn main() {
    // Using thiserror
    println!("--- Using thiserror ---");
    match read_and_parse_thiserror() {
        Ok(n) => println!("The number is: {}", n),
        Err(e) => println!("Error: {}", e),
    }

    // Using anyhow
    println!("\n--- Using anyhow ---");
    match read_and_parse_anyhow() {
        Ok(n) => println!("The number is: {}", n),
        Err(e) => println!("Error: {:?}", e),
    }
}

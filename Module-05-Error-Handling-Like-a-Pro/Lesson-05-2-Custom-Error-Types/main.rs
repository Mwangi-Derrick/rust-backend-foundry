// Lesson 05.2: Custom Error Types

// While `io::Error` is useful for I/O operations, sometimes you need to create
// your own custom error types to represent the specific ways that your code can fail.

// --- Defining a Custom Error Type ---

// A good custom error type should:
// - Be a struct or an enum.
// - Implement the `Debug` and `Display` traits.
// - Implement the `Error` trait.

use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct AppError {
    message: String,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for AppError {}

// --- Using a Custom Error Type ---

fn produce_error() -> Result<(), AppError> {
    Err(AppError { message: String::from("Something went wrong!") })
}

// --- Converting Other Error Types to Your Custom Error Type ---

// The `?` operator can automatically convert between different error types if
// you implement the `From` trait.

use std::fs::File;
use std::io;

#[derive(Debug)]
enum MyError {
    Io(io::Error),
    Parse(std::num::ParseIntError),
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MyError::Io(e) => write!(f, "IO error: {}", e),
            MyError::Parse(e) => write!(f, "Parse error: {}", e),
        }
    }
}

impl Error for MyError {}

impl From<io::Error> for MyError {
    fn from(error: io::Error) -> Self {
        MyError::Io(error)
    }
}

impl From<std::num::ParseIntError> for MyError {
    fn from(error: std::num::ParseIntError) -> Self {
        MyError::Parse(error)
    }
}

fn read_and_parse() -> Result<i32, MyError> {
    let mut file = File::open("number.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let number = contents.trim().parse()?;
    Ok(number)
}

fn main() {
    match produce_error() {
        Ok(_) => println!("Success!"),
        Err(e) => println!("Error: {}", e),
    }

    // Note: To run this code, you will need to create a file named `number.txt`
    // in the root of your project with a number in it.

    match read_and_parse() {
        Ok(n) => println!("The number is: {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

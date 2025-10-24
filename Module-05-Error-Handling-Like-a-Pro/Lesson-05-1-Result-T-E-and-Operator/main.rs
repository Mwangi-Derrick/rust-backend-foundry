// Lesson 05.1: Result<T, E> and the ? Operator

// We have already seen `Result<T, E>` in a previous lesson. This lesson will
// dive deeper into how to use it effectively for robust error handling.

// --- The `Result` Enum ---

// The `Result<T, E>` enum has two variants:
// - `Ok(T)`: A value of type `T` was returned successfully.
// - `Err(E)`: An error of type `E` occurred.

use std::fs::File;
use std::io::{self, Read};

// --- Handling `Result` with `match` ---

fn read_username_from_file_long() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// --- The `?` Operator: A Shortcut for Propagating Errors ---

// The `?` operator is a powerful tool for simplifying error handling. It can be
// used on a `Result` value. If the value is `Ok`, the value inside the `Ok` will
// be returned from the expression. If the value is `Err`, the `Err` will be
// returned from the whole function.

fn read_username_from_file_short() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

// We can even chain `?` calls to make the code even shorter.
fn read_username_from_file_shorter() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

// --- The `?` Operator and `Option<T>` ---

// The `?` operator can also be used with `Option<T>`. If the value is `Some`,
// the value inside the `Some` will be returned from the expression. If the value
// is `None`, `None` will be returned from the whole function.

fn first_word(s: &str) -> Option<&str> {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return Some(&s[0..i]);
        }
    }

    Some(&s[..])
}

fn first_word_uppercase(s: &str) -> Option<String> {
    let word = first_word(s)?;
    Some(word.to_uppercase())
}

fn main() {
    // Note: To run this code, you will need to create a file named `hello.txt`
    // in the root of your project.

    match read_username_from_file_shorter() {
        Ok(s) => println!("The username is: {}", s),
        Err(e) => println!("Error reading username: {:?}", e),
    }

    let s = "hello world";
    match first_word_uppercase(s) {
        Some(s) => println!("The first word in uppercase is: {}", s),
        None => println!("Could not find the first word."),
    }
}

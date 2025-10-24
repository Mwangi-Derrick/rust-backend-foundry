// Lesson 06.1: Reading and Writing Files

// This lesson covers the basics of reading from and writing to files in Rust.

use std::fs::File;
use std::io::{self, Read, Write};

fn main() -> io::Result<()> {
    // --- Writing to a File ---

    // The `File::create` function creates a new file for writing. If the file
    // already exists, its contents will be overwritten.
    let mut file = File::create("output.txt")?;

    // The `write_all` method takes a byte slice and writes it to the file.
    file.write_all(b"Hello, world!")?;

    // --- Reading from a File ---

    // The `File::open` function opens a file for reading.
    let mut file = File::open("output.txt")?;

    // The `read_to_string` method reads the entire contents of the file into a
    // string.
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    println!("The contents of the file are: {}", contents);

    // --- Convenience Functions ---

    // The `std::fs` module provides some convenience functions for reading and
    // writing files.

    // `std::fs::write` creates a new file and writes a byte slice to it.
    std::fs::write("convenience.txt", b"Hello from the convenience function!")?;

    // `std::fs::read_to_string` reads the entire contents of a file into a string.
    let contents = std::fs::read_to_string("convenience.txt")?;

    println!("The contents of the convenience file are: {}", contents);

    Ok(())
}

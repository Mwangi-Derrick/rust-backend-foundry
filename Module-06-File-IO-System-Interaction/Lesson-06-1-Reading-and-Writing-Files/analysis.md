# Lesson 06.1: Reading and Writing Files

## 🧠 Concept Summary

This lesson covers the basics of file I/O (input/output) in Rust. We'll look at how to read from and write to files using the `std::fs` and `std::io` modules.

- **`std::fs::File`:** The `File` struct represents a file. It has methods for reading from and writing to the file.

- **`std::io::{Read, Write}` Traits:** The `Read` and `Write` traits provide the main methods for reading from and writing to I/O devices, including files.

- **Error Handling:** Most file I/O operations return a `Result`, so you need to use `match` or the `?` operator to handle errors.

- **Convenience Functions:** The `std::fs` module provides some convenience functions (`std::fs::read_to_string` and `std::fs::write`) for common file I/O operations.

## 🧩 Code Walkthrough

Let's analyze the code in `main.rs`.

### Writing to a File

```rust
let mut file = File::create("output.txt")?;
file.write_all(b"Hello, world!")?;
```

`File::create` opens a file in write-only mode. If the file already exists, its contents are truncated. The `write_all` method takes a byte slice (`&[u8]`) and writes it to the file. The `b` prefix on the string literal creates a byte string literal.

### Reading from a File

```rust
let mut file = File::open("output.txt")?;
let mut contents = String::new();
file.read_to_string(&mut contents)?;
```

`File::open` opens a file in read-only mode. The `read_to_string` method reads the entire contents of the file into a `String`.

### Convenience Functions

```rust
std::fs::write("convenience.txt", b"Hello from the convenience function!")?;
let contents = std::fs::read_to_string("convenience.txt")?;
```

These functions are convenient for simple cases where you just want to read or write the entire contents of a file at once.

## ⚔️ Cross-Language Insights

- **vs. Golang:** Go's `os` package provides functions for reading and writing files. The `ioutil` package provides convenience functions similar to `std::fs`.

- **vs. TypeScript (Node.js):** In Node.js, the `fs` module provides functions for file I/O. There are both synchronous and asynchronous versions of these functions.

- **vs. C:** In C, you would use `fopen`, `fwrite`, `fread`, and `fclose` to work with files. You have to be careful to close the file when you are done with it. In Rust, the file is automatically closed when the `File` struct goes out of scope, thanks to the `Drop` trait.

## 🚀 Practical Reflection

- **RAII (Resource Acquisition Is Initialization):** The `File` struct is a good example of the RAII pattern in Rust. The file is opened when the `File` struct is created, and it is closed when the struct is dropped. This makes it much harder to forget to close a file.

- **Buffered I/O:** For more advanced file I/O, you can use `std::io::BufReader` and `std::io::BufWriter` to get buffered I/O, which can be more efficient. We will cover this in the next lesson.

- **Error Handling is Crucial:** File I/O is an area where errors are very common. A file might not exist, you might not have permission to read it, the disk might be full, etc. Rust's `Result` enum forces you to handle these errors, which leads to more robust code.

## 🧩 Self-Review Prompts

- Write a program that reads a file, modifies its contents, and then writes the modified contents back to the file.
- What happens if you try to open a file that doesn't exist with `File::open`?
- Look at the documentation for the `OpenOptions` struct in `std::fs`. How can you use it to open a file in append mode?

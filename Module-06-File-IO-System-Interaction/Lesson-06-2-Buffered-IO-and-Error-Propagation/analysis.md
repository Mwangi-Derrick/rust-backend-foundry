# Lesson 06.2: Buffered I/O and Error Propagation

## 🧠 Concept Summary

This lesson builds on our knowledge of file I/O by introducing **buffered I/O**, which can be more efficient for many use cases. We also see another practical example of error propagation with the `?` operator.

- **Buffered I/O:** When you read from or write to a file, each operation can correspond to a system call, which can be slow. Buffered I/O improves performance by reading a large chunk of data into a buffer in memory and then reading from the buffer, or by writing to a buffer in memory and then writing the entire buffer to the file at once. This reduces the number of system calls.

- **`BufReader` and `BufWriter`:** The `std::io` module provides the `BufReader` and `BufWriter` structs for buffered reading and writing. They wrap a reader or a writer and add buffering.

- **The `lines()` Method:** `BufReader` provides a convenient `lines()` method that returns an iterator over the lines of a file.

## 🧩 Code Walkthrough

Let's analyze the code in `main.rs`.

### Buffered Reading

```rust
fn read_lines(path: &str) -> io::Result<Vec<String>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut lines = Vec::new();
    for line in reader.lines() {
        lines.push(line?);
    }

    Ok(lines)
}
```

This function reads all the lines from a file into a `Vec<String>`. `BufReader::new(file)` creates a new `BufReader` that wraps the `File`. The `reader.lines()` method returns an iterator over the lines of the file. Each item in the iterator is a `Result<String, io::Error>`, so we use the `?` operator to unwrap the `Ok` or return the `Err`.

### Buffered Writing

```rust
fn write_lines(path: &str, lines: &[String]) -> io::Result<()> {
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);

    for line in lines {
        writer.write_all(line.as_bytes())?;
        writer.write_all(b"\n")?;
    }

    Ok(())
}
```

This function writes a slice of strings to a file, with each string on a new line. `BufWriter::new(file)` creates a new `BufWriter`. We then write each line to the `writer`. The `BufWriter` will buffer the writes and write them to the file when the buffer is full or when the `writer` is dropped.

## ⚔️ Cross-Language Insights

- **vs. Golang:** Go's `bufio` package provides `Reader` and `Writer` types that are similar to Rust's `BufReader` and `BufWriter`.

- **vs. TypeScript (Node.js):** In Node.js, streams are used for efficient I/O. You can create a readable stream from a file and pipe it to a writable stream.

- **vs. C:** In C, you can use `setvbuf` to control the buffering of a `FILE*` stream.

## 🚀 Practical Reflection

- **Performance:** Buffered I/O is almost always a good idea when you are doing a lot of small reads or writes. The performance improvement can be significant.

- **The `lines()` Iterator:** The `lines()` method is a great example of how Rust's iterator system can make I/O code more concise and readable.

- **Error Propagation:** This lesson provides another good example of how the `?` operator can be used to write clean and robust error handling code.

## 🧩 Self-Review Prompts

- Modify the `read_lines` function to print each line to the console instead of returning a `Vec<String>`.
- What happens if you don't use a `BufWriter` and just write to the `File` directly? Is it still buffered? (Hint: Look at the documentation for `File`).
- How can you flush a `BufWriter` manually?


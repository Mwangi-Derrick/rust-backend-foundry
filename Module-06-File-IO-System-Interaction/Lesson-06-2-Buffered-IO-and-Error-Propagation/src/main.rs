// Lesson 06.2: Buffered I/O and Error Propagation

// In the last lesson, we learned the basics of reading and writing files. In this
// lesson, we will look at buffered I/O, which can be more efficient, and we
// will see another example of how to propagate errors.

use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

// --- Buffered Reading ---

// When you read from a file, the operating system has to perform a system call,
// which can be slow. Buffered I/O reduces the number of system calls by reading
// a large chunk of the file into a buffer in memory, and then reading from the
// buffer.

fn read_lines(path: &str) -> io::Result<Vec<String>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut lines = Vec::new();
    for line in reader.lines() {
        lines.push(line?);
    }

    Ok(lines)
}

// --- Buffered Writing ---

// Similarly, buffered writing can be more efficient because it writes to a
// buffer in memory and then writes the buffer to the file in one go.

use std::io::BufWriter;

fn write_lines(path: &str, lines: &[String]) -> io::Result<()> {
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);

    for line in lines {
        writer.write_all(line.as_bytes())?;
        writer.write_all(b"\n")?;
    }

    Ok(())
}

fn main() -> io::Result<()> {
    let lines = vec![
        String::from("hello"),
        String::from("world"),
        String::from("from Rust!"),
    ];

    write_lines("buffered_output.txt", &lines)?;

    let read_lines = read_lines("buffered_output.txt")?;
    for line in read_lines {
        println!("{}", line);
    }

    Ok(())
}

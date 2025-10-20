// 🧩 Step 1: Import the Needed Modules
use std::fs::File;
use std::io::{self, Read, Write};

// ⚙️ Step 2: Writing to a File
fn write_to_file(filename: &str, content: &str) -> io::Result<()> {
    let mut file = File::create(filename)?; // create new file (overwrites existing)
    file.write_all(content.as_bytes())?;
    println!("✅ Successfully wrote to {}", filename);
    Ok(())
}

// 📖 Step 3: Reading from a File
fn read_from_file(filename: &str) -> io::Result<String> {
    let mut file = File::open(filename)?; // open file for reading
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

// 🚀 Step 4: Using It All in main
fn main() -> io::Result<()> {
    write_to_file("outbox_log.txt", "Outbox relay started...")?;
    let log_content = read_from_file("outbox_log.txt")?;
    println!("📜 File content:\n{}", log_content);
    Ok(())
}

// 🧠 Breakdown:

// File::create() → returns a Result<File, io::Error>

// write_all() & read_to_string() → return Result<(), io::Error>

// The ? operator bubbles up any io::Error

// The function returns io::Result<()> so the compiler knows what to expect

// ✅ Try This:
// Create a new folder 09-file-io, paste that code, and run:

// cargo run


// Then open outbox_log.txt — you should see:

// Outbox relay started...
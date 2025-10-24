// Lesson 18.2: Streaming File Processing with Async Buffers

// This lesson focuses on efficient, streaming processing of large files using
// asynchronous buffered I/O. This is crucial for AI pipelines that deal with
// datasets too large to fit into memory.

// --- Why Streaming? ---

// - **Memory Efficiency:** Process data in chunks, avoiding loading entire large
//   files into RAM.
// - **Responsiveness:** Allows other tasks to run while waiting for I/O, especially
//   important in async applications.
// - **Scalability:** Enables processing of arbitrarily large files.

// --- Async Buffered I/O with Tokio ---

// `tokio::fs::File` provides asynchronous file access.
// `tokio::io::BufReader` adds buffering on top of an `AsyncRead` implementor.

use tokio::fs::File;
use tokio::io::{self, AsyncBufReadExt, AsyncReadExt, BufReader};
use anyhow::Result;

// --- Example: Counting Lines in a Large File ---

async fn count_lines_streaming(file_path: &str) -> Result<usize> {
    let file = File::open(file_path).await?;
    let reader = BufReader::new(file);

    let mut lines = reader.lines();
    let mut line_count = 0;

    while let Some(line) = lines.next_line().await? {
        // Simulate processing each line
        // println!("Processing line: {}", line);
        line_count += 1;
    }

    Ok(line_count)
}

// --- Example: Processing Chunks of a Binary File ---

async fn process_binary_chunks(file_path: &str, chunk_size: usize) -> Result<()> {
    let file = File::open(file_path).await?;
    let mut reader = BufReader::new(file);

    let mut buffer = vec![0; chunk_size];
    let mut total_bytes_read = 0;

    loop {
        let bytes_read = reader.read(&mut buffer).await?;
        if bytes_read == 0 {
            break; // End of file
        }

        let chunk = &buffer[..bytes_read];
        total_bytes_read += bytes_read;

        // Simulate processing the chunk
        println!("Processed {} bytes. Total: {}", bytes_read, total_bytes_read);
        // Example: calculate sum of bytes in chunk
        let chunk_sum: u8 = chunk.iter().sum();
        println!("  Chunk sum: {}", chunk_sum);
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    // Create a dummy large file for demonstration
    let dummy_file_path = "large_data.txt";
    tokio::fs::write(dummy_file_path, (0..10000).map(|i| format!("Line {}", i)).collect::<Vec<String>>().join("\n")).await?;

    println!("--- Counting Lines Streaming ---");
    let lines = count_lines_streaming(dummy_file_path).await?;
    println!("Total lines: {}", lines);

    // Create a dummy binary file for demonstration
    let dummy_binary_path = "binary_data.bin";
    let binary_data: Vec<u8> = (0..255).cycle().take(1024 * 1024).collect(); // 1MB of data
    tokio::fs::write(dummy_binary_path, &binary_data).await?;

    println!("\n--- Processing Binary Chunks ---");
    process_binary_chunks(dummy_binary_path, 4096).await?;

    // Clean up dummy files
    tokio::fs::remove_file(dummy_file_path).await?;
    tokio::fs::remove_file(dummy_binary_path).await?;

    Ok(())
}

// Lesson 18.1: Zero-Copy Data Processing with Slices

// This lesson focuses on optimizing data processing in Rust, particularly for
// AI pipelines, by minimizing data copying using slices.

// --- Why Zero-Copy? ---

// Data copying is a significant source of overhead in many applications,
// especially when dealing with large datasets common in AI. Each copy consumes
// CPU cycles and memory bandwidth. Zero-copy techniques aim to process data
// in-place or by referencing existing memory, avoiding unnecessary allocations
// and copies.

// --- Slices in Rust ---

// Rust's slices (`&[T]`, `&mut [T]`, `&str`, `&[u8]`) are fundamental to
// zero-copy operations. A slice is a view into a contiguous sequence of elements
// in a collection. It does not own the data; it merely borrows it.

// --- Example: Processing a Large Buffer ---

// Imagine receiving a large byte buffer (e.g., from a network stream or file)
// and needing to extract and process parts of it.

fn process_data_zero_copy(data: &[u8]) {
    println!("Processing data (zero-copy):");

    // Extract a header (first 5 bytes) without copying
    let header = &data[0..5];
    println!("  Header: {:?}", header);

    // Extract a payload (remaining bytes) without copying
    let payload = &data[5..];
    println!("  Payload length: {}", payload.len());

    // Simulate processing the payload
    let sum: u8 = payload.iter().sum();
    println!("  Payload sum: {}", sum);
}

fn process_data_with_copy(data: Vec<u8>) {
    println!("Processing data (with copy):");

    // Extract a header (first 5 bytes) by copying
    let header = data[0..5].to_vec();
    println!("  Header: {:?}", header);

    // Extract a payload (remaining bytes) by copying
    let payload = data[5..].to_vec();
    println!("  Payload length: {}", payload.len());

    // Simulate processing the payload
    let sum: u8 = payload.iter().sum();
    println!("  Payload sum: {}", sum);
}

fn main() {
    let large_buffer: Vec<u8> = (0..255).collect(); // Simulate a large buffer

    println!("--- Zero-Copy Approach ---");
    process_data_zero_copy(&large_buffer);

    println!("\n--- With Copy Approach ---");
    // Note: We clone the buffer here to simulate passing a new owned copy
    // to the function, as `process_data_with_copy` takes ownership.
    process_data_with_copy(large_buffer.clone());

    println!("\n--- String Slices ---");
    let text = String::from("The quick brown fox jumps over the lazy dog.");
    let word_slice = &text[4..9]; // "quick"
    println!("Original text: {}", text);
    println!("Word slice: {}", word_slice);

    // No copy occurred when creating word_slice.
    // The original `text` still owns the data.
}

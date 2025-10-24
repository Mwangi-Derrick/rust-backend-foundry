// Lesson 18.3: Parallel Tokenization and Chunking

// This lesson focuses on optimizing text processing, a common task in AI/NLP
// pipelines, by using parallel tokenization and chunking techniques.

// --- Why Parallelize Text Processing? ---

// - **Large Datasets:** NLP models often train on massive text corpora.
// - **CPU-bound:** Tokenization, stemming, and other text transformations are
//   typically CPU-intensive.
// - **Performance:** Parallelizing these steps can significantly reduce processing
//   time, especially on multi-core machines.

// --- Tokenization ---

// Tokenization is the process of breaking a stream of text into smaller units
// called tokens (e.g., words, subwords, characters).

// --- Chunking ---

// Chunking involves dividing a large text into smaller, manageable segments
// for parallel processing.

// --- `rayon` for Parallel Iteration ---

// `rayon` is an excellent library for data parallelism in Rust, making it easy
// to parallelize iterators.

use rayon::prelude::*;
use std::collections::HashMap;
use std::sync::Mutex;

// --- Example: Parallel Word Count ---

fn parallel_word_count(text: &str) -> HashMap<String, usize> {
    let word_counts = Mutex::new(HashMap::new());

    text.par_split_whitespace()
        .for_each(|word| {
            let mut map = word_counts.lock().unwrap();
            *map.entry(word.to_lowercase()).or_insert(0) += 1;
        });

    word_counts.into_inner().unwrap()
}

// --- Example: Parallel Chunk Processing ---

// Imagine a large document that needs to be processed in chunks.

fn process_chunk(chunk_id: usize, chunk_text: &str) -> Vec<String> {
    println!("Processing chunk {} on thread {:?}", chunk_id, std::thread::current().id());
    // Simulate some NLP processing: simple tokenization and filtering
    chunk_text.split_whitespace()
        .filter(|&word| word.len() > 3)
        .map(|word| word.to_lowercase())
        .collect()
}

fn parallel_chunk_processing(document: &str, chunk_size: usize) -> Vec<String> {
    document.as_bytes().chunks(chunk_size)
        .enumerate()
        .par_bridge() // Bridge to Rayon's parallel iterator
        .flat_map(|(i, chunk_bytes)| {
            let chunk_text = String::from_utf8_lossy(chunk_bytes);
            process_chunk(i, &chunk_text)
        })
        .collect()
}

fn main() {
    let document = "Rust is a systems programming language. It is fast, safe, and concurrent. Rust is great for performance-critical applications.";

    println!("--- Parallel Word Count ---");
    let counts = parallel_word_count(document);
    println!("Word counts: {:?}", counts);

    println!("\n--- Parallel Chunk Processing ---");
    let processed_tokens = parallel_chunk_processing(document, 20);
    println!("Processed tokens: {:?}", processed_tokens);
}

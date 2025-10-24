# Lesson 18.3: Parallel Tokenization and Chunking

## 🧠 Concept Summary

This lesson focuses on optimizing text processing, a common task in AI/NLP pipelines, by using **parallel tokenization and chunking** techniques. This is crucial for handling large text datasets efficiently.

- **Why Parallelize Text Processing?**
    - **Large Datasets:** NLP models often train on massive text corpora that are too large for sequential processing to be efficient.
    - **CPU-bound:** Tokenization, stemming, lemmatization, and other text transformations are typically CPU-intensive operations.
    - **Performance:** Parallelizing these steps across multiple CPU cores can significantly reduce processing time.

- **Tokenization:** The process of breaking a stream of text into smaller units called tokens (e.g., words, subwords, characters, punctuation). This is often the first step in any NLP pipeline.

- **Chunking:** Involves dividing a large text or data stream into smaller, manageable segments (chunks) that can be processed independently and in parallel.

- **`rayon` for Parallel Iteration:** `rayon` is a powerful data-parallelism library for Rust. It makes it easy to convert sequential iterators into parallel iterators, automatically distributing the work across available CPU cores.

## ⚙️ Setup

To use `rayon`, you need to add it to your `Cargo.toml` file:

```toml
[dependencies]
rayon = "1.5"
```

## 🧩 Code Walkthrough

Let's analyze the code in `main.rs`.

### Parallel Word Count

```rust
use rayon::prelude::*;
use std::collections::HashMap;
use std::sync::Mutex;

fn parallel_word_count(text: &str) -> HashMap<String, usize> {
    let word_counts = Mutex::new(HashMap::new());

    text.par_split_whitespace()
        .for_each(|word| {
            let mut map = word_counts.lock().unwrap();
            *map.entry(word.to_lowercase()).or_insert(0) += 1;
        });

    word_counts.into_inner().unwrap()
}
```

This function demonstrates parallel word counting. `text.par_split_whitespace()` creates a parallel iterator over words. `for_each` then processes each word in parallel. Since multiple threads will be updating the `HashMap`, we need to protect it with a `std::sync::Mutex`. Each thread acquires the lock, updates the count for its word, and releases the lock. Finally, `into_inner()` extracts the `HashMap` from the `Mutex`.

### Parallel Chunk Processing

```rust
fn process_chunk(chunk_id: usize, chunk_text: &str) -> Vec<String> { /* ... */ }

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
```

This example processes a large `document` by dividing it into `chunk_size` byte chunks. `document.as_bytes().chunks(chunk_size)` creates an iterator over byte slices. `par_bridge()` converts this sequential iterator into a parallel one, allowing `flat_map` to process each chunk concurrently. Inside `flat_map`, each `chunk_bytes` is converted to a `String` (potentially a copy, but often unavoidable for text processing) and then passed to `process_chunk` for simulated NLP operations.

## ⚔️ Cross-Language Insights

- **Golang:** Go would achieve similar parallel processing using `goroutine`s and channels. You would typically split the input text into chunks, launch a `goroutine` for each chunk, and then use a channel to collect results.

- **TypeScript (Node.js):** Node.js would use `worker_threads` to parallelize CPU-bound text processing. The main thread would distribute chunks to worker threads, and results would be communicated back via messages.

- **Python:** Python's `multiprocessing` module is used for true parallelism (bypassing the GIL). Libraries like `spaCy` or `NLTK` might offer parallel processing capabilities for NLP tasks.

## 🚀 Practical Reflection

- **Performance Gains:** Parallelizing text processing can lead to significant speedups for large datasets, making AI/NLP pipelines more efficient.

- **`rayon` Ergonomics:** `rayon` makes it remarkably easy to parallelize existing iterator chains with minimal code changes, often just by replacing `iter()` with `par_iter()` or using `par_bridge()`.

- **Shared State vs. Message Passing:** In `parallel_word_count`, we used a `Mutex` to share state. In `parallel_chunk_processing`, each chunk is processed independently, and results are collected, which is often a more scalable approach for data parallelism.

## 🧩 Self-Review Prompts

- Modify `parallel_word_count` to use a `HashMap` per thread and then combine them at the end, instead of a single `Mutex<HashMap>`. Compare the performance.
- Implement a parallel text search function that finds all occurrences of a given pattern in a large document, returning the line numbers.
- Research how `rayon` handles thread pool management. Can you configure the number of threads it uses?

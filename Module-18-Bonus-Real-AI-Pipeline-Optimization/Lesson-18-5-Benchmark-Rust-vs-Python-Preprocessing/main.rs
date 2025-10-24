// Lesson 18.5: Benchmark: Rust vs. Python Preprocessing

// This lesson provides a concrete benchmark comparison between a Rust-implemented
// text preprocessing function (using `PyO3`) and its pure Python equivalent.
// This will quantitatively demonstrate the performance benefits of integrating
// Rust into Python AI pipelines.

// --- Setup ---

// To run this benchmark, you will need:
// 1.  A Rust library crate (`my_python_ai_module`) with the `preprocess_text`
//     function from the previous lesson (18.4).
// 2.  `maturin` installed (`pip install maturin`).
// 3.  The Rust module built and installed in your Python environment:
//     `cd my_python_ai_module && maturin develop`

// --- Python Benchmark Script ---

// This script will be executed directly in Python.

// ```python
// # benchmark_preprocessing.py
// 
// import my_python_ai_module
// import time
// import random
// import string
// 
// # --- Data Generation ---
// def generate_random_text(num_words, word_length_range=(3, 10)):
//     words = []
//     for _ in range(num_words):
//         length = random.randint(word_length_range[0], word_length_range[1])
//         word = ''.join(random.choice(string.ascii_lowercase) for _ in range(length))
//         words.append(word)
//     return ' '.join(words)
// 
// # Generate a large text for benchmarking
// LARGE_TEXT = generate_random_text(1_000_000) # 1 million words
// 
// # --- Python Implementation ---
// def python_preprocess_text(text):
//     return [word.lower() for word in text.split() if len(word) > 3]
// 
// # --- Benchmarking Function ---
// def run_benchmark(func, text, name):
//     start_time = time.perf_counter()
//     _ = func(text)
//     end_time = time.perf_counter()
//     print(f"{name} preprocessing time: {end_time - start_time:.6f} seconds")
//     return end_time - start_time
// 
// if __name__ == "__main__":
//     print("\n--- Benchmarking Text Preprocessing ---")
// 
//     # Benchmark Rust implementation
//     rust_time = run_benchmark(my_python_ai_module.preprocess_text, LARGE_TEXT, "Rust (PyO3)")
// 
//     # Benchmark Python implementation
//     python_time = run_benchmark(python_preprocess_text, LARGE_TEXT, "Pure Python")
// 
//     if rust_time > 0:
//         print(f"\nRust (PyO3) is {python_time / rust_time:.2f}x faster than Pure Python.")
//     else:
//         print("Rust time was too fast to measure or zero.")
// ```

fn main() {
    println!("This lesson provides a benchmark comparison between Rust and Python.");
    println!("The code for this lesson is a conceptual Python script that runs the benchmark.");
}

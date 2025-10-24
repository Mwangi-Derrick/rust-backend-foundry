# Lesson 18.5: Benchmark: Rust vs. Python Preprocessing

## 🧠 Concept Summary

This lesson provides a concrete **benchmark comparison** between a Rust-implemented text preprocessing function (using `PyO3`) and its pure Python equivalent. The goal is to quantitatively demonstrate the performance benefits of integrating Rust into Python AI pipelines for CPU-bound tasks.

- **Benchmarking Goal:** To measure the execution time of identical text preprocessing logic implemented in both Rust (exposed to Python via `PyO3`) and pure Python, using a sufficiently large dataset to highlight performance differences.

- **Expected Outcome:** Rust is expected to be significantly faster for CPU-bound tasks like text processing due to its compiled nature, lack of GIL overhead (when Rust code is executing), and efficient memory management.

## ⚙️ Setup

To run this benchmark, you will need:

1.  **Rust Library Crate:** A Rust library crate (e.g., `my_python_ai_module`) containing the `preprocess_text` function from Lesson 18.4.
2.  **`maturin`:** Installed in your Python environment (`pip install maturin`).
3.  **Rust Module Built and Installed:** Navigate to your Rust library crate directory (`my_python_ai_module`) and run `maturin develop`. This compiles the Rust code and makes it importable in your Python environment.

## 🧩 Code Walkthrough

This lesson's core is a Python script designed to run the benchmark.

### Python Benchmark Script (`benchmark_preprocessing.py`)

```python
# benchmark_preprocessing.py

import my_python_ai_module
import time
import random
import string

# --- Data Generation ---
def generate_random_text(num_words, word_length_range=(3, 10)):
    # ... generates a string of random words ...

LARGE_TEXT = generate_random_text(1_000_000) # 1 million words

# --- Python Implementation ---
def python_preprocess_text(text):
    return [word.lower() for word in text.split() if len(word) > 3]

# --- Benchmarking Function ---
def run_benchmark(func, text, name):
    start_time = time.perf_counter()
    _ = func(text)
    end_time = time.perf_counter()
    print(f"{name} preprocessing time: {end_time - start_time:.6f} seconds")
    return end_time - start_time

if __name__ == "__main__":
    print("\n--- Benchmarking Text Preprocessing ---")

    # Benchmark Rust implementation
    rust_time = run_benchmark(my_python_ai_module.preprocess_text, LARGE_TEXT, "Rust (PyO3)")

    # Benchmark Python implementation
    python_time = run_benchmark(python_preprocess_text, LARGE_TEXT, "Pure Python")

    if rust_time > 0:
        print(f"\nRust (PyO3) is {python_time / rust_time:.2f}x faster than Pure Python.")
    else:
        print("Rust time was too fast to measure or zero.")
```

-   **Data Generation:** The `generate_random_text` function creates a large string of random words to ensure the benchmark is run on a sufficiently large dataset, making performance differences more apparent.
-   **Python Implementation:** `python_preprocess_text` is a direct Python equivalent of the logic implemented in Rust, performing splitting, filtering, and lowercasing.
-   **`run_benchmark`:** A helper function that takes a function, the text to process, and a name. It uses `time.perf_counter()` for high-resolution timing to measure execution duration.
-   **Main Execution Block:** Calls `run_benchmark` for both the Rust-implemented `my_python_ai_module.preprocess_text` and the `python_preprocess_text` function. Finally, it calculates and prints the speedup factor of Rust over Python.

## ⚔️ Cross-Language Insights

- **Python's GIL (Global Interpreter Lock):** For CPU-bound tasks, Python's GIL prevents true parallelism within a single process. When Rust code (or any C extension) is executing, it can release the GIL, allowing other Python threads to run or for the Rust code itself to execute without GIL contention.

- **Interpretation:** This benchmark visually demonstrates that for tasks involving significant computation on data, Rust can provide substantial speedups over pure Python, even with the overhead of FFI calls.

## 🚀 Practical Reflection

- **Quantitative Proof:** Benchmarks provide quantitative evidence for performance claims. This is crucial when deciding where to invest optimization efforts.

- **Identifying Bottlenecks:** This approach helps in identifying specific CPU-bound bottlenecks in Python AI pipelines that can be effectively offloaded to Rust.

- **Hybrid Performance:** The ability to selectively optimize parts of a Python application with Rust allows developers to maintain Python's productivity for most of the codebase while achieving native performance where it matters most.

## 🧩 Self-Review Prompts

- Run the benchmark script on your machine. What speedup factor do you observe? Does it vary with the size of `LARGE_TEXT`?
- Modify the `preprocess_text` function in Rust to include more complex logic (e.g., stemming or lemmatization using a Rust NLP library). How does this affect the speedup?
- Research other Python benchmarking tools (e.g., `timeit`, `cProfile`). How could you integrate them into this comparison?

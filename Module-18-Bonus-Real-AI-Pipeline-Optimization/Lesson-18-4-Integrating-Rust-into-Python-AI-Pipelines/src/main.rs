// Lesson 18.4: Integrating Rust into Python AI Pipelines (PyO3)

// This lesson builds on our previous FFI knowledge (Lesson 12.1) and focuses
// specifically on how to integrate Rust components into Python-based AI/ML
// pipelines to accelerate performance-critical sections.

// --- Why Integrate Rust into Python AI? ---

// Python is the de-facto language for AI/ML research and development due to its
// rich libraries (TensorFlow, PyTorch, scikit-learn) and ease of use. However,
// for deployment or for specific data preprocessing/postprocessing steps,
// Python's performance can be a bottleneck. Rust offers a solution by allowing
// you to rewrite these bottlenecks in a highly performant and safe language.

// --- Common Use Cases ---

// - **Data Preprocessing:** Fast parsing, cleaning, or transformation of large
//   datasets before feeding them to ML models.
// - **Custom Operators/Layers:** Implementing custom, high-performance operations
//   for deep learning frameworks.
// - **Postprocessing:** Efficiently handling model outputs.
// - **Feature Engineering:** Complex feature calculations.

// --- Example: Accelerating a Text Preprocessing Step ---

// We'll create a Rust function that performs a common text preprocessing task:
// tokenization, lowercasing, and filtering short words. This is a CPU-bound task
// that can benefit greatly from Rust.

// This code is meant to be part of a library crate built with `maturin`.

// ```rust
// // my_python_ai_module/src/lib.rs
//
// use pyo3::prelude::*;
// use pyo3::types::PyList;
//
// #[pyfunction]
// fn preprocess_text(text: &str) -> PyResult<Py<PyList>> {
//     let gil = Python::acquire_gil();
//     let py = gil.python();
//
//     let processed_words: Vec<String> = text
//         .split_whitespace()
//         .filter(|&word| word.len() > 3) // Filter words shorter than 4 chars
//         .map(|word| word.to_lowercase())
//         .collect();
//
//     Ok(PyList::new(py, &processed_words).into())
// }
//
// #[pymodule]
// fn my_python_ai_module(_py: Python, m: &PyModule) -> PyResult<()> {
//     m.add_function(wrap_pyfunction!(preprocess_text, m)?)?;
//     Ok(())
// }
// ```

// --- Python Usage (Conceptual) ---

// ```python
// # python_ai_pipeline.py
//
// import my_python_ai_module
// import time
//
// large_text = """Rust is a systems programming language. It is fast, safe, and concurrent. Rust is great for performance-critical applications. Python is great for AI and data science. Combining them gives you the best of both worlds.""" * 1000 # Simulate large text
//
// start_time = time.time()
// processed_rust = my_python_ai_module.preprocess_text(large_text)
// rust_time = time.time() - start_time
// print(f"Rust preprocessing time: {rust_time:.4f} seconds")
//
// # Compare with pure Python implementation
// def python_preprocess_text(text):
//     return [word.lower() for word in text.split() if len(word) > 3]
//
// start_time = time.time()
// processed_python = python_preprocess_text(large_text)
// python_time = time.time() - start_time
// print(f"Python preprocessing time: {python_time:.4f} seconds")
//
// print(f"Rust is {python_time / rust_time:.2f}x faster than Python for this task.")
// ```

fn main() {
    println!("This lesson focuses on integrating Rust into Python AI pipelines.");
    println!("The code for this lesson is conceptual and demonstrates the usage");
    println!("of PyO3 to accelerate Python AI/ML tasks.");
}

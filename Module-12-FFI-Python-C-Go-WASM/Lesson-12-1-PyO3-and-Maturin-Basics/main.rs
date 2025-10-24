// Lesson 12.1: PyO3 and Maturin Basics

// This lesson introduces how to integrate Rust with Python using the `PyO3`
// crate and `maturin` for building and publishing Python packages.

// --- Why Rust with Python? ---

// Python is a great language for rapid prototyping, data science, and scripting.
// However, for CPU-bound tasks or when you need fine-grained control over memory,
// Python can be slow. Rust can be used to write performance-critical parts of
// a Python application, giving you the best of both worlds.

// --- `PyO3` ---

// `PyO3` is a Rust library that provides a safe and ergonomic way to write Python
// modules in Rust. It handles the FFI (Foreign Function Interface) boilerplate
// for you.

// --- `maturin` ---

// `maturin` is a build tool for Rust-based Python packages. It handles compiling
// your Rust code, generating the necessary Python bindings, and creating a Python
// wheel that can be installed with `pip`.

// --- Example: A Simple Rust Function for Python ---

// To demonstrate, we'll create a simple Rust function that adds two numbers
// and expose it to Python.

// This code is meant to be part of a library crate that is built with `maturin`.
// You would typically create a new library crate for your Python extension.

// ```rust
// // my_python_module/src/lib.rs
//
// use pyo3::prelude::*;
//
// /// Formats the sum of two numbers as a string.
// #[pyfunction]
// fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
//     Ok((a + b).to_string())
// }
//
// /// A Python module implemented in Rust.
// #[pymodule]
// fn my_python_module(_py: Python, m: &PyModule) -> PyResult<()> {
//     m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
//     Ok(())
// }
// ```

// --- Building with `maturin` ---

// 1. Create a new library crate: `cargo new my_python_module --lib`
// 2. Add `pyo3` as a dependency in `my_python_module/Cargo.toml`:
//
//    ```toml
//    [dependencies]
//    pyo3 = { version = "0.19", features = ["extension-module"] }
//    ```
//
// 3. Place the Rust code above into `my_python_module/src/lib.rs`.
// 4. Build the Python wheel: `maturin develop` (from the `my_python_module` directory).
//    This will compile the Rust code and install the Python module in your current
//    Python environment.

// --- Using in Python ---

// ```python
// # python_script.py
//
// import my_python_module
//
// result = my_python_module.sum_as_string(10, 20)
// print(f"The sum is: {result}")
// ```

fn main() {
    println!("This lesson is about integrating Rust with Python using PyO3 and Maturin.");
    println!("The code for this lesson is conceptual and requires setting up a separate");
    println!("Rust library crate and building it with `maturin`.");
}

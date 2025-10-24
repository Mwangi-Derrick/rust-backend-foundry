// Lesson 15.2: flamegraph & perf Tools

// This lesson introduces profiling tools like `perf` (Linux) and `flamegraph`
// for identifying performance bottlenecks in Rust applications.

// --- Why Profiling? ---

// Benchmarking tells you *how fast* your code is. Profiling tells you *why* it's
// that fast (or slow). Profilers collect data about your program's execution,
// such as CPU usage, memory allocation, and function call times.

// --- `perf` (Linux) ---

// `perf` is a powerful performance analysis tool built into the Linux kernel.
// It can collect a wide range of performance counters, including CPU cycles,
// cache misses, and branch predictions.

// Basic usage:
// `perf record -g -- <your_rust_program>`
// `perf report`

// --- `flamegraph` ---

// `flamegraph` is a visualization tool that takes profiling data (e.g., from `perf`)
// and generates an interactive SVG image called a flame graph. Flame graphs are
// excellent for quickly identifying hot spots in your code.

// --- Setup ---

// 1. Install `perf` (Linux only): `sudo apt-get install linux-tools-$(uname -r)`
// 2. Install `flamegraph`: `cargo install flamegraph`

// --- Example: A CPU-intensive Rust Program ---

// We'll use a simple CPU-intensive program to demonstrate profiling.

fn expensive_computation_a(n: u64) -> u64 {
    (0..n).map(|i| i * i).sum()
}

fn expensive_computation_b(n: u64) -> u64 {
    (0..n).filter(|i| i % 2 == 0).map(|i| i * 3).sum()
}

fn main() {
    println!("Starting CPU-intensive tasks...");

    let result_a = expensive_computation_a(10_000_000);
    println!("Result A: {}", result_a);

    let result_b = expensive_computation_b(20_000_000);
    println!("Result B: {}", result_b);

    println!("Finished.");

    println!("\nTo profile this, compile with debug info and run:");
    println!("cargo build --release");
    println!("perf record -g -- ./target/release/your_program_name");
    println!("perf script | stackcollapse-perf.pl | flamegraph.pl > flamegraph.svg");
    println!("Or simply: cargo flamegraph");
}

// Lesson 15.1: Benchmarking with Criterion.rs

// This lesson introduces `Criterion.rs`, a powerful and statistically sound
// benchmarking library for Rust. Benchmarking is crucial for measuring and
// optimizing the performance of your code.

// --- Why Benchmark? ---

// - **Measure, Don't Guess:** Performance intuitions can be misleading. Benchmarking
//   provides objective data.
// - **Identify Bottlenecks:** Pinpoint the slowest parts of your code.
// - **Track Regressions:** Ensure that new changes don't negatively impact performance.
// - **Compare Implementations:** Evaluate different algorithms or data structures.

// --- `Criterion.rs` ---

// `Criterion.rs` is a robust benchmarking tool that:
// - Runs benchmarks multiple times and performs statistical analysis.
// - Detects performance regressions automatically.
// - Generates detailed reports (HTML, CSV) with graphs.

// --- Setup ---

// To use `Criterion.rs`, you need to add it to your `Cargo.toml`:
//
// [dev-dependencies]
// criterion = { version = "0.4", features = ["html_reports"] }
//
// You also need to configure your `Cargo.toml` to enable benchmarks:
//
// [[bench]]
// name = "my_benchmark"
// harness = false

// --- Example: Benchmarking a Function ---

// This code is typically placed in a separate benchmark file (e.g., `benches/my_benchmark.rs`).

// ```rust
// // benches/my_benchmark.rs
//
// use criterion::{black_box, criterion_group, criterion_main, Criterion};
//
// fn fibonacci_recursive(n: u64) -> u64 {
//     match n {
//         0 => 1,
//         1 => 1,
//         n => fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2),
//     }
// }
//
// fn fibonacci_iterative(n: u64) -> u64 {
//     let mut a = 1;
//     let mut b = 1;
//     for _ in 0..n {
//         let c = a + b;
//         a = b;
//         b = c;
//     }
//     a
// }
//
// fn criterion_benchmark(c: &mut Criterion) {
//     c.bench_function("fib_recursive 20", |b| b.iter(|| fibonacci_recursive(black_box(20))));
//     c.bench_function("fib_iterative 20", |b| b.iter(|| fibonacci_iterative(black_box(20))));
// }
//
// criterion_group!(benches, criterion_benchmark);
// criterion_main!(benches);
// ```

// --- Running Benchmarks ---

// To run the benchmarks, use `cargo bench`.

fn main() {
    println!("This lesson focuses on benchmarking with Criterion.rs.");
    println!("The code for this lesson is conceptual and is meant to be placed");
    println!("in a separate benchmark file and run with `cargo bench`.");
}

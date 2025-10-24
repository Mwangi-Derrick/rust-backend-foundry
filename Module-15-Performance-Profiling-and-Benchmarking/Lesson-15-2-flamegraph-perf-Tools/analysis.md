# Lesson 15.2: flamegraph & perf Tools

## 🧠 Concept Summary

This lesson introduces **profiling tools** like `perf` (Linux) and `flamegraph` for identifying performance bottlenecks in Rust applications. While benchmarking tells you *how fast* your code is, profiling tells you *why*.

- **Profiling:** The process of collecting data about your program's execution, such as CPU usage, memory allocation patterns, and function call times. Profilers help you understand where your program spends its time and resources.

- **`perf` (Linux):** A powerful command-line performance analysis tool built into the Linux kernel. It can collect a wide range of hardware performance counters (e.g., CPU cycles, cache misses, branch predictions) and software events (e.g., system calls).

- **`flamegraph`:** An invaluable visualization tool that takes profiling data (often from `perf`) and generates an interactive SVG image called a **flame graph**. Flame graphs provide a hierarchical and visual representation of a program's call stack, making hot spots immediately apparent.

## ⚙️ Setup

1.  **Install `perf` (Linux only):**
    ```bash
    sudo apt-get install linux-tools-$(uname -r) # Debian/Ubuntu
    # or equivalent for your Linux distribution
    ```
2.  **Install `flamegraph` Rust tool:**
    ```bash
    cargo install flamegraph
    ```

## 🧩 Code Walkthrough

Let's analyze the code in `main.rs` and the profiling process.

### Example: A CPU-intensive Rust Program

```rust
fn expensive_computation_a(n: u64) -> u64 {
    (0..n).map(|i| i * i).sum()
}

fn expensive_computation_b(n: u64) -> u64 {
    (0..n).filter(|i| i % 2 == 0).map(|i| i * 3).sum()
}

fn main() {
    println!("Starting CPU-intensive tasks...");
    let result_a = expensive_computation_a(10_000_000);
    let result_b = expensive_computation_b(20_000_000);
    // ...
}
```

This program defines two functions, `expensive_computation_a` and `expensive_computation_b`, that perform simple but CPU-intensive arithmetic operations. These are good candidates for profiling to see which one consumes more CPU cycles.

### Profiling Workflow

1.  **Compile with debug info (for symbol resolution):**
    ```bash
    cargo build --release
    ```
    While `--release` optimizes for speed, `perf` and `flamegraph` benefit from some debug information (symbol names). Cargo's release profile typically retains enough for basic profiling.

2.  **Record performance data with `perf`:**
    ```bash
    perf record -g -- ./target/release/your_program_name
    # Replace `your_program_name` with the actual name of your executable (from Cargo.toml)
    ```
    - `perf record`: Starts recording performance events.
    - `-g`: Records call graphs (stack traces), essential for flame graphs.
    - `--`: Separates `perf` options from the command to be run.
    - `./target/release/your_program_name`: The path to your compiled Rust executable.

3.  **Generate `flamegraph`:**
    The simplest way is to use the `cargo-flamegraph` tool:
    ```bash
    cargo flamegraph
    ```
    Alternatively, a manual process involves:
    - `perf script > perf.data.txt`: Converts binary `perf.data` to human-readable format.
    - `stackcollapse-perf.pl < perf.data.txt | flamegraph.pl > flamegraph.svg`: Tools from the `flamegraph` project to process `perf` data into an SVG.

    The output `flamegraph.svg` can be opened in any modern web browser. Hovering over parts of the graph will show detailed information about the call stack and time spent.

## ⚔️ Cross-Language Insights

- **Golang:** Go has built-in profiling capabilities (`pprof`). You can generate CPU, memory, blocking, and mutex profiles directly from your running Go applications.

- **TypeScript (Node.js):** Node.js can be profiled using its built-in V8 profiler or tools like `clinic` and Chrome DevTools for flame graphs.

- **C/C++:** Similar to Rust, `perf` (`gprof`, `valgrind --callgrind`) are common profiling tools for C/C++ applications on Linux.

## 🚀 Practical Reflection

- **Visualizing Hot Spots:** Flame graphs are incredibly effective for quickly identifying functions or code paths that consume the most CPU time. Wider bars mean more time spent.

- **Understanding Call Stacks:** The vertical axis of a flame graph shows the call stack. Higher stacks mean deeper calls. By hovering, you can see the exact function names and their contribution.

- **Iterative Optimization:** Profiling is part of an iterative optimization loop: benchmark to detect a slowdown, profile to find the bottleneck, optimize the bottleneck, then re-benchmark to verify the improvement.

## 🧩 Self-Review Prompts

- Implement a known inefficient algorithm (e.g., bubble sort on a large array) and then use `cargo flamegraph` to profile it. Identify the hot spots.
- How does enabling debug info (`-g` in `perf`) impact the clarity of the flame graph?
- Research other `perf` commands (e.g., `perf stat`, `perf top`). How could they be useful for performance analysis?

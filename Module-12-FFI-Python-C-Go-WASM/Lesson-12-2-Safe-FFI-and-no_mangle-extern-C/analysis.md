# Lesson 12.2: Safe FFI and #[no_mangle] extern "C"

## 🧠 Concept Summary

This lesson delves into the **Foreign Function Interface (FFI)** in Rust, focusing on how to safely call C code from Rust and expose Rust functions to C. FFI is essential for interoperability with existing codebases written in other languages.

- **What is FFI?** FFI is a mechanism that allows code written in one programming language to interact with code written in another. In Rust, FFI is primarily used for interoperating with C, as many other languages (like Python and Go) often have C FFI as their lowest common denominator.

- **`unsafe` Rust:** FFI operations are inherently `unsafe` in Rust. This is because the Rust compiler cannot guarantee the safety of code written in other languages. When you use `unsafe` blocks, you are taking responsibility for upholding Rust's safety guarantees.

- **`extern "C"`:** This block is used to declare functions that are defined in C (or another language that uses the C calling convention). It tells Rust that these functions exist externally and how to call them.

- **`#[no_mangle]`:** This attribute prevents the Rust compiler from "mangling" the function name. Function name mangling is a technique used by compilers to encode information about a function's signature into its name, which can make it difficult for other languages to link to it.

- **`pub extern "C"`:** This combination is used to expose a Rust function to C. `pub` makes the function public, and `extern "C"` tells Rust to use the C calling convention and to not mangle the name.

## 🧩 Code Walkthrough

Let's analyze the code in `main.rs`.

### Calling C from Rust

```rust
extern "C" {
    fn abs(input: i32) -> i32;
}

// ... inside main ...
unsafe {
    println!("Absolute value of -3 is: {}", abs(-3));
}
```

We declare the `abs` function within an `extern "C"` block. This tells Rust that `abs` is a C function. When we call `abs(-3)`, we must wrap it in an `unsafe` block because the Rust compiler cannot verify the safety of the C function. We are asserting that calling `abs` with an `i32` is safe.

### Exposing Rust to C

```rust
#[no_mangle]
pub extern "C" fn rust_add(a: i32, b: i32) -> i32 {
    a + b
}
```

This function `rust_add` is designed to be called from C. `#[no_mangle]` ensures that its name is not changed by the Rust compiler, making it easy for a C linker to find it. `pub extern "C"` makes it public and specifies the C calling convention.

### Example: A C Header File

The lesson also provides a conceptual C header file (`my_library.h`) that would be used by a C program to declare the `rust_add` function. This is how C programs would know the signature of the Rust function.

## ⚔️ Cross-Language Insights

- **vs. Golang:** Go also has FFI capabilities through `cgo`, which allows you to call C code from Go and vice versa. `cgo` handles much of the boilerplate, but you still need to be mindful of C types and memory management.

- **vs. TypeScript:** TypeScript (and JavaScript) typically interact with native code through WebAssembly (WASM) or Node.js native add-ons (which are often written in C++).

- **vs. C:** C is the lingua franca of FFI. Many languages provide FFI mechanisms to interoperate with C libraries because of its widespread use and stable ABI.

## 🚀 Practical Reflection

- **`unsafe` is Not Evil:** `unsafe` Rust is not inherently bad. It's a tool that allows you to do things that the compiler cannot guarantee are safe. When you use `unsafe`, you are taking on the responsibility of ensuring correctness.

- **C ABI:** The C Application Binary Interface (ABI) is a standard for how functions are called and how data is laid out in memory. Using `extern "C"` ensures that Rust functions conform to this standard, making them callable from C.

- **Memory Management:** When working with FFI, you need to be very careful about memory management. If you allocate memory in Rust and pass it to C, you need to ensure that C doesn't try to free it, and vice versa. This is a common source of bugs.

## 🧩 Self-Review Prompts

- Compile this Rust code as a static library and then call `rust_add` from a simple C program.
- How would you pass a Rust `String` to a C function that expects a `char*`?
- What are the potential dangers of using `unsafe` Rust in FFI?

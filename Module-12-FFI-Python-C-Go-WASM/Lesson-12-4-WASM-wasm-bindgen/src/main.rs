// Lesson 12.4: WASM & wasm-bindgen

// This lesson explores WebAssembly (WASM) and `wasm-bindgen`, which allow you
// to compile Rust code to WASM and run it in web browsers or other WASM runtimes.

// --- What is WebAssembly? ---

// WebAssembly (WASM) is a binary instruction format for a stack-based virtual
// machine. WASM is designed as a portable compilation target for programming
// languages, enabling deployment on the web for client and server applications.

// Key features of WASM:
// - Fast, efficient, and portable.
// - Safe (sandboxed environment).
// - Open and debuggable.

// --- `wasm-bindgen` ---

// `wasm-bindgen` is a Rust tool that facilitates high-level interactions between
// WASM modules and JavaScript. It allows you to:
// - Call JavaScript functions from Rust.
// - Call Rust functions from JavaScript.
// - Pass complex data structures (like strings, objects, and arrays) between
//   Rust and JavaScript.

// --- Example: A Simple Rust Function for WASM ---

// To demonstrate, we'll create a simple Rust function that greets a name and
// expose it to JavaScript.

// This code is meant to be part of a library crate that is built for WASM.

// ```rust
// // my_wasm_module/src/lib.rs
//
// use wasm_bindgen::prelude::*;
//
// #[wasm_bindgen]
// extern {
//     fn alert(s: &str);
// }
//
// #[wasm_bindgen]
// pub fn greet(name: &str) {
//     alert(&format!("Hello, {}!", name));
// }
// ```

// --- Building with `wasm-pack` ---

// `wasm-pack` is a tool that helps you build Rust-generated WebAssembly packages.
// It handles compiling your Rust code to WASM, running `wasm-bindgen`,
// and generating the necessary JavaScript glue code.

// 1. Create a new library crate: `cargo new my_wasm_module --lib`
// 2. Add `wasm-bindgen` as a dependency in `my_wasm_module/Cargo.toml`:
//
//    ```toml
//    [dependencies]
//    wasm-bindgen = "0.2"
//    ```
//
// 3. Place the Rust code above into `my_wasm_module/src/lib.rs`.
// 4. Build the WASM package: `wasm-pack build` (from the `my_wasm_module` directory).
//    This will create a `pkg` directory with the WASM module and JavaScript glue code.

// --- Using in JavaScript ---

// ```javascript
// // index.js
//
// import * as wasm from "./pkg/my_wasm_module.js";
//
// wasm.greet("World");
// ```

fn main() {
    println!("This lesson is about compiling Rust to WASM.");
    println!("The code for this lesson is conceptual and requires setting up a separate");
    println!("Rust library crate and building it with `wasm-pack`.");
}

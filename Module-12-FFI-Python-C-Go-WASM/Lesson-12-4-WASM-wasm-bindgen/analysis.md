# Lesson 12.4: WASM & wasm-bindgen

## 🧠 Concept Summary

This lesson explores **WebAssembly (WASM)** and **`wasm-bindgen`**, which together enable you to compile Rust code to WASM and run it in web browsers or other WASM runtimes. This is a powerful way to bring Rust's performance and safety to the web.

- **WebAssembly (WASM):** A binary instruction format for a stack-based virtual machine. It's designed as a portable compilation target for programming languages, enabling high-performance execution in web browsers and other environments. WASM is fast, efficient, portable, and safe (sandboxed).

- **`wasm-bindgen`:** A Rust tool that facilitates high-level interactions between WASM modules and JavaScript. It automatically generates the necessary JavaScript glue code and Rust bindings to:
    - Call JavaScript functions from Rust.
    - Call Rust functions from JavaScript.
    - Pass complex data structures (like strings, objects, and arrays) between Rust and JavaScript.

- **`wasm-pack`:** A command-line tool that streamlines the entire process of building Rust-generated WebAssembly packages. It handles compiling Rust to WASM, running `wasm-bindgen`, and generating the final package structure.

## 🧩 Code Walkthrough

The code for this lesson is conceptual and requires setting up a separate Rust library crate and building it with `wasm-pack`.

### Rust Code (`my_wasm_module/src/lib.rs`)

```rust
// my_wasm_module/src/lib.rs

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}
```

- **`use wasm_bindgen::prelude::*`:** Imports necessary `wasm-bindgen` macros and types.
- **`#[wasm_bindgen] extern { ... }`:** This block declares JavaScript functions that you want to call from Rust. Here, we declare the `alert` function.
- **`#[wasm_bindgen] pub fn greet(...)`:** This attribute macro exposes the Rust `greet` function to JavaScript. `wasm-bindgen` handles the conversion of the Rust `&str` to a JavaScript string.

### Building with `wasm-pack`

1.  **Create a new library crate:** `cargo new my_wasm_module --lib`
2.  **Add `wasm-bindgen` dependency:** Modify `my_wasm_module/Cargo.toml`:
    ```toml
    [dependencies]
    wasm-bindgen = "0.2"
    ```
3.  **Place Rust code:** Put the `lib.rs` content into `my_wasm_module/src/lib.rs`.
4.  **Build WASM package:** Navigate to the `my_wasm_module` directory and run `wasm-pack build`. This creates a `pkg` directory containing the WASM module (`my_wasm_module_bg.wasm`) and the generated JavaScript glue code (`my_wasm_module.js`).

### Using in JavaScript (`index.js`)

```javascript
import * as wasm from "./pkg/my_wasm_module.js";

wasm.greet("World");
```

This JavaScript code imports the generated module and calls the `greet` function, which in turn calls the JavaScript `alert` function from Rust.

## ⚔️ Cross-Language Insights

- **vs. Golang:** Go can also compile to WASM, but its WASM output is typically larger due to the Go runtime being included. Rust's WASM output is often much smaller and more optimized.

- **vs. TypeScript:** TypeScript (and JavaScript) are the native languages of the web. WASM allows other languages like Rust to achieve near-native performance in the browser, often outperforming JavaScript for CPU-intensive tasks.

- **Performance:** WASM provides a way to run code at near-native speeds in the browser, which is a significant advantage for computationally intensive web applications.

## 🚀 Practical Reflection

- **Web Development:** WASM opens up new possibilities for web development, allowing developers to write performance-critical parts of web applications in Rust.

- **Beyond the Browser:** WASM is not just for the browser. It can also be run in other environments, such as serverless functions (e.g., Cloudflare Workers) or desktop applications.

- **Tooling:** The `wasm-pack` and `wasm-bindgen` tools make the process of compiling Rust to WASM and integrating it with JavaScript surprisingly smooth.

## 🧩 Self-Review Prompts

- Create a Rust function that takes two numbers, adds them, and returns the result to JavaScript. Expose it using `wasm-bindgen`.
- How would you pass a JavaScript object to a Rust function and then return a modified object?
- Explore how to use `console.log` from Rust in a WASM module.

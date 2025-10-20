# Lesson 12: Modularizing Rust Projects

## 🧠 Concept Summary
As a project grows, putting all the code in `main.rs` becomes unmanageable. Rust has a powerful **module system** for splitting code across multiple files and directories, creating a clean hierarchy and clear namespaces.

The core concepts are:

-   **Crate Root**: Every project has a root file, either `main.rs` (for a binary) or `lib.rs` (for a library). This is the entry point for the module tree.
-   **`mod` Keyword**: The `mod` keyword is used to declare a module. The compiler will then look for the module's code in either `<module_name>.rs` or `<module_name>/mod.rs`.
-   **Filesystem Mapping**: The module hierarchy directly maps to the file and directory structure.
-   **Visibility (`pub`)**: By default, all items in a module (functions, structs, other modules) are private. You must use the `pub` keyword to make them accessible from outside the module.
-   **Paths**: You access items in other modules using a path, like `services::upload::start_upload()`.

## 🧩 Code Walkthrough
This lesson is split across three files, demonstrating how to organize a `services` module.

### `src/main.rs` (The Crate Root)
This file is the root of the binary crate and orchestrates the module structure.
```rust
// This declares a module named 'services'.
// Because it has a body (`{}`), the compiler expects submodule declarations inside.
mod services {
    // These lines tell Rust to look for the code for modules 'upload' and 'payment'
    // inside the 'services' directory.
    // It will find 'services/upload.rs' and 'services/payment.rs'.
    // The 'pub' keyword makes these submodules accessible from outside the 'services' module.
    pub mod upload;
    pub mod payment;
}

fn main() {
    println!("🚀 Microservice starting...");

    // We call the functions using their full path from the crate root.
    services::upload::start_upload();
    services::payment::process_payment();

    println!("✅ All services operational!");
}
```

### `services/upload.rs` (A Submodule)
This file contains the code for the `services::upload` module.
```rust
// This function is part of the 'services::upload' module.
// It must be marked 'pub' to be visible and callable from 'main.rs'.
// If it weren't public, calling it from main() would be a compile-time error.
pub fn start_upload() {
    println!("📤 Upload service started...");
}
```

### `services/payment.rs` (Another Submodule)
This file contains the code for the `services::payment` module.
```rust
// Similar to the upload service, this function must be public
// to be accessible from outside its module.
pub fn process_payment() {
    println!("💳 Payment processed successfully!");
}
```

## ⚔️ Cross-Language Insights
-   **Golang Equivalent:**
    -   Go uses a package system. All files within a single directory belong to the same package. You use the `import` keyword to use code from other packages. Rust's module system is more explicit and hierarchical, as you must declare the module tree using `mod` statements starting from the crate root.
-   **TypeScript (Node.js) Equivalent:**
    -   This is similar to ES Modules, where each file is its own module. You `export` items from one file and `import` them in another. The key difference is that Rust requires you to build the module tree explicitly from the crate root using `mod` declarations. In TypeScript, you can import any file from any other file directly, which can sometimes lead to complex or circular dependency graphs.
-   **C Reference:**
    -   The C preprocessor's `#include <file.h>` directive is a primitive text-inclusion mechanism. It does not create namespaces or a module hierarchy, which is why C code often uses long, prefixed function names to avoid collisions (e.g., `my_library_widget_init()`). Rust's module system is a core, language-level feature that provides robust namespacing and encapsulation, making it far superior for organizing large projects.

## 🚀 Practical Reflection
This is not just a pattern; it's **the** way to structure any non-trivial Rust application. For your outbox-relay microservice, you should immediately start thinking in modules:

-   `src/main.rs`: The entry point. Sets up logging, configuration, and starts the main application loop.
-   `src/database.rs`: Contains all the logic for interacting with your database (e.g., `get_unsent_messages`, `mark_message_as_sent`).
-   `src/publisher.rs`: Contains the logic for publishing messages to your message broker (e.g., RabbitMQ, Kafka).
-   `src/config.rs`: Logic for loading and parsing configuration from a file or environment variables.
-   `src/error.rs`: Definition of your custom `AppError` enum.

Your `main.rs` would then look something like this:
```rust
mod database;
mod publisher;
mod config;
mod error;

fn main() {
    let config = config::load();
    // ... and so on
}
```
This separation of concerns is critical for building maintainable, testable, and scalable systems.

## 🧩 Self-Review Prompts
-   The example uses full paths like `services::upload::start_upload()`. What does the `use` keyword do, and how could you use it in `main.rs` to shorten that function call?
-   What is the difference between `mod services;` and `mod services { ... }`?
-   If you had a `services/utils.rs` file, how would you make its functions available to both `services/upload.rs` and `services/payment.rs`? (Hint: think about `pub` and `super`).
-   What is the purpose of a `lib.rs` file, and how does it differ from `main.rs`?

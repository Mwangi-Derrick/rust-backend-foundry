# Lesson 09.2: Creating Library Crates

## 🧠 Concept Summary

This lesson explains how to create and use **library crates**. A crate is a compilation unit in Rust. A crate can be a binary crate (which creates an executable) or a library crate (which creates a library that can be used by other crates).

- **Library Crate:** A library crate is a collection of code that can be used by other crates. It does not have a `main` function and cannot be run on its own.

- **Creating a Library Crate:** You can create a library crate with `cargo new my_library --lib`. This will create a new directory named `my_library` with a `src/lib.rs` file. The `src/lib.rs` file is the root of the library crate.

- **Using a Library Crate:** To use a library crate in another crate, you need to add it as a dependency in the `Cargo.toml` file of the other crate. You can then use the `use` keyword to bring the library's functions and other items into scope.

## 🧩 Code Walkthrough

The code for this lesson is conceptual and is meant to be run by creating a separate library crate and a binary crate that uses it.

### Creating a Library Crate

1.  Run `cargo new my_library --lib` to create a new library crate.
2.  Add a public function to `src/lib.rs`:

    ```rust
    // my_library/src/lib.rs

    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }
    ```

### Using the Library Crate

1.  Create a new binary crate: `cargo new my_app`.
2.  Add a path dependency to `my_library` in `my_app/Cargo.toml`:

    ```toml
    [dependencies]
    my_library = { path = "../my_library" }
    ```

3.  Use the `add` function from `my_library` in `my_app/src/main.rs`:

    ```rust
    // my_app/src/main.rs

    use my_library::add;

    fn main() {
        let sum = add(2, 3);
        println!("The sum is: {}", sum);
    }
    ```

4.  Run the binary crate: `cargo run` from within the `my_app` directory.

## ⚔️ Cross-Language Insights

- **vs. Golang:** A library crate is similar to a Go package that is not the `main` package.

- **vs. TypeScript:** A library crate is similar to a TypeScript project that is compiled to a library (e.g., a set of `.js` and `.d.ts` files) and then published to npm.

- **vs. C:** A library crate is similar to a C library (a `.a` or `.so` file) and its associated header files.

## 🚀 Practical Reflection

- **Code Reuse:** Library crates are the primary way to reuse code in Rust. You can publish your library crates to `crates.io` to share them with the Rust community.

- **Public API:** The public API of a library crate is the set of all of its public items (functions, structs, enums, traits, etc.). It is important to design a clear and stable public API for your library.

- **Semantic Versioning:** When you publish a library crate, you should follow the principles of semantic versioning to communicate the nature of your changes to your users.

## 🧩 Self-Review Prompts

- Create a library crate that has a public struct with a public method.
- How would you add a dependency on a library from `crates.io`?
- What is the difference between a `[dependencies]` section and a `[dev-dependencies]` section in `Cargo.toml`?

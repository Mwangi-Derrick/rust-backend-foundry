// Lesson 09.2: Creating Library Crates

// A crate is a compilation unit in Rust. A crate can be a binary crate (which
// creates an executable) or a library crate (which creates a library that can
// be used by other crates).

// --- Creating a Library Crate ---

// To create a library crate, you can use the `--lib` flag with `cargo new`:
//
// cargo new my_library --lib
//
// This will create a new directory named `my_library` with a `src/lib.rs` file.
// The `src/lib.rs` file is the root of the library crate.

// --- A Library Crate Example ---

// Let's say we have a library crate named `my_library` with the following
// `src/lib.rs` file:

// ```rust
// // src/lib.rs
//
// pub fn add(a: i32, b: i32) -> i32 {
//     a + b
// }
// ```

// --- Using a Library Crate ---

// To use a library crate in another crate, you need to add it as a dependency
// in the `Cargo.toml` file of the other crate.

// For a local library crate, you can use a path dependency:
//
// [dependencies]
// my_library = { path = "../my_library" }

// Then, you can use the `use` keyword to bring the library's functions into scope.

// ```rust
// // src/main.rs
//
// use my_library::add;
//
// fn main() {
//     let sum = add(2, 3);
//     println!("The sum is: {}", sum);
// }
// ```

fn main() {
    println!("This lesson is about creating library crates.");
    println!("The code for this lesson is conceptual and is meant to be run");
    println!("by creating a separate library crate and a binary crate that uses it.");
}

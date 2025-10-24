# Lesson 09.1: Mod, Pub, and use

## 🧠 Concept Summary

This lesson introduces Rust's **module system**, which allows you to organize your code into a hierarchy of modules. This is key for managing complexity in large projects.

- **`mod`:** The `mod` keyword is used to define a new module. A module is a namespace that can contain definitions of functions, structs, enums, traits, and other modules.

- **Privacy:** By default, everything in a module is private. This means it can only be accessed from within the same module.

- **`pub`:** The `pub` keyword makes an item public, which means it can be accessed from outside the module.

- **Paths:** You can refer to an item in a module using a path. A path can be absolute (starting from the crate root with `crate::`) or relative (starting from the current module).

- **`use`:** The `use` keyword brings a path into scope, so you can refer to it with a shorter name.

- **`pub use`:** The `pub use` keyword re-exports an item, which means that code that can access your module can also access the re-exported item.

- **File System:** You can separate modules into different files. A module named `foo` can be defined in a file named `foo.rs` or `foo/mod.rs`.

## 🧩 Code Walkthrough

Let's analyze the code in `main.rs`.

### Modules and Privacy

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        // ...
    }
}
```

We define a `front_of_house` module with two submodules, `hosting` and `serving`. The `hosting` module is `pub`, so it can be accessed from outside `front_of_house`. The `add_to_waitlist` function is also `pub`, so it can be accessed from outside `hosting`. The `seat_at_table` function is private, so it can only be accessed from within the `hosting` module.

### Paths and `use`

```rust
use crate::front_of_house::hosting;

fn main() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // With `use`
    hosting::add_to_waitlist();
}
```

The `use` keyword brings the `hosting` module into scope, so we can refer to it with a shorter name. It is idiomatic to bring the module into scope, rather than the function itself.

### Separating Modules into Files

The lesson also explains how to separate modules into different files. If you have a `mod front_of_house;` declaration in your `main.rs` or `lib.rs`, the compiler will look for the module's code in `front_of_house.rs` or `front_of_house/mod.rs`.

## ⚔️ Cross-Language Insights

- **vs. Golang:** Go has packages, which are similar to Rust's modules. A package is a directory of Go files. An item is exported from a package if its name starts with a capital letter.

- **vs. TypeScript:** TypeScript has modules, which are files. You can export items from a module with the `export` keyword and import them into another module with the `import` keyword.

- **vs. C:** C does not have a module system. You would typically use header files to declare the public API of a library.

## 🚀 Practical Reflection

- **Encapsulation:** The module system allows you to encapsulate implementation details. By making items private by default, you can create a clear public API for your code and hide the internal details.

- **Code Organization:** Modules are a key tool for organizing your code in a large project. You can group related functionality into modules, which makes the code easier to navigate and understand.

- **The Crate Root:** The crate root is the top-level module of your crate. For a binary crate, it is `main.rs`. For a library crate, it is `lib.rs`.

## 🧩 Self-Review Prompts

- Create a new module and move some of the functions from `main` into it.
- What is the difference between `use` and `pub use`?
- How would you create a module that has a public struct with a private field?

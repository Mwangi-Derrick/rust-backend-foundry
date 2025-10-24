# Lesson 04.4: Blanket Implementations and Default Methods

## 🧠 Concept Summary

This lesson explores two powerful features of traits that promote code reuse: **default methods** and **blanket implementations**.

- **Default Methods:** A trait can provide a default implementation for its methods. A type that implements the trait can then either use the default implementation or provide its own. This is useful for reducing the amount of code that needs to be written for each implementation of a trait.

- **Blanket Implementations:** A blanket implementation is an implementation of a trait for any type that satisfies a certain trait bound. This is a powerful way to add functionality to many types at once.

## 🧩 Code Walkthrough

Let's analyze the code in `main.rs`.

### Default Methods

```rust
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct Tweet { ... }

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}
```

The `Summary` trait has two methods. `summarize_author` must be implemented by any type that implements the trait. `summarize` has a default implementation that calls `summarize_author`. The `Tweet` struct only needs to implement `summarize_author`, and it gets `summarize` for free.

### Blanket Implementations

```rust
// impl<T: Display> ToString for T {
//     // ...
// }

use std::fmt::Display;

struct Point {
    x: i32,
    y: i32,
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

let p = Point { x: 1, y: 2 };
println!("The point is: {}", p.to_string());
```

The standard library provides a blanket implementation of the `ToString` trait for any type that implements the `Display` trait. This means that once we implement `Display` for our `Point` struct, we can call `to_string()` on it without having to write any more code.

## ⚔️ Cross-Language Insights

- **vs. Golang:** Go does not have default methods or blanket implementations. You would have to write the methods for each type that implements an interface.

- **vs. TypeScript:** TypeScript does not have a direct equivalent to blanket implementations. You could use mixins or higher-order functions to achieve a similar effect.

- **vs. C:** C does not have these features.

## 🚀 Practical Reflection

- **Code Reusability:** Default methods and blanket implementations are powerful tools for code reuse. They allow you to write code once and have it work for many different types.

- **Extending Types:** Blanket implementations are a way to extend the functionality of types, even types that are not defined in your own crate. This is a key part of what makes the Rust ecosystem so powerful.

- **The `Display` and `ToString` Traits:** The `Display` and `ToString` traits are a good example of how blanket implementations can be used to provide a consistent and convenient API.

## 🧩 Self-Review Prompts

- Create a trait with a default method, and then create a struct that implements the trait and overrides the default method.
- Can you have a blanket implementation of a trait for all types? Why or why not?
- Look at the documentation for the `Iterator` trait. How does it use default methods?

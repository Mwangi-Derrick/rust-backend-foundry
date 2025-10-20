# Rust Foundations: A Reference

## 🧭 Overview

This document provides a quick reference for the foundational concepts of the Rust language that the main 14-lesson curriculum assumes you can learn from context. Use this as a "Lesson 0" or a quick refresher.

---

## 1. Primitive Data Types

Rust is a statically typed language, which means it must know the types of all variables at compile time. 

### Scalar Types

Scalar types represent a single value.

-   **Integer Types:** Used for whole numbers. They come in signed (`i`) and unsigned (`u`) variants of different sizes. `isize` and `usize` depend on the architecture of the computer (64-bit on a 64-bit arch).
    -   `i8`, `i16`, `i32`, `i64`, `i128`, `isize`
    -   `u8`, `u16`, `u32`, `u64`, `u128`, `usize`
    -   *Example:* `let x: i32 = -5; let y: u64 = 1_000_000;`

-   **Floating-Point Types:** Numbers with decimal points.
    -   `f32` (single-precision), `f64` (double-precision, the default).
    -   *Example:* `let pi: f64 = 3.14159;`

-   **Boolean Type:** A simple `true` or `false` value.
    -   `bool`
    -   *Example:* `let is_learning_rust: bool = true;`

-   **Character Type:** Represents a single Unicode Scalar Value. This means it can represent a lot more than just ASCII.
    -   `char` (specified with single quotes).
    -   *Example:* `let initial: char = 'D'; let emoji: char = '🦀';`

### Compound Types

Compound types can group multiple values into one type.

-   **The Tuple:** A fixed-size, ordered list of values of *different* types. Once declared, a tuple's size cannot change.
    -   *Example:* `let my_info: (String, i32) = ("Derrick".to_string(), 30);`
    -   Values are accessed by index: `my_info.0`, `my_info.1`.

-   **The Array:** A fixed-size list of values of the *same* type. Stored on the stack.
    -   *Example:* `let error_codes: [i32; 3] = [404, 500, 502];`
    -   Accessed by index: `error_codes[0]`.
    -   *Note:* A `Vec<T>` (Vector) is a similar, growable list provided by the standard library, but it is stored on the heap.

---

## 2. Functions

Functions are the primary way to execute code in Rust.

-   **Syntax:** `fn function_name(parameter_name: Type) -> ReturnType { ... }`
-   **Parameters:** Inputs to the function, with their types explicitly declared.
-   **Return Values:** The `-> ReturnType` syntax declares the type of the value the function will return.

### Statements vs. Expressions

This is a critical concept in Rust.

-   **Statements** are instructions that perform some action but do not return a value. Most `let` bindings are statements. They end in a semicolon.
    -   *Example:* `let x = 5;`

-   **Expressions** evaluate to a resulting value. An `if` block, a `match` block, or a simple math operation are all expressions. **Crucially, if you omit the semicolon at the end of a line, it becomes an expression.**

-   **Function Return:** The value of the final expression in a function body is automatically returned. You can use the `return` keyword for an early return, but idiomatic Rust often relies on the final expression.

```rust
fn add_five(x: i32) -> i32 {
    // No semicolon here, so this is an expression whose value is returned.
    x + 5 
}
```

---

## 3. Control Flow

-   **`if` Expressions:** In Rust, `if` is an expression, meaning it evaluates to a value. This allows you to use it in `let` statements.
    ```rust
    let number = 5;
    let result = if number > 0 { "positive" } else { "not positive" };
    ```

-   **Loops:** Rust has three kinds of loops.
    -   `loop`: An infinite loop that you must explicitly `break` out of. Can return a value from the break point.
    -   `while`: A standard conditional loop that runs as long as a condition is `true`.
    -   `for`: The most common and safest loop. It iterates over an iterator. For example, `for i in 0..10 { ... }` or `for item in my_vector { ... }`. It handles all the bounds checking for you.

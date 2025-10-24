# Lesson 03.4: Option<T> and Result<T, E> Patterns

## 🧠 Concept Summary

This lesson introduces two of the most important enums in the Rust standard library: `Option<T>` and `Result<T, E>`. These are the primary tools that Rust uses for handling optional values and recoverable errors.

- **`Option<T>`:** This enum is for values that can be something or nothing. It has two variants:
    - `Some(T)`: A value of type `T` is present.
    - `None`: No value is present.
    This enum is designed to prevent null pointer errors. The compiler forces you to handle the `None` case, so you can't accidentally use a null value.

- **`Result<T, E>`:** This enum is for operations that can succeed or fail. It has two variants:
    - `Ok(T)`: The operation was successful, and a value of type `T` is returned.
    - `Err(E)`: The operation failed, and an error of type `E` is returned.
    This enum forces you to handle errors, making your code more robust.

- **The `?` Operator:** This is a convenient shorthand for propagating errors. If a `Result` is `Ok`, the `?` operator will unwrap it and give you the value inside. If the `Result` is `Err`, the `?` operator will return the `Err` from the current function.

## 🧩 Code Walkthrough

Let's analyze the code in `main.rs`.

### `Option<T>`

```rust
let some_number = Some(5);
let absent_number: Option<i32> = None;
```

This shows the two variants of `Option`. `some_number` contains a value, while `absent_number` does not. The type system prevents you from using an `Option<i32>` as if it were an `i32`, which forces you to handle the `None` case.

### `Result<T, E>`

```rust
use std::fs::File;

let f = File::open("hello.txt");

let f = match f {
    Ok(file) => file,
    Err(error) => {
        panic!("There was a problem opening the file: {:?}", error)
    }
};
```

The `File::open` function returns a `Result<File, std::io::Error>`. If the file is opened successfully, it returns an `Ok` containing the file handle. If there is an error, it returns an `Err` containing information about the error. The `match` statement is used to handle both cases.

### The `?` Operator

```rust
fn read_username_from_file() -> Result<String, std::io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
```

This function shows the power of the `?` operator. The `?` after `File::open("hello.txt")` is equivalent to a `match` statement that returns the `Err` if there is one, or unwraps the `Ok` if there isn't. This makes error handling much more concise.

## ⚔️ Cross-Language Insights

- **vs. Golang:** Go functions often return a value and an error (`(value, error)`). This is very similar to Rust's `Result`. Go does not have an equivalent to `Option`; you would typically use a nil pointer to represent the absence of a value.

- **vs. TypeScript:** In TypeScript, you might use `null` or `undefined` to represent the absence of a value, which is similar to `Option`. For errors, you would typically use `try`/`catch` blocks and `throw` statements.

- **vs. C:** In C, you might return a special value (like -1 or a null pointer) to indicate an error. This is less explicit than `Result` and can be a source of bugs if the caller forgets to check the return value.

## 🚀 Practical Reflection

- **No More Null Pointer Errors:** `Option` is a game-changer if you are coming from a language with null pointers. The compiler will force you to handle the `None` case, which eliminates a whole class of bugs.

- **Robust Error Handling:** `Result` makes error handling a first-class citizen in Rust. You can't ignore an error; you have to handle it. The `?` operator makes this easy and ergonomic.

- **The Happy Path:** The `?` operator allows you to write code that focuses on the "happy path" (the case where everything works), while still correctly handling errors.

## 🧩 Self-Review Prompts

- Write a function that takes a slice of integers and returns the largest one in an `Option`. Why should it return an `Option`?
- Rewrite the `read_username_from_file` function without using the `?` operator.
- What is the difference between `panic!` and returning a `Result`? When would you choose one over the other?

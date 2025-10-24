# Lesson 05.4: Handling Panics and Recoverability

## 🧠 Concept Summary

This lesson covers **panics**, which are Rust's mechanism for handling unrecoverable errors. We also look at how to catch panics and discuss when it is appropriate to panic versus returning a `Result`.

- **`panic!` Macro:** The `panic!` macro is used to cause the current thread to panic. When a thread panics, it unwinds the stack, cleaning up its resources. If the main thread panics, the program exits.

- **When to Panic:** `panic!` should be used for unrecoverable errors, where the program is in a state where it is impossible to continue. This is typically for programming errors, not expected runtime errors.

- **Catching Panics:** You can catch a panic using `std::panic::catch_unwind`. This is useful in specific scenarios, like when interfacing with foreign code or in long-running applications where you don't want a single failure to bring down the whole system.

- **`Result` vs. `panic!`:**
    - Use `Result` for expected, recoverable errors (e.g., a file not being found).
    - Use `panic!` for unexpected, unrecoverable errors (e.g., an index being out of bounds when you thought you had already checked it).

- **`unwrap()` and `expect()`:** These are convenient methods on `Result` and `Option` that will panic if the value is not what you expect it to be. `expect()` is generally preferred because it allows you to provide a custom error message.

## 🧩 Code Walkthrough

Let's analyze the code in `main.rs`.

### A Simple Panic

```rust
// panic!("A simple panic!");
```

This is the simplest way to cause a panic. It will print the given message and unwind the stack.

### Catching a Panic

```rust
let result = panic::catch_unwind(|| {
    // ...
});

match result {
    Ok(_) => println!("The code did not panic."),
    Err(_) => println!("The code panicked!"),
}
```

The `catch_unwind` function takes a closure and executes it. If the closure panics, `catch_unwind` will return an `Err` containing the cause of the panic. If the closure does not panic, it will return an `Ok` containing the value returned by the closure.

### `unwrap()` and `expect()`

```rust
let some_value: Option<i32> = None;
// let value = some_value.unwrap(); // This will panic
// let value = some_value.expect("The value should be Some"); // This will panic with a message
```

`unwrap()` and `expect()` are shortcuts for when you are sure that a `Result` is `Ok` or an `Option` is `Some`. If you are wrong, they will panic. `expect()` is generally preferred because it allows you to provide a more informative error message.

## ⚔️ Cross-Language Insights

- **vs. Golang:** Go does not have panics in the same way as Rust. It has a `panic` function, but it is generally used for unrecoverable errors in the runtime itself. Go programs are encouraged to use the `error` return value for all error handling.

- **vs. TypeScript:** In TypeScript, you would use `throw` to raise an exception, which is similar to a panic. You can catch exceptions with `try`/`catch` blocks.

- **vs. C:** C does not have a panic mechanism. You would typically use `assert` to check for programming errors, which will abort the program if the assertion fails.

## 🚀 Practical Reflection

- **Don't Abuse Panics:** Panics should be used sparingly. In most cases, it is better to return a `Result` and let the caller decide how to handle the error.

- **Prototypes and Simple Code:** `unwrap()` and `expect()` are very useful when you are writing a quick prototype or a simple program where you don't want to write a lot of error handling code. However, in production code, you should be careful about using them.

- **Robust Servers:** `catch_unwind` is a key tool for building robust servers. If you have a server that is handling multiple requests, you don't want a panic in one request handler to crash the entire server. You can use `catch_unwind` to catch the panic and return an error to the client, while the server continues to run.

## 🧩 Self-Review Prompts

- Write a function that divides two numbers and returns a `Result`. When should it return an `Err`?
- When is it appropriate to use `unwrap()` in production code?
- Look at the documentation for `catch_unwind`. What is the `UnwindSafe` trait?

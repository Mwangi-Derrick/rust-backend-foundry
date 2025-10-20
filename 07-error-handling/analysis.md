# Lesson 7: Error Handling (Result, Option & the ? Operator)

## 🧠 Concept Summary
Rust handles errors in a robust and explicit way, primarily using two special `enum` types provided by the standard library. This approach avoids exceptions (like in Java/C#/TypeScript) and the potential for null pointer errors (like in C/Java/Go).

-   **`Option<T>`**: Represents a value that can be either present or absent.
    -   `Some(T)`: A value `T` is present.
    -   `None`: No value is present.
    This enum is used for functions that might not return a value, forcing you to handle the "nothing" case at compile time.

-   **`Result<T, E>`**: Represents a value that can be either a success or a failure.
    -   `Ok(T)`: The operation succeeded, containing a value of type `T`.
    -   `Err(E)`: The operation failed, containing an error of type `E`.
    This is the primary tool for any function that can fail (e.g., I/O, network requests).

-   **The `?` Operator**: This is syntactic sugar for propagating errors. When placed after an expression that returns a `Result`, it automatically handles the `Err` case by returning it from the current function. This keeps the "happy path" code clean and readable.

## 🧩 Code Walkthrough

### Example 1: `Option<T>` (Commented Out)
This example shows how to represent a value that might be missing.
```rust
// fn find_user(id: u32) -> Option<&'static str> {
//     if id == 1 {
//         Some("derrick")
//     } else {
//         None
//     }
// }
//
// fn main() {
//     match find_user(1) {
//         Some(name) => println!("User found: {}", name),
//         None => println!("User not found"),
//     }
// }
```
-   The `find_user` function returns an `Option`. It never returns a `null` or `undefined` value. It returns a `Some` containing the data, or `None`.
-   The `match` statement in `main` forces the caller to handle both possibilities. The compiler will not let you forget to handle the `None` case, thus preventing null reference errors.

### Example 2: `Result<T, E>` and `?` (Commented Out)
This example simulates a failable operation, like reading a file.
```rust
// use std::fs::File;
// use std::io::{self, Read};
//
// fn read_config() -> Result<String, io::Error> {
//     // If File::open fails, the 'Err' is returned from read_config immediately.
//     let mut file = File::open("config.txt")?;
//     let mut contents = String::new();
//     // If read_to_string fails, the 'Err' is returned from read_config immediately.
//     file.read_to_string(&mut contents)?;
//     // If both operations succeed, wrap the result in 'Ok'.
//     Ok(contents)
// }
//
// fn main() {
//     match read_config() {
//         Ok(c) => println!("Config loaded:\n{}", c),
//         Err(e) => println!("Error reading config: {}", e),
//     }
// }
```
-   `read_config` returns a `Result<String, io::Error>`, making it clear that it can either succeed with a `String` or fail with an `io::Error`.
-   The `?` operator is a shortcut. The line `let mut file = File::open("config.txt")?;` is equivalent to:
    ```rust
    let mut file = match File::open("config.txt") {
        Ok(f) => f,
        Err(e) => return Err(e.into()), // Note the .into() for type conversion
    };
    ```
-   This keeps the main logic clean by handling the error path implicitly.

### Example 3: Mini Challenge (Corrected)
The challenge was to create a `divide` function. The provided solution had a confusing print statement in the `Ok` arm, which is corrected here for clarity.
```rust
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        // If divisor is zero, return an Err variant with an error message.
        Err("Cannot divide by zero".into())
    } else {
        // Otherwise, return an Ok variant with the result.
        Ok(a / b)
    }
}

fn main() {
    // Test case 1: Success
    println!("Testing 20.0 / 2.0");
    match divide(20.0, 2.0) {
        Ok(result) => println!("Success! Result: {}", result), // Corrected message
        Err(error_msg) => println!("Error: {}", error_msg),
    }

    println!("\nTesting 10.0 / 0.0");
    // Test case 2: Failure
    match divide(10.0, 0.0) {
        Ok(result) => println!("Success! Result: {}", result),
        Err(error_msg) => println!("Error: {}", error_msg), // Corrected to use the message from the Err
    }
}
```

## ⚔️ Cross-Language Insights
-   **Golang Equivalent:**
    -   Go's idiomatic error handling is to return a tuple `(value, error)`. This is conceptually very similar to Rust's `Result`.
        ```go
        func divide(a, b float64) (float64, error) {
            if b == 0.0 {
                return 0, errors.New("cannot divide by zero")
            }
            return a / b, nil
        }
        ```
    -   The common Go pattern `if err != nil { return 0, err }` is the manual equivalent of Rust's `?` operator.
-   **TypeScript Equivalent:**
    -   Asynchronous operations using `Promise`s are the closest analogy. A `Promise` can either be resolved (like `Ok`) or rejected (like `Err`).
        ```typescript
        async function divide(a: number, b: number): Promise<number> {
            if (b === 0) {
                throw new Error("Cannot divide by zero");
            }
            return a / b;
        }
        // Caller uses .then().catch() or async/await with try/catch
        ```
-   **C Reference:**
    -   C has no built-in error handling mechanism. Programmers rely on conventions, such as returning `-1` or `NULL` on error and setting a global `errno` variable. This is far less safe, as there is no guarantee that a caller will check the return value or `errno`. Rust's `Result` makes the possibility of an error an explicit part of the type system that the compiler forces you to handle.

## 🚀 Practical Reflection
For a robust backend service like an outbox relay, explicit error handling is non-negotiable. Every interaction with the outside world—database queries, network calls to other services, reading from a message queue—is a failable operation. By wrapping these operations in functions that return a `Result`, you force the service's logic to consciously handle failure scenarios (e.g., retry the operation, mark the message as failed, log the error). The `?` operator is invaluable here, as it allows you to compose multiple failable operations together cleanly while ensuring that any failure along the way stops the process and bubbles up the error.

## 🧩 Self-Review Prompts
-   What is the difference between `panic!` and returning an `Err`? When would you choose one over the other?
-   `Result` and `Option` have many useful methods like `map`, `and_then`, `unwrap_or`, and `expect`. Pick one and explain what it does.
-   The `?` operator can convert between different error types if the `From` trait is implemented. Why is this useful?
-   How would you define your own custom error `enum` to represent the different things that could go wrong in your outbox relay service?

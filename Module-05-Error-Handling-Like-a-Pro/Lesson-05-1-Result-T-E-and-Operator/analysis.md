# Lesson 05.1: Result<T, E> and the ? Operator

## 🧠 Concept Summary

This lesson revisits `Result<T, E>` and the `?` operator, which are the cornerstones of idiomatic error handling in Rust.

- **`Result<T, E>`:** An enum for operations that can succeed or fail. It has two variants:
    - `Ok(T)`: The operation was successful, and a value of type `T` is returned.
    - `Err(E)`: The operation failed, and an error of type `E` is returned.

- **The `?` Operator:** A powerful shorthand for propagating errors. When used on a `Result`, if the value is `Ok`, it unwraps it and gives you the value inside. If the value is `Err`, it returns the `Err` from the current function. This allows you to write clean, concise code that focuses on the "happy path" while still correctly handling errors.

- **`?` with `Option<T>`:** The `?` operator can also be used with `Option<T>`. If the value is `Some`, it unwraps it. If the value is `None`, it returns `None` from the current function.

## 🧩 Code Walkthrough

Let's analyze the code in `main.rs`.

### Handling `Result` with `match`

```rust
fn read_username_from_file_long() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
```

This function shows the verbose way to handle errors. We have to use `match` statements to check the `Result` at each step. If there is an error, we have to manually return it.

### The `?` Operator

```rust
fn read_username_from_file_short() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
```

This is the idiomatic way to write the same function. The `?` operator handles the error checking for us. If `File::open` returns an `Err`, the `?` operator will immediately return that `Err` from the `read_username_from_file_short` function. If it returns an `Ok`, it will unwrap it and assign the file handle to `f`.

### `?` with `Option<T>`

```rust
fn first_word_uppercase(s: &str) -> Option<String> {
    let word = first_word(s)?;
    Some(word.to_uppercase())
}
```

Here, the `?` operator is used on an `Option`. If `first_word(s)` returns `None`, the `?` operator will immediately return `None` from `first_word_uppercase`. If it returns `Some(word)`, it will unwrap it and assign the word to the `word` variable.

## ⚔️ Cross-Language Insights

- **vs. Golang:** The `?` operator is similar to the `if err != nil { return err }` pattern that is very common in Go.

- **vs. TypeScript:** The `?` operator is similar to the proposed `?.` (optional chaining) and `??` (nullish coalescing) operators in TypeScript, but it is more powerful because it works for both `Option` and `Result` and it can return from the current function.

- **vs. C:** C does not have a direct equivalent to the `?` operator.

## 🚀 Practical Reflection

- **Clean Error Handling:** The `?` operator is a key feature for writing clean and concise error handling code in Rust. It allows you to focus on the success case without ignoring the error case.

- **The `main` Function and `Result`:** You can even have a `main` function that returns a `Result`. This can be useful for propagating errors all the way up to the top level of your program.

- **Custom Error Types:** The `?` operator becomes even more powerful when you combine it with custom error types, which we will cover in the next lesson. The `?` operator can automatically convert between different error types, which makes it easy to compose functions that can fail in different ways.

## 🧩 Self-Review Prompts

- Rewrite the `first_word_uppercase` function without using the `?` operator.
- What is the difference between `unwrap()` and the `?` operator? When would you choose one over the other?
- Can you use the `?` operator in a function that does not return a `Result` or an `Option`? Why or why not?

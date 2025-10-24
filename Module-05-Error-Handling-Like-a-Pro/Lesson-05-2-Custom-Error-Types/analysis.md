# Lesson 05.2: Custom Error Types

## 🧠 Concept Summary

This lesson explores how to create your own custom error types. While the standard library provides many useful error types, sometimes you need to create your own to represent the specific ways that your code can fail.

- **Custom Error Types:** A custom error type is a struct or an enum that you define to represent the errors in your application. A good custom error type should implement the `Debug`, `Display`, and `Error` traits.

- **The `Error` Trait:** The `std::error::Error` trait is a trait for types that represent errors. It has one method, `source()`, which can be used to get the underlying cause of an error.

- **The `From` Trait:** The `From` trait is used to convert a value of one type into a value of another type. When you are working with custom error types, you can implement `From` to convert other error types into your custom error type. This allows you to use the `?` operator to automatically convert errors.

## 🧩 Code Walkthrough

Let's analyze the code in `main.rs`.

### Defining a Custom Error Type

```rust
#[derive(Debug)]
struct AppError {
    message: String,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for AppError {}
```

This defines a simple custom error type `AppError`. It's a struct with a single field, `message`. We implement `Debug` so we can print it for debugging, `Display` so we can print it for users, and `Error` to mark it as an error type.

### Converting Other Error Types

```rust
#[derive(Debug)]
enum MyError {
    Io(io::Error),
    Parse(std::num::ParseIntError),
}

// ... impl Display and Error ...

impl From<io::Error> for MyError {
    fn from(error: io::Error) -> Self {
        MyError::Io(error)
    }
}

impl From<std::num::ParseIntError> for MyError {
    fn from(error: std::num::ParseIntError) -> Self {
        MyError::Parse(error)
    }
}
```

This defines a more complex custom error type, `MyError`, which is an enum that can be either an `io::Error` or a `ParseIntError`. We then implement the `From` trait for both of these error types. This tells the compiler how to convert an `io::Error` or a `ParseIntError` into a `MyError`.

### Using the `?` Operator with Custom Errors

```rust
fn read_and_parse() -> Result<i32, MyError> {
    let mut file = File::open("number.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let number = contents.trim().parse()?;
    Ok(number)
}
```

This function shows the power of implementing the `From` trait. The `?` operator on `File::open` might return an `io::Error`. The `?` operator on `parse` might return a `ParseIntError`. Because we have implemented `From` for both of these types, the `?` operator can automatically convert them into a `MyError` and return it from the function.

## ⚔️ Cross-Language Insights

- **vs. Golang:** In Go, you would typically create a custom error type by creating a struct that implements the `error` interface. You would have to write your own code to wrap other error types.

- **vs. TypeScript:** In TypeScript, you would typically create a custom error class that extends the `Error` class. You would use `try`/`catch` blocks to catch and wrap other errors.

- **vs. C:** C does not have a standard way to handle errors. You would typically return an error code and use a global variable (`errno`) to get more information about the error.

## 🚀 Practical Reflection

- **Rich Error Information:** Custom error types allow you to provide rich, structured information about your errors. This can be very helpful for debugging and for providing good error messages to your users.

- **The `thiserror` and `anyhow` Crates:** While it's good to know how to create custom error types from scratch, in practice you will often use libraries like `thiserror` and `anyhow` to make it easier. `thiserror` is a library for creating custom error types, and `anyhow` is a library for creating a single, general-purpose error type that can hold any kind of error.

## 🧩 Self-Review Prompts

- Add a new variant to the `MyError` enum to handle a different kind of error.
- What is the `source()` method on the `Error` trait used for?
- Look at the documentation for the `thiserror` crate. How does it simplify the process of creating custom error types?

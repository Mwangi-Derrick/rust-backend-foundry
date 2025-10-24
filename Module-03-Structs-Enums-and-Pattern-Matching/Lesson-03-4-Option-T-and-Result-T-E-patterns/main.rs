// Lesson 03.4: Option<T> and Result<T, E> Patterns

// `Option<T>` and `Result<T, E>` are two of the most important enums in the Rust
// standard library. They are used for handling optional values and errors.

// --- `Option<T>`: For Optional Values ---

// The `Option<T>` enum has two variants:
// - `Some(T)`: A value of type `T` is present.
// - `None`: No value is present.

// It is so common that you don't need to bring it into scope explicitly.

fn main() {
    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;

    // The `Option<T>` enum is useful because it forces you to handle the case
    // where a value is not present. This prevents a whole class of bugs related
    // to null values.

    // For example, this code will not compile because you can't add an `i32` to an
    // `Option<i32>`.
    // let x: i32 = 5;
    // let y: Option<i32> = Some(5);
    // let sum = x + y;

    // --- `Result<T, E>`: For Recoverable Errors ---

    // The `Result<T, E>` enum has two variants:
    // - `Ok(T)`: A value of type `T` was returned successfully.
    // - `Err(E)`: An error of type `E` occurred.

    use std::fs::File;

    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error)
        }
    };

    // --- The `?` Operator: A Shortcut for Propagating Errors ---

    // The `?` operator can be used to simplify error handling. If the value of the
    // `Result` is an `Ok`, the value inside the `Ok` will get returned from this
    // expression, and the program will continue. If the value is an `Err`, the
    // `Err` will be returned from the whole function as if we had used the `return`
    // keyword so the error value gets propagated to the calling code.

    fn read_username_from_file() -> Result<String, std::io::Error> {
        let mut f = File::open("hello.txt")?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;
        Ok(s)
    }

    // Note: The `?` operator can only be used in functions that return a `Result`.

    match read_username_from_file() {
        Ok(s) => println!("The username is: {}", s),
        Err(e) => println!("Error reading username: {:?}", e),
    }
}

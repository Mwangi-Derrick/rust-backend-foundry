// Lesson 05.4: Handling Panics and Recoverability

// This lesson covers panics, which are a way of handling unrecoverable errors
// in Rust. We will also look at how to catch panics and when it is appropriate
// to do so.

// --- Unrecoverable Errors with `panic!` ---

// The `panic!` macro is used to cause the current thread to panic. When a thread
// panics, it will unwind the stack and clean up its resources. If the main
// thread panics, the entire program will exit.

// `panic!` should be used when a program is in a state where it is impossible
// to continue. For example, if a data structure is in an inconsistent state.

// --- Catching Panics ---

// It is possible to catch a panic using `std::panic::catch_unwind`. This can be
// useful in some situations, such as when you are calling code from another
// language that might panic, or when you are running a server and you don't want
// a single request to crash the entire server.

use std::panic;

fn main() {
    // --- A Simple Panic ---

    // Uncomment the following line to see a simple panic.
    // panic!("A simple panic!");

    // --- Catching a Panic ---

    let result = panic::catch_unwind(|| {
        println!("This code will be executed.");
        // panic!("This panic will be caught.");
        println!("This code will not be executed if there is a panic.");
    });

    match result {
        Ok(_) => println!("The code did not panic."),
        Err(_) => println!("The code panicked!"),
    }

    // --- When to Panic vs. When to Return a `Result` ---

    // The question of when to panic and when to return a `Result` is a matter of
    // good API design. In general, you should use `Result` for errors that are
    // expected and recoverable, and `panic!` for errors that are unexpected and
    // unrecoverable.

    // For example, if you are writing a function that parses a string into a
    // number, it is expected that the string might not be a valid number. In this
    // case, you should return a `Result`.

    // On the other hand, if you are writing a function that accesses an element
    // of an array, and you have already checked that the index is in bounds, it
    // would be a bug if the index was out of bounds. In this case, it would be
    // appropriate to panic.

    // The `unwrap()` and `expect()` methods on `Result` and `Option` are a
    // convenient way to panic if a value is not what you expect it to be.

    let some_value: Option<i32> = None;
    // let value = some_value.unwrap(); // This will panic
    // let value = some_value.expect("The value should be Some"); // This will panic with a message
}

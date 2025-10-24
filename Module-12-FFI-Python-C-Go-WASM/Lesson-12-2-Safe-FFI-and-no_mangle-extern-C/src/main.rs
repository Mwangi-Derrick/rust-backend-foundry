// Lesson 12.2: Safe FFI and #[no_mangle] extern "C"

// This lesson delves into the Foreign Function Interface (FFI) in Rust, focusing
// on how to safely call C code from Rust and expose Rust functions to C.

// --- What is FFI? ---

// FFI is a mechanism by which a program written in one programming language can
// call routines or handle data defined in another programming language.

// --- Calling C from Rust ---

// To call C functions from Rust, you need to declare the C functions in an
// `extern "C"` block. This tells Rust that the functions are defined externally
// and follow the C calling convention.

// Rust's FFI is inherently `unsafe` because the Rust compiler cannot guarantee
// the safety of the C code. You are responsible for ensuring that the C code
// is safe.

extern "C" {
    // This function is defined in C. We declare its signature here.
    fn abs(input: i32) -> i32;
}

// --- Exposing Rust to C ---

// To expose Rust functions to C, you need to do two things:
// 1. Use `#[no_mangle]` to prevent Rust from mangling the function name.
// 2. Use `extern "C"` to tell Rust to use the C calling convention.

#[no_mangle]
pub extern "C" fn rust_add(a: i32, b: i32) -> i32 {
    a + b
}

// --- Example: A C Header File ---

// If you were to compile this Rust code as a static or dynamic library, you
// would typically provide a C header file for other languages to use.

// ```c
// // my_library.h
//
// #include <stdint.h>
//
// int32_t rust_add(int32_t a, int32_t b);
// ```

fn main() {
    // Calling a C function from Rust
    unsafe {
        println!("Absolute value of -3 is: {}", abs(-3));
    }

    // Calling a Rust function (from the perspective of C)
    println!("Rust add 5 + 7 is: {}", rust_add(5, 7));

    println!("This lesson demonstrates FFI with C.");
    println!("To fully test this, you would need to compile the Rust code as a");
    println!("library and link it with a C program.");
}

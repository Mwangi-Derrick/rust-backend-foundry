// Lesson 15.4: Unsafe Zones and Codegen Intuition

// This lesson delves into `unsafe` Rust, explaining its purpose, how to use it
// responsibly, and how it relates to the compiler's code generation (codegen).

// --- What is `unsafe` Rust? ---

// `unsafe` Rust is a superset of safe Rust that allows you to do five things
// that the compiler cannot guarantee are memory safe:
// 1. Dereference a raw pointer.
// 2. Call an `unsafe` function or method.
// 3. Access or modify a mutable static variable.
// 4. Implement an `unsafe` trait.
// 5. Access fields of `union`s.

// The purpose of `unsafe` is to allow you to write code that interacts with
// the operating system, hardware, or foreign functions (FFI), where Rust's
// safety guarantees cannot be fully enforced.

// --- `unsafe` is Not Evil ---

// `unsafe` does not mean "untested" or "buggy". It means that the programmer
// is taking on the responsibility of upholding Rust's memory safety guarantees.
// When used correctly, `unsafe` code can be just as safe as safe Rust.

// --- Raw Pointers ---

// Raw pointers (`*const T` and `*mut T`) are similar to pointers in C/C++.
// They are not guaranteed to point to valid memory, and they do not have
// ownership semantics.

fn raw_pointers_example() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);

        *r2 = 6;
        println!("r1 is: {}", *r1);
    }
}

// --- Calling an `unsafe` Function ---

// Functions marked with `unsafe` can only be called from within an `unsafe` block.

unsafe fn dangerous() {
    println!("This is a dangerous function!");
}

// --- Codegen Intuition ---

// `unsafe` Rust often allows you to write code that is closer to the metal,
// giving you more control over how the compiler generates machine code.
// This can be important for performance-critical code.

// For example, `Vec::get_unchecked` is an `unsafe` method that returns a reference
// to an element without performing bounds checking. If you know that the index
// is valid, `get_unchecked` can be faster than `get` because it avoids the
// bounds check.

fn codegen_example() {
    let mut vec = vec![1, 2, 3, 4, 5];

    let value = unsafe {
        // This is faster because it skips the bounds check.
        // But it's only safe if you are absolutely sure the index is valid.
        *vec.get_unchecked_mut(0)
    };
    println!("Value: {}", value);
}

fn main() {
    println!("--- Raw Pointers Example ---");
    raw_pointers_example();

    println!("\n--- Calling Unsafe Function Example ---");
    unsafe {
        dangerous();
    }

    println!("\n--- Codegen Example ---");
    codegen_example();
}

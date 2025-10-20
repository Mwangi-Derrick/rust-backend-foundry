# Lesson 3: Mutable Borrowing & References

## 🧠 Concept Summary
This lesson builds on borrowing by introducing **mutable references**. While an immutable reference (`&T`) allows you to read data without taking ownership, a mutable reference (`&mut T`) allows you to *modify* it.

The most important rule for mutable borrowing is:
- **You can have only one mutable reference to a particular piece of data in a particular scope.**

This rule is one of Rust's key mechanisms for preventing data races at compile time. A data race can occur when two or more pointers access the same data at the same time, at least one of them is writing, and there's no synchronization. Rust simply forbids this scenario from compiling.

## 🧩 Code Walkthrough

The provided source file contains two `main` functions, which is not valid in a Rust binary crate. Below is a corrected and combined version of the code that demonstrates the concepts and solves the mini-challenge.

```rust
// 🎬 Lesson 3 — Mutable Borrowing & References

fn main() {
    // 'name' is a mutable String. We need 'mut' here because we intend to
    // pass a mutable reference to it later.
    let mut name = String::from("summafy");
    println!("Original name: {}", name);

    // Pass a mutable reference to 'add_io'.
    // The function will modify 'name' in place.
    add_io(&mut name);
    println!("After add_io: {}", name);

    // --- Mini Challenge Solution ---
    // 'mutable_challenge' is another mutable String for the challenge.
    let mut mutable_challenge = String::from("Summafy");
    println!("\nChallenge string before: {}", mutable_challenge);

    // Call the append function with a mutable reference.
    append_power(&mut mutable_challenge);
    println!("Challenge string after: {}", mutable_challenge);

    // --- The "One Mutable Borrow" Rule ---
    // The following commented-out code would cause a compile error:
    // let mut_ref1 = &mut name;
    // let mut_ref2 = &mut name; // ERROR: cannot borrow `name` as mutable more than once at a time
    // println!("{} {}", mut_ref1, mut_ref2);
}

// This function takes a mutable reference to a String.
// It can modify the original String.
fn add_io(s: &mut String) {
    s.push_str(".io");
    println!("Inside add_io: {}", s);
}

// Solution to the mini-challenge.
fn append_power(s: &mut String) {
    s.push_str(" - powered by Rust");
}
```

### Walkthrough Explanation
1.  We declare `name` and `mutable_challenge` as `mut` because we need to grant mutable access to them later.
2.  We call `add_io(&mut name)`. The `&mut` creates a mutable reference, which `add_io` receives as `s: &mut String`.
3.  Inside `add_io`, `s.push_str(".io")` modifies the original `name` string directly. No data is copied; the function operates on the same memory location.
4.  The same logic applies to the `append_power` function for the mini-challenge.
5.  The commented-out lines demonstrate a critical rule. Rust does not allow you to have two mutable references to the same data in the same scope. This is because it would create the potential for data races. The compiler stops this cold.

## ⚔️ Cross-Language Insights
- **Golang Equivalent:**
  - Passing a mutable reference in Rust is analogous to passing a pointer in Go (e.g., `func addIO(s *string)`). However, Go provides no compile-time guarantee that only one part of the program is writing to that pointer at a time. In concurrent Go programs, you must use mutexes or other synchronization primitives to prevent race conditions manually.
- **TypeScript Equivalent:**
  - In TypeScript, all objects are passed by reference, and they are all mutable by default. There is no built-in concept of a "mutable borrow" that is enforced by the compiler. You can pass an object to multiple functions, and any of them can modify it, which can lead to unexpected behavior if not managed carefully.
- **C Reference:**
  - A mutable reference `&mut String` is like passing a `char*` to a function in C. You can have many pointers to the same data (`char* ptr1 = &my_char; char* ptr2 = &my_char;`), and both can be used to write to the data. This is a classic source of bugs in C. Rust's rule of "one mutable reference OR many immutable references" solves this problem elegantly.

## 🚀 Practical Reflection
Mutable borrowing is essential for efficiency. In a backend service, you might have a large data structure representing a user's session or a complex entity. Instead of copying this data every time a function needs to make a small change, you can pass a mutable reference. This avoids performance overhead while guaranteeing safety. For your outbox-relay, you might need to update the status of a message in a list. Passing a mutable reference to that message struct would be the idiomatic and performant way to do it.

## 🧩 Self-Review Prompts
- Why does Rust prevent you from having multiple mutable references to the same data?
- What would happen if you tried to create a mutable reference to a variable that was not declared with `let mut`?
- Can you have an immutable reference (`&T`) and a mutable reference (`&mut T`) to the same data in the same scope? Try it and see what the compiler says.
- When would you choose to pass `&mut T` to a function versus returning a new value?

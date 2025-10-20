# Lesson 1: Variables, Mutability, and Types

## 🧠 Concept Summary
This lesson introduces fundamental Rust concepts:
- **Immutability by Default:** Variables declared with `let` are immutable, meaning their values cannot be changed after assignment. This is a core safety feature of Rust, designed to prevent unintended side-effects.
- **Explicit Mutability:** To create a mutable variable, you must use the `let mut` keyword. This makes the intention to modify a variable explicit.
- **Ownership (Preview):** The commented-out lines provide a glimpse into Rust's ownership system. When you assign one `String` to another (`let s2 = s1;`), ownership is *moved*, and the original variable (`s1`) becomes invalid to prevent potential memory safety issues like double-freeing.
- **Borrowing (Preview):** The code demonstrates borrowing with `let s2 = &s1;`. Instead of moving ownership, a reference to the original data is created. This allows multiple parts of the code to access the data without taking ownership.
- **Macros:** The `println!` macro is used for printing to the console. The `!` indicates that it's a macro, not a function. Macros are a way of writing code that writes other code (metaprogramming).

## 🧩 Code Walkthrough
```rust
// 🎯 Lesson 1: Variables, Mutability, and Types

// In Rust, variables are immutable by default — unlike Go or JS.

// 👉 Create a new playground:

fn main() {
    // Declare an immutable variable 'x' and initialize it with the value 5.
    let x = 5;
    println!("x = {}", x);

    // This block is commented out. If you were to uncomment it, the compiler
    // would throw an error because 'x' is immutable.
    // You cannot assign a new value to it.
    //x = 6;^ cannot assign twice to immutable variable

    // Declare a mutable variable 'y' with the 'mut' keyword.
    let mut y = 10;
    println!("y before = {}", y);
    // Since 'y' is mutable, we can change its value.
    y = 15;
    println!("y after = {}", y);

    // Create a String s1. Strings are heap-allocated.
    let s1 = String::from("hello");
    
    // This line is commented out. If executed, it would move ownership of the
    // String data from s1 to s2. After this line, s1 would no longer be valid.
    // let s2 = s1; 

    // This commented-out println! would fail because s1's ownership would have
    // been moved to s2. The compiler prevents use-after-move errors.
    //  //println!("{}", s1); // ❌ error! s1 no longer valid ... let s2 = s1.clone(); // ownership moves to s2
    // println!("{}", s2);

    // This is the fix: borrowing. Instead of moving ownership, we create a
    // reference (&) to s1. s2 "borrows" s1. Now both are valid.
     let s2 = &s1; // borrow, not move
    println!("s1 = {}, s2 = {}", s1, s2);
}

// 🧠 Concept:

// let = immutable

// let mut = mutable

// println!() is a macro, not a function — that’s why it has !
```

## ⚔️ Cross-Language Insights
- **Golang Equivalent:**
  - In Go, variables are mutable by default (e.g., `x := 5; x = 10;`). Rust's `let` is conceptually closer to defining a constant in Go (`const x = 5`), but Rust's `let` is about immutability, not just a compile-time constant. There is no direct equivalent to Rust's strict ownership and borrowing rules in Go.
- **TypeScript Equivalent:**
  - Rust's `let` is very similar to TypeScript's `const` (`const x = 5;`).
  - Rust's `let mut` is equivalent to TypeScript's `let` (`let y = 10; y = 15;`).
  - TypeScript has no concept of ownership or borrowing in the way Rust does.
- **C Reference:**
  - In C, variables are mutable by default (`int x = 5; x = 10;`). To achieve immutability, you'd use the `const` keyword (`const int x = 5;`). C requires manual memory management (`malloc`, `free`), and it's the programmer's responsibility to avoid issues like use-after-free, which Rust's ownership system prevents at compile time.

## 🚀 Practical Reflection
The concept of immutability by default is crucial for building robust and concurrent backend systems. In a high-throughput microservice, many threads could be accessing the same data. Rust's approach prevents race conditions by ensuring that data cannot be changed unexpectedly. This compile-time guarantee eliminates a whole class of bugs that can be very difficult to track down in languages like Go or Python, especially in concurrent scenarios.

## 🧩 Self-Review Prompts
- What are the advantages of a system where variables are immutable by default?
- When would you choose `let mut` over `let`?
- The commented-out code hints at ownership. Why do you think Rust has this feature, and how might it relate to memory safety?
- How does the concept of borrowing (`&`) differ from moving ownership?

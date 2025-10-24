# Lesson 02.1: The Ownership Model

## 🧠 Concept Summary

This lesson introduces the most unique and central feature of Rust: **Ownership**. The ownership system is how Rust guarantees memory safety without needing a garbage collector. It's a set of rules that the compiler checks at compile time. If the rules are violated, the code won't compile.

### The Three Rules of Ownership

1.  **Each value in Rust has a variable that’s called its owner.**
2.  **There can only be one owner at a time.**
3.  **When the owner goes out of scope, the value will be dropped.**

These three rules are the foundation of Rust's memory management.

- **Move Semantics:** For data stored on the heap (like `String`), when you assign one variable to another, the ownership is *moved*. The original variable is no longer valid. This prevents "double free" errors.

- **The `Copy` Trait:** For data stored entirely on the stack (like integers, booleans, and chars), it's cheap to make a copy. These types implement the `Copy` trait, which means when you assign one variable to another, the data is copied, and the original variable is still valid.

- **Ownership and Functions:** Passing a variable to a function will also move or copy it, just like assignment.

## 🧩 Code Walkthrough

Let's break down the code in `main.rs`.

### Scope and `drop`

```rust
{
    let s = "hello"; // `s` is valid from this point forward
    // do stuff with `s`
} // this scope is now over, and `s` is no longer valid
```

This demonstrates a basic scope. When `s` goes out of scope at the closing curly brace, Rust automatically calls the `drop` function for `s` and frees the memory.

### `String` and Move Semantics

```rust
let s1 = String::from("hello");
let s2 = s1;

// This would cause a compile-time error:
// println!("s1 is: {}", s1);
```

This is the core of move semantics. `s1` is a `String`, which is stored on the heap. When we write `let s2 = s1;`, we are not copying the data on the heap. Instead, we are moving the *ownership* of that data to `s2`. After the move, `s1` is no longer a valid owner, and the compiler will prevent us from using it.

### Cloning: Deep Copies

```rust
let s3 = String::from("hello");
let s4 = s3.clone();

println!("s3 = {}, s4 = {}", s3, s4);
```

If we want to create a true deep copy of the data on the heap, we can use the `clone()` method. This will create a new `String` with its own data on the heap, and `s4` will be the owner of that new data.

### The `Copy` Trait

```rust
let x = 5;
let y = x;

println!("x = {}, y = {}", x, y);
```

Integers are stored on the stack and are cheap to copy. They implement the `Copy` trait. When we write `let y = x;`, the value of `x` is copied to `y`. Both `x` and `y` are valid and independent.

### Ownership and Functions

```rust
let s = String::from("hello");
takes_ownership(s);

// This would cause a compile-time error:
// println!("s is: {}", s);

let x = 5;
makes_copy(x);

println!("x is still: {}", x);
```

Passing a value to a function follows the same rules. `takes_ownership` takes a `String`, so ownership of `s` is moved into the function. `makes_copy` takes an `i32`, which is `Copy`, so a copy of `x` is made, and `x` is still valid after the function call.

## ⚔️ Cross-Language Insights

- **vs. Golang:** Go uses a garbage collector. You don't have to think about ownership in the same way. You can have multiple references to the same data, and the garbage collector will clean it up when it's no longer needed. This is more convenient but can come with a performance cost (e.g., GC pauses).

- **vs. TypeScript:** TypeScript (and JavaScript) also uses a garbage collector. The concept of ownership doesn't exist in the same way.

- **vs. C:** In C, you have to manage memory manually with `malloc` and `free`. This is very powerful but also very error-prone. It's easy to forget to `free` memory (a memory leak) or to free it twice (a double free error). Rust's ownership system is designed to prevent these errors at compile time.

## 🚀 Practical Reflection

- **Predictable Performance:** Because Rust doesn't have a garbage collector, its memory management is very predictable. You know exactly when a value is going to be dropped, which is critical for performance-sensitive applications.

- **Thinking in Terms of Ownership:** Learning to think in terms of ownership is the biggest hurdle for new Rust programmers. It might feel restrictive at first, but it quickly becomes second nature and leads to safer, more reliable code.

- **The Borrow Checker:** The part of the Rust compiler that enforces the ownership rules is called the "borrow checker." You will become very familiar with the borrow checker's error messages. Learning to read and understand them is a key skill.

## 🧩 Self-Review Prompts

- What happens if you try to pass a `Copy` type to the `takes_ownership` function? Does it still take ownership?
- Create a function that takes a `String` and returns a `String`. How does ownership move in this case?
- Can a type be both `Copy` and `Drop`? Why or why not?

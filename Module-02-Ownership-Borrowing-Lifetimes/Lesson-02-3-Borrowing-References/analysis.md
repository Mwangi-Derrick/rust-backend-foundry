# Lesson 02.3: Borrowing & References

## 🧠 Concept Summary

This lesson introduces **borrowing**, a core concept that works hand-in-hand with ownership. Borrowing allows you to access a value without taking ownership of it. This is done through **references**.

- **References (`&`)**: A reference is like a pointer that is guaranteed to point to a valid value. It allows you to refer to a value without taking ownership of it. We call the action of creating a reference *borrowing*.

- **Immutable Borrows (`&T`)**: By default, references are immutable. You can have as many immutable references to a value as you want, but you can't change the value through them.

- **Mutable Borrows (`&mut T`)**: You can also borrow a value mutably, which allows you to modify it. However, there is a big restriction: you can only have **one** mutable reference to a particular piece of data in a particular scope. This is one of Rust's key mechanisms for preventing data races.

### The Rules of References

1.  At any given time, you can have either *one* mutable reference or *any number* of immutable references.
2.  References must always be valid (i.e., they cannot outlive the data they refer to - a concept we'll explore more with lifetimes).

## 🧩 Code Walkthrough

Let's examine the code in `main.rs`.

### The Problem with Ownership

```rust
fn calculate_length_takes_ownership(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

let s1 = String::from("hello");
let (s2, len) = calculate_length_takes_ownership(s1);
```

This code illustrates the inconvenience of ownership when we just want to read a value. The `calculate_length_takes_ownership` function takes ownership of the `String`, and then has to return it so that the caller can use it again. This is verbose and inefficient.

### References and Borrowing

```rust
fn calculate_length_borrows(s: &String) -> usize {
    s.len()
}

let s3 = String::from("hello");
let len = calculate_length_borrows(&s3);
```

This is the idiomatic way to solve the problem. The `calculate_length_borrows` function takes a reference to a `String` (`&String`). It borrows the value, but doesn't take ownership. After the function call, `s3` is still valid and can be used.

### Mutable References

```rust
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

let mut s4 = String::from("hello");
change(&mut s4);
```

Here, we create a mutable reference to `s4` (`&mut s4`) and pass it to the `change` function. This allows the function to modify the `String`.

### The One Mutable Reference Rule

```rust
let mut s5 = String::from("hello");

let r1 = &mut s5;
// let r2 = &mut s5; // This would cause a compile-time error
```

This is the compiler enforcing Rust's data race prevention. You cannot have two mutable references to the same value in the same scope. This is because if you did, you could have two parts of your program trying to modify the same data at the same time, leading to unpredictable behavior.

## ⚔️ Cross-Language Insights

- **vs. Golang:** In Go, you can have multiple pointers to the same data, and you can write to the data through any of them. Go uses channels and mutexes to handle synchronization, but it's up to the programmer to use them correctly. Rust prevents data races at compile time.

- **vs. TypeScript:** In TypeScript, objects are passed by reference, so you can have multiple references to the same object and modify it through any of them. There is no concept of a mutable vs. immutable borrow.

- **vs. C:** In C, you can have as many pointers to a piece of data as you want, and you can write to it through any of them. This is a major source of bugs in C programs. Rust's borrow checker is a direct solution to this problem.

## 🚀 Practical Reflection

- **Safety and Performance:** Borrowing is a zero-cost abstraction. It provides the safety of knowing that you don't have data races, without any runtime overhead.

- **Read vs. Write:** The borrowing rules mirror a common pattern in concurrent programming: it's safe to have many readers or one writer, but not both at the same time.

- **Compiler as Your Friend:** The borrow checker can be frustrating at first, but it's one of the most valuable tools in the Rust toolchain. It will force you to think about your data access patterns and will catch a whole class of bugs before your code even runs.

## 🧩 Self-Review Prompts

- What happens if you have an immutable reference to a value, and then you try to create a mutable reference to it in the same scope?
- Write a function that takes a slice of integers (`&[i32]`) and returns the sum.
- Can you have a mutable reference and an immutable reference to the same value in the same scope? Why or why not?

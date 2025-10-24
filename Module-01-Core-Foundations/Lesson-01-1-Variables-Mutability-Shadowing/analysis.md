# Lesson 01.1: Variables, Mutability & Shadowing

## 🧠 Concept Summary

This lesson introduces the foundational concepts of how Rust handles variables. The core principles are:

- **Immutability by Default:** Variables are immutable unless explicitly marked otherwise. This is a key feature of Rust that promotes safety and prevents unintended side effects.
- **Mutability with `mut`:** To create a variable that can be changed, you must use the `mut` keyword. This makes your intent clear and helps the compiler enforce safety guarantees.
- **Shadowing:** You can declare a new variable with the same name as a previous one. This is different from mutation, as it creates a completely new variable, allowing you to change the type of the value as well.
- **Constants:** Constants are always immutable and must have their type explicitly annotated. They are evaluated at compile time.

## 🧩 Code Walkthrough

Let's break down the code in `main.rs` line by line.

```rust
// In Rust, variables are immutable by default.
let x = 5;
println!("The value of x is: {}", x);
```

Here, we declare a variable `x` and bind it to the value `5`. Since we don't use the `mut` keyword, `x` is immutable. If you try to reassign it, the compiler will stop you. This is a deliberate design choice to make code safer and easier to reason about.

```rust
// To make a variable mutable, you use the `mut` keyword.
let mut y = 10;
println!("The initial value of y is: {}", y);
y = 20;
println!("The new value of y is: {}", y);
```

In this block, we introduce the `mut` keyword. By declaring `y` as `let mut y`, we are explicitly telling Rust that we intend to change this variable's value later in the program. This is why the assignment `y = 20;` is allowed.

```rust
// Shadowing is a powerful feature in Rust.
let z = 30;
println!("The value of z is: {}", z);
let z = z + 1;
println!("The shadowed value of z is: {}", z);
```

This demonstrates shadowing. We first declare `z` with the value `30`. Then, we declare a *new* variable, also named `z`, that takes the value of the old `z` and adds `1`. The first `z` is now "shadowed" by the second `z` and can no longer be accessed.

```rust
// You can even change the type of a shadowed variable.
let spaces = "   ";
let spaces = spaces.len();
println!("The number of spaces is: {}", spaces);
```

This is a key advantage of shadowing over mutation. The `spaces` variable starts as a string slice (`&str`). We then shadow it with a new `spaces` variable that holds the *length* of the string, which is an integer (`usize`). This is perfectly legal and often very convenient.

```rust
// Constants are always immutable and must be type-annotated.
const MAX_POINTS: u32 = 100_000;
println!("The maximum points are: {}", MAX_POINTS);
```

Finally, we have constants. Unlike variables, constants are *always* immutable and their value is inlined by the compiler at compile time. They require a type annotation (in this case, `u32` for an unsigned 32-bit integer).

## ⚔️ Cross-Language Insights

- **vs. Golang:** In Go, you declare variables with `var` or `:=`. There is no concept of immutability by default. You can reassign any variable. Go's `const` is similar to Rust's `const`.

- **vs. TypeScript:** TypeScript has `let` and `const`. `let` is for variables that can be reassigned, and `const` is for variables that cannot. Rust's `let` is like TypeScript's `const`, and Rust's `let mut` is like TypeScript's `let`.

- **vs. C:** In C, all variables are mutable by default. You can use the `const` keyword to make a variable immutable, but it's not the default and not as strictly enforced by the compiler as in Rust.

## 🚀 Practical Reflection

- **Why Immutability by Default?** In large, concurrent systems, mutable state is a common source of bugs. By making immutability the default, Rust forces you to be explicit about when and where you are introducing change, making your code easier to reason about and less prone to race conditions.

- **When to Use Shadowing vs. Mutability:**
    - Use `mut` when you want to change the value of a variable within the same scope, and the type remains the same.
    - Use shadowing when you want to perform a transformation on a value and then treat it as immutable from that point forward. It's also the only way to "change" the type of a variable.

## 🧩 Self-Review Prompts

- What happens if you try to shadow a mutable variable with an immutable one? Try it!
- When would you choose to use a `const` over an immutable `let`?
- How does Rust's approach to variables influence the way you think about writing safe code?

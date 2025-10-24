# Lesson 02.2: Move Semantics vs. Clone vs. Copy

## 🧠 Concept Summary

This lesson dives deeper into the mechanics of how Rust passes data around. We'll clarify the distinction between moving, cloning, and copying.

- **Move Semantics:** This is the default behavior in Rust for types that manage resources on the heap (like `String`). When you assign a variable to another, ownership is transferred. The original variable is invalidated to prevent issues like "double free" errors. A move is a cheap operation as it only copies the pointer and metadata on the stack, not the data on the heap.

- **`Copy` Trait:** This is an implicit, bit-for-bit copy that happens for types that are stored entirely on the stack. These types are simple and don't have any special resources to manage. After a copy, both the original and the new variable are valid.

- **`Clone` Trait:** This is an explicit, often deep copy of a value. You have to call the `.clone()` method to perform a clone. This is used for types that have data on the heap and you want to create a duplicate of that data.

## 🧩 Code Walkthrough

Let's analyze the code in `main.rs`.

### Move Semantics in Detail

```rust
let s1 = String::from("hello");
let s2 = s1;
```

A `String` is a struct that contains a pointer to a buffer on the heap, a length, and a capacity. When we write `let s2 = s1;`, we are copying the struct (the pointer, length, and capacity) from `s1` to `s2`. We are *not* copying the data on the heap. To ensure memory safety, Rust invalidates `s1`. This is the "move".

### The `Copy` Trait

```rust
let x = 5;
let y = x;

println!("x = {}, y = {}", x, y);
```

`i32` is a simple type that is stored entirely on the stack. It implements the `Copy` trait. When we write `let y = x;`, the value of `x` is copied to `y`. Both variables are still valid.

### The `Clone` Trait

```rust
let s3 = String::from("world");
let s4 = s3.clone();
```

Here, we explicitly call `.clone()` on `s3`. This creates a new `String`, `s4`, with its own buffer on the heap that contains a copy of the data from `s3`. Both `s3` and `s4` are valid and independent.

### `Copy` vs. `Clone`

```rust
let a = 5;
let b = a;
let c = a.clone();
```

This illustrates the relationship between `Copy` and `Clone`. All types that are `Copy` are also `Clone`. The `clone` method on a `Copy` type simply performs a copy. The key difference is that `Copy` is implicit (it happens on assignment), while `Clone` is explicit (you have to call `.clone()`).

## ⚔️ Cross-Language Insights

- **vs. Golang:** Go does not have move semantics in the same way. When you assign a struct, it is copied. If the struct contains a pointer, the pointer is copied, but the data it points to is not. This is sometimes called a "shallow copy".

- **vs. TypeScript:** In TypeScript, objects are assigned by reference. `let obj2 = obj1;` makes `obj2` and `obj1` point to the same object. To create a copy, you would use `Object.assign({}, obj1)` for a shallow copy or a library function for a deep copy.

- **vs. C:** In C, you are responsible for managing copies yourself. You can use `memcpy` to copy data, but you have to be careful to allocate enough memory and to free it correctly. Rust's `Clone` trait provides a safe and explicit way to handle this.

## 🚀 Practical Reflection

- **Performance:** Moves are cheap. Clones can be expensive, especially for large data structures. You should be mindful of where you are cloning and whether it's necessary.

- **API Design:** If you are writing a function and you need to take ownership of a value, you can take it by value (which will move it). If you just need to read it, you should take a reference (which we'll cover in the next lesson). If you need to make a copy inside the function, you can call `.clone()` on the argument.

- **Clarity:** The `Copy`/`Clone` distinction makes your code more explicit. When you see a `.clone()`, you know that a potentially expensive operation is happening. When you see an assignment, you know that it's either a cheap copy or a move.

## 🧩 Self-Review Prompts

- Create a struct that contains a `String`. Does it implement `Copy`? Why or why not?
- Write a function that takes a `String` and returns a `String`. How many times is the data on the heap copied?
- What is the difference between `Clone` and `Copy` in terms of their method signatures?

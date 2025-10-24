# Lesson 03.2: Associated Functions vs. Methods

## 🧠 Concept Summary

This lesson clarifies the distinction between **methods** and **associated functions** in Rust.

- **Methods:** These are functions within an `impl` block that take `self`, `&self`, or `&mut self` as their first parameter. They are called on an instance of the struct.

- **Associated Functions:** These are functions within an `impl` block that do *not* take `self` as their first parameter. They are associated with the struct type itself, not an instance. They are often used as constructors.

- **Multiple `impl` Blocks:** You can have multiple `impl` blocks for the same struct. This is a way to organize your code, but it's more common to have a single `impl` block.

## 🧩 Code Walkthrough

Let's analyze the code in `main.rs`.

### Methods vs. Associated Functions

```rust
impl Rectangle {
    // This is a method.
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // This is an associated function.
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}
```

The `area` function is a method because it takes `&self`. You call it on an instance of `Rectangle` with dot notation: `rect1.area()`. The `square` function is an associated function because it does not take `self`. You call it using the struct name and the `::` syntax: `Rectangle::square(3)`.

### Constructors

Associated functions are often used as constructors. A common convention is to have an associated function named `new` that creates a new instance of the struct. For example:

```rust
impl Rectangle {
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }
}

let rect = Rectangle::new(30, 50);
```

## ⚔️ Cross-Language Insights

- **vs. Golang:** Go does not have a direct equivalent to associated functions. You would typically create a separate factory function (e.g., `NewRectangle`) to act as a constructor.

- **vs. TypeScript:** In TypeScript, you would use a `static` method on a class to achieve the same effect as an associated function. A regular method on the class is equivalent to a Rust method.

- **vs. C:** C does not have methods or associated functions. You would use regular functions to create and operate on structs.

## 🚀 Practical Reflection

- **Clarity of Intent:** The distinction between methods and associated functions makes your code more readable. When you see `::`, you know that you are calling a function that is associated with the type, not an instance. When you see `.`, you know you are calling a method on an instance.

- **Constructors are a Convention:** Rust does not have a special `constructor` keyword like some other languages. The use of `new` or other associated functions as constructors is a convention, but it is a very common and recommended one.

## 🧩 Self-Review Prompts

- Add a `new` associated function to the `Rectangle` struct to act as a constructor.
- Can an associated function be called on an instance of a struct? Why or why not?
- When would you choose to have multiple `impl` blocks for a struct?

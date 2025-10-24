# Lesson 03.1: Structs, Tuples, and impl Blocks

## 🧠 Concept Summary

This lesson introduces **structs**, which are custom data types that allow you to group together and name related values. We also look at **`impl` blocks**, which are used to define methods associated with a struct.

- **Structs:** There are three kinds of structs in Rust:
    - **Structs with Named Fields:** The most common kind of struct, where each field has a name and a type.
    - **Tuple Structs:** Structs that look like tuples. They have types but no names for their fields. They are useful for creating new types that are distinct from the types of their fields.
    - **Unit-Like Structs:** Structs with no fields. They are useful in generic situations where you need to implement a trait on some type but don’t have any data that you want to store in the type itself.

- **`impl` Blocks:** An `impl` block (short for "implementation") is where you define methods for a struct. Methods are functions that are associated with a struct and can operate on its data.

- **Methods (`&self`):** The first parameter of a method is always `self`, which represents the instance of the struct the method is being called on. `&self` is a shorthand for `self: &Self`, where `Self` is an alias for the type that the `impl` block is for.

## 🧩 Code Walkthrough

Let's analyze the code in `main.rs`.

### Defining and Instantiating Structs

```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

let mut user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};
```

This defines a `User` struct with four fields. We then create an instance of the struct, `user1`. We can access the fields of the struct using dot notation (e.g., `user1.email`).

### Field Init Shorthand and Struct Update Syntax

```rust
fn build_user(email: String, username: String) -> User {
    User {
        email,    // Field init shorthand
        username, // Field init shorthand
        active: true,
        sign_in_count: 1,
    }
}

let user2 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername567"),
    ..user1 // Struct update syntax
};
```

The `build_user` function shows the field init shorthand. If a function parameter has the same name as a struct field, you don't have to repeat the name. The struct update syntax (`..user1`) allows you to create a new instance of a struct that has most of the same values as an old instance, but with some new values.

### `impl` Blocks and Methods

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
```

Here, we define a `Rectangle` struct and an `impl` block for it. The `area` method takes `&self` as its first parameter, which is an immutable reference to the `Rectangle` instance. The `can_hold` method takes `&self` and another `Rectangle` reference, and returns a boolean indicating whether the second rectangle can fit inside the first.

## ⚔️ Cross-Language Insights

- **vs. Golang:** Go's `struct` is very similar to Rust's. Go doesn't have `impl` blocks; instead, you define methods for a struct by creating functions that have a special receiver argument.

- **vs. TypeScript:** TypeScript has `class`es, which are similar to Rust structs with `impl` blocks. TypeScript classes can have both fields and methods. TypeScript also has `interface`s, which can be used to define the shape of an object, similar to a struct definition.

- **vs. C:** C has `struct`s, but they can only contain data, not methods. You would typically create separate functions that take a pointer to a struct to operate on its data.

## 🚀 Practical Reflection

- **Organizing Code:** `impl` blocks are a great way to organize your code. All of the functions that operate on a struct are located in one place.

- **Data and Behavior:** Structs and `impl` blocks are a powerful way to associate data with behavior. This is a key principle of object-oriented programming, but Rust is not strictly an object-oriented language. It has some object-oriented features, but it also has features from functional programming and other paradigms.

- **The `Debug` Trait:** The `#[derive(Debug)]` annotation is a convenient way to allow a struct to be printed for debugging purposes. We will learn more about traits and the `derive` attribute later.

## 🧩 Self-Review Prompts

- Add a method to the `Rectangle` struct that returns a new `Rectangle` that is a scaled version of the original.
- Create a `User` method that returns the user's username. What should the method's signature be?
- Can a method take ownership of `self`? If so, what would that mean?

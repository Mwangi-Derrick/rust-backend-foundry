# Lesson 04.1: Defining Traits

## 🧠 Concept Summary

This lesson introduces **traits**, which are Rust's way of defining shared behavior. A trait is similar to an interface in other languages.

- **Traits:** A trait is a collection of method signatures that define a set of behaviors. A type can implement a trait, which means it provides concrete implementations for the methods in the trait.

- **Implementing a Trait:** You can implement a trait for a type using an `impl Trait for Type` block. Inside this block, you provide the implementations for the methods defined in the trait.

- **Default Implementations:** Traits can have default implementations for some or all of their methods. If a type implementing the trait doesn't provide its own implementation, it will use the default one.

- **Traits as Parameters:** You can use traits to write functions that are generic over different types. By specifying a trait as a parameter, you can accept any type that implements that trait.

## 🧩 Code Walkthrough

Let's analyze the code in `main.rs`.

### Defining and Implementing a Trait

```rust
pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle { ... }

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet { ... }

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
```

We define a `Summary` trait with one method, `summarize`. We then implement this trait for two different structs, `NewsArticle` and `Tweet`. Each struct provides its own custom implementation of `summarize`.

### Default Implementations

```rust
pub trait SummaryWithDefault {
    fn summarize(&self) -> String {
        String::from("(Read more...)") // Default implementation
    }
}

impl SummaryWithDefault for NewsArticle {}
```

Here, we define a trait with a default implementation for `summarize`. When we implement this trait for `NewsArticle`, we don't need to provide an implementation for `summarize` because it will use the default one.

### Traits as Parameters

```rust
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
```

The `notify` function takes a parameter `item` of type `&impl Summary`. This means it can accept a reference to any type that implements the `Summary` trait. This is a powerful feature for writing generic and reusable code.

## ⚔️ Cross-Language Insights

- **vs. Golang:** Traits are very similar to Go's `interface`s. In Go, an interface is a set of method signatures. A type implements an interface by implementing all of its methods. Go's interfaces are implemented implicitly, whereas Rust's traits are implemented explicitly.

- **vs. TypeScript:** Traits are also similar to TypeScript's `interface`s. A TypeScript interface can define the shape of an object, including its methods. A class can then `implement` the interface.

- **vs. C:** C does not have traits or interfaces. You would typically use function pointers in a struct to achieve a similar kind of polymorphism.

## 🚀 Practical Reflection

- **Abstracting Behavior:** Traits are a powerful tool for abstracting behavior. You can write code that operates on a trait, without needing to know the concrete type of the value.

- **The `impl Trait` Syntax:** The `impl Trait` syntax for function parameters is a convenient shorthand for a more general concept called "trait bounds", which we will cover in the next lesson.

- **The Orphan Rule:** A key rule in Rust is the "orphan rule", which states that you can only implement a trait for a type if either the trait or the type is defined in your own crate. This prevents other crates from breaking your code by implementing traits for your types.

## 🧩 Self-Review Prompts

- Create a new struct and implement the `Summary` trait for it.
- Can you have a function that returns a type that implements a trait? How would you write the signature?
- What is the difference between `&impl Summary` and `&dyn Summary`? (We will cover this in more detail later).

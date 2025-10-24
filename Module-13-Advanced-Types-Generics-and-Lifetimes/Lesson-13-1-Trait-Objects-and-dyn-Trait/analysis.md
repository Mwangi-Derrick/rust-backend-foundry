# Lesson 13.1: Trait Objects and dyn Trait

## 🧠 Concept Summary

This lesson revisits **trait objects** and the **`dyn` keyword**, providing a deeper understanding of their mechanics and when to use them. This is crucial for understanding dynamic polymorphism in Rust.

- **Static vs. Dynamic Dispatch (Review):**
    - **Static Dispatch:** Achieved with generics and trait bounds (`<T: Trait>`). The compiler generates specialized code for each concrete type, resulting in zero runtime overhead. This is generally preferred for performance.
    - **Dynamic Dispatch:** Achieved with trait objects (`&dyn Trait` or `Box<dyn Trait>`). The method call is resolved at runtime via a vtable lookup, incurring a small runtime cost but offering greater flexibility.

- **Trait Objects:** A trait object is a pointer to an instance of a type that implements a certain trait. It allows you to treat different concrete types as a single abstract type. This is useful when you need to store or pass around values of different types that share a common behavior.

- **`dyn` Keyword:** The `dyn` keyword is used to explicitly indicate that you are working with a trait object. It clarifies that dynamic dispatch will be used.

- **Object Safety:** Not all traits can be made into trait objects. A trait is "object-safe" if all of its methods have a `&self` receiver (or `&mut self`, `self: Box<Self>`, etc.). This is because the trait object needs to be able to call the methods on the underlying type, and it can only do that if it has a `self` to work with.

## 🧩 Code Walkthrough

Let's analyze the code in `main.rs`.

### Using Trait Objects

```rust
trait Greeter {
    fn greet(&self);
}

// ... EnglishGreeter and SpanishGreeter implementations ...

let greeters: Vec<Box<dyn Greeter>> = vec![
    Box::new(EnglishGreeter),
    Box::new(SpanishGreeter),
];

for greeter in greeters {
    greeter.greet();
}
```

Here, we define a `Greeter` trait and two structs that implement it. We then create a `Vec<Box<dyn Greeter>>`. This vector can hold different concrete types (`EnglishGreeter` and `SpanishGreeter`) as long as they implement the `Greeter` trait. When we call `greeter.greet()`, the actual method to call is determined at runtime via dynamic dispatch.

### Trait Objects as Function Parameters

```rust
fn say_hello(g: &dyn Greeter) {
    g.greet();
}

let english = EnglishGreeter;
say_hello(&english);
```

Functions can also accept trait objects as parameters. `&dyn Greeter` means the function accepts a reference to any type that implements `Greeter`. This is another way to achieve dynamic polymorphism.

### `impl Trait` vs. `dyn Trait`

```rust
fn process_greeter_static(g: impl Greeter) {
    g.greet();
}

fn process_greeter_dynamic(g: &dyn Greeter) {
    g.greet();
}
```

- `process_greeter_static` uses `impl Greeter`, which is **static dispatch**. The compiler knows the concrete type at compile time.
- `process_greeter_dynamic` uses `&dyn Greeter`, which is **dynamic dispatch**. The concrete type is only known at runtime.

## ⚔️ Cross-Language Insights

- **vs. Golang:** Go's interfaces are always dynamically dispatched. When you pass a value that implements an interface, Go uses a vtable-like mechanism to call the correct method at runtime.

- **vs. TypeScript:** TypeScript's interfaces are primarily for compile-time type checking. Runtime polymorphism is achieved through class inheritance and method overriding.

- **vs. C++:** C++ achieves dynamic polymorphism through virtual functions and pointers/references to base classes. This is very similar to Rust's trait objects and dynamic dispatch.

## 🚀 Practical Reflection

- **Flexibility vs. Performance:** `dyn Trait` offers maximum flexibility by allowing you to work with heterogeneous collections of types. However, this comes at the cost of a small runtime overhead due to vtable lookups. `impl Trait` (static dispatch) is generally preferred when you don't need that flexibility, as it has zero runtime cost.

- **When to Use `dyn Trait`:**
    - When you need to store a collection of different types that implement the same trait (e.g., `Vec<Box<dyn Trait>>`).
    - When you need to pass different types that implement the same trait to a function, and you don't want to use generics (e.g., `&dyn Trait` as a function parameter).

- **Sized vs. Unsized Types:** Trait objects are "unsized types" (DSTs - Dynamically Sized Types). This means the compiler doesn't know their size at compile time. Because of this, you can only use trait objects behind a pointer (e.g., `&dyn Trait`, `Box<dyn Trait>`, `Arc<dyn Trait>`).

## 🧩 Self-Review Prompts

- Create a trait that is *not* object-safe. Explain why it's not object-safe.
- Implement a function that takes a `Vec<Box<dyn Greeter>>` and adds a new `Greeter` to it.
- What is the difference between `&dyn Trait` and `Box<dyn Trait>`?

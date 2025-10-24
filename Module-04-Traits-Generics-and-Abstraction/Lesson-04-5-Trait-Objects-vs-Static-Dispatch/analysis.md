# Lesson 04.5: Trait Objects vs. Static Dispatch

## 🧠 Concept Summary

This lesson explores the two ways that Rust handles polymorphism (the ability of code to work with values of different types): **static dispatch** and **dynamic dispatch**.

- **Static Dispatch:** This is the process of resolving which function to call at *compile time*. When you use generics with trait bounds (`<T: Trait>`), the compiler generates a specific version of the code for each concrete type. This is very fast because there is no runtime overhead.

- **Dynamic Dispatch:** This is the process of resolving which function to call at *runtime*. This is achieved using **trait objects**. A trait object (`&dyn Trait` or `Box<dyn Trait>`) is a pointer to an instance of a type that implements a trait. When you call a method on a trait object, Rust looks up which method to call in a virtual table (vtable) at runtime.

- **When to Use Which:**
    - Use static dispatch (generics) when you can. It's faster.
    - Use dynamic dispatch (trait objects) when you need a collection of values of different concrete types that all implement the same trait.

## 🧩 Code Walkthrough

Let's analyze the code in `main.rs`.

### Static Dispatch

```rust
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
```

This function uses static dispatch. If you call `notify` with a `&Tweet` and a `&NewsArticle`, the compiler will generate two versions of the function, one for each type. This is called **monomorphization**.

### Dynamic Dispatch with Trait Objects

```rust
pub fn notify_dynamic(item: &dyn Summary) {
    println!("Breaking news! {}", item.summarize());
}
```

This function uses dynamic dispatch. It takes a trait object, `&dyn Summary`. This is a "fat pointer" that contains a pointer to the data and a pointer to a vtable. The vtable is a table of function pointers that Rust uses to look up the correct method to call at runtime.

### A Use Case for Trait Objects

```rust
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
```

This is a classic example of where you need trait objects. The `Screen` struct has a vector of `components`. Each component can be a different type (a `Button` or a `SelectBox`), but they all implement the `Draw` trait. Because they are different types, we can't have a `Vec<Button>` or a `Vec<SelectBox>`. We need a `Vec` that can hold values of any type that implements `Draw`. This is what `Vec<Box<dyn Draw>>` gives us.

## ⚔️ Cross-Language Insights

- **vs. Golang:** Go's interfaces are always dynamically dispatched. There is no equivalent to Rust's static dispatch for interfaces.

- **vs. TypeScript:** In TypeScript, methods are always dynamically dispatched.

- **vs. C:** In C, you can implement dynamic dispatch manually using function pointers in a struct, which is similar to how Rust implements trait objects under the hood.

## 🚀 Practical Reflection

- **Performance Trade-offs:** Static dispatch is faster because the compiler can inline the function calls. Dynamic dispatch has a small runtime cost because of the vtable lookup.

- **Flexibility:** Dynamic dispatch is more flexible because it allows you to have collections of different types that share a common interface.

- **Object Safety:** Not all traits can be made into trait objects. A trait is "object-safe" if all of its methods have a `&self` receiver or can be made to have one. This is because the trait object needs to be able to call the methods on the underlying type, and it can only do that if it has a `&self` to work with.

## 🧩 Self-Review Prompts

- Create a new struct that implements the `Draw` trait and add it to the `Screen`'s components.
- What happens if you try to create a `Vec<dyn Draw>` instead of a `Vec<Box<dyn Draw>>`? Why?
- When would you choose to use static dispatch over dynamic dispatch, and vice versa?

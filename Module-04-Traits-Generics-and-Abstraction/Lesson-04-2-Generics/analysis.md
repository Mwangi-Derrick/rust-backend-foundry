# Lesson 04.2: Generics

## 🧠 Concept Summary

This lesson introduces **generics**, which are a way to write code that is abstract over types. Generics allow you to write functions, structs, enums, and methods that can work with many different concrete types.

- **Generics in Functions:** You can use generics to write a function that can take arguments of any type that has the behavior you need. This is done by declaring a generic type parameter (e.g., `<T>`) and then using it in the function signature.

- **Generics in Structs and Enums:** You can also use generics in struct and enum definitions to create data structures that can hold values of any type.

- **Generics in Methods:** You can use generics in method definitions to create methods that can operate on generic data structures.

- **Monomorphization:** Rust implements generics using a process called monomorphization. At compile time, the compiler looks at all the places where a generic function or type is used and generates a specific version of the code for each concrete type that is used. This means that using generics in Rust has no runtime cost.

## 🧩 Code Walkthrough

Let's analyze the code in `main.rs`.

### Generics in Functions

```rust
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
```

This is a generic function that can find the largest item in a slice of any type `T`. The `<T: PartialOrd + Copy>` part is a **trait bound**, which we will cover in the next lesson. It constrains the generic type `T` to only be types that can be ordered (`PartialOrd`) and copied (`Copy`).

### Generics in Structs and Enums

```rust
struct Point<T> {
    x: T,
    y: T,
}

enum Option<T> {
    Some(T),
    None,
}
```

The `Point` struct is generic over a type `T`. This means we can create a `Point` that has `i32`s for its fields, or `f64`s, or any other type. The `Option` enum is also generic over `T`, which allows it to hold a value of any type.

### Generics in Methods

```rust
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
```

When we implement methods for a generic struct, we have to declare the generic type parameter in the `impl` block (`impl<T>`). We can then use the generic type in the method signatures.

## ⚔️ Cross-Language Insights

- **vs. Golang:** Go has recently added generics. Before that, you would have to use interfaces and type assertions to write code that could operate on multiple types, which was less safe and less performant.

- **vs. TypeScript:** TypeScript is heavily based on generics. You can use generics in functions, classes, and interfaces to write reusable and type-safe code.

- **vs. C:** C does not have generics. You would typically use macros or `void*` pointers to write code that can operate on multiple types, but this is not type-safe.

## 🚀 Practical Reflection

- **Don't Repeat Yourself (DRY):** Generics are a powerful tool for reducing code duplication. If you find yourself writing the same function for different types, you can probably use a generic function instead.

- **Zero-Cost Abstraction:** Because of monomorphization, using generics in Rust is a "zero-cost abstraction". This means you can write clean, abstract code without paying any runtime performance penalty.

- **Trait Bounds:** The real power of generics comes when you combine them with traits, which we will explore in the next lesson. Trait bounds allow you to write generic code that is also safe and correct.

## 🧩 Self-Review Prompts

- Write a generic function that takes a slice of any type and returns the first element in an `Option`.
- Create a generic struct that holds two values of different types.
- Can you implement a method on a generic struct that only works for a specific concrete type? (Hint: Look up "specialization").

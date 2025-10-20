# Lesson 4: Structs & Impl Blocks

## 🧠 Concept Summary
This lesson introduces two fundamental concepts for building custom data types in Rust:

-   **`struct`**: A `struct` (short for structure) is a custom data type that lets you name and package together related values that make up a meaningful group. It's the primary way you'll define the domain objects and data containers in your application.

-   **`impl` Block**: An `impl` block (implementation block) is where you define methods associated with a `struct`. This allows you to couple data (the struct fields) with behavior (the methods that operate on that data), which is a core principle of object-oriented programming.

## 🧩 Code Walkthrough
The lesson is broken into two parts: a basic struct and then a struct with an `impl` block.

### Example 1: Basic Struct (Commented Out)
This first example lays the groundwork.
```rust
// struct Startup {
//     name: String,
//     category: String,
//     monthly_revenue: f64,
// }
//
// fn main() {
//     let summafy = Startup {
//         name: String::from("Summafy.io"),
//         category: String::from("AI & Productivity"),
//         monthly_revenue: 1200.50,
//     };
//
//     println!(
//         "{} is in {} category and earns ${} monthly",
//         summafy.name, summafy.category, summafy.monthly_revenue
//     );
// }
```
-   A `struct` named `Startup` is defined with three fields: `name`, `category`, and `monthly_revenue`.
-   In `main`, an *instance* of the `Startup` struct is created, named `summafy`.
-   The fields of the instance are accessed using dot notation (e.g., `summafy.name`).

### Example 2: Struct with Methods (`impl` Block)
This example builds on the first by adding behavior.
```rust
struct Startup {
    name: String,
    category: String,
    monthly_revenue: f64,
    employees: u32, // New field from the challenge
}

// The 'impl' block is where we define methods for the Startup struct.
impl Startup {
    // This is a method. The '&self' parameter is a reference to the
    // instance of the struct the method is being called on.
    fn annual_revenue(&self) -> f64 {
        self.monthly_revenue * 12.0
    }

    // This method also takes a reference to self.
    fn describe(&self) {
        println!(
            "{} operates in {} and makes ${} per year. We have {} empolyees each with an average revenue of ${} per year.",
            self.name,
            self.category,
            self.annual_revenue(), // Calling another method on the same instance
            self.employees,
            self.avg_revenue_per_employee() // Challenge solution
        );
    }

    // Challenge solution: New method to calculate average revenue.
    fn avg_revenue_per_employee(&self) -> f64 {
        // We must cast 'self.employees' from u32 to f64 to perform the division.
        self.annual_revenue() / (self.employees as f64)
    }
}

fn main() {
    let summafy = Startup {
        name: String::from("Summafy.io"),
        category: String::from("AI & Productivity"),
        monthly_revenue: 1200.50,
        employees: 10,
    };

    // Call the 'describe' method on our 'summafy' instance.
    summafy.describe();
}
```
-   **`&self`**: The `&self` parameter is shorthand for `self: &Self`, where `Self` is an alias for the type the `impl` block is for (`Startup` in this case). It's an immutable reference to the instance. If the method needed to change the instance, it would use `&mut self`.
-   **Methods**: `annual_revenue`, `describe`, and `avg_revenue_per_employee` are *methods* because they are associated with a `Startup` instance. You call them using dot notation (`summafy.describe()`).
-   **Type Casting**: The line `self.employees as f64` is an explicit type cast. Rust does not perform automatic type conversions between numeric types, so you must be explicit.

## ⚔️ Cross-Language Insights
-   **Golang Equivalent:**
    -   A Rust `struct` is almost identical to a Go `struct`. The `impl` block is conceptually the same as defining methods with a receiver in Go. The `describe` method in Rust would look like this in Go:
        ```go
        func (s *Startup) describe() { ... }
        ```
-   **TypeScript Equivalent:**
    -   A Rust `struct` and `impl` block are very similar to a TypeScript `class`.
        ```typescript
        class Startup {
            name: string;
            category: string;
            monthlyRevenue: number;
            employees: number;

            constructor(...) { ... }

            annualRevenue(): number {
                return this.monthlyRevenue * 12;
            }

            describe(): void { ... }
        }
        ```
    -   `&self` in Rust is analogous to `this` in TypeScript.
-   **C Reference:**
    -   A C `struct` can hold data, but it cannot have methods. The common C pattern is to create functions that take a pointer to the struct as their first argument:
        ```c
        void describe_startup(const Startup* s) { ... }
        ```
    -   Rust's `impl` blocks provide a much more ergonomic, object-oriented syntax for the same concept.

## 🚀 Practical Reflection
Structs and `impl` blocks are the backbone of any non-trivial Rust application. For your outbox-relay microservice, you will almost certainly model the core business objects as structs. For example, you could have:

```rust
struct OutboxMessage {
    id: Uuid,
    payload: JsonValue,
    topic: String,
    is_sent: bool,
}

impl OutboxMessage {
    fn mark_as_sent(&mut self) {
        self.is_sent = true;
    }

    fn serialize_payload(&self) -> String {
        // ... logic to convert payload to a JSON string ...
    }
}
```
This approach makes the code organized, easy to understand, and allows the compiler to enforce that operations are tied to the correct data types.

## 🧩 Self-Review Prompts
-   What is the difference between `&self` and `&mut self` in an `impl` block? When would you use each?
-   Besides methods, `impl` blocks can also have *associated functions* (like `String::from`) that don't take `self`. How would you define one, and how would you call it?
-   How would you change the `describe` method to take an additional argument, like a greeting `&str`?
-   Why is it good practice to put behavior related to a `struct` in an `impl` block rather than in standalone functions?

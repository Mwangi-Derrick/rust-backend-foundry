# Lesson 01.2: Primitive Types & Type Inference

## 🧠 Concept Summary

This lesson explores Rust's primitive data types, which are the most basic building blocks for storing data. We also look at type inference, a feature where the Rust compiler automatically deduces the type of a variable based on its value and context.

- **Scalar Types:** These represent a single value. Rust has four primary scalar types:
    - **Integers:** Whole numbers, which can be signed (`i`) or unsigned (`u`) and come in various sizes (e.g., `i32`, `u64`).
    - **Floating-Point Numbers:** Numbers with a decimal point, available in `f32` and `f64` sizes.
    - **Booleans:** The `bool` type, which can be either `true` or `false`.
    - **Characters:** The `char` type, representing a single Unicode Scalar Value.

- **Compound Types:** These group multiple values into one type. Rust has two primitive compound types:
    - **Tuples:** A fixed-size, ordered list of values that can have different types.
    - **Arrays:** A fixed-size list of elements that must all have the same type.

- **Type Inference:** Rust's compiler is smart. If you don't explicitly annotate a variable's type, it will try to infer it. If there's not enough information, it will default to a reasonable choice (e.g., `i32` for integers).

## 🧩 Code Walkthrough

Let's examine the code in `main.rs`.

### Scalar Types

```rust
// Integers
let a: i32 = -10;
let b: u32 = 10;
let c = 20; // Inferred as i32 by default
```

Here, we explicitly type `a` as a 32-bit signed integer and `b` as a 32-bit unsigned integer. For `c`, we let Rust infer the type, which defaults to `i32`.

```rust
// Floating-Point Numbers
let d: f64 = 2.0; // 64-bit float (default)
let e: f32 = 3.0; // 32-bit float
```

`f64` is the default floating-point type because on modern CPUs, it's roughly the same speed as `f32` but with more precision.

```rust
// Booleans
let f: bool = true;
let g = false; // Inferred as bool
```

Simple and clear: `bool` can be `true` or `false`.

```rust
// Characters
let h: char = 'a';
let i = '😻'; // Yes, emojis are chars!
```

Rust's `char` type is 4 bytes in size and represents a Unicode Scalar Value. This means it can represent a lot more than just ASCII characters, including accented letters, characters from other languages, and even emojis.

### Compound Types

```rust
// Tuples
let tup: (i32, f64, u8) = (500, 6.4, 1);
let (x, y, z) = tup; // Destructuring a tuple
let five_hundred = tup.0;
```

Tuples are a great way to group related data of different types. You can access their elements by index (e.g., `tup.0`) or by destructuring them into separate variables.

```rust
// Arrays
let arr: [i32; 5] = [1, 2, 3, 4, 5];
let first = arr[0];
let same_values = [3; 5]; // [3, 3, 3, 3, 3]
```

Arrays are useful when you want a collection of a fixed number of elements of the same type. They are stored on the stack, which makes them very fast. The `[3; 5]` syntax is a convenient way to initialize an array with the same value for every element.

## ⚔️ Cross-Language Insights

- **vs. Golang:** Go has similar primitive types (e.g., `int32`, `uint32`, `float64`, `bool`). Go does not have tuples, but it has arrays and slices. Go's type inference with `:=` is similar to Rust's.

- **vs. TypeScript:** TypeScript's number type is used for both integers and floats, which is different from Rust's distinct integer and float types. TypeScript has tuples and arrays, which are more flexible than Rust's in terms of size (they can grow and shrink).

- **vs. C:** C's primitive types are very similar to Rust's, but C is not as strict about type safety. For example, you can implicitly convert between different numeric types in C, which can lead to bugs. Rust requires explicit casting for such conversions.

## 🚀 Practical Reflection

- **Why so many integer types?** Having different integer sizes allows you to be very precise about your data. If you know a value will always be positive and will never exceed 255, you can use a `u8` and save memory. This is especially important in systems programming where memory layout and efficiency are critical.

- **Arrays vs. Tuples:**
    - Use a tuple when you have a fixed set of related values of *different* types.
    - Use an array when you have a fixed set of values of the *same* type.

- **Type Inference:** While type inference is convenient, it's often good practice to add explicit type annotations for function signatures and complex data structures. This can make your code more readable and can help you catch errors earlier.

## 🧩 Self-Review Prompts

- What happens if you try to add an `i32` and a `u32`? Why does the compiler stop you?
- When would you choose an array over a `Vec` (a growable vector, which we'll cover later)?
- Create a tuple that contains an array and another tuple. How would you access the elements?

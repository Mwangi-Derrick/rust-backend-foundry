# Lesson 01.4: Control Flow, Loops, and Match Statements

## 🧠 Concept Summary

This lesson covers the fundamental ways to control the flow of execution in Rust. We'll look at `if`/`else` expressions, the different kinds of loops (`loop`, `while`, `for`), and the powerful `match` statement.

- **`if`/`else` Expressions:** Used for branching based on a boolean condition. In Rust, `if` is an expression, meaning it can return a value.

- **Loops:** Rust provides three kinds of loops:
    - **`loop`:** An infinite loop that you can break out of, optionally returning a value.
    - **`while`:** A loop that continues as long as a boolean condition is true.
    - **`for`:** The most common and safest loop in Rust, used to iterate over a collection or a range.

- **`match` Statements:** A powerful control flow construct that allows you to compare a value against a series of patterns and execute code based on which pattern matches. `match` is exhaustive, meaning you must handle all possible cases.

## 🧩 Code Walkthrough

Let's explore the code in `main.rs`.

### `if`/`else` Expressions

```rust
let number = 6;

if number % 4 == 0 {
    println!("number is divisible by 4");
} else if number % 3 == 0 {
    println!("number is divisible by 3");
} else {
    println!("number is not divisible by 4, 3, or 2");
}

let condition = true;
let x = if condition { 5 } else { 6 };
```

The first part is a standard `if`/`else if`/`else` chain. The second part shows that `if` is an expression. The value of the `if` expression is the value of the block that gets executed. The types of the values in each block must be the same.

### Loops

```rust
let mut counter = 0;
let result = loop {
    counter += 1;

    if counter == 10 {
        break counter * 2;
    }
};
```

The `loop` keyword creates an infinite loop. We use `break` to exit the loop. Here, we also return a value from the loop (`counter * 2`), which is then assigned to `result`.

```rust
let mut number = 3;
while number != 0 {
    println!("{}!", number);
    number -= 1;
}
```

A `while` loop is a good choice when you don't know how many times you need to loop, but you have a condition to check.

```rust
let a = [10, 20, 30, 40, 50];
for element in a.iter() {
    println!("the value is: {}", element);
}

for number in (1..4).rev() {
    println!("{}!", number);
}
```

`for` loops are the workhorse of iteration in Rust. They are safer than `while` loops because you can't make an off-by-one error. The first `for` loop iterates over an array. The second one iterates over a range of numbers in reverse.

### `match` Statements

```rust
let some_number = 5;
match some_number {
    1 => println!("one"),
    2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
    13..=19 => println!("A teen"),
    _ => println!("Ain't special"),
}
```

This is where Rust's power really shines. `match` allows for complex pattern matching:
- `1`: A single value.
- `2 | 3 | 5 | 7 | 11`: Multiple values.
- `13..=19`: An inclusive range.
- `_`: A catch-all for any other value. This is required to make the `match` exhaustive.

Like `if`, `match` is also an expression and can return a value.

## ⚔️ Cross-Language Insights

- **vs. Golang:** Go has `if`/`else` and `for` loops (which are used for `while`-style loops as well). Go's `switch` statement is similar to Rust's `match`, but it's not as powerful (e.g., no pattern matching on ranges) and not exhaustive by default.

- **vs. TypeScript:** TypeScript has `if`/`else`, `while`, and `for` loops. Its `switch` statement is similar to Go's and less powerful than Rust's `match`.

- **vs. C:** C has `if`/`else`, `while`, `for`, and `switch`. C's `switch` is very limited and a common source of bugs (e.g., forgetting `break`). Rust's `match` is much safer and more expressive.

## 🚀 Practical Reflection

- **Expressions Everywhere:** The fact that `if` and `match` are expressions is a powerful feature that can make your code more concise and readable. It allows you to initialize a variable with a value that depends on some condition, without needing to declare a mutable variable first.

- **`for` Loops for Safety:** You should always prefer `for` loops over `while` loops when you are iterating over a collection. It's safer and more idiomatic.

- **`match` for Clarity:** `match` is often a better choice than a complex `if`/`else if`/`else` chain. It can make your code more readable and the exhaustiveness check from the compiler can prevent bugs.

## 🧩 Self-Review Prompts

- Rewrite a `for` loop using a `while` loop and a `loop`.
- What happens if you remove the `_` arm from the `match` statement? Why?
- Create a `match` statement that matches on a tuple (e.g., `(1, 2)`).

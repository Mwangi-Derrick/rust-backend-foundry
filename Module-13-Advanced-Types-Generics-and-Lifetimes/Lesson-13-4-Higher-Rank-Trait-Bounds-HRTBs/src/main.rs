// Lesson 13.4: Higher-Rank Trait Bounds (HRTBs)

// This lesson introduces Higher-Rank Trait Bounds (HRTBs), which are an advanced
// feature of Rust's type system that allows you to express more complex lifetime
// relationships.

// --- The Problem: Generic Closures with Lifetimes ---

// Imagine you want to write a function that takes a closure, and that closure
// needs to operate on references with *any* lifetime. Without HRTBs, this can
// be difficult or impossible to express.

// For example, if you have a function that takes a `&str` and returns a `&str`,
// and you want to pass a closure that does this, how do you specify the lifetime
// of the `&str` in the closure's signature?

// --- HRTB Syntax ---

// HRTBs use the `for<'a>` syntax. This means "for all possible lifetimes 'a".

// Example: `for<'a> F: Fn(&'a str) -> &'a str`
// This means that `F` is a function that can be called with a `&str` of *any*
// lifetime `'a`, and it will return a `&str` with the *same* lifetime `'a`.

// --- Example: A Function that Accepts a Generic Closure ---

fn process_string_with_closure<'b, F>(s: &'b str, f: F) -> &'b str
where
    F: for<'a> Fn(&'a str) -> &'a str,
{
    f(s)
}

// Without the HRTB `for<'a>`, the compiler would assume that the lifetime `'a`
// in `Fn(&'a str) -> &'a str` is tied to the outer function's lifetime `'b`,
// which is too restrictive. The HRTB tells the compiler that the closure `f`
// is generic over *its own* lifetime parameter `'a`.

// --- Example: A Trait with a Method that Requires HRTB ---

// This trait defines a method `transform` that takes a reference with any
// lifetime `'a` and returns a reference with the same lifetime `'a`.

trait Transformer {
    fn transform<'a>(&self, input: &'a str) -> &'a str;
}

struct MyTransformer;

impl Transformer for MyTransformer {
    fn transform<'a>(&self, input: &'a str) -> &'a str {
        // In a real scenario, this might return a slice of the input.
        input
    }
}

fn main() {
    let my_string = String::from("hello world");

    // --- Closure Example ---

    let my_closure = |s: &str| -> &str {
        s.split(' ').next().unwrap_or(s)
    };

    let first_word = process_string_with_closure(&my_string, my_closure);
    println!("First word: {}", first_word);

    // --- Trait Example ---

    let transformer = MyTransformer;
    let transformed_string = transformer.transform(&my_string);
    println!("Transformed string: {}", transformed_string);
}

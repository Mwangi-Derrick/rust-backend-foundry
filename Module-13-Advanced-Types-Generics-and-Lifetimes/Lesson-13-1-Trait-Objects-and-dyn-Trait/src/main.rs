// Lesson 13.1: Trait Objects and dyn Trait

// This lesson revisits trait objects, providing a deeper understanding of their
// mechanics and when to use them, especially with the `dyn` keyword.

// --- Review: Static vs. Dynamic Dispatch ---

// - **Static Dispatch:** Achieved with generics and trait bounds (`<T: Trait>`).
//   The compiler generates specialized code for each concrete type, resulting
//   in zero runtime overhead.

// - **Dynamic Dispatch:** Achieved with trait objects (`&dyn Trait` or `Box<dyn Trait>`).
//   The method call is resolved at runtime via a vtable lookup, incurring a small
//   runtime cost but offering greater flexibility.

// --- What is a Trait Object? ---

// A trait object is a pointer to an instance of a type that implements a certain
// trait. It allows you to treat different concrete types as a single abstract type.

// The `dyn` keyword is used to explicitly indicate that you are working with a
// trait object.

// --- Object Safety ---

// Not all traits can be made into trait objects. A trait is "object-safe" if
// all of its methods have a `&self` receiver (or `&mut self`, `self: Box<Self>`, etc.).
// This is because the trait object needs to be able to call the methods on the
// underlying type, and it can only do that if it has a `self` to work with.

// Traits that are not object-safe:
// - Traits with methods that return `Self`.
// - Traits with methods that have generic type parameters.

trait Greeter {
    fn greet(&self);
}

struct EnglishGreeter;
impl Greeter for EnglishGreeter {
    fn greet(&self) {
        println!("Hello!");
    }
}

struct SpanishGreeter;
impl Greeter for SpanishGreeter {
    fn greet(&self) {
        println!("¡Hola!");
    }
}

fn main() {
    // --- Using Trait Objects ---

    // We can create a vector of trait objects, allowing us to store different
    // concrete types that implement the same trait.
    let greeters: Vec<Box<dyn Greeter>> = vec![
        Box::new(EnglishGreeter),
        Box::new(SpanishGreeter),
    ];

    for greeter in greeters {
        greeter.greet();
    }

    // --- Trait Objects as Function Parameters ---

    // You can also pass trait objects as function parameters.
    fn say_hello(g: &dyn Greeter) {
        g.greet();
    }

    let english = EnglishGreeter;
    say_hello(&english);

    // --- `impl Trait` vs. `dyn Trait` ---

    // `impl Trait` (static dispatch) is generally preferred when possible because
    // it has zero runtime overhead.

    // `dyn Trait` (dynamic dispatch) is used when you need the flexibility of
    // storing or passing around values of different concrete types that implement
    // the same trait.

    fn process_greeter_static(g: impl Greeter) {
        g.greet();
    }

    fn process_greeter_dynamic(g: &dyn Greeter) {
        g.greet();
    }

    let english_static = EnglishGreeter;
    process_greeter_static(english_static);

    let spanish_dynamic = SpanishGreeter;
    process_greeter_dynamic(&spanish_dynamic);
}

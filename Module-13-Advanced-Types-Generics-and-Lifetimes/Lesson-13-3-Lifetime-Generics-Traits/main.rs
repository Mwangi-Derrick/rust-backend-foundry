// Lesson 13.3: Lifetime + Generics + Traits (Triple Combo)

// This lesson brings together lifetimes, generics, and traits to demonstrate
// how they interact in complex scenarios. This is often considered one of the
// most challenging aspects of Rust.

// --- The Problem: Generic Data with References ---

// Imagine you want to create a generic data structure that holds references.
// You need to ensure that the references inside the data structure are valid
// for as long as the data structure itself lives.

// --- Struct with Generic and Lifetime ---

// A struct that holds a reference to a generic type `T`.
struct Container<'a, T: 'a> {
    value: &'a T,
}

impl<'a, T: 'a> Container<'a, T> {
    fn new(value: &'a T) -> Self {
        Container { value }
    }

    fn get_value(&self) -> &'a T {
        self.value
    }
}

// --- Trait with Generic and Lifetime ---

// A trait that defines a method that takes a generic type `T` and returns a
// reference with a lifetime `'a`.

trait Processor<'a, T: 'a> {
    fn process(&self, input: &'a T) -> &'a T;
}

struct MyProcessor;

impl<'a, T: 'a> Processor<'a, T> for MyProcessor {
    fn process(&self, input: &'a T) -> &'a T {
        // In a real scenario, this might do some processing and return a part
        // of the input, or a reference to some internal state.
        input
    }
}

// --- Function with Generic, Trait, and Lifetime ---

// A function that takes a generic type `T` that implements a trait `P` with
// a lifetime `'a`.

fn analyze_data<'a, T, P>(data: &'a T, processor: &P) -> &'a T
where
    T: 'a,
    P: Processor<'a, T>,
{
    processor.process(data)
}

fn main() {
    // --- Struct Example ---
    let data_int = 42;
    let container_int = Container::new(&data_int);
    println!("Container int value: {}", container_int.get_value());

    let data_str = String::from("hello");
    let container_str = Container::new(&data_str);
    println!("Container string value: {}", container_str.get_value());

    // --- Trait and Function Example ---
    let my_processor = MyProcessor;
    let processed_data = analyze_data(&data_int, &my_processor);
    println!("Processed data: {}", processed_data);

    let processed_str = analyze_data(&data_str, &my_processor);
    println!("Processed string: {}", processed_str);
}

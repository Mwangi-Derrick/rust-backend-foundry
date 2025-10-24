// Lesson 04.2: Generics

// Generics are a way of writing code that can operate on a variety of different
// types, rather than being hardcoded to one specific type.

// --- In Functions ---

// We can use generics to create a function that can operate on any type that
// has the behavior we need.

// This function takes a slice of `i32`s and returns the largest one.
fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// This function takes a slice of `char`s and returns the largest one.
fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// We can use generics to write a single function that works for both cases.
// The `T: PartialOrd + Copy` is a "trait bound", which we will cover in the
// next lesson. For now, it means that `T` can be any type that can be ordered
// and copied.
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// --- In Structs ---

// We can also use generics in struct definitions.

struct Point<T> {
    x: T,
    y: T,
}

// We can also use multiple generic types.
struct PointMulti<T, U> {
    x: T,
    y: U,
}

// --- In Enums ---

// We can also use generics in enum definitions.

enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

// --- In Methods ---

// We can also use generics in method definitions.

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest_char(&char_list);
    println!("The largest char is {}", result);

    // Using the generic function
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    // Using a generic struct
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    // Using a generic struct with multiple types
    let both = PointMulti { x: 5, y: 4.0 };

    // Using a generic method
    println!("integer.x = {}", integer.x());
}

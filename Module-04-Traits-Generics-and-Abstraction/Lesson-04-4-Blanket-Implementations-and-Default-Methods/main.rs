// Lesson 04.4: Blanket Implementations and Default Methods

// This lesson covers two powerful features of traits: default methods and
// blanket implementations.

// --- Default Methods ---

// A default method is a method in a trait that has a default implementation.
// A type that implements the trait can either use the default implementation or
// provide its own.

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// --- Blanket Implementations ---

// A blanket implementation is an implementation of a trait for any type that
// satisfies a certain trait bound.

// For example, the standard library has a blanket implementation of the `ToString`
// trait for any type that implements the `Display` trait.

// impl<T: Display> ToString for T {
//     // ...
// }

// This means that any type that implements `Display` automatically gets a
// `to_string` method for free.

use std::fmt::Display;

struct Point {
    x: i32,
    y: i32,
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
    };

    // We can call `summarize` on the `Tweet` instance, and it will use the
    // default implementation.
    println!("1 new tweet: {}", tweet.summarize());

    // We can see the blanket implementation of `ToString` in action here.
    let p = Point { x: 1, y: 2 };
    println!("The point is: {}", p.to_string());
}

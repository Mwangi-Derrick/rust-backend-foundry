// Lesson 04.3: Trait Bounds and Where Clauses

// In the last lesson, we saw a brief example of a trait bound. In this lesson,
// we will explore them in more detail.

// --- Trait Bounds ---

// A trait bound is a way of constraining a generic type to only be types that
// implement a certain trait.

use std::fmt::Display;

pub trait Summary {
    fn summarize(&self) -> String;
}

// We can use the `impl Trait` syntax for simple cases.
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// The `impl Trait` syntax is sugar for a longer form called a "trait bound".
pub fn notify_long<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// We can also require multiple trait bounds.
pub fn notify_multi<T: Summary + Display>(item: &T) {}

// --- `where` Clauses ---

// When you have a lot of trait bounds, the function signature can get cluttered.
// The `where` clause is a way to clean this up.

fn some_function<T: Display + Clone, U: Clone + std::fmt::Debug>(t: &T, u: &U) -> i32 {
    // ...
    0
}

// We can rewrite this using a `where` clause:
fn some_function_where<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + std::fmt::Debug,
{
    // ...
    0
}

// --- Returning Types that Implement Traits ---

// We can also use `impl Trait` in the return position to return a value of some
// type that implements a trait, without naming the concrete type.

fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

// This can be useful when you want to hide the concrete type that you are
// returning, for example if it is very complex.

// However, you can only return a single concrete type. This code will not
// compile because it might return a `NewsArticle` or a `Tweet`.
// fn returns_summarizable_broken(switch: bool) -> impl Summary {
//     if switch {
//         NewsArticle { ... }
//     } else {
//         Tweet { ... }
//     }
// }

// We will learn how to solve this problem in the next lesson.

// --- Structs for the example ---
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn main() {
    let tweet = returns_summarizable();
    println!("1 new tweet: {}", tweet.summarize());
}

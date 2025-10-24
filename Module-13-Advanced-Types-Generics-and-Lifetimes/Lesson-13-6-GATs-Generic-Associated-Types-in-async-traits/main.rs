// Lesson 13.6: GATs (Generic Associated Types) in async traits

// This lesson introduces Generic Associated Types (GATs), a powerful and
// relatively new feature in Rust that allows associated types to be generic
// over lifetimes or other type parameters. This is particularly useful in
// `async` traits.

// --- The Problem: Associated Types with Lifetimes ---

// Before GATs, if you had an associated type that needed to hold a reference
// with a specific lifetime, it was very difficult or impossible to express.
// This often came up in `async` contexts where you wanted to return a future
// that borrowed from `self`.

// --- GATs to the Rescue ---

// GATs allow you to add generic parameters (including lifetimes) to associated
// types. This enables more flexible and expressive trait designs.

// --- Example: An Async Iterator with GATs ---

// Imagine an async iterator that yields references to items.
// Before GATs, this was very hard to model correctly.

// Note: The `async_trait` macro is used here to make the trait methods `async`.
// GATs are often used in conjunction with `async_trait`.

use async_trait::async_trait;

#[async_trait]
pub trait AsyncIterator {
    type Item<'a> where Self: 'a;

    async fn next<'a>(&'a mut self) -> Option<Self::Item<'a>>;
}

struct MyAsyncIterator {
    data: Vec<String>,
    index: usize,
}

impl MyAsyncIterator {
    fn new(data: Vec<String>) -> Self {
        MyAsyncIterator { data, index: 0 }
    }
}

#[async_trait]
impl AsyncIterator for MyAsyncIterator {
    type Item<'a> = &'a str where Self: 'a;

    async fn next<'a>(&'a mut self) -> Option<Self::Item<'a>> {
        if self.index < self.data.len() {
            let item = &self.data[self.index];
            self.index += 1;
            Some(item.as_str())
        } else {
            None
        }
    }
}

#[tokio::main]
async fn main() {
    let data = vec![
        String::from("hello"),
        String::from("world"),
        String::from("rust"),
    ];
    let mut iter = MyAsyncIterator::new(data);

    while let Some(item) = iter.next().await {
        println!("Item: {}", item);
    }
}

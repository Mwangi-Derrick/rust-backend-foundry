// Lesson 13.5: Associated Types vs. Generics

// This lesson explores the difference between associated types and generics
// in traits, and when to use each.

// --- Generics in Traits (Type Parameters) ---

// When a trait uses generics, it means that the implementor of the trait
// specifies the concrete type for the generic parameter.

trait IteratorWithGenerics<T> {
    fn next(&mut self) -> Option<T>;
}

struct CounterWithGenerics {
    count: u32,
}

impl IteratorWithGenerics<u32> for CounterWithGenerics {
    fn next(&mut self) -> Option<u32> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

// --- Associated Types in Traits ---

// When a trait uses associated types, it means that the trait itself defines
// a placeholder type, and the implementor of the trait specifies the concrete
// type for that placeholder.

// The standard library's `Iterator` trait uses an associated type.
// pub trait Iterator {
//     type Item;
//     fn next(&mut self) -> Option<Self::Item>;
// }

trait IteratorWithAssociatedType {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

struct CounterWithAssociatedType {
    count: u32,
}

impl IteratorWithAssociatedType for CounterWithAssociatedType {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

fn main() {
    // --- Generics in Traits Example ---
    let mut counter_gen = CounterWithGenerics { count: 0 };
    while let Some(i) = counter_gen.next() {
        println!("Generic Iterator: {}", i);
    }

    // --- Associated Types in Traits Example ---
    let mut counter_assoc = CounterWithAssociatedType { count: 0 };
    while let Some(i) = counter_assoc.next() {
        println!("Associated Type Iterator: {}", i);
    }

    // --- When to use which? ---

    // Use generics when you want to allow the *caller* of the trait to specify
    // the type. For example, a `Map<K, V>` trait where the user specifies K and V.

    // Use associated types when you want the *implementor* of the trait to specify
    // the type. For example, the `Iterator` trait, where the implementor specifies
    // the type of item that the iterator yields.
}

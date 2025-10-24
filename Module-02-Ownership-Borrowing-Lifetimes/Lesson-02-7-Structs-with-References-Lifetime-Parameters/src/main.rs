// Lesson 02.7: Structs with References & Lifetime Parameters

// So far, we've seen lifetimes on functions. We can also have lifetimes on structs.

// --- A Struct with a Reference ---

// If a struct has a field that is a reference, you must add a lifetime annotation
// to the struct definition.

struct ImportantExcerpt<'a> {
    part: &'a str,
}

// This says that an instance of `ImportantExcerpt` can't outlive the reference
// it holds in its `part` field.

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");

    let i = ImportantExcerpt {
        part: first_sentence,
    };

    println!("The important excerpt is: {}", i.part);

    // --- Lifetimes and Methods ---

    // If a struct has a lifetime, you need to declare it in the `impl` block as well.

    impl<'a> ImportantExcerpt<'a> {
        // This is a method on `ImportantExcerpt` that takes `&self`.
        // It passes the elision rules, so we don't need to write out the lifetimes.
        fn announce_and_return_part(&self, announcement: &str) -> &str {
            println!("Attention please: {}", announcement);
            self.part
        }
    }

    i.announce_and_return_part("I have an excerpt to share!");

    // --- The Static Lifetime ---

    // As we've seen, `'static` means the reference can live for the entire
    // duration of the program. String literals have a `'static` lifetime.

    let s: &'static str = "I have a static lifetime.";

    // This means you can create an `ImportantExcerpt` that holds a reference to a
    // string literal, and it will be valid for the whole program.
    let static_excerpt = ImportantExcerpt {
        part: s,
    };

    println!("Static excerpt: {}", static_excerpt.part);
}

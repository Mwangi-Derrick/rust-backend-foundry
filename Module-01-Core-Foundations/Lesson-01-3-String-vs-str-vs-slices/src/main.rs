// Lesson 01.3: String vs &str vs Slices

fn main() {
    // --- String: Owned and Growable ---

    // A `String` is a growable, heap-allocated string.
    // It's owned, meaning when it goes out of scope, it gets deallocated.
    let mut s1 = String::from("hello");
    s1.push_str(", world!"); // push_str() appends a literal to a String
    println!("s1 is: {}", s1);

    // --- &str: A "String Slice" or "String Literal" ---

    // A `&str` (pronounced "string slice") is an immutable reference to a sequence of UTF-8 bytes.
    // String literals are `&str`s that are stored in the binary of your program.
    let s2 = "hello, world!";
    println!("s2 is: {}", s2);

    // You can create a slice from a `String`.
    let s3: &str = &s1[0..5]; // s3 is a slice of s1
    println!("s3 is: {}", s3);

    // --- Slices: A View into a Collection ---

    // Slices are not just for strings! They are a general concept in Rust.
    // A slice is a reference to a contiguous sequence of elements in a collection.

    let arr = [1, 2, 3, 4, 5];
    let slice = &arr[1..3]; // A slice of the array
    println!("The first element of the slice is: {}", slice[0]);

    // --- Functions and &str ---

    // It's generally better to accept `&str` as a function argument
    // instead of `String`, because it's more flexible.
    // This function can accept both a `String` (by reference) and a `&str`.
    fn first_word(s: &str) -> &str {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }

        &s[..]
    }

    let word = first_word(&s1);
    println!("The first word of s1 is: {}", word);

    let word2 = first_word(s2);
    println!("The first word of s2 is: {}", word2);
}

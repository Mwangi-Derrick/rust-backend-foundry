# Lesson 01.3: String vs &str vs Slices

## 🧠 Concept Summary

This lesson tackles one of the most important and often confusing topics for newcomers to Rust: the difference between `String` and `&str`, and the more general concept of slices.

- **`String`**: An owned, growable, mutable string type. It is allocated on the heap. You should use `String` when you need to own or modify your string data.

- **`&str` (String Slice)**: An immutable reference to a sequence of UTF-8 bytes. It is a "view" into a string. String literals (e.g., `"hello"`) are of type `&'static str`. You should use `&str` when you just need to read a string, not own or modify it. It is more efficient to pass `&str` to functions than `String`.

- **Slices**: A more general concept that `&str` is a specific implementation of. A slice is a reference to a contiguous sequence of elements in a collection. You can have slices of arrays, vectors, and other collections.

## 🧩 Code Walkthrough

Let's dissect the code in `main.rs`.

### `String`: The Owned String Type

```rust
let mut s1 = String::from("hello");
s1.push_str(", world!");
println!("s1 is: {}", s1);
```

We create a `String` from a string literal. Because `String` is growable, we declare `s1` as mutable (`mut`) so we can append to it with `push_str`. `s1` owns the data on the heap.

### `&str`: The String Slice

```rust
let s2 = "hello, world!";
println!("s2 is: {}", s2);

let s3: &str = &s1[0..5];
println!("s3 is: {}", s3);
```

`s2` is a string literal, and its type is `&'static str`. The data is hardcoded into the executable. `s3` is a slice of our `String` `s1`. It's a reference to a portion of the data owned by `s1`. It does not own the data itself.

### Slices in General

```rust
let arr = [1, 2, 3, 4, 5];
let slice = &arr[1..3];
println!("The first element of the slice is: {}", slice[0]);
```

This shows that slices are not just for strings. `slice` is a reference to a part of the array `arr`. It allows us to work with a portion of the array without copying the data.

### Functions and `&str`

```rust
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
```

This is a key part of the lesson. The `first_word` function takes a `&str` as an argument. This is idiomatic Rust because it allows the function to be flexible. You can pass it a `&String` (by using `&s1`) or a `&str` literal (`s2`) directly. This is because Rust can coerce a `&String` into a `&str` (this is called a "deref coercion").

## ⚔️ Cross-Language Insights

- **vs. Golang:** Go has a `string` type, which is immutable. It's similar to Rust's `&str`. Go does not have a mutable string type like Rust's `String` in its standard library; you would typically use a byte slice (`[]byte`) for that.

- **vs. TypeScript:** In TypeScript, all strings are immutable. If you want to build a string, you typically use template literals or string concatenation, which create new strings. There isn't a direct equivalent to the `String`/`&str` distinction.

- **vs. C:** In C, a "string" is just a pointer to a character (`char*`) that is terminated by a null byte (`\0`). This is a common source of bugs (buffer overflows, use-after-free). Rust's `String`/`&str` system is designed to prevent these kinds of errors by tracking ownership and lifetimes.

## 🚀 Practical Reflection

- **Why the two types?** The `String`/`&str` distinction is a direct consequence of Rust's ownership system. `String` is for when you need to own and control the lifetime of the string data. `&str` is for when you just need to borrow a reference to a string for a short time.

- **API Design:** When you are writing a function that needs to read some text, you should almost always accept a `&str` as the argument. This makes your function more general and efficient.

- **Slices and Safety:** Slices are a safe way to work with portions of data. The borrow checker ensures that you can't have a slice that outlives the data it points to, which prevents dangling pointers.

## 🧩 Self-Review Prompts

- Write a function that takes a string slice (`&str`) and returns the *last* word.
- What happens if you try to modify a `String` while you have a slice of it? Try it and see what the compiler says.
- Why is a string literal a `&'static str`? What does the `'static` lifetime mean?

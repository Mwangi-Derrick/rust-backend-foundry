

// 🧩 Lesson 2: Ownership — The Secret Sauce of Rust

// This is what makes Rust special. No GC (Garbage Collector) like Go or JS — Rust enforces memory safety through rules.

// Rule 1: Each value has a single owner
// Rule 2: When the owner goes out of scope → value is dropped (freed)
// Rule 3: You can borrow, but not own twice

// Try this:
fn main() {
    let mut name = String::from("summafy");
    print_length(&name); // borrow here
    println!("Back in main: {}", name);
}

fn print_length(s: &String) {
    let len = s.len();
    println!("The length of '{}' is {}", s, len);
}

// “Notice the &name when calling the function? That’s a borrow.
// We’re saying: hey, print_length, you can look at this data, but don’t take ownership of it.
// This lets us use name again in main() — and Rust’s compiler guarantees we can’t accidentally mess with memory safety. No segfaults, no leaks, no GC needed.”



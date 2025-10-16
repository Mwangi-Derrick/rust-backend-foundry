

// 🎬 Lesson 3 — Mutable Borrowing & References

// In Lesson 2, we borrowed immutably — the function could read but not change the data.
// Now let’s learn to borrow mutably so a function can modify the value while keeping ownership safe.
fn main() {
    let mut name = String::from("summafy");
    add_io(&mut name);
    // let mut_ref1 = &mut name;
    // let mut_ref2 = &mut name;
println!("{} {}", mut_ref1, mut_ref2);
    println!("After function call: {}", name);
}

fn add_io(s: &mut String) {
    s.push_str(".io");
    println!("Inside function: {}", s);
}
// 💡 Instructor Commentary:
// “So what happened here?
// We passed &mut name — a mutable reference — into add_io().
// That means add_io can mutate the original string without taking ownership.
// Rust allows only one mutable reference at a time — no shared mut access!
// This rule prevents race conditions and data corruption at compile time 💥”

// 🧠 Mini Challenge

// Write a function that:

// Takes a mutable String

// Appends " - powered by Rust"

// Returns nothing, just modifies the input
// Then print the final string in main().

fn main(){
    let mut mutable = String::from("Summafy");
     append(&mut mutable);
    println!("concatenated string is {}", mutable);
}

fn append(string: &mut String){
string.push_str("- powered by Rust");
}


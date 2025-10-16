
// 🎯 Lesson 1: Variables, Mutability, and Types

// In Rust, variables are immutable by default — unlike Go or JS.

// 👉 Create a new playground:

fn main() {
    let x = 5;
    println!("x = {}", x);

    //Uncomment this and run — it will throw an error
    //x = 6;^ cannot assign twice to immutable variable

    let mut y = 10;
    println!("y before = {}", y);
    y = 15;
    println!("y after = {}", y);

    let s1 = String::from("hello");
    // let s2 = s1; // ownership moves to s2

    //  //println!("{}", s1); // ❌ error! s1 no longer valid ... let s2 = s1.clone(); // ownership moves to s2
    // println!("{}", s2);

     let s2 = &s1; // borrow, not move
    println!("s1 = {}, s2 = {}", s1, s2);
}

🧠 Concept:

let = immutable

let mut = mutable

println!() is a macro, not a function — that’s why it has !


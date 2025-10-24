// Lesson 02.1: The Ownership Model

fn main() {
    // --- Ownership Rules ---
    // 1. Each value in Rust has a variable that’s called its owner.
    // 2. There can only be one owner at a time.
    // 3. When the owner goes out of scope, the value will be dropped.

    // --- Variable Scope ---

    // `s` is not valid here, it’s not yet declared
    {
        let s = "hello"; // `s` is valid from this point forward
        // do stuff with `s`
        println!("s is: {}", s);
    } // this scope is now over, and `s` is no longer valid

    // --- The `String` Type and Move Semantics ---

    // `String` is allocated on the heap.
    let s1 = String::from("hello");

    // When we assign `s1` to `s2`, the data on the heap is not copied.
    // Instead, the pointer to the data is moved from `s1` to `s2`.
    // `s1` is now considered invalid.
    let s2 = s1;

    // This would cause a compile-time error: value borrowed here after move
    // println!("s1 is: {}", s1);

    println!("s2 is: {}", s2);

    // --- Cloning: Making a Deep Copy ---

    // If we want to have a deep copy of the heap data, we can use the `clone` method.
    let s3 = String::from("hello");
    let s4 = s3.clone();

    println!("s3 = {}, s4 = {}", s3, s4);

    // --- Stack-Only Data: The `Copy` Trait ---

    // For types that are stored entirely on the stack, like integers,
    // it's cheap to make a copy. These types implement the `Copy` trait.
    let x = 5;
    let y = x; // `x` is copied to `y`

    println!("x = {}, y = {}", x, y);

    // --- Ownership and Functions ---

    let s = String::from("hello"); // `s` comes into scope

    takes_ownership(s); // `s`'s value moves into the function...
                        // ... and so is no longer valid here

    // This would cause a compile-time error:
    // println!("s is: {}", s);

    let x = 5; // `x` comes into scope

    makes_copy(x); // `x` would move into the function,
                   // but i32 is Copy, so it’s okay to still
                   // use `x` afterward

    println!("x is still: {}", x);
} // Here, `x` goes out of scope, then `s`. But because `s`'s value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, `some_string` goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, `some_integer` goes out of scope. Nothing special happens.

"""// Welcome to the Rust Masterclass!
// This is Lesson 01.1: Variables, Mutability, and Shadowing

fn main() {
    // In Rust, variables are immutable by default.
    // This means once a value is bound to a name, you can't change it.
    let x = 5;
    println!("The value of x is: {}", x);

    // If you try to uncomment the line below, the compiler will throw an error.
    // x = 6; // Error: cannot assign twice to immutable variable `x`

    // To make a variable mutable, you use the `mut` keyword.
    let mut y = 10;
    println!("The initial value of y is: {}", y);
    y = 20;
    println!("The new value of y is: {}", y);

    // Shadowing is a powerful feature in Rust.
    // You can declare a new variable with the same name as a previous variable.
    // The new variable "shadows" the previous one.
    let z = 30;
    println!("The value of z is: {}", z);
    let z = z + 1;
    println!("The shadowed value of z is: {}", z);

    // You can even change the type of a shadowed variable.
    let spaces = "   ";
    let spaces = spaces.len();
    println!("The number of spaces is: {}", spaces);

    // Constants are always immutable and must be type-annotated.
    // They can be declared in any scope, including the global scope.
    const MAX_POINTS: u32 = 100_000;
    println!("The maximum points are: {}", MAX_POINTS);
}
""
// Lesson 01.2: Primitive Types & Type Inference

fn main() {
    // Rust has a variety of primitive types.

    // --- Scalar Types ---

    // Integers
    // Rust has signed (i) and unsigned (u) integers in sizes 8, 16, 32, 64, and 128 bits.
    let a: i32 = -10;
    let b: u32 = 10;

    // Type inference: Rust can often infer the type of a variable.
    let c = 20; // Inferred as i32 by default

    // Floating-Point Numbers
    let d: f64 = 2.0; // 64-bit float (default)
    let e: f32 = 3.0; // 32-bit float

    // Booleans
    let f: bool = true;
    let g = false; // Inferred as bool

    // Characters
    // Note the single quotes. `char` represents a single Unicode Scalar Value.
    let h: char = 'a';
    let i = '😻'; // Yes, emojis are chars!

    // --- Compound Types ---

    // Tuples
    // A tuple is a fixed-size collection of values of different types.
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup; // Destructuring a tuple
    println!("The value of y is: {}", y);

    let five_hundred = tup.0;
    println!("The first value of the tuple is: {}", five_hundred);

    // Arrays
    // An array is a fixed-size collection of values of the same type.
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let first = arr[0];
    println!("The first element of the array is: {}", first);

    // You can also initialize an array with the same value for all elements.
    let same_values = [3; 5]; // [3, 3, 3, 3, 3]
    println!("The second element of same_values is: {}", same_values[1]);

    println!("Scalar Types:");
    println!("  Integers: a={}, b={}, c={}", a, b, c);
    println!("  Floats: d={}, e={}", d, e);
    println!("  Booleans: f={}, g={}", f, g);
    println!("  Chars: h={}, i={}", h, i);
}

// Lesson 02.2: Move Semantics vs. Clone vs. Copy

fn main() {
    // --- Move Semantics in Detail ---

    // Let's revisit move semantics with a focus on what's happening in memory.
    let s1 = String::from("hello");
    // `s1` is a struct on the stack with three fields:
    // - a pointer to the heap-allocated buffer containing "hello"
    // - a `len` (length) of 5
    // - a `capacity` of 5

    let s2 = s1;
    // When we "move" `s1` to `s2`, we are copying the three fields on the stack.
    // We are NOT copying the data on the heap.
    // `s1` is now invalidated to prevent a "double free" error.

    // This is why this line would fail to compile:
    // println!("s1 is no longer valid: {}", s1);

    println!("s2 has taken ownership: {}", s2);

    // --- The `Copy` Trait ---

    // Types that implement the `Copy` trait do not move. They are copied.
    // These are types that are stored entirely on the stack.
    let x = 5; // i32 implements Copy
    let y = x;

    // Both `x` and `y` are valid.
    println!("x = {}, y = {}", x, y);

    // What types are `Copy`?
    // - All integer types (i32, u64, etc.)
    // - The boolean type (bool)
    // - All floating point types (f32, f64)
    // - The character type (char)
    // - Tuples, if they only contain types that are also `Copy`.
    //   (i32, i32) is `Copy`, but (i32, String) is not.

    // --- The `Clone` Trait ---

    // `Clone` is a trait for when you want to explicitly create a deep copy of a value.
    let s3 = String::from("world");
    let s4 = s3.clone();

    // `s3` and `s4` are two independent `String`s. They both have their own data on the heap.
    println!("s3 = {}, s4 = {}", s3, s4);

    // --- `Copy` vs. `Clone` ---

    // A key distinction: `Copy` is implicit and happens on assignment.
    // `Clone` is explicit and is called with the `.clone()` method.

    // If a type implements `Copy`, it will also implement `Clone`.
    // The implementation of `clone` for a `Copy` type is just to return a copy of the value.

    let a = 5; // i32 is Copy
    let b = a; // `a` is copied to `b`
    let c = a.clone(); // We can also explicitly clone it

    println!("a = {}, b = {}, c = {}", a, b, c);
}

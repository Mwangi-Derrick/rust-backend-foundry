// Lesson 13.2: PhantomData and Zero-Cost Abstractions

// This lesson explores `PhantomData` and the concept of zero-cost abstractions
// in Rust.

// --- `PhantomData<T>` ---

// `PhantomData<T>` is a marker type that tells the Rust compiler that a struct
// or enum acts as though it owns data of type `T`, even though it doesn't
// actually contain a `T`.

// It's used to trick the borrow checker into thinking that a type parameter
// is used, even if it's not. This is important for ensuring that lifetimes
// are correctly enforced.

use std::marker::PhantomData;

// A wrapper around a reference that has a lifetime parameter.
// We need `PhantomData` to tell the compiler that `T` is related to the lifetime `'a`.
struct MyWrapper<'a, T: 'a> {
    data: &'a T,
    _marker: PhantomData<T>,
}

impl<'a, T: 'a> MyWrapper<'a, T> {
    fn new(data: &'a T) -> Self {
        MyWrapper { data, _marker: PhantomData }
    }

    fn get_data(&self) -> &'a T {
        self.data
    }
}

// --- Zero-Cost Abstractions ---

// A zero-cost abstraction is an abstraction that doesn't incur any runtime
// overhead. Rust's generics and traits are examples of zero-cost abstractions.

// The compiler performs monomorphization, which means it generates specialized
// code for each concrete type that uses a generic function or struct. This
// means that there is no runtime cost for using generics.

// --- Example: A Generic Function ---

// This generic function has no runtime overhead compared to a specialized function.
fn add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

fn main() {
    // --- PhantomData Example ---

    let x = 10;
    let wrapper = MyWrapper::new(&x);
    println!("Wrapped data: {}", wrapper.get_data());

    // This would cause a compile-time error if `MyWrapper` didn't have `PhantomData`,
    // because the compiler wouldn't know that `MyWrapper` depends on the lifetime of `x`.
    //
    // { // new scope
    //     let y = 20;
    //     let wrapper2 = MyWrapper::new(&y);
    // } // y is dropped here, wrapper2 would have a dangling reference

    // --- Zero-Cost Abstractions Example ---

    println!("5 + 10 = {}", add(5, 10));
    println!("5.0 + 10.0 = {}", add(5.0, 10.0));
}

// Lesson 02.3: Borrowing & References

fn main() {
    // --- The Problem with Ownership ---

    // Sometimes we want to use a value without taking ownership of it.
    // For example, if we have a function that calculates the length of a String:

    fn calculate_length_takes_ownership(s: String) -> (String, usize) {
        let length = s.len(); // len() returns the length of a String
        (s, length)
    }

    let s1 = String::from("hello");
    let (s2, len) = calculate_length_takes_ownership(s1);
    println!("The length of '{}' is {}.
", s2, len);

    // This is cumbersome. We had to pass the String to the function, and then the
    // function had to return the String so we could use it again.

    // --- References and Borrowing ---

    // A reference is like a pointer in that it’s an address we can follow to access
    // data stored at that address. Unlike a pointer, a reference is guaranteed to
    // point to a valid value of a particular type for the life of that reference.

    // We can create a reference by using the `&` operator.
    // We call this "borrowing" because we are borrowing the value, not taking ownership.

    fn calculate_length_borrows(s: &String) -> usize {
        s.len()
    } // `s` goes out of scope, but because it does not have ownership of what
      // it refers to, nothing happens.

    let s3 = String::from("hello");
    let len = calculate_length_borrows(&s3);
    println!("The length of '{}' is {}.
", s3, len);

    // --- The Rules of References ---

    // 1. At any given time, you can have either one mutable reference or any number
    //    of immutable references.
    // 2. References must always be valid.

    // --- Mutable References ---

    // We can also borrow a value mutably, which allows us to change it.

    fn change(some_string: &mut String) {
        some_string.push_str(", world");
    }

    let mut s4 = String::from("hello");
    change(&mut s4);
    println!("s4 is now: {}", s4);

    // You can only have one mutable reference to a particular piece of data in a
    // particular scope.
    let mut s5 = String::from("hello");

    let r1 = &mut s5;
    // let r2 = &mut s5; // This would cause a compile-time error

    // println!("{}, {}", r1, r2);

    // This restriction prevents data races at compile time.
    // A data race is similar to a race condition and happens when these three
    // behaviors occur:
    // - Two or more pointers access the same data at the same time.
    // - At least one of the pointers is being used to write to the data.
    // - There’s no mechanism being used to synchronize access to the data.
}

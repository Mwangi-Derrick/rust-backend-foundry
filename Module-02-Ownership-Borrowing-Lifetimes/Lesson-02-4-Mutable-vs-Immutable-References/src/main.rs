// Lesson 02.4: Mutable vs. Immutable References

fn main() {
    // --- The Core Rule ---
    // You can have either ONE mutable reference OR ANY number of immutable references.
    // You cannot have both at the same time.

    // --- Many Immutable References ---

    let s1 = String::from("hello");

    let r1 = &s1;
    let r2 = &s1;

    println!("r1 = {}, r2 = {}", r1, r2);
    // This is fine. We can have as many immutable references as we want.

    // --- One Mutable Reference ---

    let mut s2 = String::from("hello");

    let r3 = &mut s2;
    // let r4 = &mut s2; // COMPILE ERROR: cannot borrow `s2` as mutable more than once at a time

    println!("r3 = {}", r3);

    // --- Mixing Mutable and Immutable References ---

    let mut s3 = String::from("hello");

    let r5 = &s3;
    let r6 = &s3;
    // let r7 = &mut s3; // COMPILE ERROR: cannot borrow `s3` as mutable because it is also borrowed as immutable

    // println!("r5 = {}, r6 = {}, r7 = {}", r5, r6, r7);

    // Why does this rule exist? Because if you have an immutable reference, you are
    // expecting the value not to change. If you could have a mutable reference at
    // the same time, the value could change out from under you.

    // --- The Scope of References ---

    // A reference’s scope starts from where it is introduced and continues through
    // the last time that reference is used.

    let mut s4 = String::from("hello");

    let r8 = &s4;
    let r9 = &s4;

    println!("{} and {}", r8, r9);
    // `r8` and `r9` are no longer used after this point, so their scope ends here.

    let r10 = &mut s4; // This is fine, because the immutable references are no longer in scope.
    r10.push_str(", world");
    println!("{}", r10);

    // --- Dangling References ---

    // The borrow checker also prevents dangling references.
    // A dangling reference is a reference that points to a memory location that has
    // been deallocated.

    // let reference_to_nothing = dangle();

    // fn dangle() -> &String { // dangle returns a reference to a String
    //     let s = String::from("hello"); // s is a new String

    //     &s // we return a reference to the String, s
    // } // Here, s goes out of scope, and is dropped. Its memory is deallocated.
      // Danger! `dangle` returns a reference to memory that has been deallocated.

    // The compiler will prevent this with the error: "missing lifetime specifier".
    // We will learn more about lifetimes in the next lesson.
}

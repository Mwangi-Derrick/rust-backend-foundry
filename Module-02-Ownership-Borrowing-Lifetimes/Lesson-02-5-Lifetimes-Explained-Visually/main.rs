// Lesson 02.5: Lifetimes Explained Visually

// Lifetimes are a way of telling the Rust compiler how long a reference is valid.
// Most of the time, lifetimes are inferred by the compiler. You don't have to
// write them out. But sometimes, the compiler needs your help.

// The main goal of lifetimes is to prevent dangling references.

fn main() {
    // --- A Simple Case ---

    let r;

    {
        let x = 5;
        r = &x;
        println!("r: {}", r);
    } // `x` is dropped here

    // println!("r: {}", r); // COMPILE ERROR: `x` does not live long enough

    // In the code above, `r` has a longer lifetime than `x`.
    // The compiler sees this and prevents it.

    // --- Lifetimes in Functions ---

    // Let's look at a function that takes two string slices and returns the longest one.

    fn longest(x: &str, y: &str) -> &str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    // This function won't compile without lifetime annotations.
    // The compiler doesn't know whether the returned reference will refer to `x` or `y`.
    // It needs to know this to ensure that the returned reference is valid.

    // --- Lifetime Annotation Syntax ---

    // Lifetime annotations don't change how long any of the references live.
    // They are a way of describing the relationships between the lifetimes of
    // multiple references to the compiler.

    // The syntax is `&'a T` for a reference with lifetime `'a`.

    fn longest_with_lifetimes<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    // This function signature now tells the compiler:
    // "For some lifetime 'a, the function takes two parameters, both of which are
    // string slices that live at least as long as 'a. The function will return a
    // string slice that also lives at least as long as 'a."

    // This is a contract. It tells the compiler that the lifetime of the returned
    // reference is connected to the lifetimes of the input references.

    let string3 = String::from("long string is long");
    {
        let string4 = String::from("xyz");
        let result = longest_with_lifetimes(string3.as_str(), string4.as_str());
        println!("The longest string is {}", result);
    }

    // --- The `'static` Lifetime ---

    // `'static` is a special lifetime that means the reference can live for the
    // entire duration of the program. All string literals have the `'static`
    // lifetime.

    let s: &'static str = "I have a static lifetime.";
}

// Lesson 02.6: Function Lifetimes

// This lesson focuses on how to use lifetime annotations in function signatures.

// --- The Three Lifetime Elision Rules ---

// The Rust compiler has three rules that it applies to function signatures to see
// if it can figure out the lifetimes without you having to write them out.
// These are called the "lifetime elision rules".

// Rule 1: Each parameter that is a reference gets its own lifetime parameter.
// fn foo(x: &i32) becomes fn foo<'a>(x: &'a i32)

// Rule 2: If there is exactly one input lifetime parameter, that lifetime is
// assigned to all output lifetime parameters.
// fn foo(x: &i32) -> &i32 becomes fn foo<'a>(x: &'a i32) -> &'a i32

// Rule 3: If there are multiple input lifetime parameters, but one of them is
// `&self` or `&mut self`, the lifetime of `self` is assigned to all output
// lifetime parameters.

// If the compiler can't figure out the lifetimes after applying these three
// rules, it will give you a compile-time error and you will have to add the
// lifetime annotations manually.

// --- A Function That Needs Lifetimes ---

// Let's revisit our `longest` function.

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// This function doesn't compile without lifetimes because it fails the elision
// rules. It has two input references, so Rule 2 doesn't apply. Neither of them
// is `&self`, so Rule 3 doesn't apply. The compiler doesn't know what to do.

// By adding the `'a` lifetime, we are telling the compiler that the returned
// reference will be valid for as long as both `x` and `y` are valid.

fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    // println!("The longest string is {}", result); // COMPILE ERROR

    // The code above won't compile because `string2` does not live long enough.
    // The lifetime of the returned reference `result` is tied to the shorter
    // lifetime of `string1` and `string2`. `string2` is dropped at the end of
    // the inner scope, so `result` is no longer valid.

    // --- A Function That Doesn't Need Lifetimes ---

    // This function takes a string slice and returns the first word.
    fn first_word(s: &str) -> &str {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }

        &s[..]
    }

    // This function compiles without lifetime annotations because it passes the
    // elision rules. It has one input reference, so Rule 2 applies. The compiler
    // gives the input and output references the same lifetime.

    let my_string = String::from("hello world");
    let word = first_word(&my_string);
    println!("the first word is: {}", word);
}

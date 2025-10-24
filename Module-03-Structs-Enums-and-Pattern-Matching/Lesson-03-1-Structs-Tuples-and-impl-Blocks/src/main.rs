// Lesson 03.1: Structs, Tuples, and impl Blocks

// A struct, or structure, is a custom data type that lets you package together
// and name multiple related values that make up a meaningful group.

// --- Defining and Instantiating Structs ---

// There are three types of structs: structs with named fields, tuple structs,
// and unit-like structs.

// A struct with named fields
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// A tuple struct
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// A unit-like struct (useful for generics and traits)
struct AlwaysEqual;

fn main() {
    // Creating an instance of a struct
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    // Accessing and modifying fields
    user1.email = String::from("anotheremail@example.com");

    // Field init shorthand syntax
    fn build_user(email: String, username: String) -> User {
        User {
            email, // `email: email,` is not needed
            username, // `username: username,` is not needed
            active: true,
            sign_in_count: 1,
        }
    }

    // Struct update syntax
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1 // The `..` syntax specifies that the remaining fields not
                // explicitly set should have the same value as the fields
                // in the given instance.
    };

    // Creating instances of tuple structs
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // --- `impl` Blocks: Adding Methods to Structs ---

    // An `impl` block is where you define methods for a struct.

    #[derive(Debug)] // This allows us to print the struct for debugging
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        // This is a method. The first parameter is always `&self`.
        fn area(&self) -> u32 {
            self.width * self.height
        }

        // We can have methods with the same name as a field.
        fn width(&self) -> bool {
            self.width > 0
        }

        // A method that takes another instance of the same struct
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
    }

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

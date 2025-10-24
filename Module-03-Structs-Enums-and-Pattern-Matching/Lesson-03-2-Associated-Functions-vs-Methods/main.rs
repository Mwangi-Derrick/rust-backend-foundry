// Lesson 03.2: Associated Functions vs. Methods

// In the last lesson, we learned about methods, which are functions that are
// associated with a struct and take `&self` as their first parameter.

// Rust also has **associated functions**, which are functions that are associated
// with a struct but do not take `&self` as a parameter. They are often used as
// constructors.

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // This is a method. It takes `&self`.
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // This is an associated function. It does not take `&self`.
    // It is often used as a constructor to create a new instance of the struct.
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    // Calling a method
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("The area of the rectangle is {}", rect1.area());

    // Calling an associated function
    // We use the `::` syntax with the struct name to call an associated function.
    let sq = Rectangle::square(3);
    println!("sq is {:#?}", sq);

    // --- Multiple `impl` Blocks ---

    // You can have multiple `impl` blocks for the same struct.
    // This can be useful for organizing your code.

    impl Rectangle {
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
    }

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
}

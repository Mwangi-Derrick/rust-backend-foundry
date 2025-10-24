// Lesson 04.5: Trait Objects vs. Static Dispatch

// This lesson covers two different ways that Rust implements polymorphism:
// static dispatch and dynamic dispatch.

// --- Static Dispatch ---

// Static dispatch is the process of resolving which function to call at compile
// time. This is what happens when we use generics and trait bounds.

pub trait Summary {
    fn summarize(&self) -> String;
}

// This function uses static dispatch. The compiler will generate a specific
// version of this function for each type that it is called with.
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// --- Dynamic Dispatch with Trait Objects ---

// Dynamic dispatch is the process of resolving which function to call at runtime.
// This is what happens when we use trait objects.

// A trait object is a pointer to an instance of a type that implements a certain
// trait. We can create a trait object by taking a reference to a type that
// implements a trait and casting it to a reference to the trait.

// This function uses dynamic dispatch. It takes a trait object, which is a
// reference to any type that implements the `Summary` trait.
pub fn notify_dynamic(item: &dyn Summary) {
    println!("Breaking news! {}", item.summarize());
}

// We can also have a vector of trait objects.

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub trait Draw {
    fn draw(&self);
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // code to actually draw a button
        println!("Drawing a button: {}", self.label);
    }
}

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // code to actually draw a select box
        println!("Drawing a select box with options: {:?}", self.options);
    }
}

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();
}

// Lesson 11.2: RefCell & Interior Mutability

// This lesson introduces `RefCell<T>` and the concept of interior mutability.

// --- The Problem with Immutable References ---

// Rust's borrowing rules state that you can have either one mutable reference
// or any number of immutable references. This is great for safety, but sometimes
// you need to mutate data even when you have an immutable reference to it.

// For example, if you have a data structure that is shared by multiple owners
// (using `Rc<T>`), and you want to mutate that data, you can't just get a
// mutable reference because `Rc<T>` only gives you immutable references.

// --- Interior Mutability ---

// Interior mutability is a design pattern in Rust that allows you to mutate
// data even when you have an immutable reference to it. This is achieved by
// using smart pointers that enforce the borrowing rules at runtime instead of
// compile time.

// --- `RefCell<T>` ---

// `RefCell<T>` is a single-threaded smart pointer that provides interior
// mutability. It allows you to get a mutable reference to the inner data even
// when you only have an immutable reference to the `RefCell` itself.

// `RefCell` enforces the borrowing rules at runtime. If you try to violate the
// rules (e.g., by having multiple mutable references), `RefCell` will panic.

use std::cell::RefCell;
use std::rc::Rc;

fn refcell_example() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::clone(&value);
    let b = Rc::clone(&value);

    // We can get an immutable reference to the inner value.
    println!("a = {}", *a.borrow());

    // We can get a mutable reference to the inner value.
    *b.borrow_mut() += 10;

    println!("a = {}", *a.borrow());

    // This would cause a runtime panic:
    // let mut one_borrow = value.borrow_mut();
    // let mut two_borrow = value.borrow_mut();
}

// --- Use Case: Mock Objects ---

// `RefCell` is often used in testing to create mock objects that can be mutated
// even when they are passed as immutable references.

trait Messenger {
    fn send(&self, msg: &str);
}

struct MockMessenger {
    sent_messages: RefCell<Vec<String>>,
}

impl MockMessenger {
    fn new() -> MockMessenger {
        MockMessenger {
            sent_messages: RefCell::new(vec![]),
        }
    }
}

impl Messenger for MockMessenger {
    fn send(&self, msg: &str) {
        self.sent_messages.borrow_mut().push(String::from(msg));
    }
}

fn main() {
    println!("--- RefCell Example ---");
    refcell_example();

    println!("\n--- Mock Messenger Example ---");
    let mock_messenger = MockMessenger::new();
    mock_messenger.send("hello");
    mock_messenger.send("world");
    println!("Sent messages: {:?}", mock_messenger.sent_messages.borrow());
}


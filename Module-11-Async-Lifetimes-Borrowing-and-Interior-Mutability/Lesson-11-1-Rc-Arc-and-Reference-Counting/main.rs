// Lesson 11.1: Rc, Arc, and Reference Counting

// This lesson introduces `Rc` (Reference Counted) and `Arc` (Atomic Reference
// Counted), which are smart pointers that enable multiple ownership of data.

// --- The Problem with Multiple Ownership ---

// In Rust, the ownership rules state that there can only be one owner of a piece
// of data at a time. This is great for memory safety, but sometimes you need
// to have multiple owners.

// For example, if you have a graph data structure, multiple nodes might want to
// own the same edge. Or if you have a UI, multiple widgets might want to own
// the same data.

// --- `Rc<T>`: Reference Counting ---

// `Rc<T>` is a single-threaded reference counting smart pointer. It allows
// multiple owners of data. When the last `Rc` goes out of scope, the data is
// dropped.

use std::rc::Rc;

fn rc_example() {
    let a = Rc::new(String::from("hello"));
    println!("Count after creating a: {}", Rc::strong_count(&a));

    let b = Rc::clone(&a);
    println!("Count after creating b: {}", Rc::strong_count(&a));

    { // new scope
        let c = Rc::clone(&a);
        println!("Count after creating c: {}", Rc::strong_count(&a));
    } // c goes out of scope

    println!("Count after c goes out of scope: {}", Rc::strong_count(&a));
}

// --- `Arc<T>`: Atomic Reference Counting ---

// `Arc<T>` is a multi-threaded reference counting smart pointer. It is similar
// to `Rc<T>`, but it is safe to share across threads. This comes with a slight
// performance overhead compared to `Rc<T>`.

use std::sync::Arc;
use tokio::task;

async fn arc_example() {
    let a = Arc::new(String::from("hello"));
    println!("Count after creating a: {}", Arc::strong_count(&a));

    let mut handles = vec![];

    for i in 0..3 {
        let a_clone = Arc::clone(&a);
        let handle = task::spawn(async move {
            println!("Task {} has a: {}", i, a_clone);
            // Simulate some work
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap();
    }

    println!("Count after tasks finish: {}", Arc::strong_count(&a));
}

fn main() {
    println!("--- Rc Example ---");
    rc_example();

    println!("\n--- Arc Example ---");
    // Arc example needs to be run in an async context
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(arc_example());
}

// 🎯 Example 1 — Defining a Trait
trait Relay {
    fn relay(&self);
}

// Let’s model something close to your system: any service that can relay messages.


// // Now let’s implement it for two different structs:
struct UploadService;
struct PaymentService;

impl Relay for UploadService {
    fn relay(&self) {
        println!("Relaying upload message to Cloud Run...");
    }
}

impl Relay for PaymentService {
    fn relay(&self) {
        println!("Relaying payment message to billing microservice...");
    }
}

// fn main() {
//     let upload = UploadService;
//     let payment = PaymentService;

//     upload.relay();
//     payment.relay();
// }

// 🧠 Concept Breakdown

// trait = defines shared behavior (like Go interfaces)

// impl Trait for Type = gives that type the ability

// You can use any type that implements the trait in functions and generics



// 🎯 Example 2 — Traits + Generics Together

// We can write a generic function that accepts any type implementing the Relay trait:

fn process<T: Relay>(service: T) {
    service.relay();
}

// fn main() {
//     let upload = UploadService;
//     let payment = PaymentService;

//     process(upload);
//     process(payment);
// }


// Same behavior — but now we can pass in any future service that implements Relay.

// 🎥 Instructor Commentary:

// “This is how Rust achieves zero-cost abstraction.
// You get polymorphism like OOP, but at compile time.
// No runtime overhead, no boxing, no GC — pure speed and safety.” ⚡

// 🧩 Mini Challenge

// Create a new struct:

// struct NotificationService;


// Implement the Relay trait for it — print something like:
// "Relaying user notification event..."

// Then call process(notification_service) inside main().
struct NotificationService;
impl Relay for NotificationService{
    fn relay(&self) {
      println!("Relaying user notification event...")
    }
}

fn main() {
    let notification = NotificationService;
  
    process(notification);
  
}

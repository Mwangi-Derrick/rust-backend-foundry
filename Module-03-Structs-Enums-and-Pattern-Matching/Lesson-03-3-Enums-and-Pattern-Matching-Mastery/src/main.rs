// Lesson 03.3: Enums and Pattern Matching Mastery

// Enums, or enumerations, allow you to define a type by enumerating its
// possible variants.

// --- Defining an Enum ---

// An enum can have variants with no data, with data, or with different kinds of data.

enum Message {
    Quit, // No data
    Move { x: i32, y: i32 }, // Anonymous struct
    Write(String), // A single String
    ChangeColor(i32, i32, i32), // A tuple of three i32s
}

// We can also define methods on enums, just like with structs.
impl Message {
    fn call(&self) {
        // method body would be defined here
        println!("Calling a message...");
    }
}

fn main() {
    let m = Message::Write(String::from("hello"));
    m.call();

    // --- The `match` Control Flow Operator ---

    // `match` is a powerful control flow operator that allows you to compare a
    // value against a series of patterns and execute code based on which pattern
    // matches.

    fn process_message(msg: Message) {
        match msg {
            Message::Quit => {
                println!("The Quit variant has no data to destructure.");
            }
            Message::Move { x, y } => {
                println!(
                    "Move in the x direction {} and y direction {}",
                    x,
                    y
                );
            }
            Message::Write(text) => {
                println!("Text message: {}", text);
            }
            Message::ChangeColor(r, g, b) => {
                println!(
                    "Change the color to red {}, green {}, and blue {}",
                    r,
                    g,
                    b
                );
            }
        }
    }

    process_message(Message::Quit);
    process_message(Message::Move { x: 10, y: 20 });
    process_message(Message::Write(String::from("hello")));
    process_message(Message::ChangeColor(0, 160, 255));

    // --- `if let` ---

    // `if let` is a less verbose way to handle a `match` that only cares about
    // one of the variants.

    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }

    // This can be written as:
    if let Some(3) = some_u8_value {
        println!("three
");
    } else {
        println!("not three");
    }
}

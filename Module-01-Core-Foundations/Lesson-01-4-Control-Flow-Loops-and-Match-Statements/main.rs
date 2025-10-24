// Lesson 01.4: Control Flow, Loops, and Match Statements

fn main() {
    // --- if/else Expressions ---

    // `if` expressions are pretty standard. The condition must be a `bool`.
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // `if` is an expression, which means it can return a value.
    // The types of the values in each block must be the same.
    let condition = true;
    let x = if condition { 5 } else { 6 };
    println!("The value of x is: {}", x);

    // --- Loops ---

    // `loop` creates an infinite loop. You can use `break` to exit.
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result of the loop is: {}", result);

    // `while` loops are also standard.
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");

    // `for` loops are used to iterate over a collection.
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is: {}", element);
    }

    // You can also create a range to iterate over.
    for number in (1..4).rev() { // (1..4) is a range from 1 to 3. .rev() reverses it.
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");

    // --- match Statements ---

    // `match` is like a `switch` statement on steroids.
    // It's exhaustive, meaning you must cover all possible cases.
    let some_number = 5;
    match some_number {
        1 => println!("one"),
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        13..=19 => println!("A teen"),
        _ => println!("Ain't special"),
    }

    // `match` can also return a value.
    let boolean = true;
    let binary = match boolean {
        false => 0,
        true => 1,
    };
    println!("{} -> {}", boolean, binary);
}

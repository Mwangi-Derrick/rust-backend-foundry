// Lesson 09.1: Mod, Pub, and use

// This lesson introduces Rust's module system, which allows you to organize
// your code into a hierarchy of modules.

// --- Modules ---

// A module is a namespace that contains definitions of functions, structs,
// enums, traits, and other modules.

// By default, everything in a module is private. You can use the `pub` keyword
// to make an item public, which means it can be accessed from outside the module.

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

// --- Paths for Referring to an Item in the Module Tree ---

// You can refer to an item in a module using a path. A path can be absolute
// (starting from the crate root) or relative (starting from the current module).

// The `use` keyword brings a path into scope, so you can refer to it with a
// shorter name.

use crate::front_of_house::hosting;
// use self::front_of_house::hosting; // This is also valid

// You can also use `pub use` to re-export an item, which means that code that
// can access your module can also access the re-exported item.
pub use self::front_of_house::hosting as hosting_reexported;

fn main() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // With `use`
    hosting::add_to_waitlist();

    // With `pub use`
    hosting_reexported::add_to_waitlist();
}

// --- Separating Modules into Different Files ---

// You can also put modules in separate files. If you have a module named
// `front_of_house`, you can put its contents in a file named
// `front_of_house.rs` or `front_of_house/mod.rs`.

// For example, if we had a file named `front_of_house.rs` with the following
// contents:
//
// pub mod hosting {
//     pub fn add_to_waitlist() {}
// }
//
// We could then have the following in our `main.rs` file:
//
// mod front_of_house;
//
// use crate::front_of_house::hosting;
//
// fn main() {
//     hosting::add_to_waitlist();
// }

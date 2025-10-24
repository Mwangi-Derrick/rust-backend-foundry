// Lesson 10.1: Tokio vs. Async-Std vs. Smol

// This lesson is a discussion of the different async runtimes that are available
// in the Rust ecosystem. We will compare Tokio, async-std, and smol.

// --- The Big Three ---

// - **Tokio:** The most popular and widely used async runtime. It is very
//   performant and has a rich ecosystem of libraries. It is a good choice for
//   building high-performance network services.

// - **async-std:** A runtime that aims to be a "standard library" for async
//   programming. It provides async versions of the standard library's APIs,
//   such as `async_std::fs` and `async_std::net`.

// - **smol:** A smaller, simpler runtime that is designed to be easy to understand
//   and use. It is a good choice for smaller projects or for learning about
//   async programming.

// --- Choosing a Runtime ---

// The choice of which runtime to use depends on your needs.

// - If you are building a high-performance network service, Tokio is a good
//   choice.
// - If you want an API that is similar to the standard library, async-std is a
//   good choice.
// - If you want a simple runtime for a smaller project, smol is a good choice.

// It is also possible to write libraries that are runtime-agnostic, which means
// they can be used with any runtime. This is done by using the `Future` trait
// from the standard library and not depending on any specific runtime features.

fn main() {
    println!("This lesson is a discussion of the different async runtimes.");
    println!("There is no code to run for this lesson.");
}

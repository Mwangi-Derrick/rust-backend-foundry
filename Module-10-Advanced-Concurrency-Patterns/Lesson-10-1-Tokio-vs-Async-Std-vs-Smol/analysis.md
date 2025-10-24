# Lesson 10.1: Tokio vs. Async-Std vs. Smol

## 🧠 Concept Summary

This lesson is a discussion of the different **async runtimes** that are available in the Rust ecosystem. The `async/await` syntax is part of the Rust language, but the runtime that executes the `async` code is a library. The three most popular runtimes are **Tokio**, **async-std**, and **smol**.

- **Tokio:** The most popular and widely used async runtime. It is very performant and has a rich ecosystem of libraries. It is a good choice for building high-performance network services. Tokio uses a multi-threaded, work-stealing scheduler.

- **async-std:** A runtime that aims to be a "standard library" for async programming. It provides async versions of the standard library's APIs, such as `async_std::fs` and `async_std::net`. It is a good choice if you want an API that is familiar and easy to use.

- **smol:** A smaller, simpler runtime that is designed to be easy to understand and use. It is a good choice for smaller projects or for learning about async programming.

- **Runtime-agnostic Libraries:** It is possible to write libraries that are runtime-agnostic, which means they can be used with any runtime. This is done by using the `Future` trait from the standard library and not depending on any specific runtime features.

## 🧩 Code Walkthrough

There is no code to run for this lesson. It is a conceptual overview of the different async runtimes.

## ⚔️ Cross-Language Insights

- The existence of multiple async runtimes is somewhat unique to Rust. In other languages, there is typically one standard async runtime (e.g., the Node.js runtime for TypeScript, the Go runtime for Go).

## 🚀 Practical Reflection

- **Which one should I use?** For most projects, especially network services, **Tokio** is the recommended choice. It has the largest community, the most libraries, and the best performance.

- **The `async-compat` Crate:** The `async-compat` crate is a library that allows you to use libraries from different runtimes together. For example, you can use a library that is written for `async-std` in a Tokio application.

- **The Future of Async Runtimes:** There is ongoing discussion in the Rust community about whether there should be a single, standard async runtime. For now, the ecosystem of multiple runtimes is a source of both innovation and some fragmentation.

## 🧩 Self-Review Prompts

- Look at the documentation for `async-std`. How does its API compare to Tokio's?
- What is a work-stealing scheduler?
- Find a library on `crates.io` that is runtime-agnostic.

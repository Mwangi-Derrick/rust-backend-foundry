// Lesson 12.5: Deploying to Cloudflare Workers

// This lesson demonstrates how to deploy Rust-generated WebAssembly (WASM)
// to Cloudflare Workers, a serverless platform that runs on Cloudflare's edge
// network.

// --- What are Cloudflare Workers? ---

// Cloudflare Workers are serverless functions that run on Cloudflare's global
// network. They allow you to deploy code that runs at the edge, close to your
// users, which can significantly reduce latency.

// Workers are built on WebAssembly and JavaScript, making Rust a natural fit
// for writing high-performance edge logic.

// --- `wrangler` ---

// `wrangler` is the official CLI tool for Cloudflare Workers. It helps you
// develop, test, and deploy your Workers.

// --- Example: A Simple Rust Worker ---

// To demonstrate, we'll create a simple Rust Worker that responds with "Hello, World!"

// This code is meant to be part of a library crate that is built for WASM
// and deployed with `wrangler`.

// ```rust
// // my_worker/src/lib.rs
//
// use wasm_bindgen::prelude::*;
// use worker::{event, console_log, Response, Result};
//
// #[event(fetch)]
// pub async fn main(req: worker::Request, env: worker::Env) -> Result<Response> {
//     console_log!("Request received: {:?}", req.url()?);
//     Response::ok("Hello, World from Rust Worker!")
// }
// ```

// --- Building and Deploying with `wrangler` ---

// 1. Create a new worker project: `wrangler generate my-worker --type rust`
//    This will set up a new Rust project configured for Cloudflare Workers.
// 2. Place the Rust code above into `my-worker/src/lib.rs`.
// 3. Build and deploy: `wrangler deploy` (from the `my-worker` directory).
//    This will compile the Rust code to WASM and deploy it to Cloudflare.

fn main() {
    println!("This lesson is about deploying Rust WASM to Cloudflare Workers.");
    println!("The code for this lesson is conceptual and requires setting up a separate");
    println!("Rust worker project and deploying it with `wrangler`.");
}

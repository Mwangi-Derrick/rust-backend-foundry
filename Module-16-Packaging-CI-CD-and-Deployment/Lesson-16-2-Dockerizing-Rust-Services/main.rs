// Lesson 16.2: Dockerizing Rust Services

// This lesson focuses on Dockerizing Rust services, which involves creating
// Docker images for your Rust applications. Docker provides a consistent
// environment for building, shipping, and running applications.

// --- Why Dockerize Rust Services? ---

// - **Consistency:** Ensures your application runs the same way in development,
//   testing, and production environments.
// - **Isolation:** Containers isolate your application and its dependencies from
//   the host system and other containers.
// - **Portability:** Docker images can be easily moved between different hosts
//   and cloud providers.
// - **Scalability:** Containers are a fundamental building block for scalable
//   microservices architectures.

// --- Dockerfile Strategies for Rust ---

// There are several strategies for creating Dockerfiles for Rust applications:
// 1.  **Multi-stage Builds:** Use one stage to compile the Rust application and
//     another stage to create a minimal runtime image. This is the recommended
//     approach for production.
// 2.  **Single-stage Builds:** Compile and run in the same image. Simpler but
//     results in larger images.

// --- Example: Multi-stage Dockerfile for a Rust Service ---

// Let's assume we have a simple Rust binary crate named `my_rust_app`.

// ```dockerfile
// # Stage 1: Builder
// FROM rust:1.70-slim-bullseye AS builder
//
// # Install musl-tools for static linking
// RUN apt-get update && apt-get install -y musl-tools && rm -rf /var/lib/apt/lists/*
//
// # Set the musl target
// RUN rustup target add x86_64-unknown-linux-musl
//
// WORKDIR /app
//
// # Copy Cargo.toml and Cargo.lock to leverage Docker cache
// COPY Cargo.toml Cargo.lock ./ 
//
// # Create a dummy src/main.rs to build dependencies
// RUN mkdir src && echo "fn main() {}" > src/main.rs
// RUN cargo build --release --target x86_64-unknown-linux-musl
//
// # Copy actual source code
// COPY src ./src
//
// # Rebuild with actual source code
// RUN cargo build --release --target x86_64-unknown-linux-musl
//
// # Stage 2: Runner
// FROM scratch
//
// # Copy the static binary from the builder stage
// COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/my_rust_app /usr/local/bin/my_rust_app
//
// # Run the application
// CMD ["/usr/local/bin/my_rust_app"]
// ```

// --- Example: A Simple Rust Program for Docker ---

fn main() {
    println!("Hello from a Dockerized Rust service!");
    println!("This program demonstrates Dockerizing Rust applications.");
}

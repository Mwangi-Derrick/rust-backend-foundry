// Lesson 16.1: Building Static Binaries

// This lesson focuses on building static binaries in Rust, which are self-contained
// executables that do not depend on external shared libraries. This is particularly
// useful for deployment in containerized environments (like Docker) or for creating
// easily distributable command-line tools.

// --- Why Static Binaries? ---

// - **Portability:** A static binary can be run on any system with a compatible
//   CPU architecture, without needing to install specific runtime libraries.
// - **Simplicity of Deployment:** No need to manage dependencies on the target system.
// - **Containerization:** Results in smaller Docker images, as you don't need to
//   include a full operating system with shared libraries.

// --- `musl` Target ---

// The `musl` libc implementation is a lightweight alternative to `glibc` (the
// default on most Linux distributions). Compiling with the `musl` target produces
// fully static executables.

// --- Steps to Build a Static Binary ---

// 1.  **Add the `musl` target:**
//     `rustup target add x86_64-unknown-linux-musl`
//     (Replace `x86_64` with your target architecture if different).

// 2.  **Build with the `musl` target:**
//     `cargo build --release --target x86_64-unknown-linux-musl`

// --- Example: A Simple Rust Program ---

fn main() {
    println!("Hello from a potentially static binary!");
    println!("This program demonstrates building static executables.");
    println!("To verify, check the executable's dependencies on a Linux system:");
    println!("ldd ./target/x86_64-unknown-linux-musl/release/your_program_name");
    println!("It should report 'not a dynamic executable'.");
}

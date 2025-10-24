// Lesson 09.3: Multi-crate Workspaces

// A workspace is a set of crates that share a common `Cargo.lock` file and
// output directory. Workspaces are a convenient way to manage multiple related
// crates.

// --- Creating a Workspace ---

// To create a workspace, you create a `Cargo.toml` file in the root of your
// project that contains a `[workspace]` section. This `Cargo.toml` file does
// not have a `[package]` section.

// The `[workspace]` section should contain a `members` key, which is a list of
// the crates that are part of the workspace.

// For example, if we have a workspace with a library crate `my_library` and a
// binary crate `my_app`, the root `Cargo.toml` file would look like this:

// ```toml
// [workspace]
// members = [
//     "my_app",
//     "my_library",
// ]
// ```

// --- Building and Running a Workspace ---

// You can build all of the crates in a workspace with `cargo build` from the
// root of the workspace.

// You can run a specific binary crate in a workspace with `cargo run -p <crate_name>`.
// For example, `cargo run -p my_app`.

// --- Dependencies in a Workspace ---

// Crates in a workspace can depend on each other. For example, `my_app` can
// depend on `my_library` by adding it to its `[dependencies]` section in its
// own `Cargo.toml` file.

// ```toml
// # my_app/Cargo.toml
//
// [dependencies]
// my_library = { path = "../my_library" }
// ```

// A key advantage of workspaces is that all crates in the workspace share the
// same `Cargo.lock` file. This means that they will all use the same versions
// of their dependencies.

fn main() {
    println!("This lesson is about multi-crate workspaces.");
    println!("The code for this lesson is conceptual and is meant to be run");
    println!("by creating a workspace with multiple crates.");
}

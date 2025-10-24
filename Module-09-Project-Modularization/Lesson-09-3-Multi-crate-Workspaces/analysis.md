# Lesson 09.3: Multi-crate Workspaces

## 🧠 Concept Summary

This lesson introduces **workspaces**, which are a feature of Cargo that makes it easier to work with multiple related crates.

- **Workspace:** A workspace is a set of crates that share a common `Cargo.lock` file and output directory. This is useful when you have a project that is composed of multiple library crates and a binary crate that uses them.

- **Creating a Workspace:** To create a workspace, you create a `Cargo.toml` file in the root of your project that contains a `[workspace]` section. This `Cargo.toml` file does not have a `[package]` section. The `[workspace]` section should contain a `members` key, which is a list of the crates that are part of the workspace.

- **Benefits of Workspaces:**
    - All crates in the workspace share the same `Cargo.lock` file, which means they will all use the same versions of their dependencies.
    - You can run commands like `cargo build` and `cargo test` from the root of the workspace to build and test all of the crates at once.

## 🧩 Code Walkthrough

The code for this lesson is conceptual and is meant to be run by creating a workspace with multiple crates.

### Creating a Workspace

1.  Create a new directory for your workspace: `mkdir my_workspace && cd my_workspace`.
2.  Create the root `Cargo.toml` file with the `[workspace]` section:

    ```toml
    // my_workspace/Cargo.toml

    [workspace]
    members = [
        "my_app",
        "my_library",
    ]
    ```

3.  Create the `my_library` crate: `cargo new my_library --lib`.
4.  Create the `my_app` crate: `cargo new my_app`.
5.  Add a dependency on `my_library` in `my_app/Cargo.toml`:

    ```toml
    // my_app/Cargo.toml

    [dependencies]
    my_library = { path = "../my_library" }
    ```

### Building and Running the Workspace

-   Build all crates: `cargo build` (from the root of the workspace).
-   Run the binary crate: `cargo run -p my_app`.
-   Test all crates: `cargo test`.

## ⚔️ Cross-Language Insights

- **vs. Golang:** A workspace is similar to a Go module that contains multiple packages.

- **vs. TypeScript:** A workspace is similar to a monorepo that is managed with a tool like Lerna or Nx.

- **vs. C:** C does not have a direct equivalent to workspaces. You would typically use a build system like Make or CMake to manage multiple related libraries and executables.

## 🚀 Practical Reflection

- **Large Projects:** Workspaces are essential for managing large Rust projects. They allow you to break your code down into smaller, more manageable crates, while still having a single, unified build process.

- **The Outbox Relay Project:** Our outbox relay project is a good candidate for a workspace. We could have a library crate for the core logic, and a binary crate for the executable that runs the relay.

- **Publishing Crates from a Workspace:** You can publish crates from a workspace to `crates.io`. Each crate is published separately.

## 🧩 Self-Review Prompts

- Create a workspace with two library crates and a binary crate that uses both of them.
- What is the difference between a virtual manifest and a regular manifest in Cargo?
- How can you use a `[patch]` section in `Cargo.toml` to override a dependency in a workspace?

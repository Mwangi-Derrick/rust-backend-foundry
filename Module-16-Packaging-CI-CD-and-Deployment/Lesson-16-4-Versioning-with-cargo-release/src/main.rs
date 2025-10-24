// Lesson 16.4: Versioning with cargo-release

// This lesson introduces `cargo-release`, a Cargo subcommand that automates
// the process of releasing new versions of your Rust crates, including version
// bumping, Git tagging, and publishing to `crates.io`.

// --- Why Automated Versioning? ---

// - **Consistency:** Ensures that version numbers are updated consistently across
//   your project (e.g., in `Cargo.toml`).
// - **Reproducibility:** Tags your Git repository with the release version, making
//   it easy to find the exact code state for any release.
// - **Efficiency:** Automates tedious manual steps, reducing human error and
//   speeding up the release process.
// - **Semantic Versioning:** Encourages adherence to Semantic Versioning (SemVer),
//   which is crucial for library users.

// --- `cargo-release` Features ---

// `cargo-release` can:
// - Bump the version number in `Cargo.toml`.
// - Commit the version bump to Git.
// - Create a Git tag for the release.
// - Push the changes and tags to the remote repository.
// - Publish the crate to `crates.io`.

// --- Setup ---

// To use `cargo-release`, you need to install it:
//
// `cargo install cargo-release`

// --- Example Usage ---

// Assuming your `Cargo.toml` has a version (e.g., `version = "0.1.0"`)

// 1.  **Dry run (recommended first):**
//     `cargo release --dry-run patch`
//     This will show you what `cargo-release` would do without actually making
//     any changes.

// 2.  **Release a patch version:**
//     `cargo release patch`
//     This will bump the version from `0.1.0` to `0.1.1`, commit, tag, and push.
//     If you have `publish = true` in your `Cargo.toml`, it will also publish
//     to `crates.io`.

// 3.  **Release a minor version:**
//     `cargo release minor`
//     Bumps from `0.1.1` to `0.2.0`.

// 4.  **Release a major version:**
//     `cargo release major`
//     Bumps from `0.2.0` to `1.0.0`.

// --- `Cargo.toml` Configuration ---

// You can configure `cargo-release` in your `Cargo.toml` under the `[package.metadata.release]`
// section. For example, to prevent publishing:
//
// [package.metadata.release]
// publish = false

fn main() {
    println!("This lesson focuses on versioning with cargo-release.");
    println!("The code for this lesson is conceptual and demonstrates the usage");
    println!("of the `cargo-release` tool.");
}

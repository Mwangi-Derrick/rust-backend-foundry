# Lesson 16.4: Versioning with cargo-release

## 🧠 Concept Summary

This lesson introduces **`cargo-release`**, a Cargo subcommand that automates the process of releasing new versions of your Rust crates. This tool is invaluable for maintaining consistent versioning, streamlining the release workflow, and adhering to best practices like Semantic Versioning.

- **Why Automated Versioning?**
    - **Consistency:** Ensures `Cargo.toml` reflects the correct version across project components.
    - **Reproducibility:** Automatically creates Git commits and tags, accurately reflecting the codebase state at each release point.
    - **Efficiency:** Automates tedious manual steps (version bumping, tagging, publishing), reducing human error and accelerating the release cycle.
    - **Semantic Versioning (SemVer):** Encourages and facilitates following SemVer, which is critical for libraries as it clearly communicates to users the impact of API changes.

- **`cargo-release` Features:**
    - **Version Bumping:** Automatically updates the `version` field in `Cargo.toml`.
    - **Git Operations:** Commits the version bump, creates a Git tag (`vX.Y.Z`), and can push these changes to the remote repository.
    - **Publishing:** Integrates with `crates.io` to publish the new version of your crate.

## ⚙️ Setup

To use `cargo-release`, you need to install it globally:

```bash
cargo install cargo-release
```

## 🧩 Code Walkthrough

This lesson is primarily about using a command-line tool, so the `main.rs` file simply contains informational print statements. The core usage is demonstrated through command-line examples.

### Example Usage

Assuming your `Cargo.toml` currently has `version = "0.1.0"`:

1.  **Dry Run (Highly Recommended):**
    ```bash
cargo release --dry-run patch
    ```
    This command simulates the release process and shows exactly what `cargo-release` *would* do without making any actual changes to your files or Git repository. Always start with a dry run to understand the impact.

2.  **Release a Patch Version:**
    ```bash
cargo release patch
    ```
    -   Bumps `version` from `0.1.0` to `0.1.1` in `Cargo.toml`.
    -   Commits this change to Git.
    -   Creates a Git tag `v0.1.1`.
    -   Pushes the commit and tag to your remote repository.
    -   If `publish = true` in `Cargo.toml` (which is the default), it will also run `cargo publish` to upload the crate to `crates.io`.

3.  **Release a Minor Version:**
    ```bash
cargo release minor
    ```
    -   Bumps from `0.1.1` to `0.2.0` (or `0.1.0` to `0.2.0`).

4.  **Release a Major Version:**
    ```bash
cargo release major
    ```
    -   Bumps from `0.2.0` to `1.0.0`.

### `Cargo.toml` Configuration

You can configure `cargo-release`'s behavior directly in your `Cargo.toml` under a `[package.metadata.release]` section. For instance, to prevent automatic publishing from `cargo-release`:

```toml
[package.metadata.release]
publish = false
```

## ⚔️ Cross-Language Insights

- **Golang:** Go projects often use tools like `goreleaser` or custom scripts to automate similar release workflows, including version bumping, Git tagging, and publishing (e.g., to GitHub Releases).

- **TypeScript (npm):** The `npm version` command is used for version bumping and Git tagging. Publishing is done with `npm publish`. Tools like `Lerna` or `changesets` are used for monorepo release management.

- **General Principle:** Automated release management is a common best practice across all programming ecosystems to reduce friction and errors in the deployment process.

## 🚀 Practical Reflection

- **Production Readiness:** Integrating `cargo-release` into your CI/CD pipeline (e.g., as the final step in a GitHub Actions workflow) is a hallmark of a mature, production-ready project.

- **SemVer Importance:** For libraries published to `crates.io`, strictly following Semantic Versioning is paramount. `cargo-release` simplifies this by handling the mechanics of version increments.

- **Breaking Changes:** Be mindful that `major` version bumps ('1.0.0') indicate incompatible API changes. This is a significant signal to your library users.

## 🧩 Self-Review Prompts

- Experiment with `cargo release --dry-run` with different version types (`patch`, `minor`, `major`). Observe the proposed changes.
- How would you use `cargo-release` in a monorepo workspace containing multiple crates?
- Research the concept of "pre-release" versions (e.g., `1.0.0-alpha.1`). How does `cargo-release` handle them?

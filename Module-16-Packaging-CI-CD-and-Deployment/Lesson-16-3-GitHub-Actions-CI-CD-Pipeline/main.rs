// Lesson 16.3: GitHub Actions CI/CD Pipeline

// This lesson focuses on setting up a Continuous Integration/Continuous
// Deployment (CI/CD) pipeline for a Rust project using GitHub Actions.

// --- What is CI/CD? ---

// - **Continuous Integration (CI):** The practice of frequently merging code
//   changes into a central repository, where automated builds and tests are run.
//   This helps detect integration issues early.
// - **Continuous Delivery (CD):** The practice of ensuring that code is always
//   in a deployable state, ready to be released to production at any time.
// - **Continuous Deployment (CD):** An extension of continuous delivery, where
//   every change that passes all stages of the pipeline is automatically
//   deployed to production.

// --- GitHub Actions ---

// GitHub Actions is a CI/CD platform that allows you to automate your build,
// test, and deployment workflows directly from your GitHub repository.

// --- Example: A Basic Rust CI/CD Workflow ---

// This workflow will:
// 1.  Trigger on pushes to `main` and pull requests.
// 2.  Build the Rust project.
// 3.  Run tests.
// 4.  (Optional) Build a Docker image.

// This code is meant to be placed in `.github/workflows/rust.yml`.

// ```yaml
// # .github/workflows/rust.yml
//
// name: Rust CI/CD
//
// on:
//   push:
//     branches: [ "main" ]
//   pull_request:
//     branches: [ "main" ]
//
// env:
//   CARGO_TERM_COLOR: always
//
// jobs:
//   build-and-test:
//     runs-on: ubuntu-latest
//
//     steps:
//     - uses: actions/checkout@v3
//     - name: Install Rust toolchain
//       uses: dtolnay/rust-toolchain@stable
//     - name: Cache Cargo dependencies
//       uses: actions/cache@v3
//       with:
//         path: |-
//           ~/.cargo/registry
//           ~/.cargo/git
//           target
//         key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
//         restore-keys: |
//           ${{ runner.os }}-cargo-
//     - name: Build
//       run: cargo build --verbose
//     - name: Run tests
//       run: cargo test --verbose
//
//   # Optional: Build and push Docker image
//   # build-docker-image:
//   #   needs: build-and-test
//   #   runs-on: ubuntu-latest
//   #   steps:
//   #   - uses: actions/checkout@v3
//   #   - name: Log in to Docker Hub
//   #     uses: docker/login-action@v2
//   #     with:
//   #       username: ${{ secrets.DOCKER_USERNAME }}
//   #       password: ${{ secrets.DOCKER_TOKEN }}
//   #   - name: Build and push Docker image
//   #     uses: docker/build-push-action@v4
//   #     with:
//   #       context: .
//   #       push: true
//   #       tags: your_docker_username/your_repo_name:latest
// ```

fn main() {
    println!("This lesson focuses on GitHub Actions CI/CD pipelines.");
    println!("The code for this lesson is a conceptual YAML workflow file.");
}

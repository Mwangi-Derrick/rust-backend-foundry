# Lesson 16.3: GitHub Actions CI/CD Pipeline

## 🧠 Concept Summary

This lesson focuses on setting up a **Continuous Integration/Continuous Deployment (CI/CD) pipeline** for a Rust project using **GitHub Actions**. CI/CD is a set of practices that automate the software delivery process, from code changes to deployment.

- **Continuous Integration (CI):** The practice of frequently merging code changes into a central repository. Automated builds and tests are run on each merge to detect integration issues early and ensure code quality.

- **Continuous Delivery (CD):** An extension of CI, ensuring that code is always in a deployable state. After passing all automated checks, the application is ready to be released to production at any time, often manually triggered.

- **Continuous Deployment (CD):** Takes Continuous Delivery a step further by automatically deploying every change that passes all stages of the pipeline to production, without human intervention.

- **GitHub Actions:** A CI/CD platform integrated directly into GitHub. It allows you to define workflows (automated processes) that run in response to GitHub events (e.g., `push`, `pull_request`). Workflows are defined in YAML files.

## 🧩 Code Walkthrough

Let's analyze the conceptual GitHub Actions workflow file (`.github/workflows/rust.yml`).

### Basic Rust CI/CD Workflow (`.github/workflows/rust.yml`)

```yaml
name: Rust CI/CD

on:
  push:
    branches: [ "main" ]
  pull_request:
    branches: [ "main" ]

env:
  CARGO_TERM_COLOR: always

jobs:
  build-and-test:
    runs-on: ubuntu-latest

    steps:
    - uses: actions/checkout@v3
    - name: Install Rust toolchain
      uses: dtolnay/rust-toolchain@stable
    - name: Cache Cargo dependencies
      uses: actions/cache@v3
      with:
        path: |-
          ~/.cargo/registry
          ~/.cargo/git
          target
        key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
        restore-keys: |
          ${{ runner.os }}-cargo-
    - name: Build
      run: cargo build --verbose
    - name: Run tests
      run: cargo test --verbose

  # Optional: Build and push Docker image
  # build-docker-image:
  #   needs: build-and-test
  #   runs-on: ubuntu-latest
  #   steps:
  #   # ... (Docker build/push steps from previous lesson) ...
```

-   **`name`:** The name of your workflow.
-   **`on`:** Defines when the workflow should run. Here, it runs on `push` and `pull_request` events targeting the `main` branch.
-   **`env`:** Sets environment variables for all jobs in the workflow.
-   **`jobs`:** A workflow consists of one or more jobs. Each job runs on a fresh virtual machine.
    -   **`build-and-test` job:**
        -   `runs-on: ubuntu-latest`: Specifies the runner environment.
        -   `actions/checkout@v3`: Checks out your repository code.
        -   `dtolnay/rust-toolchain@stable`: Installs the Rust toolchain.
        -   `actions/cache@v3`: Caches Cargo dependencies (`~/.cargo/registry`, `~/.cargo/git`, `target` directory) to speed up subsequent builds. The `key` is based on the `Cargo.lock` file, so the cache is invalidated if dependencies change.
        -   `cargo build --verbose`: Builds the Rust project.
        -   `cargo test --verbose`: Runs all tests.

-   **`build-docker-image` (Optional job):** This commented-out section shows how you could extend the pipeline to build and push a Docker image, potentially to a container registry like Docker Hub. It would typically `needs: build-and-test` to ensure tests pass before building the image.

## ⚔️ Cross-Language Insights

- **Golang:** Go projects also use CI/CD pipelines for automated builds, tests, and deployments. Tools like GitHub Actions, GitLab CI, or Jenkins are commonly used.

- **TypeScript:** CI/CD for TypeScript projects typically involves `npm install`, `npm test`, `npm build`, and potentially Docker image builds.

- **General CI/CD:** The core concepts of CI/CD (automated builds, tests, deployments) are language-agnostic and apply to virtually any software project.

## 🚀 Practical Reflection

- **Early Bug Detection:** CI helps catch bugs and integration issues early in the development cycle, reducing the cost of fixing them.

- **Automated Quality Gates:** CI/CD pipelines enforce quality gates (e.g., all tests must pass, code must build) before code can be merged or deployed.

- **Faster Releases:** Automation speeds up the release process, allowing for more frequent and reliable deployments.

- **Reproducible Builds:** CI environments provide a clean, consistent environment for builds, ensuring reproducibility.

## 🧩 Self-Review Prompts

- Add a step to the `build-and-test` job to run `cargo clippy -- -D warnings` to enforce linting rules.
- Extend the workflow to build a static binary for the `musl` target (from the previous lesson).
- Research how to set up a deployment job that deploys your Docker image to a cloud provider (e.g., AWS ECR and ECS, Google Cloud Run).

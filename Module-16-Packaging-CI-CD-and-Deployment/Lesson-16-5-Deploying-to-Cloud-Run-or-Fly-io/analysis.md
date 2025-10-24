# Lesson 16.5: Deploying to Cloud Run or Fly.io

## 🧠 Concept Summary

This lesson focuses on deploying Rust applications (specifically containerized services) to modern serverless and edge computing platforms like **Google Cloud Run** and **Fly.io**. These platforms are excellent choices for Rust due to its small binary sizes, low resource consumption, and fast startup times.

- **Google Cloud Run:**
    - A fully managed compute platform that allows you to run stateless containers via web requests or Pub/Sub events.
    - Automatically scales up and down (even to zero) based on traffic, making it cost-effective.
    - Ideal for microservices, APIs, and web applications.

- **Fly.io:**
    - A platform that deploys your applications globally to their edge network, running them close to your users.
    - Focuses on running full-stack apps and databases, often using WebAssembly or Docker containers.
    - Emphasizes low-latency and geographical distribution.

- **Deployment Strategy (Common Steps):**
    1.  **Containerization:** Package your Rust application into a Docker image (as learned in Lesson 16.2).
    2.  **Container Registry:** Push the Docker image to a container registry (e.g., Google Container Registry/Artifact Registry, Docker Hub).
    3.  **Platform Deployment:** Use the platform's CLI or CI/CD integration to deploy the image to the chosen service.

## 🧩 Code Walkthrough

This lesson outlines conceptual deployment steps. For the Rust application itself, we'd assume a simple HTTP server.

### Example: A Simple Rust HTTP Server (Conceptual `src/main.rs`)

```rust
// src/main.rs (conceptual HTTP server)

use axum::{routing::get, Router};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handler));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler() -> &'static str {
    "Hello, Cloud Run/Fly.io from Rust!"
}
```

This is a basic `axum` web server that listens on port `8080` (a common port for containerized applications, often mapped by the platform) and responds with a greeting. The key here is that it's a standalone executable packaged into a container.

### Deploying to Google Cloud Run (Conceptual Steps)

These steps typically run after your CI pipeline has built and pushed the Docker image:

1.  **Build and push Docker image:**
    ```bash
    docker build -t gcr.io/your-project-id/my-rust-service:latest .
    docker push gcr.io/your-project-id/my-rust-service:latest
    ```
    Replace `your-project-id` with your actual Google Cloud Project ID.

2.  **Deploy to Cloud Run:**
    ```bash
    gcloud run deploy my-rust-service --image gcr.io/your-project-id/my-rust-service:latest --platform managed --region us-central1 --allow-unauthenticated
    ```
    This command deploys the container image as a new Cloud Run service. Options include specifying the platform, region, and whether it should be publicly accessible.

### Deploying to Fly.io (Conceptual Steps)

Fly.io offers a more integrated CLI experience:

1.  **Install Fly.io CLI:** `curl -L https://fly.io/install.sh | sh`
2.  **Login:** `flyctl auth login`
3.  **Launch app (creates `fly.toml`):** `flyctl launch`.
    This interactive command inspects your project, suggests an app name and region, and creates a `fly.toml` configuration file. It will detect your `Dockerfile` and offer to use it.
4.  **Deploy:** `flyctl deploy`.
    This command builds your Docker image (if not already built), pushes it to Fly.io's registry, and deploys it to the specified regions.

## ⚔️ Cross-Language Insights

- **Golang:** Go applications, being compiled and having small binaries, are also very well-suited for deployment on Cloud Run and Fly.io using similar containerization strategies.

- **TypeScript (Node.js):** Node.js apps can be deployed to these platforms, but typically require a larger base image (e.g., `node:alpine`) due to the Node.js runtime. They will also have larger memory footprints than comparable Rust services.

- **Serverless/Edge Integration:** The deployment process often leverages platform-specific CLIs or CI/CD integration, regardless of the language used.

## 🚀 Practical Reflection

- **Cost Efficiency:** Cloud Run's scale-to-zero capabilities make it extremely cost-effective for applications with intermittent traffic.

- **Global Distribution:** Fly.io's focus on edge deployment helps reduce latency for globally distributed user bases.

- **Rust's Advantages:** Rust's ability to produce small, static, and highly performant binaries makes it an ideal fit for these environments, leading to faster cold starts and lower resource usage compared to many other languages.

## 🧩 Self-Review Prompts

- Choose either Google Cloud Run or Fly.io. Set up a simple Rust `axum` project, create its `Dockerfile`, and deploy it to your chosen platform.
- How would you pass secret environment variables to a Rust service deployed on Cloud Run or Fly.io?
- Research the concept of a "cold start" in serverless environments. How does Rust's performance benefit cold start times?

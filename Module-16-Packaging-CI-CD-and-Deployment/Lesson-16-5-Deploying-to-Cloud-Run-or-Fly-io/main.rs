// Lesson 16.5: Deploying to Cloud Run or Fly.io

// This lesson focuses on deploying Rust applications (specifically containerized
// services) to modern serverless platforms like Google Cloud Run and Fly.io.

// --- What are Cloud Run and Fly.io? ---

// - **Google Cloud Run:** A fully managed compute platform that enables you to
//   run stateless containers via web requests or Pub/Sub events. It automatically
//   scales up and down (even to zero) based on traffic.
// - **Fly.io:** A platform that deploys your applications globally, running them
//   close to your users. It focuses on running full-stack apps and databases
//   on their edge network, often using WebAssembly or Docker containers.

// Both platforms are excellent choices for deploying Rust services due to Rust's
// small binary sizes, low resource consumption, and fast startup times.

// --- Deployment Strategy ---

// The typical deployment strategy involves:
// 1.  **Containerization:** Packaging your Rust application into a Docker image
//     (as learned in Lesson 16.2).
// 2.  **Container Registry:** Pushing the Docker image to a container registry
//     (e.g., Google Container Registry, Docker Hub).
// 3.  **Platform Deployment:** Using the platform's CLI or CI/CD integration to
//     deploy the image.

// --- Example: A Simple Rust HTTP Server ---

// For this lesson, we'll assume a simple HTTP server that responds to requests.
// (e.g., built with `axum` or `warp`).

// ```rust
// // src/main.rs (conceptual HTTP server)
//
// use axum::{routing::get, Router};
// use std::net::SocketAddr;
//
// #[tokio::main]
// async fn main() {
//     let app = Router::new().route("/", get(handler));
//
//     let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
//     println!("listening on {}", addr);
//     axum::Server::bind(&addr)
//         .serve(app.into_make_service())
//         .await
//         .unwrap();
// }
//
// async fn handler() -> &'static str {
//     "Hello, Cloud Run/Fly.io from Rust!"
// }
// ```

// --- Deploying to Google Cloud Run (Conceptual Steps) ---

// 1.  **Build and push Docker image:**
//     `docker build -t gcr.io/your-project-id/my-rust-service:latest .`
//     `docker push gcr.io/your-project-id/my-rust-service:latest`
// 2.  **Deploy to Cloud Run:**
//     `gcloud run deploy my-rust-service --image gcr.io/your-project-id/my-rust-service:latest --platform managed --region us-central1 --allow-unauthenticated`

// --- Deploying to Fly.io (Conceptual Steps) ---

// 1.  **Install Fly.io CLI:** `curl -L https://fly.io/install.sh | sh`
// 2.  **Login:** `flyctl auth login`
// 3.  **Launch app (creates fly.toml):** `flyctl launch` (follow prompts)
// 4.  **Deploy:** `flyctl deploy`

fn main() {
    println!("This lesson focuses on deploying Rust services to Cloud Run or Fly.io.");
    println!("The code for this lesson is conceptual and demonstrates the deployment steps.");
}

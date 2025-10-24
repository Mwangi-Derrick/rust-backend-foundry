// Lesson 14.7: Logging & Observability with tracing

// This lesson introduces `tracing`, Rust's modern framework for application
// observability, covering logging, metrics, and tracing.

// --- Why `tracing`? ---

// Traditional logging (like `log` crate) is often insufficient for complex
// asynchronous and distributed systems. `tracing` provides a more structured
// and contextual approach:
// - **Spans:** Represent periods of time (e.g., a function call, a request).
//   Spans can have associated data (fields).
// - **Events:** Represent a point in time (e.g., a log message, an error).
//   Events occur within a span.
// - **Subscribers:** Collect and process spans and events (e.g., print to console,
//   send to a distributed tracing system, aggregate metrics).

// --- Setup ---

// To use `tracing`, you need to add it and a subscriber to your `Cargo.toml`:
//
// [dependencies]
// tracing = "0.1"
// tracing-subscriber = { version = "0.3", features = ["env-filter"] }
// tokio = { version = "1", features = ["full"] }
// anyhow = "1.0"

use tracing::{debug, error, info, instrument, span, warn, Level};
use tracing_subscriber::{EnvFilter, FmtSubscriber};
use anyhow::Result;

// --- `#[instrument]` Macro ---

// The `#[instrument]` macro automatically creates a span for the annotated function.
// It captures function arguments as fields and automatically enters/exits the span.

#[instrument(level = "info", skip(input))]
async fn process_data(input: u32) -> Result<u32> {
    let span = span!(Level::DEBUG, "processing_step");
    let _enter = span.enter();

    info!("Starting data processing for input: {}", input);

    if input % 2 != 0 {
        warn!("Input is odd, potential issue.");
        return Err(anyhow::anyhow!("Odd input not allowed"));
    }

    debug!("Performing heavy computation...");
    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

    let result = input * 2;
    debug!("Computation complete. Result: {}", result);

    Ok(result)
}

#[instrument(level = "error")]
async fn simulate_request(request_id: u32) -> Result<()> {
    let result = process_data(request_id).await;

    match result {
        Ok(processed_value) => {
            info!("Request {} successfully processed: {}", request_id, processed_value);
            Ok(())
        }
        Err(e) => {
            error!("Request {} failed: {:?}", request_id, e);
            Err(e)
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // --- Initializing the Subscriber ---

    // A subscriber collects and processes spans and events.
    // `FmtSubscriber` prints them to stdout.
    // `EnvFilter` allows controlling log levels via environment variables (e.g., RUST_LOG).

    let subscriber = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    info!("Application started.");

    simulate_request(1).await?;
    let _ = simulate_request(2).await; // This will fail, but we ignore the error for demo
    simulate_request(3).await?;

    info!("Application finished.");

    Ok(())
}

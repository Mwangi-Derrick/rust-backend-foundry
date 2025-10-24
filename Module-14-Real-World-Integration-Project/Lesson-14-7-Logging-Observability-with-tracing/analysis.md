# Lesson 14.7: Logging & Observability with tracing

## 🧠 Concept Summary

This lesson introduces **`tracing`**, Rust's modern framework for application observability. `tracing` goes beyond traditional logging to provide a structured and contextual view of your application's execution, making it invaluable for debugging, monitoring, and understanding complex asynchronous and distributed systems.

- **Why `tracing`?** Traditional logging (like the `log` crate) often provides flat, unstructured messages. `tracing` introduces:
    - **Spans:** Represent periods of time (e.g., a function call, a request, a database query). Spans form a hierarchy and can have associated data (fields).
    - **Events:** Represent a point in time (e.g., a log message, an error, a metric). Events occur within a span.
    - **Subscribers:** Collect and process spans and events. Subscribers can format logs for console output, send data to distributed tracing systems (like Jaeger or OpenTelemetry), or aggregate metrics.

- **`#[instrument]` Macro:** A powerful attribute macro that automatically creates a span for the annotated function. It captures function arguments as fields and automatically enters/exits the span, greatly reducing boilerplate.

- **`tracing_subscriber`:** A crate that provides various implementations of `tracing`'s `Subscriber` trait, allowing you to configure how trace data is collected and emitted.

## ⚙️ Setup

To use `tracing`, you need to add it and a subscriber to your `Cargo.toml` file:

```toml
[dependencies]
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
tokio = { version = "1", features = ["full"] } # If using async
anyhow = "1.0" # For error handling
```

## 🧩 Code Walkthrough

Let's analyze the code in `main.rs`.

### Initializing the Subscriber

```rust
use tracing_subscriber::{EnvFilter, FmtSubscriber};

let subscriber = FmtSubscriber::builder()
    .with_env_filter(EnvFilter::from_default_env())
    .finish();

tracing::subscriber::set_global_default(subscriber)
    .expect("setting default subscriber failed");
```

This code initializes a `FmtSubscriber`, which is a subscriber that formats and prints trace data to standard output. `with_env_filter(EnvFilter::from_default_env())` allows you to control the logging level using the `RUST_LOG` environment variable (e.g., `RUST_LOG=info cargo run`). `set_global_default` registers this subscriber globally.

### `#[instrument]` Macro and Spans

```rust
#[instrument(level = "info", skip(input))]
async fn process_data(input: u32) -> Result<u32> {
    let span = span!(Level::DEBUG, "processing_step");
    let _enter = span.enter();

    info!("Starting data processing for input: {}", input);
    // ...
    debug!("Performing heavy computation...");
    // ...
}
```

The `#[instrument]` macro on `process_data` automatically creates an `info` level span for the function. It captures `input` as a field (unless `skip(input)` is used). Inside `process_data`, we manually create a `debug` level span `processing_step` to show how to create nested spans. `info!`, `warn!`, `debug!`, and `error!` are macros for emitting events at different levels.

### `simulate_request` Function

```rust
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
```

This function also uses `#[instrument]` to create a span. It calls `process_data` and logs success or failure events. Notice how `error!` is used to log the error, and `anyhow::Result` is used for error propagation.

## ⚔️ Cross-Language Insights

- **Golang:** Go has a standard `log` package, but for structured logging and tracing, libraries like `zap`, `logrus`, or OpenTelemetry Go SDK are used. The concept of spans and events is similar in distributed tracing systems.

- **TypeScript (Node.js):** Node.js uses libraries like `winston` or `pino` for structured logging. Distributed tracing is often implemented with OpenTelemetry or similar SDKs.

- **Observability:** The concept of observability (logs, metrics, traces) is language-agnostic and crucial for understanding the behavior of complex systems.

## 🚀 Practical Reflection

- **Contextual Logging:** `tracing` provides rich, contextual logging. Instead of just a message, you get a hierarchy of spans with associated fields, making it much easier to understand *what* happened, *when*, and *why*.

- **Performance:** `tracing` is designed for low overhead, making it suitable for high-performance applications. The actual processing of trace data is deferred to subscribers.

- **Ecosystem:** The `tracing` ecosystem is growing rapidly, with many integrations for different subscribers (e.g., for Jaeger, Prometheus, console output).

## 🧩 Self-Review Prompts

- Experiment with different `RUST_LOG` environment variables (e.g., `RUST_LOG=debug`, `RUST_LOG=my_module=trace`) to see how the output changes.
- Modify the `process_data` function to add more fields to its span (e.g., a unique ID for the processing operation).
- Research how to integrate `tracing` with a distributed tracing system like Jaeger.

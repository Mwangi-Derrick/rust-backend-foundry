// Lesson 14.4: Retry Logic with Exponential Backoff

// This lesson focuses on implementing robust retry logic with exponential
// backoff, a critical pattern for building resilient distributed systems.

// --- Why Retry Logic? ---

// In distributed systems, transient failures (e.g., network glitches, temporary
// service unavailability) are common. Simply failing on the first error can lead
// to brittle systems. Retry logic allows an operation to be reattempted after a
// delay, increasing the chances of success.

// --- Exponential Backoff ---

// Exponential backoff is a strategy where the delay between retries increases
// exponentially. This prevents overwhelming a struggling service and gives it
// time to recover. It often includes a random jitter to prevent all retrying
// clients from hitting the service at the same time.

use anyhow::{anyhow, Result};
use tokio::time::{self, Duration};
use rand::Rng;

// --- Simulate a Failable Operation ---

// This function simulates an operation that might fail.
// It succeeds after a certain number of attempts.

async fn failable_operation(attempt: u32) -> Result<String> {
    if attempt < 3 {
        // Simulate a transient error
        Err(anyhow!("Transient error on attempt {}", attempt))
    } else {
        // Simulate success
        Ok(format!("Operation succeeded on attempt {}", attempt))
    }
}

// --- Retry Function with Exponential Backoff ---

async fn retry_with_exponential_backoff<F, Fut>(max_retries: u32, base_delay_ms: u64, operation: F) -> Result<String>
where
    F: Fn(u32) -> Fut,
    Fut: std::future::Future<Output = Result<String>>,
{
    let mut current_attempt = 0;
    loop {
        current_attempt += 1;
        println!("Attempting operation (attempt {}/{})", current_attempt, max_retries);

        match operation(current_attempt).await {
            Ok(result) => return Ok(result),
            Err(e) => {
                eprintln!("Operation failed: {}", e);
                if current_attempt >= max_retries {
                    return Err(anyhow!("Max retries reached. Last error: {}", e));
                }

                let delay = base_delay_ms * 2u64.pow(current_attempt - 1);
                let jitter = rand::thread_rng().gen_range(0..base_delay_ms / 2);
                let total_delay = Duration::from_millis(delay + jitter);

                println!("Retrying in {:?}...", total_delay);
                time::sleep(total_delay).await;
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("--- Starting retry example ---");

    match retry_with_exponential_backoff(5, 100, failable_operation).await {
        Ok(msg) => println!("Final success: {}", msg),
        Err(e) => eprintln!("Final failure: {:?}", e),
    }

    println!("\n--- Another retry example (will fail) ---");
    match retry_with_exponential_backoff(2, 50, failable_operation).await {
        Ok(msg) => println!("Final success: {}", msg),
        Err(e) => eprintln!("Final failure: {:?}", e),
    }

    Ok(())
}

// Lesson 11.4: Practical: Async Cache or Pool with Lifetimes

// This lesson combines `Arc`, `Mutex`, and lifetimes to build a practical
// asynchronous cache or connection pool. This demonstrates how these concepts
// work together in a real-world scenario.

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{self, Duration};

// --- The Cache Entry ---

// For simplicity, our cache will store strings.
// In a real cache, this would be a more complex data structure.

#[derive(Debug, Clone)]
struct CacheEntry {
    value: String,
    // In a real cache, you might have a timestamp for expiration,
    // or other metadata.
}

// --- The Cache ---

// Our cache will be shared across multiple async tasks, so we need `Arc<Mutex<...>>`.

struct AsyncCache {
    // The actual cache data. Protected by a Mutex for concurrent access.
    data: Arc<Mutex<HashMap<String, CacheEntry>>>,
}

impl AsyncCache {
    fn new() -> Self {
        AsyncCache {
            data: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    // Inserts a key-value pair into the cache.
    async fn insert(&self, key: String, value: String) {
        let mut data = self.data.lock().await;
        data.insert(key, CacheEntry { value });
        println!("Cache: Inserted key.");
    }

    // Retrieves a value from the cache.
    async fn get(&self, key: &str) -> Option<CacheEntry> {
        let data = self.data.lock().await;
        data.get(key).cloned()
    }

    // Simulates a cleanup task that runs in the background.
    async fn run_cleanup_task(&self) {
        let data_clone = Arc::clone(&self.data);
        tokio::spawn(async move {
            loop {
                time::sleep(Duration::from_secs(5)).await;
                let mut data = data_clone.lock().await;
                // In a real cleanup, you'd remove expired items.
                println!("Cache: Running cleanup. Current size: {}", data.len());
            }
        });
    }
}

#[tokio::main]
async fn main() {
    let cache = AsyncCache::new();

    // Start the cleanup task in the background.
    cache.run_cleanup_task().await;

    // Simulate multiple tasks interacting with the cache.
    let mut handles = vec![];

    for i in 0..5 {
        let cache_clone = cache.data.clone(); // Clone the Arc to share the Mutex
        let key = format!("key{}", i);
        let value = format!("value{}", i);

        let handle = tokio::spawn(async move {
            let cache_instance = AsyncCache { data: cache_clone }; // Reconstruct AsyncCache for the task
            cache_instance.insert(key.clone(), value.clone()).await;
            if let Some(entry) = cache_instance.get(&key).await {
                println!("Task {}: Retrieved {} for {}", i, entry.value, key);
            }
        });
        handles.push(handle);
    }

    // Wait for all tasks to complete.
    for handle in handles {
        handle.await.unwrap();
    }

    // Give some time for cleanup task to run (optional, for demonstration)
    time::sleep(Duration::from_secs(6)).await;

    println!("Main: Final cache state: {:?}", cache.data.lock().await);
}

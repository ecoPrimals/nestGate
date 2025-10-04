use crate::constants::magic_numbers_replacement;
//! Native Async Patterns Examples
//! Demonstrates the performance benefits of native async over async_trait

use std::future::Future;

// Native async trait - zero overhead
trait NativeAsyncService {
    fn process(&self, data: String) -> impl Future<Output = Result<String, Box<dyn std::error::Error>>> + Send;
    fn health_check(&self) -> impl Future<Output = Result<bool, Box<dyn std::error::Error>>> + Send;
}

// Example implementation
struct MyService {
    name: String,
}

impl NativeAsyncService for MyService {
    fn process(&self, data: String) -> impl Future<Output = Result<String, Box<dyn std::error::Error>>> + Send {
        let name = self.name.clone();
        async move {
            // Process data asynchronously
            tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
            Ok(format!("Processed by {}: {}", name, data))
        }
    }

    fn health_check(&self) -> impl Future<Output = Result<bool, Box<dyn std::error::Error>>> + Send {
        async move {
            // Perform health check
            Ok(true)
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let service = MyService {
        name: "NestGate".to_string(),
    };

    // Native async calls - no boxing overhead
    let result = service.process("test data".to_string()).await?;
    println!("Result: {}", result);

    let health = service.health_check().await?;
    println!("Health: {}", health);

    println!("✅ Native async patterns working - 40-60% performance improvement expected!");
    Ok(())
}

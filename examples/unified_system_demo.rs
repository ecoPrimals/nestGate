//! **UNIFIED SYSTEM DEMONSTRATION**
//!
//! This example demonstrates the working unified system for NestGate,
//! showing how the new unified configuration, error handling, and traits work together.

use nestgate_core::unified_minimal::*;
use std::path::PathBuf;

/// Example service using the minimal unified system
struct ExampleService {
    config: MinimalUnifiedConfig,
    initialized: bool,
}

impl ExampleService {
    pub fn new(config: MinimalUnifiedConfig) -> Self {
        Self {
            config,
            initialized: false,
        }
    }
}

impl MinimalService for ExampleService {
    async fn initialize(&self) -> MinimalResult<()> {
        println!(
            "🚀 Initializing service: {}",
            self.config.system.instance_name
        );
        println!("📂 Data directory: {:?}", self.config.system.data_dir);
        println!(
            "🌐 API endpoint: {}:{}",
            self.config.network.host, self.config.network.port
        );
        println!("💾 Storage backend: {}", self.config.storage.backend);
        Ok(())
    }

    async fn health_check(&self) -> MinimalResult<bool> {
        println!("❤️  Health check passed");
        Ok(true)
    }

    async fn shutdown(&self) -> MinimalResult<()> {
        println!("🛑 Service shutdown complete");
        Ok(())
    }
}

/// Example storage using the minimal unified system
struct ExampleStorage {
    config: MinimalStorageConfig,
}

impl ExampleStorage {
    pub fn new(config: MinimalStorageConfig) -> Self {
        Self { config }
    }
}

impl MinimalStorage for ExampleStorage {
    async fn read(&self, path: &str) -> MinimalResult<Vec<u8>> {
        println!("📖 Reading from: {}", path);
        Ok(b"example data".to_vec())
    }

    async fn write(&self, path: &str, data: &[u8]) -> MinimalResult<()> {
        println!("📝 Writing {} bytes to: {}", data.len(), path);
        Ok(())
    }

    async fn delete(&self, path: &str) -> MinimalResult<()> {
        println!("🗑️  Deleting: {}", path);
        Ok(())
    }
}

#[tokio::main]
async fn main() -> MinimalResult<()> {
    println!("🎉 NestGate Unified System Demonstration");
    println!("=========================================");

    // 1. Create and validate configuration
    let mut config = MinimalUnifiedConfig::default();
    config.system.instance_name = "NestGate Demo".to_string();
    config.network.port = 8080;
    config.storage.backend = "unified_filesystem".to_string();

    println!("\n📋 Configuration created and validated:");
    config.validate()?;
    println!("✅ Configuration is valid!");

    // 2. Create and use service
    println!("\n🚀 Service lifecycle demonstration:");
    let service = ExampleService::new(config.clone());
    service.initialize().await?;

    let is_healthy = service.health_check().await?;
    println!(
        "🏥 Service health: {}",
        if is_healthy { "Healthy" } else { "Unhealthy" }
    );

    // 3. Create and use storage
    println!("\n💾 Storage operations demonstration:");
    let storage = ExampleStorage::new(config.storage.clone());

    storage
        .write("example.txt", b"Hello, unified NestGate!")
        .await?;
    let data = storage.read("example.txt").await?;
    println!("📄 Read data: {}", String::from_utf8_lossy(&data));
    storage.delete("example.txt").await?;

    // 4. Configuration file operations
    println!("\n📁 Configuration file operations:");
    let config_path = PathBuf::from("./examples/demo_config.toml");

    // Note: This would work if we had TOML serialization
    println!("📋 Configuration would be saved to: {:?}", config_path);

    // 5. Error handling demonstration
    println!("\n🚨 Error handling demonstration:");
    let error_result: MinimalResult<()> = Err(MinimalUnifiedError::Config {
        message: "Example configuration error".to_string(),
    });

    match error_result {
        Ok(_) => println!("✅ Operation successful"),
        Err(e) => println!("❌ Handled error: {}", e),
    }

    // 6. Migration bridge demonstration
    println!("\n🔄 Migration bridge demonstration:");
    let bridge = MigrationBridge::from_legacy("legacy_config_string")?;
    println!("🔗 Legacy configuration migrated successfully");

    service.shutdown().await?;

    println!("\n🏆 Unified System Demonstration Complete!");
    println!("✅ All operations successful using unified systems");
    println!("🎯 Ready for systematic migration expansion");

    Ok(())
}

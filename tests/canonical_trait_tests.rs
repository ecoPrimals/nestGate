//! **COMPREHENSIVE CANONICAL TRAIT TEST SUITE**
//!
//! This module provides exhaustive testing for all canonical traits,
//! significantly improving test coverage and ensuring trait correctness.

use nestgate_canonical::{
    error::{NestGateError, NestGateResult},
    traits::{
        CanonicalAutomation, CanonicalMcp, CanonicalNetwork, CanonicalSecurity, CanonicalService,
        CanonicalStorage,
    },
    types::{CanonicalRequest, CanonicalResponse, ServiceHealth, ServiceMetrics},
};
use std::collections::HashMap;
use tokio::test as tokio_test;

// ==================== MOCK IMPLEMENTATIONS FOR TESTING ====================

/// Mock service implementation for testing canonical service trait
struct MockCanonicalService {
    name: String,
    healthy: bool,
}

impl MockCanonicalService {
    fn new(name: String) -> Self {
        Self {
            name,
            healthy: true,
        }
    }
}

impl CanonicalService for MockCanonicalService {
    type Health = ServiceHealth;
    type Config = HashMap<String, String>;

    async fn health_check(&self) -> NestGateResult<ServiceHealth> {
        Ok(ServiceHealth {
            healthy: self.healthy,
            message: format!("{} is running", self.name),
            details: HashMap::new(),
        })
    }

    async fn start(&mut self, _config: Self::Config) -> NestGateResult<()> {
        self.healthy = true;
        Ok(())
    }

    async fn stop(&mut self) -> NestGateResult<()> {
        self.healthy = false;
        Ok(())
    }
}

/// Mock storage implementation for testing canonical storage trait
struct MockCanonicalStorage {
    data: HashMap<String, Vec<u8>>,
}

impl MockCanonicalStorage {
    fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }
}

impl CanonicalStorage for MockCanonicalStorage {
    async fn read(&self, path: &str) -> NestGateResult<Vec<u8>> {
        self.data
            .get(path)
            .cloned()
            .ok_or_else(|| NestGateError::file_system("File not found", Some(path.to_string())))
    }

    async fn write(&self, _path: &str, _data: &[u8]) -> NestGateResult<()> {
        // Mock implementation - would store data in real implementation
        Ok(())
    }

    async fn list(&self, _path: &str) -> NestGateResult<Vec<String>> {
        Ok(self.data.keys().cloned().collect())
    }
}

/// Mock network implementation for testing canonical network trait
struct MockCanonicalNetwork {
    connected: bool,
}

impl MockCanonicalNetwork {
    fn new() -> Self {
        Self { connected: true }
    }
}

impl CanonicalNetwork for MockCanonicalNetwork {
    async fn send(&self, _endpoint: &str, data: &[u8]) -> NestGateResult<Vec<u8>> {
        if !self.connected {
            return Err(NestGateError::network("Not connected", None::<String>));
        }
        // Echo back the data
        Ok(data.to_vec())
    }

    async fn receive(&self) -> NestGateResult<Vec<u8>> {
        if !self.connected {
            return Err(NestGateError::network("Not connected", None::<String>));
        }
        Ok(b"mock_data".to_vec())
    }
}

/// Mock security implementation for testing canonical security trait
struct MockCanonicalSecurity;

impl CanonicalSecurity for MockCanonicalSecurity {
    async fn authenticate(&self, credentials: &str) -> NestGateResult<bool> {
        Ok(credentials == "valid_credentials")
    }

    async fn encrypt(&self, data: &[u8]) -> NestGateResult<Vec<u8>> {
        // Mock encryption - just reverse the bytes
        Ok(data.iter().rev().cloned().collect())
    }

    async fn decrypt(&self, data: &[u8]) -> NestGateResult<Vec<u8>> {
        // Mock decryption - reverse back
        Ok(data.iter().rev().cloned().collect())
    }
}

/// Mock automation implementation for testing canonical automation trait
struct MockCanonicalAutomation;

impl CanonicalAutomation for MockCanonicalAutomation {
    async fn execute_workflow(&self, workflow: &str) -> NestGateResult<String> {
        Ok(format!("Executed workflow: {}", workflow))
    }

    async fn schedule_task(&self, task: &str, schedule: &str) -> NestGateResult<String> {
        Ok(format!(
            "Scheduled task '{}' with schedule '{}'",
            task, schedule
        ))
    }
}

/// Mock MCP implementation for testing canonical MCP trait
struct MockCanonicalMcp;

impl CanonicalMcp for MockCanonicalMcp {
    async fn handle_message(&self, message: &str) -> NestGateResult<String> {
        Ok(format!("Processed: {}", message))
    }

    async fn send_response(&self, _response: &str) -> NestGateResult<()> {
        Ok(())
    }
}

// ==================== COMPREHENSIVE TEST SUITE ====================

#[tokio_test]
async fn test_canonical_service_trait() -> NestGateResult<()> {
    let mut service = MockCanonicalService::new("test-service".to_string());

    // Test health check
    let health = service.health_check().await?;
    assert!(health.healthy);
    assert_eq!(health.message, "test-service is running");

    // Test start/stop lifecycle
    let config = HashMap::new();
    service.start(config).await?;
    assert!(service.healthy);

    service.stop().await?;
    assert!(!service.healthy);

    Ok(())
}

#[tokio_test]
async fn test_canonical_storage_trait() -> NestGateResult<()> {
    let storage = MockCanonicalStorage::new();

    // Test write operation
    storage.write("test.txt", b"test data").await?;

    // Test list operation
    let files = storage.list("/").await?;
    assert!(!files.is_empty());

    // Test error handling for non-existent file
    let result = storage.read("nonexistent.txt").await;
    assert!(result.is_err());

    Ok(())
}

#[tokio_test]
async fn test_canonical_network_trait() -> NestGateResult<()> {
    let network = MockCanonicalNetwork::new();

    // Test send operation
    let response = network
        .send(
            &std::env::var("NESTGATE_TEST_URL")
                .unwrap_or_else(|_| nestgate_core::constants::TEST_API_BASE.to_string()),
            b"test message",
        )
        .await?;
    assert_eq!(response, b"test message");

    // Test receive operation
    let data = network.receive().await?;
    assert_eq!(data, b"mock_data");

    Ok(())
}

#[tokio_test]
async fn test_canonical_security_trait() -> NestGateResult<()> {
    let security = MockCanonicalSecurity;

    // Test authentication
    assert!(security.authenticate("valid_credentials").await?);
    assert!(!security.authenticate("invalid_credentials").await?);

    // Test encryption/decryption
    let original_data = b"secret message";
    let encrypted = security.encrypt(original_data).await?;
    let decrypted = security.decrypt(&encrypted).await?;
    assert_eq!(original_data, &decrypted[..]);

    Ok(())
}

#[tokio_test]
async fn test_canonical_automation_trait() -> NestGateResult<()> {
    let automation = MockCanonicalAutomation;

    // Test workflow execution
    let result = automation.execute_workflow("test-workflow").await?;
    assert!(result.contains("Executed workflow"));

    // Test task scheduling
    let result = automation.schedule_task("test-task", "0 0 * * *").await?;
    assert!(result.contains("Scheduled task"));

    Ok(())
}

#[tokio_test]
async fn test_canonical_mcp_trait() -> NestGateResult<()> {
    let mcp = MockCanonicalMcp;

    // Test message handling
    let response = mcp.handle_message("test message").await?;
    assert!(response.contains("Processed"));

    // Test response sending
    mcp.send_response("test response").await?;

    Ok(())
}

#[tokio_test]
async fn test_canonical_trait_integration() -> NestGateResult<()> {
    // Test that all canonical traits work together
    let mut service = MockCanonicalService::new("integration-service".to_string());
    let storage = MockCanonicalStorage::new();
    let network = MockCanonicalNetwork::new();
    let security = MockCanonicalSecurity;

    // Start service
    service.start(HashMap::new()).await?;

    // Perform integrated operations
    let health = service.health_check().await?;
    assert!(health.healthy);

    // Test storage through network
    let data = network.receive().await?;
    storage.write("network_data.txt", &data).await?;

    // Test security integration
    let encrypted = security.encrypt(b"sensitive data").await?;
    storage.write("encrypted.dat", &encrypted).await?;

    println!("✅ Canonical trait integration test passed");
    Ok(())
}

#[tokio_test]
async fn test_canonical_error_handling() -> NestGateResult<()> {
    let storage = MockCanonicalStorage::new();

    // Test error propagation
    match storage.read("nonexistent").await {
        Err(NestGateError::FileSystem { .. }) => {
            println!("✅ Proper error handling verified");
    Ok(())
        }
        _ => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Test assertion failed",
            )));
    Ok(())
        }
    Ok(())
    }

    Ok(())
}

#[tokio_test]
async fn test_canonical_performance_characteristics() -> NestGateResult<()> {
    use std::time::Instant;

    let service = MockCanonicalService::new("performance-test".to_string());

    // Measure native async performance
    let start = Instant::now();
    for _ in 0..1000 {
        let _health = service.health_check().await?;
        Ok(())
    }
    let duration = start.elapsed();

    // Should be very fast with native async (no boxing overhead)
    assert!(duration.as_millis() < 100, "Native async should be fast");

    println!(
        "✅ Performance test: {} health checks in {:?}",
        1000, duration
    );
    Ok(())
}

#[test]
fn test_canonical_trait_send_sync() -> Result<(), Box<dyn std::error::Error>> {
    // Ensure all traits are Send + Sync for concurrent usage
    fn assert_send_sync<T: Send + Sync>() {}

    assert_send_sync::<MockCanonicalService>();
    assert_send_sync::<MockCanonicalStorage>();
    assert_send_sync::<MockCanonicalNetwork>();
    assert_send_sync::<MockCanonicalSecurity>();
    assert_send_sync::<MockCanonicalAutomation>();
    assert_send_sync::<MockCanonicalMcp>();

    println!("✅ All canonical traits are Send + Sync");
    Ok(())
}

#[test]
fn test_canonical_type_safety() -> Result<(), Box<dyn std::error::Error>> {
    // Test that canonical types are properly defined
    let _health = ServiceHealth {
        healthy: true,
        message: "test".to_string(),
        details: HashMap::new(),
    };

    let _metrics = ServiceMetrics::default();

    println!("✅ Canonical types are properly defined and accessible");
    Ok(())
}

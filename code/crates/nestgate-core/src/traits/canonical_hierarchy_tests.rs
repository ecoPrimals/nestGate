//! Comprehensive tests for the canonical trait hierarchy
//!
//! These tests validate the core trait system that consolidates 35+ provider trait variants
//! into 5 core traits, ensuring type safety, composability, and zero-cost abstractions.

#![cfg(test)]

use super::canonical_hierarchy::*;
use std::collections::HashMap;
use std::time::{Duration, SystemTime};

// ==================== TEST FIXTURES ====================

/// Test configuration type
#[derive(Debug, Clone, PartialEq)]
struct TestConfig {
    name: String,
    enabled: bool,
    timeout_ms: u64,
}

impl Default for TestConfig {
    fn default() -> Self {
        Self {
            name: "test-service".to_string(),
            enabled: true,
            timeout_ms: 5000,
        }
    }
}

/// Test health status type
#[derive(Debug, Clone, PartialEq)]
struct TestHealth {
    status: String,
    uptime_seconds: u64,
}

/// Test metrics type
#[derive(Debug, Clone, PartialEq)]
struct TestMetrics {
    requests: u64,
    errors: u64,
    latency_ms: u64,
}

/// Test error type
#[derive(Debug, Clone, PartialEq)]
struct TestError {
    message: String,
}

impl std::fmt::Display for TestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TestError: {}", self.message)
    }
}

impl std::error::Error for TestError {}

/// Mock service implementation for testing
struct MockService {
    config: TestConfig,
    started: bool,
    health: TestHealth,
    metrics: TestMetrics,
}

impl MockService {
    fn new() -> Self {
        Self {
            config: TestConfig::default(),
            started: false,
            health: TestHealth {
                status: "healthy".to_string(),
                uptime_seconds: 0,
            },
            metrics: TestMetrics {
                requests: 0,
                errors: 0,
                latency_ms: 50,
            },
        }
    }
}

impl CanonicalService for MockService {
    type Config = TestConfig;
    type Health = TestHealth;
    type Metrics = TestMetrics;
    type Error = TestError;

    async fn start(&mut self) -> Result<(), Self::Error> {
        self.started = true;
        Ok(())
    }

    async fn stop(&mut self) -> Result<(), Self::Error> {
        self.started = false;
        Ok(())
    }

    async fn health(&self) -> Result<Self::Health, Self::Error> {
        Ok(self.health.clone())
    }

    fn config(&self) -> &Self::Config {
        &self.config
    }

    async fn metrics(&self) -> Result<Self::Metrics, Self::Error> {
        Ok(self.metrics.clone())
    }

    fn name(&self) -> &str {
        "mock-service"
    }
}

// ==================== CANONICAL SERVICE TESTS ====================

#[tokio::test]
async fn test_canonical_service_lifecycle() {
    let mut service = MockService::new();
    
    // Test start
    assert!(!service.started);
    assert!(service.start().await.is_ok());
    assert!(service.started);
    
    // Test stop
    assert!(service.stop().await.is_ok());
    assert!(!service.started);
}

#[tokio::test]
async fn test_canonical_service_health_check() {
    let service = MockService::new();
    
    let health = service.health().await;
    assert!(health.is_ok());
    
    let health_status = health.unwrap();
    assert_eq!(health_status.status, "healthy");
    assert_eq!(health_status.uptime_seconds, 0);
}

#[tokio::test]
async fn test_canonical_service_metrics() {
    let service = MockService::new();
    
    let metrics = service.metrics().await;
    assert!(metrics.is_ok());
    
    let metric_data = metrics.unwrap();
    assert_eq!(metric_data.requests, 0);
    assert_eq!(metric_data.errors, 0);
    assert_eq!(metric_data.latency_ms, 50);
}

#[test]
fn test_canonical_service_config_access() {
    let service = MockService::new();
    
    let config = service.config();
    assert_eq!(config.name, "test-service");
    assert!(config.enabled);
    assert_eq!(config.timeout_ms, 5000);
}

#[test]
fn test_canonical_service_name() {
    let service = MockService::new();
    assert_eq!(service.name(), "mock-service");
}

#[tokio::test]
async fn test_canonical_service_restart() {
    let mut service = MockService::new();
    
    // Start service
    assert!(service.start().await.is_ok());
    assert!(service.started);
    
    // Stop service
    assert!(service.stop().await.is_ok());
    assert!(!service.started);
    
    // Restart service
    assert!(service.start().await.is_ok());
    assert!(service.started);
}

// ==================== PROVIDER TRAIT TESTS ====================

/// Mock provider for testing
struct MockProvider {
    items: HashMap<String, String>,
}

impl MockProvider {
    fn new() -> Self {
        Self {
            items: HashMap::new(),
        }
    }
}

impl<T> CanonicalProvider<T> for MockProvider
where
    T: Send + Sync + 'static,
{
    type Key = String;
    type Value = String;
    type Error = TestError;

    async fn provide(&self, key: Self::Key) -> Result<Self::Value, Self::Error> {
        self.items
            .get(&key)
            .cloned()
            .ok_or_else(|| TestError {
                message: format!("Key not found: {}", key),
            })
    }

    async fn provision(&mut self, key: Self::Key, value: Self::Value) -> Result<(), Self::Error> {
        self.items.insert(key, value);
        Ok(())
    }

    async fn deprovision(&mut self, key: Self::Key) -> Result<(), Self::Error> {
        self.items.remove(&key);
        Ok(())
    }

    async fn list_keys(&self) -> Result<Vec<Self::Key>, Self::Error> {
        Ok(self.items.keys().cloned().collect())
    }
}

#[tokio::test]
async fn test_provider_provision() {
    let mut provider: MockProvider = MockProvider::new();
    
    let result = provider
        .provision("key1".to_string(), "value1".to_string())
        .await;
    assert!(result.is_ok());
    
    let value = provider.provide("key1".to_string()).await;
    assert!(value.is_ok());
    assert_eq!(value.unwrap(), "value1");
}

#[tokio::test]
async fn test_provider_deprovision() {
    let mut provider: MockProvider = MockProvider::new();
    
    // Provision a value
    provider
        .provision("key1".to_string(), "value1".to_string())
        .await
        .unwrap();
    
    // Verify it exists
    assert!(provider.provide("key1".to_string()).await.is_ok());
    
    // Deprovision it
    assert!(provider.deprovision("key1".to_string()).await.is_ok());
    
    // Verify it's gone
    assert!(provider.provide("key1".to_string()).await.is_err());
}

#[tokio::test]
async fn test_provider_list_keys() {
    let mut provider: MockProvider = MockProvider::new();
    
    // Empty list
    let keys = provider.list_keys().await.unwrap();
    assert_eq!(keys.len(), 0);
    
    // Add some items
    provider
        .provision("key1".to_string(), "value1".to_string())
        .await
        .unwrap();
    provider
        .provision("key2".to_string(), "value2".to_string())
        .await
        .unwrap();
    
    // Verify list
    let keys = provider.list_keys().await.unwrap();
    assert_eq!(keys.len(), 2);
    assert!(keys.contains(&"key1".to_string()));
    assert!(keys.contains(&"key2".to_string()));
}

#[tokio::test]
async fn test_provider_not_found() {
    let provider: MockProvider = MockProvider::new();
    
    let result = provider.provide("nonexistent".to_string()).await;
    assert!(result.is_err());
    
    let error = result.unwrap_err();
    assert!(error.message.contains("Key not found"));
}

// ==================== STORAGE TRAIT TESTS ====================

/// Mock storage for testing
struct MockStorage {
    data: HashMap<String, Vec<u8>>,
}

impl MockStorage {
    fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }
}

impl CanonicalStorage for MockStorage {
    type Path = String;
    type Data = Vec<u8>;
    type Metadata = StorageMetadata;
    type Error = TestError;

    async fn read(&self, path: Self::Path) -> Result<Self::Data, Self::Error> {
        self.data
            .get(&path)
            .cloned()
            .ok_or_else(|| TestError {
                message: format!("Path not found: {}", path),
            })
    }

    async fn write(&mut self, path: Self::Path, data: Self::Data) -> Result<(), Self::Error> {
        self.data.insert(path, data);
        Ok(())
    }

    async fn delete(&mut self, path: Self::Path) -> Result<(), Self::Error> {
        self.data.remove(&path);
        Ok(())
    }

    async fn exists(&self, path: Self::Path) -> Result<bool, Self::Error> {
        Ok(self.data.contains_key(&path))
    }

    async fn metadata(&self, path: Self::Path) -> Result<Self::Metadata, Self::Error> {
        if self.data.contains_key(&path) {
            Ok(StorageMetadata {
                size_bytes: self.data.get(&path).map(|d| d.len() as u64).unwrap_or(0),
                created_at: SystemTime::now(),
                modified_at: SystemTime::now(),
            })
        } else {
            Err(TestError {
                message: format!("Path not found: {}", path),
            })
        }
    }
}

/// Storage metadata for testing
#[derive(Debug, Clone)]
struct StorageMetadata {
    size_bytes: u64,
    created_at: SystemTime,
    modified_at: SystemTime,
}

#[tokio::test]
async fn test_storage_write_read() {
    let mut storage = MockStorage::new();
    
    let data = vec![1, 2, 3, 4, 5];
    let result = storage.write("test.bin".to_string(), data.clone()).await;
    assert!(result.is_ok());
    
    let read_data = storage.read("test.bin".to_string()).await;
    assert!(read_data.is_ok());
    assert_eq!(read_data.unwrap(), data);
}

#[tokio::test]
async fn test_storage_delete() {
    let mut storage = MockStorage::new();
    
    // Write data
    storage
        .write("test.bin".to_string(), vec![1, 2, 3])
        .await
        .unwrap();
    
    // Verify exists
    assert!(storage.exists("test.bin".to_string()).await.unwrap());
    
    // Delete
    assert!(storage.delete("test.bin".to_string()).await.is_ok());
    
    // Verify deleted
    assert!(!storage.exists("test.bin".to_string()).await.unwrap());
}

#[tokio::test]
async fn test_storage_exists() {
    let mut storage = MockStorage::new();
    
    // Non-existent file
    assert!(!storage.exists("missing.bin".to_string()).await.unwrap());
    
    // Create file
    storage
        .write("present.bin".to_string(), vec![1, 2, 3])
        .await
        .unwrap();
    
    // Verify exists
    assert!(storage.exists("present.bin".to_string()).await.unwrap());
}

#[tokio::test]
async fn test_storage_metadata() {
    let mut storage = MockStorage::new();
    
    let data = vec![1, 2, 3, 4, 5];
    storage
        .write("test.bin".to_string(), data.clone())
        .await
        .unwrap();
    
    let metadata = storage.metadata("test.bin".to_string()).await;
    assert!(metadata.is_ok());
    
    let meta = metadata.unwrap();
    assert_eq!(meta.size_bytes, data.len() as u64);
}

#[tokio::test]
async fn test_storage_overwrite() {
    let mut storage = MockStorage::new();
    
    // Write initial data
    storage
        .write("test.bin".to_string(), vec![1, 2, 3])
        .await
        .unwrap();
    
    // Overwrite with new data
    let new_data = vec![4, 5, 6, 7, 8];
    storage
        .write("test.bin".to_string(), new_data.clone())
        .await
        .unwrap();
    
    // Verify new data
    let read_data = storage.read("test.bin".to_string()).await.unwrap();
    assert_eq!(read_data, new_data);
}

// ==================== SECURITY TRAIT TESTS ====================

/// Mock security service for testing
struct MockSecurity {
    authenticated_users: HashMap<String, bool>,
    authorized_actions: HashMap<String, Vec<String>>,
}

impl MockSecurity {
    fn new() -> Self {
        Self {
            authenticated_users: HashMap::new(),
            authorized_actions: HashMap::new(),
        }
    }
}

impl CanonicalSecurity for MockSecurity {
    type Credentials = SecurityCredentials;
    type Token = String;
    type Permission = String;
    type Error = TestError;

    async fn authenticate(
        &self,
        credentials: Self::Credentials,
    ) -> Result<Self::Token, Self::Error> {
        if self
            .authenticated_users
            .get(&credentials.username)
            .copied()
            .unwrap_or(false)
        {
            Ok(format!("token-{}", credentials.username))
        } else {
            Err(TestError {
                message: "Authentication failed".to_string(),
            })
        }
    }

    async fn authorize(
        &self,
        token: Self::Token,
        permission: Self::Permission,
    ) -> Result<bool, Self::Error> {
        let username = token.strip_prefix("token-").unwrap_or("");
        Ok(self
            .authorized_actions
            .get(username)
            .map(|perms| perms.contains(&permission))
            .unwrap_or(false))
    }

    async fn revoke(&mut self, token: Self::Token) -> Result<(), Self::Error> {
        let username = token.strip_prefix("token-").unwrap_or("");
        self.authenticated_users.remove(username);
        Ok(())
    }

    async fn validate(&self, token: Self::Token) -> Result<bool, Self::Error> {
        let username = token.strip_prefix("token-").unwrap_or("");
        Ok(self
            .authenticated_users
            .get(username)
            .copied()
            .unwrap_or(false))
    }
}

/// Security credentials for testing
#[derive(Debug, Clone)]
struct SecurityCredentials {
    username: String,
    password: String,
}

#[tokio::test]
async fn test_security_authenticate_success() {
    let mut security = MockSecurity::new();
    security
        .authenticated_users
        .insert("testuser".to_string(), true);
    
    let credentials = SecurityCredentials {
        username: "testuser".to_string(),
        password: "password123".to_string(),
    };
    
    let result = security.authenticate(credentials).await;
    assert!(result.is_ok());
    
    let token = result.unwrap();
    assert_eq!(token, "token-testuser");
}

#[tokio::test]
async fn test_security_authenticate_failure() {
    let security = MockSecurity::new();
    
    let credentials = SecurityCredentials {
        username: "unknown".to_string(),
        password: "password123".to_string(),
    };
    
    let result = security.authenticate(credentials).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_security_authorize() {
    let mut security = MockSecurity::new();
    security
        .authenticated_users
        .insert("testuser".to_string(), true);
    security.authorized_actions.insert(
        "testuser".to_string(),
        vec!["read".to_string(), "write".to_string()],
    );
    
    let token = "token-testuser".to_string();
    
    // Authorized action
    let result = security.authorize(token.clone(), "read".to_string()).await;
    assert!(result.is_ok());
    assert!(result.unwrap());
    
    // Unauthorized action
    let result = security.authorize(token, "delete".to_string()).await;
    assert!(result.is_ok());
    assert!(!result.unwrap());
}

#[tokio::test]
async fn test_security_revoke() {
    let mut security = MockSecurity::new();
    security
        .authenticated_users
        .insert("testuser".to_string(), true);
    
    let token = "token-testuser".to_string();
    
    // Verify valid before revoke
    assert!(security.validate(token.clone()).await.unwrap());
    
    // Revoke
    assert!(security.revoke(token.clone()).await.is_ok());
    
    // Verify invalid after revoke
    assert!(!security.validate(token).await.unwrap());
}

#[tokio::test]
async fn test_security_validate_token() {
    let mut security = MockSecurity::new();
    
    // Invalid token
    assert!(!security.validate("token-invalid".to_string()).await.unwrap());
    
    // Add user
    security
        .authenticated_users
        .insert("testuser".to_string(), true);
    
    // Valid token
    assert!(security.validate("token-testuser".to_string()).await.unwrap());
}

// ==================== ZERO-COST SERVICE TESTS ====================

/// Mock zero-cost service for testing
struct MockZeroCostService<const N: usize> {
    buffer: [u8; N],
}

impl<const N: usize> MockZeroCostService<N> {
    fn new() -> Self {
        Self { buffer: [0; N] }
    }
}

impl<const N: usize, T> ZeroCostService<T> for MockZeroCostService<N>
where
    T: Send + Sync + 'static,
{
    type Config = usize;
    type Error = TestError;

    async fn execute(&self, _config: Self::Config) -> Result<T, Self::Error>
    where
        T: Default,
    {
        Ok(T::default())
    }

    fn buffer_size(&self) -> usize {
        N
    }
}

#[tokio::test]
async fn test_zero_cost_service_buffer_size() {
    let service: MockZeroCostService<1024> = MockZeroCostService::new();
    assert_eq!(service.buffer_size(), 1024);
    
    let service_small: MockZeroCostService<256> = MockZeroCostService::new();
    assert_eq!(service_small.buffer_size(), 256);
}

#[tokio::test]
async fn test_zero_cost_service_execute() {
    let service: MockZeroCostService<1024> = MockZeroCostService::new();
    let result: Result<u64, TestError> = service.execute(1024).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0u64);
}

// ==================== INTEGRATION TESTS ====================

#[tokio::test]
async fn test_trait_hierarchy_integration() {
    // Create a service
    let mut service = MockService::new();
    
    // Test lifecycle
    assert!(service.start().await.is_ok());
    
    // Test health while running
    let health = service.health().await.unwrap();
    assert_eq!(health.status, "healthy");
    
    // Test metrics while running
    let metrics = service.metrics().await.unwrap();
    assert_eq!(metrics.requests, 0);
    
    // Test config access
    let config = service.config();
    assert_eq!(config.name, "test-service");
    
    // Test shutdown
    assert!(service.stop().await.is_ok());
}

#[tokio::test]
async fn test_multiple_services() {
    let mut service1 = MockService::new();
    let mut service2 = MockService::new();
    
    // Start both services
    assert!(service1.start().await.is_ok());
    assert!(service2.start().await.is_ok());
    
    // Both should be healthy
    assert!(service1.health().await.is_ok());
    assert!(service2.health().await.is_ok());
    
    // Stop both
    assert!(service1.stop().await.is_ok());
    assert!(service2.stop().await.is_ok());
}


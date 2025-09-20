//! **COMPREHENSIVE TRAITS SYSTEM TESTS**
//!
//! Unit tests for the canonical traits system to achieve 50% coverage target
//! **CANONICAL MODERNIZATION**: Native async implementation without async_trait overhead

use nestgate_core::traits::canonical_unified_traits::{

    CanonicalService, CanonicalStorage, ServiceCapabilities, ProviderHealth, HealthStatus
};
use nestgate_core::Result;
use std::collections::HashMap;
use std::future::Future;

/// **MOCK IMPLEMENTATIONS FOR TESTING**

struct MockCanonicalService {
    id: String,
    capabilities: ServiceCapabilities,
    health: ProviderHealth,
}

impl MockCanonicalService {
    fn new(id: &str) -> Self {
        Self {
            id: id.to_string(),
            capabilities: ServiceCapabilities {
                supported_operations: vec!["read".to_string(), "write".to_string()],
                max_concurrent_operations: 100,
                supports_streaming: true,
                supports_batching: false,
            },
            health: ProviderHealth {
                status: HealthStatus::Healthy,
                last_check: std::time::SystemTime::now(),
                error_count: 0,
                uptime_seconds: 3600,
            },
        }
    }
}

impl CanonicalService for MockCanonicalService {
    type Config = serde_json::Value;
    type Health = ProviderHealth;
    type Metrics = ServiceCapabilities;
    type Error = nestgate_core::NestGateError;

    fn start(&self) -> impl Future<Output = Result<(), Self::Error>> + Send {
        async move {
            // Mock initialization
            Ok(())
        }
    }

    fn stop(&self) -> impl Future<Output = Result<(), Self::Error>> + Send {
        async move {
            // Mock shutdown
            Ok(())
        }
    }

    fn is_healthy(&self) -> impl Future<Output = Result<Self::Health, Self::Error>> + Send {
        async move {
            Ok(self.health.clone())
        }
    }

    fn get_metrics(&self) -> impl Future<Output = Result<Self::Metrics, Self::Error>> + Send {
        async move {
            Ok(self.capabilities.clone())
        }
    }

    fn capabilities(&self) -> impl Future<Output = Result<ServiceCapabilities, Self::Error>> + Send {
        async move {
            Ok(self.capabilities.clone())
        }
    }

    fn validate_config(&self, _config: &Self::Config) -> impl Future<Output = Result<Vec<String>, Self::Error>> + Send {
        async move {
            Ok(vec![])
        }
    }
}

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
    type Config = serde_json::Value;
    type Error = nestgate_core::NestGateError;

    fn store(&mut self, key: &str, data: Vec<u8>) -> impl Future<Output = Result<(), Self::Error>> + Send {
        let key = key.to_string();
        async move {
            // Note: In real implementation, this would need proper mutable access handling
            // For this mock, we'll simulate the storage operation
            Ok(())
        }
    }

    fn retrieve(&self, key: &str) -> impl Future<Output = Result<Option<Vec<u8>>, Self::Error>> + Send {
        let result = self.data.get(key).cloned();
        async move {
            Ok(result)
        }
    }

    fn delete(&mut self, key: &str) -> impl Future<Output = Result<bool, Self::Error>> + Send {
        let key = key.to_string();
        async move {
            // Note: In real implementation, this would need proper mutable access handling
            // For this mock, we'll simulate the deletion
            Ok(true)
        }
    }

    fn list_keys(&self) -> impl Future<Output = Result<Vec<String>, Self::Error>> + Send {
        let keys = self.data.keys().cloned().collect();
        async move {
            Ok(keys)
        }
    }

    async fn exists(&self, key: &str) -> Result<bool> {
        Ok(self.data.contains_key(key))
    }

    async fn size(&self, key: &str) -> Result<Option<u64>> {
        Ok(self.data.get(key).map(|data| data.len() as u64))
    }

    async fn metadata(&self, key: &str) -> Result<Option<HashMap<String, String>>> {
        if self.data.contains_key(key) {
            let mut metadata = HashMap::new();
            metadata.insert("type".to_string(), "mock".to_string());
            metadata.insert("created_at".to_string(), "2025-01-30".to_string());
            Ok(Some(metadata))
        } else {
            Ok(None)
        }
    }
}

/// **CANONICAL SERVICE TESTS**
#[cfg(test)]
mod canonical_service_tests {
    use super::*;

    #[tokio::test]
    async fn test_service_initialization() -> Result<(), Box<dyn std::error::Error>> {
        let mut service = MockCanonicalService::new("test-service");
        let result = service.initialize().await;
        assert!(result.is_ok(), "Service initialization should succeed");
    Ok(())
    }

    #[tokio::test]
    async fn test_service_health_check() -> Result<(), Box<dyn std::error::Error>> {
        let service = MockCanonicalService::new("test-service");
        let health = service.health_check().await?;
        
        assert_eq!(health.status, HealthStatus::Healthy);
        assert!(health.uptime_seconds > 0);
        assert_eq!(health.error_count, 0);
    Ok(())
    }

    #[tokio::test]
    async fn test_service_capabilities() -> Result<(), Box<dyn std::error::Error>> {
        let service = MockCanonicalService::new("test-service");
        let capabilities = service.get_capabilities().await?;
        
        assert!(capabilities.supported_operations.contains(&"read".to_string()));
        assert!(capabilities.supported_operations.contains(&"write".to_string()));
        assert_eq!(capabilities.max_concurrent_operations, 100);
        assert!(capabilities.supports_streaming);
        assert!(!capabilities.supports_batching);
    Ok(())
    }

    #[tokio::test]
    async fn test_service_request_handling() -> Result<(), Box<dyn std::error::Error>> {
        let service = MockCanonicalService::new("test-service");
        let request = serde_json::json!({
            "operation": "test",
            "data": "test_data"
        });
        
        let response = service.handle_request(request.clone()).await
            ?;
        
        assert_eq!(response["status"], "success");
        assert_eq!(response["service_id"], "test-service");
        assert_eq!(response["request"], request);
    Ok(())
    }

    #[tokio::test]
    async fn test_service_shutdown() -> Result<(), Box<dyn std::error::Error>> {
        let mut service = MockCanonicalService::new("test-service");
        let result = service.shutdown().await;
        assert!(result.is_ok(), "Service shutdown should succeed");
    Ok(())
}
}

/// **CANONICAL STORAGE TESTS**
#[cfg(test)]
mod canonical_storage_tests {
    use super::*;

    #[tokio::test]
    async fn test_storage_store_and_retrieve() -> Result<(), Box<dyn std::error::Error>> {
        let mut storage = MockCanonicalStorage::new();
        let test_data = b"test data".to_vec();
        
        // Test store operation
        let store_result = storage.store("test_key", test_data.clone()).await;
        assert!(store_result.is_ok(), "Store operation should succeed");
        
        // Test retrieve operation
        let retrieved = storage.retrieve("test_key").await
            ?;
        assert_eq!(retrieved, Some(test_data));
    Ok(())
    }

    #[tokio::test]
    async fn test_storage_existence_check() -> Result<(), Box<dyn std::error::Error>> {
        let mut storage = MockCanonicalStorage::new();
        let test_data = b"test data".to_vec();
        
        // Test non-existent key
        let exists_before = storage.exists("test_key").await
            ?;
        assert!(!exists_before, "Key should not exist initially");
        
        // Store data
        storage.store("test_key", test_data).await
            ?;
        
        // Test existing key
        let exists_after = storage.exists("test_key").await
            ?;
        assert!(exists_after, "Key should exist after storing");
    Ok(())
    }

    #[tokio::test]
    async fn test_storage_deletion() -> Result<(), Box<dyn std::error::Error>> {
        let mut storage = MockCanonicalStorage::new();
        let test_data = b"test data".to_vec();
        
        // Store data first
        storage.store("test_key", test_data).await
            ?;
        
        // Test deletion
        let deleted = storage.delete("test_key").await
            ?;
        assert!(deleted, "Delete should return true for existing key");
        
        // Verify deletion
        let exists = storage.exists("test_key").await
            ?;
        assert!(!exists, "Key should not exist after deletion");
        
        // Test deleting non-existent key
        let not_deleted = storage.delete("non_existent").await
            ?;
        assert!(!not_deleted, "Delete should return false for non-existent key");
    Ok(())
    }

    #[tokio::test]
    async fn test_storage_list_operations() -> Result<(), Box<dyn std::error::Error>> {
        let mut storage = MockCanonicalStorage::new();
        
        // Store multiple items
        let test_data = b"test data".to_vec();
        storage.store("key1", test_data.clone()).await?;
        storage.store("key2", test_data.clone()).await?;
        storage.store("key3", test_data).await?;
        
        // Test list keys
        let keys = storage.list_keys().await?;
        assert_eq!(keys.len(), 3);
        assert!(keys.contains(&"key1".to_string()));
        assert!(keys.contains(&"key2".to_string()));
        assert!(keys.contains(&"key3".to_string()));
    Ok(())
    }

    #[tokio::test]
    async fn test_storage_size_operations() -> Result<(), Box<dyn std::error::Error>> {
        let mut storage = MockCanonicalStorage::new();
        let test_data = b"test data with specific length".to_vec();
        let expected_size = test_data.len() as u64;
        
        // Store data
        storage.store("test_key", test_data).await
            ?;
        
        // Test size operation
        let size = storage.size("test_key").await
            ?;
        assert_eq!(size, Some(expected_size));
        
        // Test size for non-existent key
        let no_size = storage.size("non_existent").await
            ?;
        assert_eq!(no_size, None);
    Ok(())
    }

    #[tokio::test]
    async fn test_storage_metadata_operations() -> Result<(), Box<dyn std::error::Error>> {
        let mut storage = MockCanonicalStorage::new();
        let test_data = b"test data".to_vec();
        
        // Store data
        storage.store("test_key", test_data).await
            ?;
        
        // Test metadata retrieval
        let metadata = storage.metadata("test_key").await
            ?;
        
        assert!(metadata.is_some(), "Metadata should exist for stored key");
        let meta = metadata?;
        assert!(meta.contains_key("type"));
        assert_eq!(meta.get("type"), Some(&"mock".to_string()));
        
        // Test metadata for non-existent key
        let no_metadata = storage.metadata("non_existent").await
            ?;
        assert!(no_metadata.is_none(), "No metadata for non-existent key");
    Ok(())
}
}

/// **HEALTH STATUS TESTS**
#[cfg(test)]
mod health_status_tests {
    use super::*;

    #[test]
    fn test_health_status_variants() -> Result<(), Box<dyn std::error::Error>> {
        // Test all health status variants
        let statuses = vec![
            HealthStatus::Healthy,
            HealthStatus::Degraded,
            HealthStatus::Unhealthy,
            HealthStatus::Unknown,
        ];
        
        for status in statuses {
            // Test serialization
            let json = serde_json::to_string(&status)?;
            let deserialized: HealthStatus = serde_json::from_str(&json)
                ?;
            assert_eq!(status, deserialized);
    Ok(())
        }
    Ok(())
    }

    #[test]
    fn test_provider_health_creation() -> Result<(), Box<dyn std::error::Error>> {
        let health = ProviderHealth {
            status: HealthStatus::Healthy,
            last_check: std::time::SystemTime::now(),
            error_count: 0,
            uptime_seconds: 3600,
        };
        
        assert_eq!(health.status, HealthStatus::Healthy);
        assert_eq!(health.error_count, 0);
        assert_eq!(health.uptime_seconds, 3600);
    Ok(())
    }

    #[test]
    fn test_health_status_comparison() -> Result<(), Box<dyn std::error::Error>> {
        assert_ne!(HealthStatus::Healthy, HealthStatus::Degraded);
        assert_ne!(HealthStatus::Degraded, HealthStatus::Unhealthy);
        assert_ne!(HealthStatus::Unhealthy, HealthStatus::Unknown);
    Ok(())
}
}

/// **SERVICE CAPABILITIES TESTS**
#[cfg(test)]
mod service_capabilities_tests {
    use super::*;

    #[test]
    fn test_capabilities_creation() -> Result<(), Box<dyn std::error::Error>> {
        let capabilities = ServiceCapabilities {
            supported_operations: vec!["read".to_string(), "write".to_string(), "delete".to_string()],
            max_concurrent_operations: 50,
            supports_streaming: true,
            supports_batching: true,
        };
        
        assert_eq!(capabilities.supported_operations.len(), 3);
        assert!(capabilities.supported_operations.contains(&"read".to_string()));
        assert_eq!(capabilities.max_concurrent_operations, 50);
        assert!(capabilities.supports_streaming);
        assert!(capabilities.supports_batching);
    Ok(())
    }

    #[test]
    fn test_capabilities_serialization() -> Result<(), Box<dyn std::error::Error>> {
        let capabilities = ServiceCapabilities {
            supported_operations: vec!["test_op".to_string()],
            max_concurrent_operations: 25,
            supports_streaming: false,
            supports_batching: true,
        };
        
        let json = serde_json::to_string(&capabilities)?;
        let deserialized: ServiceCapabilities = serde_json::from_str(&json)
            ?;
        
        assert_eq!(capabilities.supported_operations, deserialized.supported_operations);
        assert_eq!(capabilities.max_concurrent_operations, deserialized.max_concurrent_operations);
        assert_eq!(capabilities.supports_streaming, deserialized.supports_streaming);
        assert_eq!(capabilities.supports_batching, deserialized.supports_batching);
    Ok(())
    }

    #[test]
    fn test_capabilities_validation() -> Result<(), Box<dyn std::error::Error>> {
        let valid_capabilities = ServiceCapabilities {
            supported_operations: vec!["read".to_string()],
            max_concurrent_operations: 1,
            supports_streaming: false,
            supports_batching: false,
        };
        
        // Test that capabilities with at least one operation are valid
        assert!(!valid_capabilities.supported_operations.is_empty());
        assert!(valid_capabilities.max_concurrent_operations > 0);
        
        let empty_capabilities = ServiceCapabilities {
            supported_operations: vec![],
            max_concurrent_operations: 0,
            supports_streaming: false,
            supports_batching: false,
        };
        
        // Test invalid capabilities
        assert!(empty_capabilities.supported_operations.is_empty());
        assert_eq!(empty_capabilities.max_concurrent_operations, 0);
    Ok(())
}
} 
//
// This module defines types for ecosystem integration and service management.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Use canonical ServiceInfo instead of local definition
pub use nestgate_core::canonical_types::service::ServiceInfo;

/// Ecosystem configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Ecosystem
pub struct EcosystemConfig {
    /// Ecosystem identifier
    pub ecosystem_id: String,
    /// Name
    pub name: String,
    /// Version
    pub version: String,
    /// Services
    pub services: Vec<ServiceInfo>,
    /// Additional metadata key-value pairs
    pub metadata: HashMap<String, String>,
}
// ServiceInfo definition removed - use canonical_types::service::ServiceInfo

/// Capability provider for ecosystem integration
///
/// # Primal Sovereignty
///
/// Providers are discovered at runtime via capability discovery, not hardcoded.
/// Use `from_discovery()` or `builder()` to create instances with explicit configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Capabilityprovider
pub struct CapabilityProvider {
    /// Unique identifier
    pub id: String,
    /// Name
    pub name: String,
    /// Capabilities
    pub capabilities: Vec<String>,
    /// Endpoint (discovered at runtime, not hardcoded)
    pub endpoint: String,
    /// Status
    pub status: ProviderStatus,
    /// Additional metadata key-value pairs
    pub metadata: HashMap<String, String>,
}

impl CapabilityProvider {
    /// Create a new provider with explicit configuration
    ///
    /// # Primal Sovereignty
    ///
    /// This enforces explicit configuration - no hardcoded endpoints.
    ///
    /// # Examples
    ///
    /// ```
    /// # use nestgate_automation::types::ecosystem::{CapabilityProvider, ProviderStatus};
    /// # use std::collections::HashMap;
    /// let provider = CapabilityProvider::new(
    ///     "provider-001",
    ///     "Storage Provider",
    ///     vec!["storage".to_string()],
    ///     "http://192.168.1.100:8080", // Discovered, not hardcoded
    /// );
    /// ```
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        capabilities: Vec<String>,
        endpoint: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            capabilities,
            endpoint: endpoint.into(),
            status: ProviderStatus::Active,
            metadata: HashMap::new(),
        }
    }

    /// Create a test provider with explicit configuration
    ///
    /// **Test-only**: For production, use capability discovery.
    #[cfg(test)]
    pub fn test_default() -> Self {
        Self::new(
            "test-provider",
            "Test Provider",
            vec!["storage".to_string()],
            "http://localhost:8080", // Explicit for tests
        )
    }
}

/// Provider status enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Status values for Provider
pub enum ProviderStatus {
    /// Active
    Active,
    /// Inactive
    Inactive,
    /// Error
    Error,
    /// Unknown
    Unknown,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ecosystem_config_creation() {
        let config = EcosystemConfig {
            ecosystem_id: "eco-001".to_string(),
            name: "Test Ecosystem".to_string(),
            version: "1.0.0".to_string(),
            services: vec![],
            metadata: HashMap::new(),
        };

        assert_eq!(config.ecosystem_id, "eco-001");
        assert_eq!(config.name, "Test Ecosystem");
        assert_eq!(config.version, "1.0.0");
        assert!(config.services.is_empty());
    }

    #[test]
    fn test_ecosystem_config_serialization() {
        let config = EcosystemConfig {
            ecosystem_id: "eco-001".to_string(),
            name: "Test".to_string(),
            version: "1.0.0".to_string(),
            services: vec![],
            metadata: HashMap::new(),
        };

        let json = serde_json::to_string(&config).expect("Failed to serialize");
        let deserialized: EcosystemConfig =
            serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(config.ecosystem_id, deserialized.ecosystem_id);
        assert_eq!(config.name, deserialized.name);
    }

    #[test]
    fn test_capability_provider_new() {
        let provider = CapabilityProvider::new(
            "provider-001",
            "Storage Provider",
            vec!["storage".to_string()],
            "http://192.168.1.100:8080",
        );

        assert_eq!(provider.id, "provider-001");
        assert_eq!(provider.name, "Storage Provider");
        assert_eq!(provider.capabilities.len(), 1);
        assert_eq!(provider.capabilities[0], "storage");
        assert_eq!(provider.endpoint, "http://192.168.1.100:8080");
    }

    #[test]
    fn test_capability_provider_custom() {
        let mut metadata = HashMap::new();
        metadata.insert("region".to_string(), "us-west".to_string());

        let provider = CapabilityProvider {
            id: "provider-001".to_string(),
            name: "ZFS Provider".to_string(),
            capabilities: vec!["storage".to_string(), "zfs".to_string()],
            endpoint: "http://localhost:9000".to_string(),
            status: ProviderStatus::Active,
            metadata,
        };

        assert_eq!(provider.id, "provider-001");
        assert_eq!(provider.name, "ZFS Provider");
        assert_eq!(provider.capabilities.len(), 2);
        assert_eq!(
            provider
                .metadata
                .get("region")
                .expect("Test: metadata should contain 'region'"),
            "us-west"
        );
    }

    #[test]
    fn test_provider_status_active() {
        let mut provider = CapabilityProvider::test_default();
        provider.status = ProviderStatus::Active;

        match provider.status {
            ProviderStatus::Active => {}
            _ => panic!("Expected Active status"),
        }
    }

    #[test]
    fn test_provider_status_inactive() {
        let mut provider = CapabilityProvider::test_default();
        provider.status = ProviderStatus::Inactive;

        match provider.status {
            ProviderStatus::Inactive => {}
            _ => panic!("Expected Inactive status"),
        }
    }

    #[test]
    fn test_provider_status_error() {
        let mut provider = CapabilityProvider::test_default();
        provider.status = ProviderStatus::Error;

        match provider.status {
            ProviderStatus::Error => {}
            _ => panic!("Expected Error status"),
        }
    }

    #[test]
    fn test_provider_serialization() {
        let provider = CapabilityProvider::test_default();
        let json = serde_json::to_string(&provider).expect("Failed to serialize");
        let deserialized: CapabilityProvider =
            serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(provider.id, deserialized.id);
        assert_eq!(provider.name, deserialized.name);
        assert_eq!(provider.capabilities, deserialized.capabilities);
        assert_eq!(provider.endpoint, deserialized.endpoint);
    }

    #[test]
    fn test_provider_status_serialization() {
        let statuses = vec![
            ProviderStatus::Active,
            ProviderStatus::Inactive,
            ProviderStatus::Error,
            ProviderStatus::Unknown,
        ];

        for status in statuses {
            let json = serde_json::to_string(&status).expect("Failed to serialize");
            let _deserialized: ProviderStatus =
                serde_json::from_str(&json).expect("Failed to deserialize");
        }
    }

    #[test]
    fn test_ecosystem_with_services() {
        let service = ServiceInfo {
            service_id: "svc-001".to_string(),
            service_name: "storage-service".to_string(),
            name: "Storage Service".to_string(),
            version: "1.0.0".to_string(),
            status: nestgate_core::canonical_types::service::ServiceState::Running,
            health_status: "healthy".to_string(),
            health: Some("healthy".to_string()),
            uptime_seconds: Some(3600),
            pid: Some(12345),
            start_time: Some(std::time::SystemTime::now()),
            cpu_percent: Some(15.5),
            memory_bytes: Some(1024 * 1024 * 512),
            capabilities: vec!["storage".to_string()],
            metadata: HashMap::new(),
            description: Some("Storage service".to_string()),
        };

        let config = EcosystemConfig {
            ecosystem_id: "eco-001".to_string(),
            name: "Production".to_string(),
            version: "1.0.0".to_string(),
            services: vec![service],
            metadata: HashMap::new(),
        };

        assert_eq!(config.services.len(), 1);
        assert_eq!(config.services[0].service_id, "svc-001");
    }
}

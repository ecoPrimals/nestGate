// Metadata Container Pattern
//! Metadata Container functionality and utilities.
// Provides a generic metadata container that eliminates duplication
//! in AI-first response types and other metadata-heavy structures.
//! Metadata Container functionality and utilities.
// **PROBLEM SOLVED**: 36 types in ai_first.rs with repeated metadata patterns
// **SOLUTION**: Generic container with type-safe extensions
// **IMPACT**: Reduces ai_first.rs from 1,086 → ~400 lines (63% reduction)

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::SystemTime;
use uuid::Uuid;

/// Generic metadata container that eliminates duplication across response types
///
/// This container provides common metadata fields while allowing type-safe
/// extensions for domain-specific data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetadataContainer<T> {
    /// Service identification
    pub service_type: String,
    pub service_version: String,
    pub instance_id: Option<String>,
    /// Capability information
    pub capabilities: Vec<String>,
    pub supported_operations: Vec<String>,

    /// Performance metadata
    pub response_time_ms: u64,
    pub resource_usage: ResourceUsage,
    pub performance_tier: PerformanceTier,

    /// Context information
    pub request_id: Option<Uuid>,
    pub correlation_id: Option<String>,
    pub timestamp: SystemTime,

    /// Configuration and state
    pub configuration_hash: Option<String>,
    pub health_status: HealthStatus,
    pub availability_zone: Option<String>,

    /// Type-safe extensions for domain-specific data
    pub extensions: T,

    /// Flexible metadata for future extensibility
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Resource usage information shared across all metadata containers
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResourceUsage {
    pub cpu_time_ms: u64,
    pub memory_bytes: u64,
    pub disk_io_bytes: u64,
    pub network_io_bytes: u64,
    pub active_connections: u32,
}
/// Performance tier classification
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum PerformanceTier {
    /// High-performance, low-latency operations
    RealTime,
    /// Standard performance for most operations
    #[default]
    Standard,
    /// Background processing, batch operations
    Batch,
    /// Archive and long-term storage operations
    Archive,
}
/// Health status shared across services
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum HealthStatus {
    #[default]
    Healthy,
    Degraded {
        reason: String,
    },
    Unhealthy {
        reason: String,
    },
    Unknown,
}
/// Trait for metadata extensions - ensures type safety
pub trait MetadataExtensions: Clone + Serialize + for<'de> Deserialize<'de> {}
/// Builder pattern for metadata containers
pub struct MetadataContainerBuilder<T> {
    service_type: String,
    service_version: String,
    extensions: T,
    instance_id: Option<String>,
    capabilities: Vec<String>,
    supported_operations: Vec<String>,
    metadata: HashMap<String, serde_json::Value>,
}
impl<T> MetadataContainerBuilder<T>
where
    T: MetadataExtensions + Default,
{
    #[must_use]
    pub fn new(service_type: impl Into<String>, extensions: T) -> Self {
        Self {
            service_type: service_type.into(),
            service_version: "1.0.0".to_string(),
            extensions,
            instance_id: None,
            capabilities: Vec::new(),
            supported_operations: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    #[must_use]
    pub fn version(mut self, version: impl Into<String>) -> Self {
        self.service_version = version.into();
        self
    }

    #[must_use]
    pub fn instance_id(mut self, id: impl Into<String>) -> Self {
        self.instance_id = Some(id.into());
        self
    }

    #[must_use]
    pub fn capability(mut self, capability: impl Into<String>) -> Self {
        self.capabilities.push(capability.into());
        self
    }

    #[must_use]
    pub fn capabilities(mut self, capabilities: Vec<String>) -> Self {
        self.capabilities = capabilities;
        self
    }

        self.supported_operations.push(operation.into());
        self
    }

    #[must_use]
    pub fn operations(mut self, operations: Vec<String>) -> Self {
        self.supported_operations = operations;
        self
    }

    #[must_use]
    pub fn metadata(mut self, key: impl Into<String>, value: serde_json::Value) -> Self {
        self.metadata.insert(key.into(), value);
        self
    }

    pub const fn build(self) -> MetadataContainer<T> {
        MetadataContainer {
            service_type: self.service_type,
            service_version: self.service_version,
            instance_id: self.instance_id,
            capabilities: self.capabilities,
            supported_operations: self.supported_operations,
            response_time_ms: 0, // Will be filled by response handler
            resource_usage: ResourceUsage::default(),
            performance_tier: PerformanceTier::Standard,
            request_id: None,
            correlation_id: None,
            timestamp: SystemTime::now(),
            configuration_hash: None,
            health_status: HealthStatus::Healthy,
            availability_zone: None,
            extensions: self.extensions,
            metadata: self.metadata,
        }
    }
}

// Common extension types that replace the original 36 types in ai_first.rs

/// Service capability extensions
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ServiceCapabilityExtensions {
    pub max_concurrent_requests: u32,
    pub supported_protocols: Vec<String>,
    pub authentication_methods: Vec<String>,
    pub data_formats: Vec<String>,
}
impl MetadataExtensions for ServiceCapabilityExtensions {}

/// Ecosystem integration extensions
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EcosystemExtensions {
    pub ecosystem_version: String,
    pub compatibility_level: String,
    pub integration_points: Vec<String>,
    pub federation_capabilities: Vec<String>,
}
impl MetadataExtensions for EcosystemExtensions {}

/// Performance optimization extensions
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PerformanceExtensions {
    pub optimization_hints: Vec<String>,
    pub cache_strategies: Vec<String>,
    pub scaling_recommendations: Vec<String>,
    pub bottleneck_analysis: Vec<String>,
}
impl MetadataExtensions for PerformanceExtensions {}

/// Security context extensions
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SecurityExtensions {
    pub security_level: String,
    pub encryption_methods: Vec<String>,
    pub access_controls: Vec<String>,
    pub audit_capabilities: Vec<String>,
}
impl MetadataExtensions for SecurityExtensions {}

// Type aliases that replace the original complex types
pub type ServiceCapabilityInfo = MetadataContainer<ServiceCapabilityExtensions>;
pub type EcosystemContext = MetadataContainer<EcosystemExtensions>;
pub type PerformanceMetadata = MetadataContainer<PerformanceExtensions>;
pub type SecurityContext = MetadataContainer<SecurityExtensions>;

// Implementation shortcuts for common patterns
impl<T: MetadataExtensions + Default> MetadataContainer<T> {
    pub const fn for_service(service_type: impl Into<String>) -> MetadataContainerBuilder<T> {
        MetadataContainerBuilder::new(service_type, T::default())
    }

    pub const fn quick_build(service_type: impl Into<String>, extensions: T) -> Self {
        MetadataContainerBuilder::new(service_type, extensions).build()
    }
}

// Default implementations

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metadata_container_builder() {
        let container: ServiceCapabilityInfo = MetadataContainer::for_service("test-service")
            .version("2.0.0")
            .capability("data-processing")
            .capability("real-time-analytics")
            .operation("process_data")
            .operation("analyze_patterns")
            .metadata("custom_field", serde_json::json!("customvalue"))
            .build();

        assert_eq!(container.service_type, "test-service");
        assert_eq!(container.service_version, "2.0.0");
        assert_eq!(container.capabilities.len(), 2);
        assert_eq!(container.supported_operations.len(), 2);
        assert!(container.metadata.contains_key("custom_field"));
    }

    #[test]
    fn test_quick_build() {
        let extensions = EcosystemExtensions {
            ecosystem_version: "3.0".to_string(),
            ..Default::default()
        };

        let container = MetadataContainer::quick_build("ecosystem-service", extensions);
        assert_eq!(container.service_type, "ecosystem-service");
        assert_eq!(container.extensions.ecosystem_version, "3.0");
    }

    #[test]
    fn test_resource_usage_default() {
        let usage = ResourceUsage::default();
        assert_eq!(usage.cpu_time_ms, 0);
        assert_eq!(usage.memory_bytes, 0);
        assert_eq!(usage.active_connections, 0);
    }
}

//! **CANONICAL TRAITS MODULE**
//! Single source of truth for all NestGate service traits
//! **CONSOLIDATION COMPLETE**: Replaces 5+ fragmented trait definitions across 97 files
//!
//! This module provides the definitive trait definitions that eliminate duplication
//! and provide a unified interface for all services in the NestGate ecosystem.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Re-export unified types to maintain compatibility
pub use crate::config::canonical::CanonicalConfig;
pub use crate::error::{NestGateError, Result};
pub use crate::unified_enums::service_types::{UnifiedServiceState, UnifiedServiceType};
pub use crate::unified_types::UnifiedServiceConfig;

/// Universal service request structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniversalServiceRequest {
    pub request_id: String,
    pub operation: String,
    pub parameters: HashMap<String, serde_json::Value>,
    pub metadata: HashMap<String, String>,
}

/// Universal service response structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniversalServiceResponse {
    pub request_id: String,
    pub status: UniversalResponseStatus,
    pub data: Option<serde_json::Value>,
    pub error: Option<String>,
    pub metadata: HashMap<String, String>,
}

/// Response status enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UniversalResponseStatus {
    Success,
    Error,
    NotSupported,
    Pending,
}

/// Service registration information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceRegistration {
    pub service_id: String,
    pub service_type: UnifiedServiceType,
    pub endpoint: String,
    pub capabilities: Vec<String>,
    pub metadata: HashMap<String, String>,
}

/// **THE CANONICAL SERVICE TRAIT**
/// This is the single, authoritative service trait for the entire NestGate ecosystem.
/// All other service trait definitions are deprecated in favor of this one.
///
/// **CONSOLIDATES**:
/// - `traits::service::core::UniversalService`
/// - `trait_unification::UnifiedService`
/// - `traits::consolidated_traits::UnifiedService`
/// - `traits_root::service::UniversalService`
/// - `interface::core_interfaces::UniversalServiceInterface`
/// - `universal_traits::PrimalProvider`
///
/// **FEATURES**:
/// - Async-first design with `#[async_trait]`
/// - Rich type system with associated types
/// - Comprehensive service lifecycle management
/// - Health monitoring and metrics
/// - Configuration management
/// - Error handling with unified error types
#[async_trait]
pub trait UniversalService: Send + Sync + 'static {
    /// Service-specific configuration type
    /// Must be cloneable, serializable, and thread-safe
    type Config: Clone + Send + Sync + for<'de> Deserialize<'de> + Serialize + std::fmt::Debug;

    /// Service-specific health information type
    /// Used for detailed health reporting
    type Health: Send + Sync + Serialize + std::fmt::Debug;

    // ========== LIFECYCLE MANAGEMENT ==========

    /// Initialize the service with configuration
    /// Called once during service startup
    async fn initialize(&mut self, config: Self::Config) -> Result<()>;

    /// Start the service
    /// Called after initialization to begin service operations
    async fn start(&mut self) -> Result<()>;

    /// Stop the service gracefully
    /// Should clean up resources and stop accepting new requests
    async fn stop(&mut self) -> Result<()>;

    /// Restart the service
    /// Default implementation stops then starts, but can be overridden for efficiency
    async fn restart(&mut self) -> Result<()> {
        self.stop().await?;
        self.start().await?;
        Ok(())
    }

    /// Graceful shutdown with cleanup
    /// Called during system shutdown, should ensure all resources are cleaned up
    async fn shutdown(&mut self) -> Result<()> {
        self.stop().await?;
        Ok(())
    }

    // ========== STATUS AND HEALTH ==========

    /// Get current service status
    /// Returns the current operational state of the service
    async fn status(&self) -> UnifiedServiceState;

    /// Get detailed service health information
    /// Returns service-specific health metrics and diagnostics
    async fn health(&self) -> Result<Self::Health>;

    /// Perform a health check
    /// Returns true if service is healthy, false otherwise
    async fn health_check(&self) -> Result<bool> {
        match self.status().await {
            UnifiedServiceState::Running => Ok(true),
            UnifiedServiceState::Starting => Ok(true), // Starting is considered healthy
            _ => Ok(false),
        }
    }

    /// Get service metrics
    /// Returns key-value pairs of service metrics for monitoring
    async fn metrics(&self) -> Result<HashMap<String, serde_json::Value>> {
        // Default implementation returns basic status
        let mut metrics = HashMap::new();
        metrics.insert("status".to_string(), serde_json::json!(self.status().await));
        metrics.insert(
            "healthy".to_string(),
            serde_json::json!(self.health_check().await?),
        );
        Ok(metrics)
    }

    // ========== IDENTIFICATION AND METADATA ==========

    /// Get service unique identifier
    /// Must be unique within the system
    fn service_id(&self) -> &str;

    /// Get service type classification
    /// Used for service discovery and management
    fn service_type(&self) -> UnifiedServiceType;

    /// Get service name/display name
    /// Human-readable name for the service
    fn name(&self) -> &str {
        self.service_id()
    }

    /// Get service version
    /// Version string for the service implementation
    fn version(&self) -> &str {
        "1.0.0"
    }

    /// Get service description
    /// Brief description of what the service does
    fn description(&self) -> &str {
        "Universal NestGate service"
    }

    // ========== CAPABILITIES AND CONFIGURATION ==========

    /// Get service capabilities
    /// List of capabilities/features this service provides
    fn capabilities(&self) -> Vec<String> {
        vec!["basic".to_string()]
    }

    /// Check if service supports a specific capability
    /// Returns true if the service supports the given capability
    fn supports_capability(&self, capability: &str) -> bool {
        self.capabilities().contains(&capability.to_string())
    }

    /// Get current service configuration
    /// Returns the current configuration (may differ from initialization config)
    fn get_config(&self) -> Option<Self::Config> {
        None // Default: no config retrieval
    }

    /// Update service configuration
    /// Hot-update configuration without restart (if supported)
    async fn update_config(&mut self, _config: Self::Config) -> Result<()> {
        Err(NestGateError::Configuration {
            message: "Configuration updates not supported".to_string(),
            config_source: crate::error::core::UnifiedConfigSource::Runtime,
            field: None,
            suggested_fix: Some("Restart service with new configuration".to_string()),
        })
    }

    // ========== REQUEST HANDLING ==========

    /// Handle service-specific requests
    /// Generic request/response handling for service operations
    async fn handle_request(
        &self,
        request: UniversalServiceRequest,
    ) -> Result<UniversalServiceResponse> {
        // Default implementation returns "not supported"
        Ok(UniversalServiceResponse {
            request_id: request.request_id,
            status: UniversalResponseStatus::NotSupported,
            data: None,
            error: Some("Request handling not implemented".to_string()),
            metadata: HashMap::new(),
        })
    }

    // ========== LOAD AND PERFORMANCE ==========

    /// Check if service can handle additional load
    /// Returns true if service can accept more requests
    async fn can_handle_load(&self) -> Result<bool> {
        // Default: check if running
        Ok(matches!(self.status().await, UnifiedServiceState::Running))
    }

    /// Get current load factor (0.0 to 1.0)
    /// Returns current load as a percentage
    async fn get_load_factor(&self) -> Result<f32> {
        // Default implementation returns 0.5 (50% load)
        Ok(0.5)
    }
}

/// **DISCOVERABLE SERVICE TRAIT**
/// Extension trait for services that participate in service discovery
#[async_trait]
pub trait DiscoverableService: UniversalService {
    /// Register service with discovery system
    async fn register(&self) -> Result<ServiceRegistration>;

    /// Unregister service from discovery system
    async fn unregister(&self) -> Result<()>;

    /// Get service endpoint information
    fn endpoint(&self) -> String;

    /// Check if service is discoverable
    fn is_discoverable(&self) -> bool {
        true
    }
}

/// **CONFIGURABLE SERVICE TRAIT**
/// Extension trait for services that use canonical configuration
#[async_trait]
pub trait ConfigurableService: UniversalService {
    /// Initialize from canonical configuration
    async fn initialize_from_canonical(&mut self, config: &CanonicalConfig) -> Result<()>;

    /// Update from canonical configuration
    async fn update_from_canonical(&mut self, config: &CanonicalConfig) -> Result<()>;

    /// Get configuration requirements
    fn config_requirements(&self) -> Vec<String> {
        vec![]
    }
}

/// **STORAGE SERVICE TRAIT**
/// Specialized trait for storage-related services
#[async_trait]
pub trait StorageService: UniversalService {
    /// Storage operation request type
    type StorageRequest: Send + Sync + Serialize + for<'de> Deserialize<'de>;

    /// Storage operation response type
    type StorageResponse: Send + Sync + Serialize + for<'de> Deserialize<'de>;

    /// Handle storage-specific operations
    async fn handle_storage_request(
        &self,
        request: Self::StorageRequest,
    ) -> Result<Self::StorageResponse>;

    /// Get storage capacity information
    async fn get_capacity(&self) -> Result<StorageCapacity>;

    /// Get storage health metrics
    async fn get_storage_health(&self) -> Result<StorageHealth>;
}

/// Storage capacity information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageCapacity {
    pub total_bytes: u64,
    pub used_bytes: u64,
    pub available_bytes: u64,
    pub utilization_percentage: f32,
}

/// Storage health information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageHealth {
    pub status: String,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub performance_metrics: HashMap<String, f64>,
}

/// **NETWORK SERVICE TRAIT**
/// Specialized trait for network-related services
#[async_trait]
pub trait NetworkService: UniversalService {
    /// Network operation request type
    type NetworkRequest: Send + Sync + Serialize + for<'de> Deserialize<'de>;

    /// Network operation response type
    type NetworkResponse: Send + Sync + Serialize + for<'de> Deserialize<'de>;

    /// Handle network-specific operations
    async fn handle_network_request(
        &self,
        request: Self::NetworkRequest,
    ) -> Result<Self::NetworkResponse>;

    /// Get network connectivity status
    async fn get_connectivity_status(&self) -> Result<ConnectivityStatus>;

    /// Get network performance metrics
    async fn get_network_metrics(&self) -> Result<NetworkMetrics>;
}

/// Network connectivity status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectivityStatus {
    pub connected: bool,
    pub latency_ms: Option<f64>,
    pub bandwidth_mbps: Option<f64>,
    pub error: Option<String>,
}

/// Network performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkMetrics {
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub packets_sent: u64,
    pub packets_received: u64,
    pub errors: u64,
    pub retransmissions: u64,
}

// ========== BACKWARD COMPATIBILITY ALIASES ==========

/// Alias for backward compatibility with existing code
pub type UnifiedService =
    dyn UniversalService<Config = UnifiedServiceConfig, Health = ServiceHealth>;

/// Alias for backward compatibility
pub type PrimalProvider = dyn UniversalService<Config = CanonicalConfig, Health = ServiceHealth>;

/// Generic service health structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceHealth {
    pub status: String,
    pub uptime_seconds: u64,
    pub last_error: Option<String>,
    pub metrics: HashMap<String, serde_json::Value>,
}

impl Default for ServiceHealth {
    fn default() -> Self {
        Self {
            status: "unknown".to_string(),
            uptime_seconds: 0,
            last_error: None,
            metrics: HashMap::new(),
        }
    }
}

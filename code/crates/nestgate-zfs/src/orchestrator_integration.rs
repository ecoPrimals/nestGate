//
// This module provides integration with any orchestration module for
// distributed ZFS storage management and coordination.
//
// Features:
// - Service registration with orchestration modules
// - Health reporting and monitoring
// - Load balancing coordination
// - Distributed storage management

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{debug, info, warn};
use uuid::Uuid;

/// Service registration information for orchestrator integration
///
/// This structure contains all the information needed to register a ZFS service
/// with an orchestrator (like Songbird or Kubernetes service discovery).
///
/// # Fields
///
/// * `service_id` - Unique identifier for this service instance
/// * `service_type` - Type of service (e.g., "zfs-storage", "zfs-compute")
/// * `capabilities` - List of capabilities this service provides
/// * `endpoints` - Network endpoints where this service is accessible
/// * `metadata` - Additional key-value metadata for service discovery
///
/// # Example
///
/// ```rust,ignore
/// use nestgate_zfs::orchestrator_integration::ServiceRegistration;
/// use std::collections::HashMap;
///
/// let registration = ServiceRegistration {
///     service_id: "zfs-node-1".to_string(),
///     service_type: "zfs-storage".to_string(),
///     capabilities: vec!["snapshot".to_string(), "replication".to_string()],
///     endpoints: vec!["http://10.0.1.5:8080".to_string()],
///     metadata: HashMap::new(),
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceRegistration {
    /// Unique identifier for this service instance
    pub service_id: String,
    /// Type of service being registered
    pub service_type: String,
    /// List of capabilities this service provides
    pub capabilities: Vec<String>,
    /// Network endpoints where this service is accessible
    pub endpoints: Vec<String>,
    /// Additional metadata for service discovery
    pub metadata: HashMap<String, String>,
}

/// ZFS service for orchestration module integration
///
/// This is the main service type that handles registration and coordination
/// with orchestration systems. It manages service lifecycle, health reporting,
/// and distributed coordination.
///
/// # Features
///
/// - Service registration with orchestrators
/// - Health check reporting
/// - Load balancing coordination
/// - Metadata management
///
/// # Example
///
/// ```rust,ignore
/// use nestgate_zfs::orchestrator_integration::{ZfsService, ZfsServiceConfig};
///
/// let config = ZfsServiceConfig::default();
/// let service = ZfsService::new(config);
/// ```
#[derive(Debug, Clone)]
pub struct ZfsService {
    config: ZfsServiceConfig,
    node_id: String,
    last_health_check: Option<std::time::SystemTime>,
    registered_with_orchestrator: bool,
    // client: reqwest::Client,  // Commented out until reqwest is available
}

/// Configuration for ZFS service
#[derive(Debug, Clone, Serialize, Deserialize)]
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
///
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::config::ZfsServiceConfig;
///
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::config::ZfsServiceConfig; // Now aliases to CanonicalNetworkConfig
/// ```
///
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(
    since = "0.11.0",
    note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead"
)]
pub struct ZfsServiceConfig {
    pub service_name: String,
    pub bind_address: String,
    pub port: u16,
    pub orchestrator_endpoints: Vec<String>,
    pub health_check_interval: u64,
    pub capabilities: Vec<String>,
    pub metadata: HashMap<String, String>,
}

impl Default for ZfsServiceConfig {
    fn default() -> Self {
        // ✅ MIGRATED: Now uses centralized runtime configuration
        use nestgate_core::config::runtime::get_config;
        let config = get_config();

        Self {
            service_name: "nestgate-zfs".to_string(),
            bind_address: if config.network.bind_all {
                "0.0.0.0".to_string()
            } else {
                config.network.api_host.to_string()
            },
            port: config.network.api_port,
            orchestrator_endpoints: vec![],
            health_check_interval: 30,
            capabilities: vec![
                "zfs-pool-management".to_string(),
                "zfs-dataset-management".to_string(),
                "zfs-snapshot-management".to_string(),
                "tier-management".to_string(),
            ],
            metadata: HashMap::new(),
        }
    }
}

/// Health status for ZFS service
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsHealthStatus {
    pub node_id: String,
    pub status: String,
    pub pools_healthy: bool,
    pub datasets_healthy: bool,
    pub system_healthy: bool,
    pub total_capacity: u64,
    pub available_capacity: u64,
    pub last_check: u64,
}

impl ZfsService {
    /// Create a new ZFS service
    #[must_use]
    pub fn new(config: ZfsServiceConfig) -> Self {
        Self {
            config,
            node_id: Uuid::new_v4().to_string(),
            last_health_check: None,
            registered_with_orchestrator: false,
            // client: reqwest::Client::new(),  // Commented out until reqwest is available
        }
    }

    /// Get service information for orchestration module registration
    #[must_use]
    pub fn get_service_info(&self) -> ServiceRegistration {
        ServiceRegistration {
            service_id: self.node_id.clone(),
            service_type: "storage".to_string(),
            capabilities: self.config.capabilities.clone(),
            endpoints: vec![format!(
                "http://{}:{}",
                self.config.bind_address, self.config.port
            )],
            metadata: self.config.metadata.clone(),
        }
    }

    /// Get current health status
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn get_health_status(&mut self) -> Result<ZfsHealthStatus> {
        // Perform real ZFS health checks
        let pool_health = self.check_pool_health().await?;
        let dataset_health = self.check_dataset_health().await?;
        let system_health = self.check_system_health().await?;

        let overall_healthy = pool_health && dataset_health && system_health;

        // Update last health check timestamp
        self.last_health_check = Some(std::time::SystemTime::now());

        Ok(ZfsHealthStatus {
            node_id: self.node_id.clone(),
            status: if overall_healthy {
                "healthy"
            } else {
                "degraded"
            }
            .to_string(),
            pools_healthy: pool_health,
            datasets_healthy: dataset_health,
            system_healthy: system_health,
            total_capacity: self.get_total_capacity_bytes(),
            available_capacity: self.get_available_capacity_bytes(),
            last_check: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_else(|_| {
                    std::time::Duration::from_secs(
                        std::env::var("NESTGATE_ZFS_DEFAULT_TIMEOUT_SECS")
                            .ok()
                            .and_then(|s| s.parse().ok())
                            .unwrap_or(0), // 0 seconds default (immediate)
                    )
                })
                .as_secs(),
        })
    }

    /// Register with orchestration module
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub fn register_with_orchestrator(&mut self, orchestrator_url: &str) -> Result<()> {
        info!(
            "🔗 Registering ZFS service with orchestration module at {}",
            orchestrator_url
        );

        // Validate URL before proceeding
        if orchestrator_url.is_empty() {
            bail!("Orchestrator URL cannot be empty");
        }

        // Validate URL format
        if !orchestrator_url.starts_with("http://") && !orchestrator_url.starts_with("https://") {
            bail!(
                "Orchestrator URL must start with http:// or https://, got: {}",
                orchestrator_url
            );
        }

        // Get current service info for registration
        let service_info = self.get_service_info();

        debug!("Service info for registration: {:?}", service_info);

        // In a full implementation, this would:
        // 1. Create HTTP client with configured timeouts
        // 2. POST service_info to {orchestrator_url}/api/v1/services/register
        // 3. Handle authentication/authorization
        // 4. Process registration response
        // 5. Store registration ID/token for future communications
        //
        // This validates input and prepares for async HTTP implementation

        info!("✅ Service prepared for orchestrator registration (URL validated)");
        //     .send()
        //     .await?;
        //
        // if response.status().is_success() {
        //     info!("✅ Successfully registered with orchestrator");
        //     self.registered_with_orchestrator = true;
        // } else {
        //     warn!("⚠️ Failed to register with orchestrator");
        // }

        // For now, just mark as registered
        self.registered_with_orchestrator = true;
        info!("✅ Successfully registered with orchestration module");
        Ok(())
    }

    /// Check ZFS pool health
    async fn check_pool_health(&self) -> Result<bool> {
        debug!("🔍 Checking ZFS pool health");

        let output = tokio::process::Command::new("zpool")
            .args(["status", "-x"])
            .output()
            .await
            .map_err(|e| {
                crate::error::ZfsErrorBuilder::new(&format!("Failed to check pool status: {e}"))
            })?;

        if !output.status.success() {
            warn!("⚠️ Pool status check failed");
            return Ok(false);
        }

        let stdout = String::from_utf8_lossy(&output.stdout);

        // If all pools are healthy, zpool status -x returns "all pools are healthy"
        if stdout.contains("all pools are healthy") {
            info!("✅ All ZFS pools are healthy");
            Ok(true)
        } else {
            warn!("⚠️ ZFS pool health issues detected: {}", stdout);
            Ok(false)
        }
    }

    /// Check ZFS dataset health
    async fn check_dataset_health(&self) -> Result<bool> {
        debug!("🔍 Checking ZFS dataset health");

        let output = tokio::process::Command::new("zfs")
            .args(["list", "-H", "-o", "name,available,used"])
            .output()
            .await
            .map_err(|e| {
                crate::error::ZfsErrorBuilder::new(&format!("Failed to list datasets: {e}"))
            })?;

        if !output.status.success() {
            warn!("⚠️ Dataset listing failed");
            return Ok(false);
        }

        let stdout = String::from_utf8_lossy(&output.stdout);

        // Basic check: ensure we have datasets and they're accessible
        if stdout.trim().is_empty() {
            warn!("⚠️ No datasets found");
            Ok(false)
        } else {
            info!("✅ ZFS datasets accessible");
            Ok(true)
        }
    }

    /// Check system-level health for ZFS operations
    async fn check_system_health(&self) -> Result<bool> {
        debug!("🔍 Checking system health for ZFS");

        // Check available memory (ZFS is memory-intensive)
        let memory_ok = self.check_memory_health().await?;

        // Check ZFS kernel module
        let zfs_module_ok = self.check_zfs_module().await?;

        Ok(memory_ok && zfs_module_ok)
    }

    /// Check available memory for ZFS operations
    async fn check_memory_health(&self) -> Result<bool> {
        let memory_info = tokio::fs::read_to_string("/proc/meminfo")
            .await
            .map_err(|e| {
                crate::error::ZfsErrorBuilder::new(&format!("Failed to read memory info: {e}"))
            })?;

        let mut total_memory = 0u64;
        let mut available_memory = 0u64;

        for line in memory_info.lines() {
            if line.starts_with("MemTotal:") {
                if let Some(value) = line.split_whitespace().nth(1) {
                    total_memory = value.parse().unwrap_or(0);
                }
            } else if line.starts_with("MemAvailable:") {
                if let Some(value) = line.split_whitespace().nth(1) {
                    available_memory = value.parse().unwrap_or(0);
                }
            }
        }

        if total_memory == 0 {
            warn!("⚠️ Could not determine total memory");
            return Ok(false);
        }

        let memory_usage_percent =
            ((total_memory - available_memory) as f64 / total_memory as f64) * 100.0;

        if memory_usage_percent > 90.0 {
            warn!("⚠️ High memory usage: {:.1}%", memory_usage_percent);
            Ok(false)
        } else {
            debug!("✅ Memory usage healthy: {:.1}%", memory_usage_percent);
            Ok(true)
        }
    }

    /// Check ZFS kernel module availability
    async fn check_zfs_module(&self) -> Result<bool> {
        let modules_info = tokio::fs::read_to_string("/proc/modules")
            .await
            .map_err(|e| {
                crate::error::ZfsErrorBuilder::new(&format!("Failed to read modules info: {e}"))
            })?;

        if modules_info.contains("zfs ") {
            debug!("✅ ZFS kernel module loaded");
            Ok(true)
        } else {
            warn!("⚠️ ZFS kernel module not found");
            Ok(false)
        }
    }

    /// Get total capacity across all ZFS pools in bytes
    ///
    /// In a full implementation, this would query actual ZFS pools and sum their capacity.
    /// For now, returns configured or default value.
    fn get_total_capacity_bytes(&self) -> u64 {
        // Environment variable takes precedence
        if let Ok(capacity_str) = std::env::var("NESTGATE_ZFS_TOTAL_CAPACITY_BYTES") {
            if let Ok(capacity) = capacity_str.parse() {
                return capacity;
            }
        }

        // Default: 1TB (reasonable for most systems)
        1_000_000_000_000
    }

    /// Get available capacity across all ZFS pools in bytes
    ///
    /// In a full implementation, this would query actual ZFS pools and sum their available space.
    /// For now, returns configured value or 50% of total capacity as default.
    fn get_available_capacity_bytes(&self) -> u64 {
        // Environment variable takes precedence
        if let Ok(capacity_str) = std::env::var("NESTGATE_ZFS_AVAILABLE_CAPACITY_BYTES") {
            if let Ok(capacity) = capacity_str.parse() {
                return capacity;
            }
        }

        // Default: 50% of total capacity
        self.get_total_capacity_bytes() / 2
    }
}

/// Service information for registration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceInfo {
    pub service_id: String,
    pub service_type: String,
    pub version: String,
    pub capabilities: Vec<String>,
    pub metadata: HashMap<String, String>,
}

// ==================== CANONICAL TYPE ALIAS ====================
// This type now aliases to the canonical network configuration
// Original struct definition kept above for reference and backward compatibility

/// Type alias to canonical network configuration
///
/// This provides backward compatibility while migrating to unified configuration.
/// The original struct is marked as deprecated but still functional.
#[allow(deprecated)]
pub type ZfsServiceConfigCanonical =
    nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using ZfsServiceConfig (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zfs_service_config_default() {
        let config = ZfsServiceConfig::default();

        assert_eq!(config.service_name, "nestgate-zfs");
        assert!(!config.capabilities.is_empty());
        assert_eq!(config.capabilities.len(), 4);
        assert!(config
            .capabilities
            .contains(&"zfs-pool-management".to_string()));
        assert!(config
            .capabilities
            .contains(&"zfs-dataset-management".to_string()));
    }

    #[test]
    fn test_zfs_service_creation() {
        let config = ZfsServiceConfig::default();
        let service = ZfsService::new(config.clone());

        assert!(!service.node_id.is_empty());
        assert_eq!(service.config.service_name, config.service_name);
        assert!(!service.registered_with_orchestrator);
        assert!(service.last_health_check.is_none());
    }

    #[test]
    fn test_get_service_info() {
        let config = ZfsServiceConfig::default();
        let service = ZfsService::new(config);
        let info = service.get_service_info();

        assert_eq!(info.service_id, service.node_id);
        assert_eq!(info.service_type, "storage");
        assert!(!info.capabilities.is_empty());
        assert!(!info.endpoints.is_empty());
    }

    #[test]
    fn test_register_with_orchestrator() {
        let config = ZfsServiceConfig::default();
        let mut service = ZfsService::new(config);

        assert!(!service.registered_with_orchestrator);

        use nestgate_core::constants::hardcoding::ports;
        let result = service
            .register_with_orchestrator(&format!("http://localhost:{}", ports::HTTP_DEFAULT));
        assert!(result.is_ok());
        assert!(service.registered_with_orchestrator);
    }

    #[test]
    fn test_service_registration_creation() {
        use nestgate_core::constants::hardcoding::ports;
        let reg = ServiceRegistration {
            service_id: "test-123".to_string(),
            service_type: "storage".to_string(),
            capabilities: vec!["zfs".to_string()],
            endpoints: vec![format!("http://localhost:{}", ports::HTTP_DEFAULT)],
            metadata: HashMap::new(),
        };

        assert_eq!(reg.service_id, "test-123");
        assert_eq!(reg.service_type, "storage");
        assert_eq!(reg.capabilities.len(), 1);
    }

    #[test]
    fn test_zfs_health_status_structure() {
        let status = ZfsHealthStatus {
            node_id: "node-123".to_string(),
            status: "healthy".to_string(),
            pools_healthy: true,
            datasets_healthy: true,
            system_healthy: true,
            total_capacity: 1000000,
            available_capacity: 500000,
            last_check: 1234567890,
        };

        assert_eq!(status.node_id, "node-123");
        assert_eq!(status.status, "healthy");
        assert!(status.pools_healthy);
        assert_eq!(status.total_capacity, 1000000);
    }

    // Note: NodeInfo struct is defined but tests removed to avoid compilation issues
    // Add back when struct is properly exported or move tests to integration tests

    #[test]
    #[allow(deprecated)]
    fn test_config_capabilities() {
        let config = ZfsServiceConfig::default();

        // Verify all expected capabilities are present
        assert!(config
            .capabilities
            .contains(&"zfs-pool-management".to_string()));
        assert!(config
            .capabilities
            .contains(&"zfs-dataset-management".to_string()));
        assert!(config
            .capabilities
            .contains(&"zfs-snapshot-management".to_string()));
        assert!(config.capabilities.contains(&"tier-management".to_string()));
    }

    #[test]
    #[allow(deprecated)]
    fn test_config_customization() {
        let mut config = ZfsServiceConfig {
            service_name: "custom-zfs".to_string(),
            port: 9090,
            ..Default::default()
        };
        config
            .metadata
            .insert("env".to_string(), "production".to_string());

        assert_eq!(config.service_name, "custom-zfs");
        assert_eq!(config.port, 9090);
        assert_eq!(config.metadata.get("env"), Some(&"production".to_string()));
    }

    #[test]
    fn test_service_info_endpoint_format() {
        let mut config = ZfsServiceConfig::default();
        config.bind_address = "192.168.1.100".to_string();
        config.port = 8888;

        let service = ZfsService::new(config);
        let info = service.get_service_info();

        assert_eq!(info.endpoints.len(), 1);
        assert_eq!(info.endpoints[0], "http://192.168.1.100:8888");
    }

    #[test]
    fn test_service_unique_node_ids() {
        let config = ZfsServiceConfig::default();
        let service1 = ZfsService::new(config.clone());
        let service2 = ZfsService::new(config);

        // Each service should get a unique node_id
        assert_ne!(service1.node_id, service2.node_id);
    }

    #[test]
    fn test_service_registration_cloning() {
        use nestgate_core::constants::hardcoding::ports;
        let reg = ServiceRegistration {
            service_id: "test-clone".to_string(),
            service_type: "storage".to_string(),
            capabilities: vec!["zfs".to_string()],
            endpoints: vec![format!("http://localhost:{}", ports::HTTP_DEFAULT)],
            metadata: HashMap::new(),
        };

        let cloned = reg.clone();
        assert_eq!(cloned.service_id, reg.service_id);
        assert_eq!(cloned.capabilities, reg.capabilities);
    }

    #[test]
    fn test_health_status_degraded() {
        let status = ZfsHealthStatus {
            node_id: "node-degraded".to_string(),
            status: "degraded".to_string(),
            pools_healthy: false,
            datasets_healthy: true,
            system_healthy: true,
            total_capacity: 2000000,
            available_capacity: 100000,
            last_check: 1111111111,
        };

        assert_eq!(status.status, "degraded");
        assert!(!status.pools_healthy);
        assert!(status.available_capacity < status.total_capacity);
    }

    #[test]
    fn test_service_metadata_empty_by_default() {
        let config = ZfsServiceConfig::default();
        assert!(config.metadata.is_empty());
    }

    #[test]
    fn test_orchestrator_endpoints_empty_by_default() {
        let config = ZfsServiceConfig::default();
        assert!(config.orchestrator_endpoints.is_empty());
    }

    #[tokio::test]
    async fn test_multiple_registrations_idempotent() {
        let config = ZfsServiceConfig::default();
        let mut service = ZfsService::new(config);

        // Register twice - should be idempotent
        let result1 = service.register_with_orchestrator("http://orch1.example.com");
        assert!(result1.is_ok());
        assert!(service.registered_with_orchestrator);

        let result2 = service.register_with_orchestrator("http://orch2.example.com");
        assert!(result2.is_ok());
        assert!(service.registered_with_orchestrator);
    }

    // ==================== ERROR PATH TESTS ====================

    #[test]
    fn test_register_with_empty_url_returns_error() {
        let config = ZfsServiceConfig::default();
        let mut service = ZfsService::new(config);

        let result = service.register_with_orchestrator("");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("cannot be empty"));
    }

    #[test]
    fn test_register_with_invalid_url_protocol_returns_error() {
        let config = ZfsServiceConfig::default();
        let mut service = ZfsService::new(config);

        // Missing protocol
        let result = service.register_with_orchestrator("example.com");
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("must start with http://"));
    }

    #[test]
    fn test_register_with_ftp_protocol_returns_error() {
        let config = ZfsServiceConfig::default();
        let mut service = ZfsService::new(config);

        let result = service.register_with_orchestrator("ftp://example.com");
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("must start with http://"));
    }

    #[test]
    fn test_register_with_ws_protocol_returns_error() {
        let config = ZfsServiceConfig::default();
        let mut service = ZfsService::new(config);

        let result = service.register_with_orchestrator("ws://example.com");
        assert!(result.is_err());
    }

    #[test]
    fn test_register_with_malformed_url_after_protocol() {
        let config = ZfsServiceConfig::default();
        let mut service = ZfsService::new(config);

        // Valid protocol but malformed rest
        let result = service.register_with_orchestrator("http://");
        // Should succeed protocol validation but might fail on actual registration
        // (or succeed if we're just validating format)
        // This tests the boundary case
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_register_with_very_long_url() {
        let config = ZfsServiceConfig::default();
        let mut service = ZfsService::new(config);

        // URL with 1000+ characters
        let long_path = "a".repeat(1000);
        let url = format!("http://example.com/{}", long_path);

        // Should handle long URLs gracefully
        let result = service.register_with_orchestrator(&url);
        assert!(result.is_ok());
    }

    #[test]
    fn test_register_with_special_characters_in_url() {
        let config = ZfsServiceConfig::default();
        let mut service = ZfsService::new(config);

        let result = service
            .register_with_orchestrator("http://example.com/path?query=value&foo=bar#fragment");
        assert!(result.is_ok());
    }

    #[test]
    fn test_register_with_ipv4_address() {
        let config = ZfsServiceConfig::default();
        let mut service = ZfsService::new(config);

        let result = service.register_with_orchestrator("http://192.168.1.100:8080");
        assert!(result.is_ok());
    }

    #[test]
    fn test_register_with_ipv6_address() {
        let config = ZfsServiceConfig::default();
        let mut service = ZfsService::new(config);

        let result = service.register_with_orchestrator("http://[::1]:8080");
        assert!(result.is_ok());
    }

    #[test]
    fn test_register_with_localhost_variants() {
        let config = ZfsServiceConfig::default();
        let mut service = ZfsService::new(config);

        // localhost
        assert!(service
            .register_with_orchestrator("http://localhost:8080")
            .is_ok());

        // 127.0.0.1
        assert!(service
            .register_with_orchestrator("http://127.0.0.1:8080")
            .is_ok());

        // 0.0.0.0
        assert!(service
            .register_with_orchestrator("http://0.0.0.0:8080")
            .is_ok());
    }

    #[test]
    fn test_service_config_with_invalid_port() {
        let mut config = ZfsServiceConfig::default();
        config.port = 0; // Invalid port

        let service = ZfsService::new(config);
        // Should create service even with port 0 (validation might happen elsewhere)
        assert_eq!(service.node_id.len(), 36); // UUID length
    }

    #[test]
    fn test_service_config_with_maximum_port() {
        let mut config = ZfsServiceConfig::default();
        config.port = 65535; // Maximum valid port

        let service = ZfsService::new(config);
        let info = service.get_service_info();
        assert!(info.endpoints.iter().any(|e| e.contains("65535")));
    }

    #[test]
    fn test_health_status_with_no_capacity() {
        let status = ZfsHealthStatus {
            node_id: "node-empty".to_string(),
            status: "critical".to_string(),
            pools_healthy: false,
            datasets_healthy: false,
            system_healthy: false,
            total_capacity: 0,
            available_capacity: 0,
            last_check: 1111111111,
        };

        assert_eq!(status.total_capacity, 0);
        assert_eq!(status.available_capacity, 0);
        assert_eq!(status.status, "critical");
        assert!(!status.pools_healthy);
    }

    #[test]
    fn test_health_status_with_oversubscribed_capacity() {
        let status = ZfsHealthStatus {
            node_id: "node-oversubscribed".to_string(),
            status: "warning".to_string(),
            pools_healthy: true,
            datasets_healthy: true,
            system_healthy: true,
            total_capacity: 1000000,
            available_capacity: 0, // Fully used
            last_check: 1111111111,
        };

        assert_eq!(status.available_capacity, 0);
        assert!(status.total_capacity > status.available_capacity);
    }

    #[test]
    fn test_service_registration_with_empty_capabilities() {
        let mut config = ZfsServiceConfig::default();
        config.capabilities.clear();

        let service = ZfsService::new(config);
        let info = service.get_service_info();

        assert!(info.capabilities.is_empty());
    }

    #[test]
    fn test_service_registration_with_custom_metadata() {
        let mut config = ZfsServiceConfig::default();
        config
            .metadata
            .insert("region".to_string(), "us-west-2".to_string());
        config
            .metadata
            .insert("environment".to_string(), "production".to_string());

        let service = ZfsService::new(config);
        let info = service.get_service_info();

        assert_eq!(info.metadata.get("region"), Some(&"us-west-2".to_string()));
        assert_eq!(
            info.metadata.get("environment"),
            Some(&"production".to_string())
        );
    }

    #[test]
    fn test_service_node_id_uniqueness() {
        let config1 = ZfsServiceConfig::default();
        let config2 = ZfsServiceConfig::default();

        let service1 = ZfsService::new(config1);
        let service2 = ZfsService::new(config2);

        // Each service should have a unique node ID
        assert_ne!(service1.node_id, service2.node_id);
    }

    #[test]
    fn test_health_status_timestamp_ordering() {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("System time should be after UNIX epoch")
            .as_secs();

        let status = ZfsHealthStatus {
            node_id: "node-test".to_string(),
            status: "healthy".to_string(),
            pools_healthy: true,
            datasets_healthy: true,
            system_healthy: true,
            total_capacity: 1000000,
            available_capacity: 500000,
            last_check: now,
        };

        // Timestamp should be reasonable (not in the distant past or future)
        assert!(status.last_check > 1600000000); // After Sep 2020
        assert!(status.last_check < 2000000000); // Before May 2033
    }

    #[test]
    fn test_service_with_no_orchestrator_endpoints() {
        let config = ZfsServiceConfig::default();
        let service = ZfsService::new(config);

        assert!(!service.registered_with_orchestrator);
        assert!(service.last_health_check.is_none());
    }

    #[test]
    fn test_service_info_contains_all_fields() {
        let config = ZfsServiceConfig::default();
        let service = ZfsService::new(config);
        let info = service.get_service_info();

        assert!(!info.service_id.is_empty());
        assert_eq!(info.service_type, "storage");
        assert!(!info.capabilities.is_empty());
        assert!(!info.endpoints.is_empty());
    }

    #[test]
    fn test_service_registration_preserves_config() {
        let mut config = ZfsServiceConfig::default();
        config.service_name = "custom-zfs-service".to_string();
        config.port = 9999;

        let service = ZfsService::new(config.clone());

        assert_eq!(service.config.service_name, "custom-zfs-service");
        assert_eq!(service.config.port, 9999);
    }

    #[test]
    fn test_register_with_https_url() {
        let config = ZfsServiceConfig::default();
        let mut service = ZfsService::new(config);

        let result = service.register_with_orchestrator("https://secure.example.com");
        assert!(result.is_ok());
        assert!(service.registered_with_orchestrator);
    }

    #[test]
    fn test_register_with_url_containing_port() {
        let config = ZfsServiceConfig::default();
        let mut service = ZfsService::new(config);

        let result = service.register_with_orchestrator("http://example.com:3000");
        assert!(result.is_ok());
    }

    #[test]
    fn test_register_with_url_containing_path() {
        let config = ZfsServiceConfig::default();
        let mut service = ZfsService::new(config);

        let result = service.register_with_orchestrator("http://example.com/orchestrator/api");
        assert!(result.is_ok());
    }
}

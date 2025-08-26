///
/// Core network configuration types and utilities.
use serde::{Deserialize, Serialize};

// ==================== UNIFIED NETWORK CONFIGURATION ====================

// Re-export the canonical network configuration
pub use crate::canonical_modernization::network_config::UnifiedNetworkConfig;

// Alias for backward compatibility during transition
pub type NetworkConfig = UnifiedNetworkConfig;

// ==================== LEGACY CONFIGURATION REMOVED ====================
// All legacy network configuration has been successfully migrated to UnifiedNetworkConfig
// No deprecated types remain - migration complete

// ==================== HELPER FUNCTIONS ====================

/// Create default network configuration
pub fn create_default_config() -> UnifiedNetworkConfig {
    UnifiedNetworkConfig::default()
}

/// Validate network configuration
pub fn validate_config(config: &UnifiedNetworkConfig) -> Result<(), String> {
    // Basic validation - can be expanded as needed
    if config.base.ports.api_port == 0 {
        return Err("API port cannot be 0".to_string());
    }
    
    if config.base.timeouts.connection_timeout_ms == 0 {
        return Err("Connection timeout cannot be 0".to_string());
    }
    
    Ok(())
}

/// Network service status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ServiceStatus {
    /// Service is healthy and operational
    Healthy,
    /// Service is running normally
    Running,
    /// Service is unhealthy but still operational
    Degraded,
    /// Service is not operational
    Unhealthy,
    /// Service has encountered an error
    Error,
    /// Service is stopped
    Stopped,
    /// Service status is unknown
    Unknown,
}

impl Default for ServiceStatus {
    fn default() -> Self {
        ServiceStatus::Unknown
    }
}

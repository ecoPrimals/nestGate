///
/// Core network configuration types and utilities.
use serde::{Deserialize, Serialize};
// ==================== SECTION ====================

// Re-export the canonical network configuration
pub use crate::canonical_modernization::network_config::UnifiedNetworkConfig;

// Alias for backward compatibility during transition
pub type NetworkConfig = UnifiedNetworkConfig;

// ==================== SECTION ====================
// All legacy network configuration has been successfully migrated to UnifiedNetworkConfig
// No deprecated types remain - migration complete

// ==================== SECTION ====================

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
/// Status values for Service
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
    /// Returns the default instance
    fn default() -> Self {
        ServiceStatus::Unknown
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_default_config() {
        let config = create_default_config();
        
        assert!(config.base.ports.api_port > 0);
        assert!(config.base.timeouts.connection_timeout_ms > 0);
    }

    #[test]
    fn test_validate_config_success() {
        let config = create_default_config();
        let result = validate_config(&config);
        
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_config_zero_api_port() {
        let mut config = create_default_config();
        config.base.ports.api_port = 0;
        
        let result = validate_config(&config);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "API port cannot be 0");
    }

    #[test]
    fn test_validate_config_zero_timeout() {
        let mut config = create_default_config();
        config.base.timeouts.connection_timeout_ms = 0;
        
        let result = validate_config(&config);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Connection timeout cannot be 0");
    }

    #[test]
    fn test_service_status_variants() {
        let statuses = vec![
            ServiceStatus::Healthy,
            ServiceStatus::Running,
            ServiceStatus::Degraded,
            ServiceStatus::Unhealthy,
            ServiceStatus::Error,
            ServiceStatus::Stopped,
            ServiceStatus::Unknown,
        ];
        
        assert_eq!(statuses.len(), 7);
    }

    #[test]
    fn test_service_status_equality() {
        assert_eq!(ServiceStatus::Healthy, ServiceStatus::Healthy);
        assert_eq!(ServiceStatus::Running, ServiceStatus::Running);
        assert_ne!(ServiceStatus::Healthy, ServiceStatus::Unhealthy);
    }

    #[test]
    fn test_service_status_default() {
        let status = ServiceStatus::default();
        assert_eq!(status, ServiceStatus::Unknown);
    }

    #[test]
    fn test_service_status_cloning() {
        let status = ServiceStatus::Healthy;
        let cloned = status.clone();
        assert_eq!(status, cloned);
    }

    #[test]
    fn test_service_status_serialization() {
        let status = ServiceStatus::Running;
        let json = serde_json::to_string(&status);
        assert!(json.is_ok());
        
        let json_str = json
            .expect("Test: status serialization should succeed");
        let deserialized: ServiceStatus = serde_json::from_str(&json_str)
            .expect("Test: status deserialization should succeed");
        assert_eq!(deserialized, ServiceStatus::Running);
    }

    #[test]
    fn test_service_status_all_variants_serialize() {
        let statuses = vec![
            ServiceStatus::Healthy,
            ServiceStatus::Running,
            ServiceStatus::Degraded,
            ServiceStatus::Unhealthy,
            ServiceStatus::Error,
            ServiceStatus::Stopped,
            ServiceStatus::Unknown,
        ];
        
        for status in statuses {
            let json = serde_json::to_string(&status);
            assert!(json.is_ok(), "Failed to serialize {:?}", status);
        }
    }

    #[test]
    fn test_network_config_alias() {
        // NetworkConfig should be an alias for UnifiedNetworkConfig
        let _config: NetworkConfig = create_default_config();
    }

    #[test]
    fn test_validate_config_valid_ports() {
        use nestgate_core::constants::hardcoding::ports;
        let mut config = create_default_config();
        config.base.ports.api_port = ports::HTTP_DEFAULT;
        
        let result = validate_config(&config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_config_valid_timeouts() {
        let mut config = create_default_config();
        config.base.timeouts.connection_timeout_ms = 5000;
        
        let result = validate_config(&config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_service_status_debug_format() {
        let status = ServiceStatus::Healthy;
        let debug_str = format!("{:?}", status);
        assert!(debug_str.contains("Healthy"));
    }
}

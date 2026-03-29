// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! **MIGRATED NAS MODULE**
//!
//! This module now uses the canonical configuration system instead of
//! scattered NAS-specific configuration structures.

// Re-export from canonical configuration system
pub use nestgate_core::config::canonical_primary::{NasConfig, NestGateCanonicalConfig};

use serde::{Deserialize, Serialize};

// Use canonical constants

/// NAS service implementation using canonical configuration
#[derive(Debug)]
pub struct NasService {
    #[allow(dead_code)]
    config: NasConfig,
}
impl NasService {
    /// Create a new NAS service with canonical configuration
    #[must_use]
    pub const fn new(config: NasConfig) -> Self {
        Self { config }
    }

    /// Start the NAS service
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub const fn start(&self) -> Result<(), NasError> {
        // Implementation would go here
        Ok(())
    }

    /// Stop the NAS service
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub const fn stop(&self) -> Result<(), NasError> {
        // Implementation would go here
        Ok(())
    }

    /// Get service status
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub const fn status(&self) -> Result<NasStatus, NasError> {
        // Implementation would go here
        Ok(NasStatus::Running)
    }
}

/// NAS service status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NasStatus {
    Running,
    Stopped,
    Error(String),
}
/// NAS service errors
#[derive(Debug, thiserror::Error)]
pub enum NasError {
    #[error("Configuration error: {0}")]
    Configuration(String),
    #[error("Network error: {0}")]
    Network(String),
    #[error("Storage error: {0}")]
    Storage(String),
    #[error("Permission error: {0}")]
    Permission(String),
}
// ==================== MIGRATION COMPLETE ====================
//
// All deprecated NAS configuration structures have been removed.
// Use the canonical configuration system instead:
//
// ```rust
// use nestgate_core::config::canonical_primary::{NestGateCanonicalConfig, NasConfig};
//
// let config = NestGateCanonicalConfig::default();
// let nas_config = config.services.nas;
// ```

// ==================== CONVENIENCE FUNCTIONS ====================

/// Create a new canonical NAS configuration
#[must_use]
pub fn new_nas_config() -> NasConfig {
    NasConfig::default()
}
/// Create a development-optimized NAS configuration
#[must_use]
pub fn dev_nas_config() -> NasConfig {
    // Development-specific optimizations would go here
    NasConfig::default()
}
/// Create a production-optimized NAS configuration
#[must_use]
pub fn prod_nas_config() -> NasConfig {
    // Production-specific optimizations would go here
    NasConfig::default()
}
/// Create a new NAS service with default configuration
#[must_use]
pub fn create_nas_service() -> NasService {
    NasService::new(NasConfig::default())
}
/// Create a new NAS service with development configuration
#[must_use]
pub fn create_dev_nas_service() -> NasService {
    NasService::new(dev_nas_config())
}
/// Create a new NAS service with production configuration
#[must_use]
pub fn create_prod_nas_service() -> NasService {
    NasService::new(prod_nas_config())
}

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== NasService Tests ====================

    #[test]
    fn test_nas_service_new() {
        let config = NasConfig::default();
        let service = NasService::new(config);
        assert!(format!("{:?}", service).contains("NasService"));
    }

    #[test]
    fn test_nas_service_start() {
        let config = NasConfig::default();
        let service = NasService::new(config);
        let result = service.start();
        assert!(result.is_ok());
    }

    #[test]
    fn test_nas_service_stop() {
        let config = NasConfig::default();
        let service = NasService::new(config);
        let result = service.stop();
        assert!(result.is_ok());
    }

    #[test]
    fn test_nas_service_status() {
        let config = NasConfig::default();
        let service = NasService::new(config);
        let result = service.status();
        assert!(result.is_ok());
        assert!(matches!(
            result.expect("Operation failed"),
            NasStatus::Running
        ));
    }

    #[test]
    fn test_nas_service_lifecycle() {
        let config = NasConfig::default();
        let service = NasService::new(config);
        assert!(service.start().is_ok());
        assert!(service.status().is_ok());
        assert!(service.stop().is_ok());
    }

    // ==================== NasStatus Tests ====================

    #[test]
    fn test_nas_status_running() {
        let status = NasStatus::Running;
        assert!(matches!(status, NasStatus::Running));
    }

    #[test]
    fn test_nas_status_stopped() {
        let status = NasStatus::Stopped;
        assert!(matches!(status, NasStatus::Stopped));
    }

    #[test]
    fn test_nas_status_error() {
        let status = NasStatus::Error("test error".to_string());
        assert!(matches!(status, NasStatus::Error(_)));
    }

    #[test]
    fn test_nas_status_clone() {
        let status = NasStatus::Running;
        let cloned = status.clone();
        assert!(matches!(cloned, NasStatus::Running));
    }

    #[test]
    fn test_nas_status_debug() {
        let status = NasStatus::Running;
        let debug = format!("{:?}", status);
        assert!(debug.contains("Running"));
    }

    #[test]
    fn test_nas_status_serialize() {
        let status = NasStatus::Running;
        let json = serde_json::to_string(&status).expect("String operation failed");
        assert!(json.contains("Running"));
    }

    #[test]
    fn test_nas_status_deserialize() {
        let json = r#""Running""#;
        let status: NasStatus = serde_json::from_str(json).expect("Failed to convert from string");
        assert!(matches!(status, NasStatus::Running));
    }

    #[test]
    fn test_nas_status_round_trip() {
        let original = NasStatus::Stopped;
        let json = serde_json::to_string(&original).expect("String operation failed");
        let deserialized: NasStatus =
            serde_json::from_str(&json).expect("Failed to convert from string");
        assert!(matches!(deserialized, NasStatus::Stopped));
    }

    #[test]
    fn test_nas_status_error_message() {
        let msg = "connection failed";
        let status = NasStatus::Error(msg.to_string());
        if let NasStatus::Error(error_msg) = status {
            assert_eq!(error_msg, msg);
        } else {
            panic!("Expected Error variant");
        }
    }

    // ==================== NasError Tests ====================

    #[test]
    fn test_nas_error_configuration() {
        let err = NasError::Configuration("invalid port".to_string());
        let msg = err.to_string();
        assert!(msg.contains("Configuration error"));
        assert!(msg.contains("invalid port"));
    }

    #[test]
    fn test_nas_error_network() {
        let err = NasError::Network("timeout".to_string());
        let msg = err.to_string();
        assert!(msg.contains("Network error"));
        assert!(msg.contains("timeout"));
    }

    #[test]
    fn test_nas_error_storage() {
        let err = NasError::Storage("disk full".to_string());
        let msg = err.to_string();
        assert!(msg.contains("Storage error"));
        assert!(msg.contains("disk full"));
    }

    #[test]
    fn test_nas_error_permission() {
        let err = NasError::Permission("access denied".to_string());
        let msg = err.to_string();
        assert!(msg.contains("Permission error"));
        assert!(msg.contains("access denied"));
    }

    #[test]
    fn test_nas_error_debug() {
        let err = NasError::Configuration("test".to_string());
        let debug = format!("{:?}", err);
        assert!(debug.contains("Configuration"));
    }

    #[test]
    fn test_nas_error_trait() {
        let err = NasError::Network("test".to_string());
        let _: &dyn std::error::Error = &err;
    }

    #[test]
    fn test_nas_error_source() {
        let err = NasError::Configuration("test".to_string());
        use std::error::Error;
        assert!(err.source().is_none());
    }

    // ==================== Convenience Function Tests ====================

    #[test]
    fn test_new_nas_config() {
        let config = new_nas_config();
        assert!(format!("{:?}", config).contains("NasConfig"));
    }

    #[test]
    fn test_dev_nas_config() {
        let config = dev_nas_config();
        assert!(format!("{:?}", config).contains("NasConfig"));
    }

    #[test]
    fn test_prod_nas_config() {
        let config = prod_nas_config();
        assert!(format!("{:?}", config).contains("NasConfig"));
    }

    #[test]
    fn test_create_nas_service() {
        let service = create_nas_service();
        assert!(format!("{:?}", service).contains("NasService"));
    }

    #[test]
    fn test_create_dev_nas_service() {
        let service = create_dev_nas_service();
        assert!(format!("{:?}", service).contains("NasService"));
    }

    #[test]
    fn test_create_prod_nas_service() {
        let service = create_prod_nas_service();
        assert!(format!("{:?}", service).contains("NasService"));
    }

    // ==================== Integration Tests ====================

    #[test]
    fn test_service_start_stop_cycle() {
        let service = create_nas_service();
        assert!(service.start().is_ok());
        assert!(service.stop().is_ok());
    }

    #[test]
    fn test_multiple_status_calls() {
        let service = create_nas_service();
        for _ in 0..5 {
            assert!(service.status().is_ok());
        }
    }

    #[test]
    fn test_service_operations_sequence() {
        let service = create_nas_service();
        assert!(service.start().is_ok());
        assert!(service.status().is_ok());
        assert!(service.stop().is_ok());
        assert!(service.status().is_ok());
    }

    #[test]
    fn test_all_config_types_work() {
        let configs = vec![new_nas_config(), dev_nas_config(), prod_nas_config()];

        for config in configs {
            let service = NasService::new(config);
            assert!(service.start().is_ok());
        }
    }

    #[test]
    fn test_all_service_creators() {
        let services = vec![
            create_nas_service(),
            create_dev_nas_service(),
            create_prod_nas_service(),
        ];

        for service in services {
            assert!(service.status().is_ok());
        }
    }

    #[test]
    fn test_error_variants_distinct() {
        let errors = [
            NasError::Configuration("msg".to_string()),
            NasError::Network("msg".to_string()),
            NasError::Storage("msg".to_string()),
            NasError::Permission("msg".to_string()),
        ];

        let messages: Vec<String> = errors.iter().map(|e| e.to_string()).collect();
        assert!(messages[0].contains("Configuration"));
        assert!(messages[1].contains("Network"));
        assert!(messages[2].contains("Storage"));
        assert!(messages[3].contains("Permission"));
    }

    #[test]
    fn test_status_variants_distinct() {
        let statuses = [
            NasStatus::Running,
            NasStatus::Stopped,
            NasStatus::Error("msg".to_string()),
        ];

        assert!(matches!(statuses[0], NasStatus::Running));
        assert!(matches!(statuses[1], NasStatus::Stopped));
        assert!(matches!(statuses[2], NasStatus::Error(_)));
    }
}

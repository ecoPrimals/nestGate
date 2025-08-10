//! Configuration Validation
//!
//! This module handles validation logic for configuration.
//! Single responsibility: Validate configuration correctness.

use super::types::*;
use crate::error::{NestGateError, Result};

/// Configuration validator
pub struct ConfigValidator;

impl ConfigValidator {
    /// Validate a complete configuration
    pub fn validate(config: &CanonicalConfig) -> Result<()> {
        Self::validate_system(&config.system)?;
        Self::validate_network(&config.network)?;
        Self::validate_storage(&config.storage)?;
        Self::validate_security(&config.security)?;
        Self::validate_performance(&config.performance)?;
        Self::validate_monitoring(&config.monitoring)?;
        Ok(())
    }

    fn validate_system(system: &SystemConfig) -> Result<()> {
        if system.instance_name.is_empty() {
            return Err(NestGateError::Validation {
                field: "system.instance_name".to_string(),
                message: "Instance name cannot be empty".to_string(),
                current_value: Some(system.instance_name.clone()),
                expected: Some("non-empty string".to_string()),
                user_error: true,
            });
        }
        Ok(())
    }

    fn validate_network(network: &NetworkConfig) -> Result<()> {
        Self::validate_api_server(&network.api)?;
        Ok(())
    }

    fn validate_api_server(api: &ApiServerConfig) -> Result<()> {
        if api.port == 0 {
            return Err(NestGateError::Validation {
                field: "network.api.port".to_string(),
                message: "Port cannot be 0".to_string(),
                current_value: Some(api.port.to_string()),
                expected: Some("port number 1-65535".to_string()),
                user_error: true,
            });
        }
        Ok(())
    }

    fn validate_storage(_storage: &StorageConfig) -> Result<()> {
        // Storage validation logic
        Ok(())
    }

    fn validate_security(_security: &SecurityConfig) -> Result<()> {
        // Security validation logic
        Ok(())
    }

    fn validate_performance(_performance: &PerformanceConfig) -> Result<()> {
        // Performance validation logic
        Ok(())
    }

    fn validate_monitoring(_monitoring: &MonitoringConfig) -> Result<()> {
        // Monitoring validation logic
        Ok(())
    }
}

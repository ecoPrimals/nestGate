// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

use super::platform::SystemRequirements;
/// Pre-install checks, post-install validation, health monitoring, and system requirements validation
use serde::{Deserialize, Serialize};
// Migration utilities no longer needed - using canonical configurations

/// Aggregates validation toggles for requirements, pre/post checks, and health probes.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ValidationSettings {
    /// System requirements
    pub system_requirements: SystemRequirements,
    /// Pre-installation checks
    pub pre_install_checks: PreInstallCheckSettings,
    /// Post-installation validation
    pub post_install_validation: PostInstallValidationSettings,
    /// Health monitoring
    pub health_checks: HealthCheckSettings,
}
/// Which pre-install checks run before copying binaries or configuring services.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreInstallCheckSettings {
    /// Check system requirements
    pub check_system_requirements: bool,
    /// Check available disk space
    pub check_disk_space: bool,
    /// Check memory availability
    pub check_memory: bool,
    /// Check CPU requirements
    pub check_cpu: bool,
    /// Check required dependencies
    pub check_dependencies: bool,
    /// Check permissions
    pub check_permissions: bool,
    /// Check existing installation
    pub check_existing_install: bool,
    /// Check system compatibility
    pub check_compatibility: bool,
    /// Check network connectivity
    pub check_network: bool,
    /// Validate checksums during installation
    pub validate_checksums: bool,
}
/// Post-install smoke tests and configuration validation switches.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostInstallValidationSettings {
    /// Validate service installation
    pub validate_service: bool,
    /// Validate file permissions
    pub validate_permissions: bool,
    /// Validate configuration
    pub validate_config: bool,
    /// Run smoke tests
    pub run_smoke_tests: bool,
}
/// Periodic health check behavior after installation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckSettings {
    /// Enable health checks
    pub enabled: bool,
    /// Health check interval
    pub check_interval: std::time::Duration,
    /// Health check timeout
    pub timeout: std::time::Duration,
    /// Retry attempts
    pub retry_attempts: u32,
}

impl Default for PreInstallCheckSettings {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            check_system_requirements: true,
            check_disk_space: true,
            check_memory: true,
            check_cpu: true,
            check_dependencies: true,
            check_permissions: true,
            check_existing_install: true,
            check_compatibility: true,
            check_network: true,
            validate_checksums: true,
        }
    }
}

impl Default for PostInstallValidationSettings {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            validate_service: true,
            validate_permissions: true,
            validate_config: true,
            run_smoke_tests: true,
        }
    }
}

impl Default for HealthCheckSettings {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            enabled: true,
            check_interval: std::time::Duration::from_secs(30),
            timeout: std::time::Duration::from_secs(10),
            retry_attempts: 3,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validation_settings_default_roundtrip_json() {
        let v = ValidationSettings::default();
        let json = serde_json::to_string(&v).unwrap();
        let back: ValidationSettings = serde_json::from_str(&json).unwrap();
        assert!(back.pre_install_checks.check_network);
        assert_eq!(back.health_checks.retry_attempts, 3);
    }

    #[test]
    fn nested_defaults_smoke() {
        let p = PreInstallCheckSettings::default();
        assert!(p.validate_checksums);
        let post = PostInstallValidationSettings::default();
        assert!(post.run_smoke_tests);
        let h = HealthCheckSettings::default();
        assert!(h.enabled);
    }
}

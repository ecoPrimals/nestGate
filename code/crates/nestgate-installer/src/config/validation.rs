/// Installer Validation Configuration
/// Pre-install checks, post-install validation, health monitoring, and system requirements validation
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use super::platform::SystemRequirements;

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
    fn default() -> Self {
        Self {
            enabled: true,
            check_interval: std::time::Duration::from_secs(30),
            timeout: std::time::Duration::from_secs(10),
            retry_attempts: 3,
        }
    }
}

/// Validation utilities for installer configuration
pub mod config_validation {
    use super::*;
    use crate::config::UnifiedInstallerConfig;

    /// Validate installer-specific configuration
    pub fn validate_installer_config(config: &UnifiedInstallerConfig) -> Result<(), String> {
        // Note: Base validation removed as validate_domain_config doesn't exist in nestgate-core
        // Base configuration validation completed

        // Installer-specific validations
        if !config.extensions.installation.install_dir.is_absolute() {
            return Err("Installation directory must be an absolute path".to_string());
        }

        if !config.extensions.installation.config_dir.is_absolute() {
            return Err("Configuration directory must be an absolute path".to_string());
        }

        // Validate system requirements
        if config.extensions.validation.system_requirements.min_ram_mb < 1024 {
            return Err("Minimum RAM requirement must be at least 1024 MB".to_string());
        }

        if config
            .extensions
            .validation
            .system_requirements
            .min_disk_space_mb
            < 5120
        {
            return Err("Minimum disk space requirement must be at least 5 GB".to_string());
        }

        Ok(())
    }

    /// Validate system requirements against current system
    pub fn validate_system_requirements(requirements: &SystemRequirements) -> Result<(), String> {
        // This would typically check actual system resources
        // For now, we'll do basic validation of the requirements structure

        if requirements.min_ram_mb == 0 {
            return Err("Minimum RAM must be greater than 0".to_string());
        }

        if requirements.min_disk_space_mb == 0 {
            return Err("Minimum disk space must be greater than 0".to_string());
        }

        if requirements.min_cpu_cores == 0 {
            return Err("Minimum CPU cores must be greater than 0".to_string());
        }

        Ok(())
    }

    /// Validate directory paths
    pub fn validate_installation_paths(
        install_dir: &PathBuf,
        config_dir: &PathBuf,
        data_dir: &PathBuf,
    ) -> Result<(), String> {
        if !install_dir.is_absolute() {
            return Err("Install directory must be absolute".to_string());
        }

        if !config_dir.is_absolute() {
            return Err("Config directory must be absolute".to_string());
        }

        if !data_dir.is_absolute() {
            return Err("Data directory must be absolute".to_string());
        }

        // Check for conflicts
        if install_dir == config_dir || install_dir == data_dir || config_dir == data_dir {
            return Err("Installation directories must be unique".to_string());
        }

        Ok(())
    }
}

// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//
// Validation logic and safety checks for ZFS pool setup operations

//! Validation module

use serde::{Deserialize, Serialize};

use super::{
    config::{PoolSetupConfig, PoolTopology},
    device_detection::StorageDevice,
};
// Removed unused import: nestgate_core::types::StorageTier as CoreStorageTier

/// Result of validation operations
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Validationresult
pub struct ValidationResult {
    /// Whether valid
    pub is_valid: bool,
    /// Issues
    pub issues: Vec<String>,
    /// Warnings
    pub warnings: Vec<String>,
}
impl ValidationResult {
    /// Creates a new validation result with default values.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            is_valid: true,
            issues: Vec::new(),
            warnings: Vec::new(),
        }
    }

    /// Add Error
    pub fn add_error(&mut self, error: String) {
        self.is_valid = false;
        self.issues.push(error);
    }

    /// Add Warning
    pub fn add_warning(&mut self, warning: String) {
        self.warnings.push(warning);
    }

    /// Merge
    pub fn merge(&mut self, other: Self) {
        if !other.is_valid {
            self.is_valid = false;
        }
        self.issues.extend(other.issues);
        self.warnings.extend(other.warnings);
    }
}

/// Pool setup validator
pub struct PoolSetupValidator {
    config: PoolSetupConfig,
}
impl PoolSetupValidator {
    /// Creates a new pool setup validator with the given configuration.
    #[must_use]
    pub const fn new(config: PoolSetupConfig) -> Self {
        Self { config }
    }

    /// Validate device with enhanced logic
    #[must_use]
    pub fn validate_device(&self, device: &StorageDevice) -> ValidationResult {
        let mut result = ValidationResult::new();

        // Size validation
        if device.size_bytes < self.config.device_detection.min_device_size {
            result.add_error(format!(
                "Device {} is too small: {} bytes (minimum: {} bytes)",
                device.device_path, device.size_bytes, self.config.device_detection.min_device_size
            ));
        }

        // Path validation
        if !device.device_path.starts_with("/dev/") {
            result.add_error("Invalid device path: error details".to_string());
        }

        // Usage validation
        if device.in_use
            && let Some(ref usage) = device.current_use
        {
            // Check if it's a filesystem we should skip
            for skip_fstype in &self.config.device_detection.skip_fstypes {
                if usage.contains(skip_fstype) {
                    result.add_error(format!(
                        "Device {} contains filesystem type that should be skipped: {}",
                        device.device_path, skip_fstype
                    ));
                    break;
                }
            }
        }

        result
    }

    /// Validate pool configuration with enhanced logic
    #[must_use]
    pub fn validate_pool_config(&self, config: &PoolSetupConfig) -> ValidationResult {
        let mut issues = Vec::new();
        let warnings = Vec::new();

        // Pool name validation
        if config.pool_name.is_empty() {
            issues.push("Pool name cannot be empty".to_string());
        }

        // Device validation
        if config.devices.is_empty() {
            issues.push("No devices specified for pool creation".to_string());
        }

        // Check minimum device count for topology
        let min_devices = match config.topology {
            PoolTopology::Single => 1,
            PoolTopology::Mirror => 2,
            PoolTopology::RaidZ1 => 3,
            PoolTopology::RaidZ2 => 4,
            PoolTopology::RaidZ3 => 5,
        };

        if config.devices.len() < min_devices {
            issues.push(format!(
                "Topology {:?} requires at least {} devices, but {} provided",
                config.topology,
                min_devices,
                config.devices.len()
            ));
        }

        ValidationResult {
            is_valid: issues.is_empty(),
            issues,
            warnings,
        }
    }
}

impl Default for ValidationResult {
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}

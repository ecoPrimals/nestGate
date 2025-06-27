//! Pool Setup Validation
//!
//! Validation logic and safety checks for ZFS pool setup operations

use std::collections::HashMap;
use tracing::{debug, warn};
use serde::{Serialize, Deserialize};

use nestgate_core::{Result as CoreResult, StorageTier};
use super::{
    config::{PoolSetupConfiguration, DeviceDetectionConfig},
    device_detection::{StorageDevice, DeviceType, SpeedClass},
};

/// Result of validation operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub issues: Vec<String>,
    pub warnings: Vec<String>,
}

impl ValidationResult {
    pub fn new() -> Self {
        Self {
            is_valid: true,
            issues: Vec::new(),
            warnings: Vec::new(),
        }
    }
    
    pub fn add_error(&mut self, error: String) {
        self.is_valid = false;
        self.issues.push(error);
    }
    
    pub fn add_warning(&mut self, warning: String) {
        self.warnings.push(warning);
    }
    
    pub fn merge(&mut self, other: ValidationResult) {
        if !other.is_valid {
            self.is_valid = false;
        }
        self.issues.extend(other.issues);
        self.warnings.extend(other.warnings);
    }
}

/// Pool setup configuration for creation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolSetupConfig {
    /// Pool name
    pub pool_name: String,
    /// Devices to use for the pool
    pub devices: Vec<String>,
    /// Pool topology (mirror, raidz, etc.)
    pub topology: PoolTopology,
    /// ZFS properties to set
    pub properties: HashMap<String, String>,
    /// Whether to create tier structure
    pub create_tiers: bool,
    /// Tier mappings to device types
    pub tier_mappings: HashMap<StorageTier, Vec<DeviceType>>,
}

/// Pool topology options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PoolTopology {
    Single,
    Mirror,
    RaidZ1,
    RaidZ2,
    RaidZ3,
}

/// Pool setup validator
pub struct PoolSetupValidator {
    config: PoolSetupConfiguration,
}

impl PoolSetupValidator {
    pub fn new(config: PoolSetupConfiguration) -> Self {
        Self { config }
    }

    /// Validate device with enhanced logic
    pub fn validate_device(&self, device: &StorageDevice) -> ValidationResult {
        let mut result = ValidationResult::new();
        
        // Size validation
        if device.size_bytes < self.config.device_detection.min_device_size {
            result.add_error(format!(
                "Device {} is too small: {} bytes (minimum: {} bytes)",
                device.device_path,
                device.size_bytes,
                self.config.device_detection.min_device_size
            ));
        }
        
        // Path validation
        if !device.device_path.starts_with("/dev/") {
            result.add_error(format!("Invalid device path: {}", device.device_path));
        }
        
        // Usage validation
        if device.in_use {
            if let Some(ref usage) = device.current_use {
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
        }
        
        result
    }
    
    /// Validate pool configuration with enhanced logic
    pub fn validate_pool_config(&self, config: &PoolSetupConfig) -> ValidationResult {
        let mut issues = Vec::new();
        let mut warnings = Vec::new();
        
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
                config.topology, min_devices, config.devices.len()
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
    fn default() -> Self {
        Self::new()
    }
}

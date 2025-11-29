//
// Comprehensive ZFS pool setup with device detection, validation, and creation

//! Pool Setup module

use tracing::info;
use tracing::warn;
pub mod config;
pub mod creation;
pub mod device_detection;
pub mod validation;

// Re-export main types for convenience
pub use config::{
    DeviceDetectionConfig, DeviceType as ConfigDeviceType, PoolSetupConfig, PoolTopology,
    RedundancyLevel, StorageTier as ConfigStorageTier,
};
pub use creation::PoolCreator;
pub use device_detection::{
    DeviceScanner, DeviceType as DetectionDeviceType, SpeedClass, StorageDevice,
};
pub use validation::{PoolSetupValidator, ValidationResult};

// Tests
#[cfg(test)]
mod config_tests;
// #[cfg(test)]
// mod creation_tests;  // Disabled: Tests non-existent types (PoolName, VdevType)
#[cfg(test)]
mod device_detection_tests;
#[cfg(test)]
mod tests;

#[cfg(test)]
mod pool_setup_tests;
#[cfg(test)]
mod types_tests;
#[cfg(test)]
mod validation_tests;

use std::collections::HashMap;
// Removed unused tracing import
use nestgate_core::{NestGateError, Result as CoreResult};
use serde::{Deserialize, Serialize};

/// Convert detection `DeviceType` to config `DeviceType`
fn convert_device_type(detection_type: &DetectionDeviceType) -> ConfigDeviceType {
    match detection_type {
        DetectionDeviceType::NvmeSsd => ConfigDeviceType::NvmeSsd,
        DetectionDeviceType::SataSsd => ConfigDeviceType::SataSsd,
        DetectionDeviceType::Hdd => ConfigDeviceType::SpinningDisk,
        DetectionDeviceType::OptaneMemory => ConfigDeviceType::OptaneMemory,
        DetectionDeviceType::Unknown => ConfigDeviceType::SpinningDisk, // Default fallback
    }
}

/// Pool setup specific errors
#[derive(Debug, thiserror::Error)]
/// Errors that can occur during PoolSetup operations
pub enum PoolSetupError {
    #[error("Device validation failed: {0}")]
    DeviceValidation(String),
    #[error("Pool creation failed: {0}")]
    PoolCreation(String),

    #[error("Configuration error: {0}")]
    Configuration(String),

    #[error("Device scanning failed: {0}")]
    DeviceScanning(String),

    #[error("Insufficient devices: {0}")]
    InsufficientDevices(String),

    #[error("ZFS command failed: {0}")]
    ZfsCommand(String),

    #[error("Core error: {0}")]
    Core(#[from] NestGateError),
}

/// Result of pool setup operations
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Poolsetupresult
pub struct PoolSetupResult {
    /// Pool name that was created
    pub pool_name: String,
    /// Whether the operation succeeded
    pub success: bool,
    /// Operation result message
    pub message: String,
    /// Devices used in the pool
    pub devices_used: Vec<String>,
    /// Pool topology that was created
    pub topology: config::PoolTopology,
}

/// Main ZFS pool setup orchestrator
pub struct ZfsPoolSetup {
    /// Available storage devices
    devices: Vec<StorageDevice>,
    /// Existing ZFS pools
    existing_pools: Vec<String>,
    /// Configuration
    #[allow(dead_code)]
    config: PoolSetupConfig,
    /// Device scanner
    scanner: DeviceScanner,
    /// Validator
    validator: PoolSetupValidator,
    /// Pool creator
    creator: PoolCreator,
}
impl ZfsPoolSetup {
    /// Create new pool setup with custom configuration
    pub async fn new_with_config(config: PoolSetupConfig) -> CoreResult<Self> {
        let scanner = DeviceScanner::new(config.device_detection.clone());
        let validator = PoolSetupValidator::new(config.clone());
        let creator = PoolCreator::new();

        let mut setup = Self {
            devices: Vec::new(),
            existing_pools: Vec::new(),
            config,
            scanner,
            validator,
            creator,
        };

        // Initialize by scanning devices and pools
        setup.scan_devices().await?;
        setup.scan_existing_pools().await?;

        Ok(setup)
    }

    /// Create new pool setup with default configuration
    pub async fn new() -> CoreResult<Self> {
        Self::new_with_config(PoolSetupConfig::default()).await
    }

    /// Scan for available storage devices
    async fn scan_devices(&mut self) -> CoreResult<()> {
        self.devices = self.scanner.scan_devices().await?;
        Ok(())
    }

    /// Scan for existing ZFS pools
    async fn scan_existing_pools(&mut self) -> CoreResult<()> {
        use tokio::process::Command;

        let output = Command::new("zpool")
            .args(["list", "-H", "-o", "name"])
            .output()
            .await;

        match output {
            Ok(output) if output.status.success() => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                self.existing_pools = stdout
                    .lines()
                    .map(|line| line.trim().to_string())
                    .filter(|line| !line.is_empty())
                    .collect();

                info!("Found {} existing ZFS pools", self.existing_pools.len());
            }
            Ok(output) => {
                let stderr = String::from_utf8_lossy(&output.stderr);
                warn!("Failed to list ZFS pools: {}", stderr);
                // Not a fatal error - pools might not exist yet
            }
            Err(e) => {
                warn!("Could not execute zpool command: {}", e);
                // Not a fatal error - ZFS might not be installed
            }
        }
        Ok(())
    }

    /// Get available (unused) devices
    #[must_use]
    pub fn get_available_devices(&self) -> Vec<&StorageDevice> {
        DeviceScanner::filter_available(&self.devices)
    }

    /// Get devices by type
    #[must_use]
    pub fn get_devices_by_type(&self, device_type: DetectionDeviceType) -> Vec<&StorageDevice> {
        DeviceScanner::filter_by_type(&self.devices, device_type)
    }

    /// Get devices by speed class
    #[must_use]
    pub fn get_devices_by_speed(&self, speed_class: SpeedClass) -> Vec<&StorageDevice> {
        DeviceScanner::filter_by_speed(&self.devices, speed_class)
    }

    /// Recommend pool configuration based on available devices
    pub fn recommend_pool_config(&self, pool_name: &str) -> CoreResult<PoolSetupConfig> {
        if pool_name.is_empty() {
            return Err(NestGateError::internal_error(
                "Pool name cannot be empty".to_string(),
                "pool_validation",
            ));
        }

        let available_devices = self.get_available_devices();

        if available_devices.is_empty() {
            return Err(NestGateError::internal_error(
                "No available devices found for pool setup",
                "recommend_pool_config",
            ));
        }

        info!(
            "Recommending pool configuration for {} available devices",
            available_devices.len()
        );

        // Determine optimal topology based on device count
        let topology = match available_devices.len() {
            1 => PoolTopology::Single,
            2 => PoolTopology::Mirror,
            3..=5 => PoolTopology::RaidZ1,
            6..=11 => PoolTopology::RaidZ2,
            _ => PoolTopology::RaidZ3,
        };

        // Select devices for the pool
        let mut selected_devices = Vec::new();
        let device_count = match topology {
            PoolTopology::Single => 1,
            PoolTopology::Mirror => 2,
            PoolTopology::RaidZ1 => std::cmp::min(available_devices.len(), 5),
            PoolTopology::RaidZ2 => std::cmp::min(available_devices.len(), 8),
            PoolTopology::RaidZ3 => std::cmp::min(available_devices.len(), 12),
        };

        // Prefer faster devices
        let mut sorted_devices = available_devices.clone();
        sorted_devices.sort_by(|a, b| {
            b.speed_class
                .cmp(&a.speed_class)
                .then_with(|| b.size_bytes.cmp(&a.size_bytes))
        });

        for device in sorted_devices.iter().take(device_count) {
            selected_devices.push(device.device_path.clone());
        }

        // Set up default properties
        let mut properties = HashMap::new();
        properties.insert("ashift".to_string(), "12".to_string());
        properties.insert("autoexpand".to_string(), "on".to_string());
        properties.insert("autotrim".to_string(), "on".to_string());

        // Configure tier mappings
        let mut tier_mappings = HashMap::new();
        self.configure_tier_mappings(&mut tier_mappings, &sorted_devices)?;

        Ok(PoolSetupConfig {
            pool_name: pool_name.to_string(),
            devices: selected_devices,
            topology,
            properties,
            create_tiers: true,
            tier_mappings,
            redundancy: RedundancyLevel::None,
            device_detection: DeviceDetectionConfig::default(),
        })
    }

    /// Configure tier mappings based on available devices
    fn configure_tier_mappings(
        &self,
        tier_mappings: &mut HashMap<ConfigStorageTier, Vec<ConfigDeviceType>>,
        devices: &[&StorageDevice],
    ) -> CoreResult<()> {
        let device_types: std::collections::HashSet<ConfigDeviceType> = devices
            .iter()
            .map(|d| convert_device_type(&d.device_type))
            .collect();

        if device_types.len() == 1 {
            // Single device type - use for all tiers
            let primary_type = device_types
                .iter()
                .next()
                .ok_or_else(|| {
                    NestGateError::internal_error(
                        "No device types found for tier mapping",
                        "configure_tier_mappings",
                    )
                })?
                .clone();
            tier_mappings.insert(ConfigStorageTier::Hot, vec![primary_type.clone()]);
            tier_mappings.insert(ConfigStorageTier::Warm, vec![primary_type.clone()]);
            tier_mappings.insert(ConfigStorageTier::Cold, vec![primary_type]);
        } else {
            // Multiple device types - optimize assignment
            let mut hot_types = Vec::new();
            let mut warm_types = Vec::new();
            let mut cold_types = Vec::new();

            for device_type in &device_types {
                match device_type {
                    ConfigDeviceType::OptaneMemory | ConfigDeviceType::NvmeSsd => {
                        hot_types.push(device_type.clone());
                        warm_types.push(device_type.clone());
                    }
                    ConfigDeviceType::SataSsd => {
                        warm_types.push(device_type.clone());
                        cold_types.push(device_type.clone());
                    }
                    ConfigDeviceType::SpinningDisk => {
                        cold_types.push(device_type.clone());
                    }
                }
            }

            // Ensure each tier has at least one device type
            if hot_types.is_empty() {
                hot_types = warm_types.clone();
            }
            if warm_types.is_empty() {
                if let Some(device_type) = device_types.iter().next() {
                    warm_types = vec![device_type.clone()];
                } else {
                    return Err(NestGateError::internal_error(
                        "Invalid tier configuration detected",
                        "configure_tier_mappings",
                    ));
                }
            }
            if cold_types.is_empty() {
                cold_types = warm_types.clone();
            }

            tier_mappings.insert(ConfigStorageTier::Hot, hot_types);
            tier_mappings.insert(ConfigStorageTier::Warm, warm_types);
            tier_mappings.insert(ConfigStorageTier::Cold, cold_types);
        }
        Ok(())
    }

    /// Validate device
    #[must_use]
    pub fn validate_device(&self, device: &StorageDevice) -> ValidationResult {
        self.validator.validate_device(device)
    }

    /// Validate pool configuration
    #[must_use]
    pub fn validate_pool_config(&self, config: &PoolSetupConfig) -> ValidationResult {
        self.validator.validate_pool_config(config)
    }

    /// Create pool with safety checks
    pub async fn create_pool_safe(&self, config: &PoolSetupConfig) -> CoreResult<PoolSetupResult> {
        // Pre-flight validation
        let validation = self.validate_pool_config(config);
        if !validation.is_valid {
            return Err(NestGateError::internal_error(
                format!(
                    "Pool configuration validation failed: {:?}",
                    validation.issues
                ),
                "create_pool_safe",
            ));
        }

        self.creator.create_pool_safe(config).await
    }

    /// Get system report
    #[must_use]
    pub fn get_system_report(&self) -> SystemReport {
        SystemReport {
            total_devices: self.devices.len(),
            available_devices: self.get_available_devices().len(),
            devices_by_type: self.get_device_type_summary(),
            devices_by_speed: self.get_speed_class_summary(),
            existing_pools: self.existing_pools.clone(),
            recommendations: self.get_recommendations(),
        }
    }

    /// Gets Device Type Summary
    fn get_device_type_summary(&self) -> HashMap<DetectionDeviceType, usize> {
        let mut summary = HashMap::new();
        for device in &self.devices {
            *summary.entry(device.device_type.clone()).or_insert(0) += 1;
        }
        summary
    }

    /// Gets Speed Class Summary
    fn get_speed_class_summary(&self) -> HashMap<SpeedClass, usize> {
        let mut summary = HashMap::new();
        for device in &self.devices {
            *summary.entry(device.speed_class.clone()).or_insert(0) += 1;
        }
        summary
    }

    /// Gets Recommendations
    fn get_recommendations(&self) -> Vec<String> {
        let mut recommendations = Vec::new();
        let available = self.get_available_devices();

        if available.is_empty() {
            recommendations.push("No available devices found for pool creation".to_string());
        } else if available.len() == 1 {
            recommendations.push("Consider adding more devices for redundancy".to_string());
        } else if available.len() >= 3 {
            recommendations.push(
                "RAID-Z configuration recommended for optimal redundancy and performance"
                    .to_string(),
            );
        }

        // Check for mixed device types
        let device_types: std::collections::HashSet<_> =
            available.iter().map(|d| &d.device_type).collect();

        if device_types.len() > 1 {
            recommendations.push(
                "Mixed device types detected - consider separate pools for optimal performance"
                    .to_string(),
            );
        }

        recommendations
    }
}

/// System report structure
#[derive(Debug, Clone)]
/// Systemreport
pub struct SystemReport {
    /// Total Devices
    pub total_devices: usize,
    /// Available Devices
    pub available_devices: usize,
    /// Devices By Type
    pub devices_by_type: HashMap<DetectionDeviceType, usize>,
    /// Devices By Speed
    pub devices_by_speed: HashMap<SpeedClass, usize>,
    /// Existing Pools
    pub existing_pools: Vec<String>,
    /// Recommendations
    pub recommendations: Vec<String>,
}
/// Production ZFS setup function
pub async fn setup_production_zfs() -> CoreResult<PoolSetupResult> {
    info!("Setting up production ZFS configuration");
    let setup = ZfsPoolSetup::new().await?;
    let config = setup.recommend_pool_config("nestgate-main")?;

    info!("Recommended configuration: {:?}", config);

    setup.create_pool_safe(&config).await
}

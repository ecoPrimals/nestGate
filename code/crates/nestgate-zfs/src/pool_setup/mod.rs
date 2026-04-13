// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//
// Comprehensive ZFS pool setup with device detection, validation, and creation

//! Pool Setup module

use tracing::info;
use tracing::warn;
/// Pool configuration utilities
pub mod config;
/// Pool creation and management
pub mod creation;
pub mod device_detection;
pub mod validation;

// Re-export main types for convenience
pub use config::{
    CanonicalZfsPoolConfig, DeviceDetectionConfig, DeviceType as ConfigDeviceType, PoolSetupConfig,
    PoolTopology, RedundancyLevel, StorageTier as ConfigStorageTier,
};
pub use creation::PoolCreator;
pub use device_detection::{
    DeviceScanner, DeviceType as DetectionDeviceType, SpeedClass, StorageDevice,
};
pub use validation::{PoolSetupValidator, ValidationResult};

// Tests
#[cfg(test)]
mod config_tests;
// creation_tests disabled: referenced non-existent types (PoolName, VdevType); restore when aligned.
#[cfg(test)]
mod device_detection_tests;
#[cfg(test)]
mod tests;

// comprehensive_tests disabled: outdated API; refactor to current types before re-enabling.
#[cfg(test)]
mod pool_setup_tests;
#[cfg(test)]
mod types_tests;
#[cfg(test)]
mod validation_tests;

use nestgate_core::{NestGateError, Result as CoreResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Pool setup specific errors
#[derive(Debug, thiserror::Error)]
/// Errors that can occur during `PoolSetup` operations
pub enum PoolSetupError {
    /// Device validation failed
    #[error("Device validation failed: {0}")]
    DeviceValidation(String),
    /// Pool creation failed
    #[error("Pool creation failed: {0}")]
    PoolCreation(String),

    /// Configuration error
    #[error("Configuration error: {0}")]
    Configuration(String),

    /// Device scanning failed
    #[error("Device scanning failed: {0}")]
    DeviceScanning(String),

    /// Insufficient devices available
    #[error("Insufficient devices: {0}")]
    InsufficientDevices(String),

    /// ZFS command execution failed
    #[error("ZFS command failed: {0}")]
    ZfsCommand(String),

    #[error("Core error: {0}")]
    /// Core `NestGate` error
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
    config: PoolSetupConfig,
    /// Device scanner
    scanner: DeviceScanner,
    /// Validator
    validator: PoolSetupValidator,
    /// Pool creator
    creator: PoolCreator,
}

include!("impl_scanning.rs");
include!("impl_recommendations.rs");
include!("impl_creation.rs");

#[cfg(test)]
impl ZfsPoolSetup {
    /// Construct a pool setup instance without scanning disks (tests only).
    pub(crate) fn test_fixture(devices: Vec<StorageDevice>, existing_pools: Vec<String>) -> Self {
        let config = PoolSetupConfig::default();
        Self {
            devices,
            existing_pools,
            config: config.clone(),
            scanner: DeviceScanner::new(config.device_detection.clone()),
            validator: PoolSetupValidator::new(config),
            creator: PoolCreator::new(),
        }
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

#[cfg(test)]
mod pool_setup_unit_tests;

// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//
// ZFS pool creation, tier setup, and management operations

use super::PoolSetupResult;
use super::config::{PoolSetupConfig, PoolTopology};
use nestgate_core::{NestGateError, Result as CoreResult};
use tokio::process::Command as AsyncCommand;
use tracing::{error, info, warn};

/// ZFS pool creator implementation
pub struct PoolCreator {
    dry_run: bool,
}

impl PoolCreator {
    /// Create a new pool creator
    #[must_use]
    pub const fn new() -> Self {
        Self { dry_run: false }
    }

    /// Create a new pool creator in dry-run mode
    #[must_use]
    pub const fn new_dry_run() -> Self {
        Self { dry_run: true }
    }

    /// Create a ZFS pool with safety checks
    pub async fn create_pool_safe(&self, config: &PoolSetupConfig) -> CoreResult<PoolSetupResult> {
        info!("Creating ZFS pool: {}", config.pool_name);

        if self.dry_run {
            return self.dry_run_pool_creation(config).await;
        }

        self.create_pool_internal(config).await
    }

    /// Internal pool creation logic
    async fn create_pool_internal(&self, config: &PoolSetupConfig) -> CoreResult<PoolSetupResult> {
        // Validate configuration
        if config.pool_name.is_empty() {
            return Err(NestGateError::internal_error(
                "Pool name cannot be empty",
                "create_pool_internal",
            ));
        }

        if config.devices.is_empty() {
            return Err(NestGateError::internal_error(
                "No devices specified for pool creation",
                "create_pool_internal",
            ));
        }

        // Build ZFS create command
        let mut cmd = AsyncCommand::new("zpool");
        cmd.arg("create");

        // Add pool name
        cmd.arg(&config.pool_name);

        // Add topology-specific arguments
        match config.topology {
            PoolTopology::Single => {
                // Single device pool
                if config.devices.len() != 1 {
                    return Err(NestGateError::internal_error(
                        "Single topology requires exactly one device",
                        "create_pool_internal",
                    ));
                }
                cmd.arg(&config.devices[0]);
            }
            PoolTopology::Mirror => {
                cmd.arg("mirror");
                for device in &config.devices {
                    cmd.arg(device);
                }
            }
            PoolTopology::RaidZ1 => {
                cmd.arg("raidz1");
                for device in &config.devices {
                    cmd.arg(device);
                }
            }
            PoolTopology::RaidZ2 => {
                cmd.arg("raidz2");
                for device in &config.devices {
                    cmd.arg(device);
                }
            }
            PoolTopology::RaidZ3 => {
                cmd.arg("raidz3");
                for device in &config.devices {
                    cmd.arg(device);
                }
            }
        }

        // Add pool properties
        for (key, value) in &config.properties {
            if key.is_empty() || value.is_empty() {
                return Err(NestGateError::internal_error(
                    "Invalid pool property: key or value is empty",
                    "create_pool_internal",
                ));
            }
            cmd.args(["-o", &format!("{key}={value}")]);
        }

        // Add default dataset properties
        cmd.args(["-O", "compression=lz4"]);
        cmd.args(["-O", "atime=off"]);

        // Execute the command
        info!("Executing ZFS pool creation command");
        let output = cmd.output().await.map_err(|_| {
            NestGateError::internal_error(
                "Failed to execute ZFS pool creation command",
                "create_pool_internal",
            )
        })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            error!("ZFS pool creation failed: {}", stderr);
            return Err(NestGateError::internal_error(
                format!("ZFS pool creation failed: {stderr}"),
                "create_pool_internal",
            ));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        info!("ZFS pool created successfully: {}", stdout);

        Ok(PoolSetupResult {
            pool_name: config.pool_name.clone(),
            success: true,
            message: "Pool created successfully".to_string(),
            devices_used: config.devices.clone(),
            topology: config.topology.clone(),
        })
    }

    /// Perform a dry run of pool creation
    async fn dry_run_pool_creation(&self, config: &PoolSetupConfig) -> CoreResult<PoolSetupResult> {
        info!("DRY RUN: Would create ZFS pool: {}", config.pool_name);
        info!("DRY RUN: Topology: {:?}", config.topology);
        info!("DRY RUN: Devices: {:?}", config.devices);
        info!("DRY RUN: Properties: {:?}", config.properties);

        // Validate configuration without actually creating
        if config.pool_name.is_empty() {
            return Err(NestGateError::internal_error(
                "Pool name cannot be empty",
                "dry_run_pool_creation",
            ));
        }

        if config.devices.is_empty() {
            return Err(NestGateError::internal_error(
                "No devices specified for pool creation",
                "dry_run_pool_creation",
            ));
        }

        // Check if devices exist
        for device in &config.devices {
            if !self.device_exists(device).await? {
                warn!("DRY RUN: Device does not exist: {}", device);
            }
        }

        Ok(PoolSetupResult {
            pool_name: config.pool_name.clone(),
            success: true,
            message: "Dry run completed successfully - pool would be created".to_string(),
            devices_used: config.devices.clone(),
            topology: config.topology.clone(),
        })
    }

    /// Check if a device exists
    async fn device_exists(&self, device: &str) -> CoreResult<bool> {
        let output = AsyncCommand::new("test")
            .args(["-b", device])
            .output()
            .await
            .map_err(|_| {
                NestGateError::internal_error("Failed to check device existence", "device_exists")
            })?;

        Ok(output.status.success())
    }

    /// Import an existing ZFS pool
    pub async fn import_pool(&self, pool_name: &str) -> CoreResult<PoolSetupResult> {
        info!("Importing ZFS pool: {}", pool_name);

        if self.dry_run {
            info!("DRY RUN: Would import ZFS pool: {}", pool_name);
            return Ok(PoolSetupResult {
                pool_name: pool_name.to_string(),
                success: true,
                message: "Dry run - pool would be imported".to_string(),
                devices_used: vec![],
                topology: PoolTopology::Single, // Unknown topology
            });
        }

        let output = AsyncCommand::new("zpool")
            .args(["import", pool_name])
            .output()
            .await
            .map_err(|_| {
                NestGateError::internal_error(
                    "Failed to execute pool import command",
                    "import_pool",
                )
            })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            error!("ZFS pool import failed: {}", stderr);
            return Err(NestGateError::internal_error(
                format!("ZFS pool import failed: {stderr}"),
                "import_pool",
            ));
        }

        Ok(PoolSetupResult {
            pool_name: pool_name.to_string(),
            success: true,
            message: "Pool imported successfully".to_string(),
            devices_used: vec![], // Would need to query for actual devices
            topology: PoolTopology::Single, // Would need to query for actual topology
        })
    }

    /// Destroy a ZFS pool
    pub async fn destroy_pool(&self, pool_name: &str, force: bool) -> CoreResult<()> {
        warn!("Destroying ZFS pool: {} (force: {})", pool_name, force);

        if self.dry_run {
            warn!("DRY RUN: Would destroy ZFS pool: {}", pool_name);
            return Ok(());
        }

        let mut cmd = AsyncCommand::new("zpool");
        cmd.args(["destroy"]);

        if force {
            cmd.arg("-f");
        }

        cmd.arg(pool_name);

        let output = cmd.output().await.map_err(|_| {
            NestGateError::internal_error("Failed to execute pool destroy command", "destroy_pool")
        })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            error!("ZFS pool destruction failed: {}", stderr);
            return Err(NestGateError::internal_error(
                format!("ZFS pool destruction failed: {stderr}"),
                "destroy_pool",
            ));
        }

        info!("ZFS pool destroyed successfully: {}", pool_name);
        Ok(())
    }

    /// List available ZFS pools
    pub async fn list_pools(&self) -> CoreResult<Vec<String>> {
        let output = AsyncCommand::new("zpool")
            .args(["list", "-H", "-o", "name"])
            .output()
            .await
            .map_err(|_| {
                NestGateError::internal_error("Failed to execute pool list command", "list_pools")
            })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(NestGateError::internal_error(
                format!("Failed to list pools: {stderr}"),
                "list_pools",
            ));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let pools: Vec<String> = stdout
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| line.trim().to_string())
            .collect();

        Ok(pools)
    }
}

impl Default for PoolCreator {
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pool_setup::config::RedundancyLevel;
    use std::collections::HashMap;

    #[tokio::test]
    async fn test_dry_run_pool_creation() {
        let creator = PoolCreator::new_dry_run();
        let config = PoolSetupConfig {
            pool_name: "test-pool".to_string(),
            devices: vec!["/dev/sdb".to_string()],
            topology: PoolTopology::Single,
            properties: HashMap::new(),
            tier_mappings: HashMap::new(),
            redundancy: RedundancyLevel::None,
            device_detection: crate::pool_setup::config::DeviceDetectionConfig::default(),
            create_tiers: false,
        };

        let result = creator.create_pool_safe(&config).await;
        assert!(result.is_ok());

        let result = result.expect("ZFS operation failed");
        assert!(result.success);
        assert!(result.message.contains("Dry run"));
    }

    #[tokio::test]
    async fn test_invalid_pool_name() {
        let creator = PoolCreator::new_dry_run();
        let config = PoolSetupConfig {
            pool_name: "".to_string(),
            devices: vec!["/dev/sdb".to_string()],
            topology: PoolTopology::Single,
            properties: HashMap::new(),
            tier_mappings: HashMap::new(),
            redundancy: RedundancyLevel::None,
            device_detection: crate::pool_setup::config::DeviceDetectionConfig::default(),
            create_tiers: false,
        };

        let result = creator.create_pool_safe(&config).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_no_devices() {
        let creator = PoolCreator::new_dry_run();
        let config = PoolSetupConfig {
            pool_name: "test-pool".to_string(),
            devices: vec![],
            topology: PoolTopology::Single,
            properties: HashMap::new(),
            tier_mappings: HashMap::new(),
            redundancy: RedundancyLevel::None,
            device_detection: crate::pool_setup::config::DeviceDetectionConfig::default(),
            create_tiers: false,
        };

        let result = creator.create_pool_safe(&config).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn dry_run_import_pool_returns_success_message() {
        let creator = PoolCreator::new_dry_run();
        let r = creator.import_pool("imported-pool").await.expect("import");
        assert!(r.success);
        assert!(r.message.contains("Dry run"));
    }

    #[tokio::test]
    async fn dry_run_destroy_pool_ok() {
        let creator = PoolCreator::new_dry_run();
        creator
            .destroy_pool("gone-pool", true)
            .await
            .expect("destroy dry run");
    }

    #[test]
    fn pool_creator_default_constructible() {
        let _ = PoolCreator::default();
        let _ = PoolCreator::new();
    }
}

#[cfg(test)]
mod round3_validation_tests {
    use crate::pool_setup::config::{PoolSetupConfig, PoolTopology, RedundancyLevel};
    use std::collections::HashMap;

    /// Mirrors `create_pool_internal` topology + property checks without shelling out to `zpool`.
    fn validate_zfs_create_args(config: &PoolSetupConfig) -> bool {
        if config.pool_name.is_empty() || config.devices.is_empty() {
            return false;
        }
        for (key, value) in &config.properties {
            if key.is_empty() || value.is_empty() {
                return false;
            }
        }
        match config.topology {
            PoolTopology::Single => config.devices.len() == 1,
            PoolTopology::Mirror
            | PoolTopology::RaidZ1
            | PoolTopology::RaidZ2
            | PoolTopology::RaidZ3 => !config.devices.is_empty(),
        }
    }

    #[test]
    fn single_topology_requires_exactly_one_device() {
        let mut c = PoolSetupConfig {
            pool_name: "p".into(),
            devices: vec!["/dev/a".into(), "/dev/b".into()],
            topology: PoolTopology::Single,
            properties: HashMap::new(),
            tier_mappings: HashMap::new(),
            redundancy: RedundancyLevel::None,
            device_detection: crate::pool_setup::config::DeviceDetectionConfig::default(),
            create_tiers: false,
        };
        assert!(!validate_zfs_create_args(&c));
        c.devices = vec!["/dev/a".into()];
        assert!(validate_zfs_create_args(&c));
    }

    #[test]
    fn empty_pool_property_pair_invalid() {
        let mut c = PoolSetupConfig {
            pool_name: "p".into(),
            devices: vec!["/dev/a".into()],
            topology: PoolTopology::Single,
            properties: HashMap::from([("".into(), "v".into())]),
            tier_mappings: HashMap::new(),
            redundancy: RedundancyLevel::None,
            device_detection: crate::pool_setup::config::DeviceDetectionConfig::default(),
            create_tiers: false,
        };
        assert!(!validate_zfs_create_args(&c));
        c.properties = HashMap::from([("k".into(), "".into())]);
        assert!(!validate_zfs_create_args(&c));
    }
}

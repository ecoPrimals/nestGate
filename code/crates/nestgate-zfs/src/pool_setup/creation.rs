//! Pool Creation and Management
//!
//! ZFS pool creation, tier setup, and management operations

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, Instant};
use tokio::process::Command;
use tracing::{debug, error, info, warn};

use super::{
    config::{PoolSetupConfiguration, TierProperties},
    validation::{PoolSetupConfig, PoolTopology},
};
use nestgate_core::{NestGateError, Result as CoreResult, StorageTier};

/// Result of pool setup operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolSetupResult {
    /// Pool name that was created
    pub pool_name: String,
    /// Devices used
    pub devices_used: Vec<String>,
    /// Tier datasets created
    pub tiers_created: Vec<StorageTier>,
    /// Pool properties set
    pub properties_set: HashMap<String, String>,
    /// Setup duration in milliseconds
    pub setup_duration_ms: u64,
    /// Any warnings during setup
    pub warnings: Vec<String>,
}

/// Pool creator for ZFS pool creation operations
pub struct PoolCreator {
    config: PoolSetupConfiguration,
}

impl PoolCreator {
    pub fn new(config: PoolSetupConfiguration) -> Self {
        Self { config }
    }

    /// Safe pool creation with comprehensive validation and rollback
    pub async fn create_pool_safe(&self, config: &PoolSetupConfig) -> CoreResult<PoolSetupResult> {
        let start_time = Instant::now();
        let mut warnings = Vec::new();

        info!("Starting safe pool creation: {}", config.pool_name);

        // Safety confirmation if required
        if self.config.safety.require_confirmation {
            info!("Pool creation requires confirmation (safety.require_confirmation = true)");
            if self.config.safety.default_dry_run {
                info!(
                    "DRY RUN: Would create pool with configuration: {:#?}",
                    config
                );
                return Ok(PoolSetupResult {
                    pool_name: format!("{} (DRY RUN)", config.pool_name),
                    devices_used: config.devices.clone(),
                    tiers_created: Vec::new(),
                    properties_set: config.properties.clone(),
                    setup_duration_ms: start_time.elapsed().as_millis() as u64,
                    warnings,
                });
            }
        }

        // Backup existing data if requested
        if self.config.safety.auto_backup {
            warnings.push("Auto-backup requested but not yet implemented".to_string());
        }

        // Create pool with enhanced error handling
        match self.create_pool_internal(config).await {
            Ok(mut result) => {
                result.warnings.extend(warnings);
                Ok(result)
            }
            Err(e) => {
                error!("Pool creation failed, initiating cleanup: {}", e);

                // Attempt to cleanup failed pool creation
                if let Err(cleanup_error) = self.cleanup_failed_creation(&config.pool_name).await {
                    warn!(
                        "Cleanup after failed creation also failed: {}",
                        cleanup_error
                    );
                }

                Err(e)
            }
        }
    }

    /// Internal pool creation with proper error handling
    async fn create_pool_internal(&self, config: &PoolSetupConfig) -> CoreResult<PoolSetupResult> {
        let start_time = Instant::now();

        // Build zpool create command with validation
        let mut cmd = Command::new("zpool");
        cmd.arg("create");

        // Add pool properties with validation
        for (key, value) in &config.properties {
            if key.is_empty() || value.is_empty() {
                return Err(NestGateError::Internal(format!(
                    "Invalid property: {}={}",
                    key, value
                )));
            }
            cmd.args(["-o", &format!("{}={}", key, value)]);
        }

        // Add dataset properties
        cmd.args(["-O", "compression=lz4"]);
        cmd.args(["-O", "atime=off"]);
        cmd.args(["-O", "xattr=sa"]);
        cmd.args(["-O", "dnodesize=auto"]);
        cmd.args(["-O", "normalization=formD"]);
        cmd.args(["-O", "mountpoint=none"]);

        // Add pool name
        cmd.arg(&config.pool_name);

        // Add topology and devices
        match config.topology {
            PoolTopology::Single => {
                cmd.args(&config.devices);
            }
            PoolTopology::Mirror => {
                cmd.arg("mirror");
                cmd.args(&config.devices);
            }
            PoolTopology::RaidZ1 => {
                cmd.arg("raidz1");
                cmd.args(&config.devices);
            }
            PoolTopology::RaidZ2 => {
                cmd.arg("raidz2");
                cmd.args(&config.devices);
            }
            PoolTopology::RaidZ3 => {
                cmd.arg("raidz3");
                cmd.args(&config.devices);
            }
        }

        // Execute with timeout
        let timeout_duration = Duration::from_secs(300); // 5 minutes
        let output = tokio::time::timeout(timeout_duration, cmd.output())
            .await
            .map_err(|_| NestGateError::Internal("Pool creation timed out".to_string()))?
            .map_err(|e| {
                NestGateError::Internal(format!("Failed to execute zpool create: {}", e))
            })?;

        if !output.status.success() {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            let stdout_msg = String::from_utf8_lossy(&output.stdout);
            return Err(NestGateError::Internal(format!(
                "Pool creation failed: stderr: {}, stdout: {}",
                error_msg, stdout_msg
            )));
        }

        info!("Successfully created ZFS pool: {}", config.pool_name);

        // Create tier structure if requested
        let mut tiers_created = Vec::new();
        if config.create_tiers {
            match self.create_tier_structure_safe(&config.pool_name).await {
                Ok(tiers) => tiers_created = tiers,
                Err(e) => {
                    warn!("Failed to create tier structure: {}", e);
                    // Don't fail the entire operation for tier creation failure
                }
            }
        }

        let duration = start_time.elapsed();

        Ok(PoolSetupResult {
            pool_name: config.pool_name.clone(),
            devices_used: config.devices.clone(),
            tiers_created,
            properties_set: config.properties.clone(),
            setup_duration_ms: duration.as_millis() as u64,
            warnings: Vec::new(),
        })
    }

    /// Create tier structure with enhanced error handling
    async fn create_tier_structure_safe(&self, pool_name: &str) -> CoreResult<Vec<StorageTier>> {
        info!("Creating tier structure for pool: {}", pool_name);

        let tiers = vec![
            (StorageTier::Hot, "hot"),
            (StorageTier::Warm, "warm"),
            (StorageTier::Cold, "cold"),
        ];

        let mut created_tiers = Vec::new();

        for (tier, name) in tiers {
            match self.create_single_tier(pool_name, &tier, name).await {
                Ok(()) => {
                    info!("Created tier dataset: {}/{}", pool_name, name);
                    created_tiers.push(tier);
                }
                Err(e) => {
                    warn!("Failed to create tier {}: {}", name, e);
                    // Continue with other tiers
                }
            }
        }

        Ok(created_tiers)
    }

    /// Create a single tier with proper configuration
    async fn create_single_tier(
        &self,
        pool_name: &str,
        _tier: &StorageTier,
        tier_name: &str,
    ) -> CoreResult<()> {
        let dataset_name = format!("{}/{}", pool_name, tier_name);
        let mountpoint = format!("/{}", dataset_name);

        // Get tier-specific properties from configuration
        let tier_props = self
            .config
            .tier_config
            .tier_properties
            .get(tier_name)
            .cloned()
            .unwrap_or_else(|| {
                warn!("No tier properties found for {}, using defaults", tier_name);
                TierProperties {
                    compression: "lz4".to_string(),
                    recordsize: "128K".to_string(),
                    primarycache: "all".to_string(),
                    secondarycache: "all".to_string(),
                    logbias: "throughput".to_string(),
                    sync: "standard".to_string(),
                    atime: "off".to_string(),
                }
            });

        // Create dataset with tier-specific properties
        let mut cmd = Command::new("zfs");
        cmd.arg("create");

        // Set tier-specific properties
        cmd.args(["-o", &format!("compression={}", tier_props.compression)]);
        cmd.args(["-o", &format!("recordsize={}", tier_props.recordsize)]);
        cmd.args(["-o", &format!("primarycache={}", tier_props.primarycache)]);
        cmd.args([
            "-o",
            &format!("secondarycache={}", tier_props.secondarycache),
        ]);
        cmd.args(["-o", &format!("logbias={}", tier_props.logbias)]);
        cmd.args(["-o", &format!("sync={}", tier_props.sync)]);
        cmd.args(["-o", &format!("atime={}", tier_props.atime)]);
        cmd.args(["-o", &format!("mountpoint={}", mountpoint)]);

        cmd.arg(&dataset_name);

        let output = cmd
            .output()
            .await
            .map_err(|e| NestGateError::Internal(format!("Failed to execute zfs create: {}", e)))?;

        if !output.status.success() {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            return Err(NestGateError::Internal(format!(
                "Failed to create tier dataset {}: {}",
                dataset_name, error_msg
            )));
        }

        Ok(())
    }

    /// Destroy pool with safety checks
    pub async fn destroy_pool_safe(&self, pool_name: &str, force: bool) -> CoreResult<()> {
        info!("Destroying pool: {} (force: {})", pool_name, force);

        if !force && self.config.safety.require_confirmation {
            return Err(NestGateError::Internal(
                "Pool destruction requires explicit force flag when confirmation is required"
                    .to_string(),
            ));
        }

        let mut cmd = Command::new("zpool");
        cmd.arg("destroy");

        if force {
            cmd.arg("-f");
        }

        cmd.arg(pool_name);

        let output = cmd.output().await.map_err(|e| {
            NestGateError::Internal(format!("Failed to execute zpool destroy: {}", e))
        })?;

        if !output.status.success() {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            return Err(NestGateError::Internal(format!(
                "Failed to destroy pool {}: {}",
                pool_name, error_msg
            )));
        }

        info!("Successfully destroyed pool: {}", pool_name);
        Ok(())
    }

    /// Cleanup after failed pool creation
    async fn cleanup_failed_creation(&self, pool_name: &str) -> CoreResult<()> {
        info!("Cleaning up failed pool creation: {}", pool_name);

        // Try to destroy the partially created pool
        let destroy_result = tokio::process::Command::new("zpool")
            .args(&["destroy", "-f", pool_name])
            .output()
            .await;

        match destroy_result {
            Ok(output) => {
                if output.status.success() {
                    info!("Successfully cleaned up failed pool: {}", pool_name);
                } else {
                    warn!(
                        "Pool cleanup command failed (pool may not exist): {}",
                        String::from_utf8_lossy(&output.stderr)
                    );
                }
            }
            Err(e) => {
                warn!("Failed to execute cleanup command: {}", e);
            }
        }

        Ok(())
    }

    /// Log pool creation attempt
    async fn log_creation_attempt(&self, config: &PoolSetupConfig) -> CoreResult<()> {
        info!("Attempting to create ZFS pool: {}", config.pool_name);
        info!("  Devices: {:?}", config.devices);
        info!("  Topology: {:?}", config.topology);
        info!("  Properties: {:?}", config.properties);
        Ok(())
    }

    /// Cleanup failed pool creation
    async fn cleanup_failed_pool_creation(&self, pool_name: &str) -> CoreResult<()> {
        info!("🧹 Cleaning up failed pool creation: {}", pool_name);

        // Check if pool exists in any state
        let pool_check = tokio::process::Command::new("zpool")
            .args(&["status", pool_name])
            .output()
            .await;

        match pool_check {
            Ok(output) if output.status.success() => {
                // Pool exists, attempt to destroy it
                warn!("🗑️ Destroying partially created pool: {}", pool_name);

                let destroy_result = tokio::process::Command::new("zpool")
                    .args(&["destroy", "-f", pool_name])
                    .output()
                    .await;

                match destroy_result {
                    Ok(destroy_output) if destroy_output.status.success() => {
                        info!("✅ Successfully cleaned up failed pool: {}", pool_name);
                    }
                    Ok(destroy_output) => {
                        let stderr = String::from_utf8_lossy(&destroy_output.stderr);
                        warn!("⚠️ Pool cleanup may have failed: {}", stderr);
                    }
                    Err(e) => {
                        error!("❌ Failed to destroy pool during cleanup: {}", e);
                        return Err(NestGateError::Internal(format!(
                            "Cleanup destroy command failed: {}",
                            e
                        )));
                    }
                }
            }
            Ok(_) => {
                // Pool doesn't exist, no cleanup needed
                debug!("Pool {} not found during cleanup (expected)", pool_name);
            }
            Err(e) => {
                warn!("Could not check pool status during cleanup: {}", e);
            }
        }

        // Additional cleanup: check for device labels that might need clearing
        // This helps prevent "device busy" errors on retry
        self.cleanup_device_labels(pool_name).await?;

        Ok(())
    }

    /// Clear ZFS labels from devices that were part of failed pool creation
    async fn cleanup_device_labels(&self, pool_name: &str) -> CoreResult<()> {
        debug!("🏷️ Checking for device labels to clear after failed pool creation");

        // Get list of devices that might have ZFS labels
        let import_check = tokio::process::Command::new("zpool")
            .args(&["import"])
            .output()
            .await;

        if let Ok(output) = import_check {
            let stdout = String::from_utf8_lossy(&output.stdout);

            // Look for references to our failed pool
            if stdout.contains(pool_name) {
                warn!("🔧 Found traces of failed pool in import list, attempting cleanup");

                // Force cleanup of the importable pool
                let force_cleanup = tokio::process::Command::new("zpool")
                    .args(&["import", "-f", "-D", pool_name])
                    .output()
                    .await;

                if let Ok(cleanup_output) = force_cleanup {
                    if cleanup_output.status.success() {
                        // Now destroy it properly
                        let _ = tokio::process::Command::new("zpool")
                            .args(&["destroy", "-f", pool_name])
                            .output()
                            .await;
                    }
                }
            }
        }

        Ok(())
    }
}

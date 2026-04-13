// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

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
}

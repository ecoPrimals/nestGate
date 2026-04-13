// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

/// Convert detection `DeviceType` to config `DeviceType`
const fn convert_device_type(detection_type: DetectionDeviceType) -> ConfigDeviceType {
    match detection_type {
        DetectionDeviceType::NvmeSsd => ConfigDeviceType::NvmeSsd,
        DetectionDeviceType::SataSsd => ConfigDeviceType::SataSsd,
        DetectionDeviceType::OptaneMemory => ConfigDeviceType::OptaneMemory,
        DetectionDeviceType::Hdd | DetectionDeviceType::Unknown => ConfigDeviceType::SpinningDisk,
    }
}

impl ZfsPoolSetup {
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

        // Prefer faster devices (index sort avoids cloning the `Vec<&StorageDevice>`).
        let mut order: Vec<usize> = (0..available_devices.len()).collect();
        order.sort_by(|&i, &j| {
            let a = available_devices[i];
            let b = available_devices[j];
            b.speed_class
                .cmp(&a.speed_class)
                .then_with(|| b.size_bytes.cmp(&a.size_bytes))
        });

        for &idx in order.iter().take(device_count) {
            selected_devices.push(available_devices[idx].device_path.clone());
        }

        let sorted_devices: Vec<&StorageDevice> =
            order.iter().map(|&idx| available_devices[idx]).collect();

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
            .map(|d| convert_device_type(d.device_type))
            .collect();

        if device_types.len() == 1 {
            // Single device type - use for all tiers
            let primary_type = device_types.iter().copied().next().ok_or_else(|| {
                NestGateError::internal_error(
                    "No device types found for tier mapping",
                    "configure_tier_mappings",
                )
            })?;
            tier_mappings.insert(ConfigStorageTier::Hot, vec![primary_type]);
            tier_mappings.insert(ConfigStorageTier::Warm, vec![primary_type]);
            tier_mappings.insert(ConfigStorageTier::Cold, vec![primary_type]);
        } else {
            // Multiple device types - optimize assignment
            let mut hot_types = Vec::new();
            let mut warm_types = Vec::new();
            let mut cold_types = Vec::new();

            for device_type in device_types.iter().copied() {
                match device_type {
                    ConfigDeviceType::OptaneMemory | ConfigDeviceType::NvmeSsd => {
                        hot_types.push(device_type);
                        warm_types.push(device_type);
                    }
                    ConfigDeviceType::SataSsd => {
                        warm_types.push(device_type);
                        cold_types.push(device_type);
                    }
                    ConfigDeviceType::SpinningDisk => {
                        cold_types.push(device_type);
                    }
                }
            }

            // Ensure each tier has at least one device type
            if hot_types.is_empty() {
                hot_types.clone_from(&warm_types);
            }
            if warm_types.is_empty() {
                if let Some(device_type) = device_types.iter().copied().next() {
                    warm_types = vec![device_type];
                } else {
                    return Err(NestGateError::internal_error(
                        "Invalid tier configuration detected",
                        "configure_tier_mappings",
                    ));
                }
            }
            if cold_types.is_empty() {
                cold_types.clone_from(&warm_types);
            }

            tier_mappings.insert(ConfigStorageTier::Hot, hot_types);
            tier_mappings.insert(ConfigStorageTier::Warm, warm_types);
            tier_mappings.insert(ConfigStorageTier::Cold, cold_types);
        }
        Ok(())
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
            *summary.entry(device.device_type).or_insert(0) += 1;
        }
        summary
    }

    /// Gets Speed Class Summary
    fn get_speed_class_summary(&self) -> HashMap<SpeedClass, usize> {
        let mut summary = HashMap::new();
        for device in &self.devices {
            *summary.entry(device.speed_class).or_insert(0) += 1;
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

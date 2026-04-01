// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! **System Orchestration**
//!
//! Domain: Temporal storage system coordination and era mapping
//!
//! This module handles:
//! - System-level orchestration
//! - Era mapping and transitions
//! - System initialization and lifecycle

use nestgate_types::error::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::device::{StorageEra, TemporalDevice};

/// Temporal storage system
///
/// Orchestrates storage across all technology eras.
/// Provides unified interface for temporal storage management.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalStorageSystem {
    /// Available storage devices across all eras
    pub devices: Vec<TemporalDevice>,
    /// Era mappings for data placement
    pub era_mappings: HashMap<String, EraMapping>,
}

/// Era mapping configuration
///
/// Maps data types or categories to storage eras.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EraMapping {
    /// Preferred storage era for this data type
    pub preferred_era: StorageEra,
    /// Fallback eras if preferred is unavailable
    pub fallback_eras: Vec<StorageEra>,
    /// Whether cross-era replication is enabled
    pub cross_era_replication: bool,
}

impl TemporalStorageSystem {
    /// Create a new temporal storage system
    ///
    /// # Returns
    ///
    /// Empty temporal storage system
    #[must_use]
    pub fn new() -> Self {
        Self {
            devices: Vec::new(),
            era_mappings: HashMap::new(),
        }
    }

    /// Initialize with device discovery
    ///
    /// # Returns
    ///
    /// System initialized with discovered devices
    ///
    /// # Errors
    ///
    /// Returns error if device discovery fails
    pub fn with_discovery() -> Result<Self> {
        let devices = TemporalDevice::auto_detect_any_storage()?;
        Ok(Self {
            devices,
            era_mappings: Self::default_era_mappings(),
        })
    }

    /// Get default era mappings
    ///
    /// # Returns
    ///
    /// Default mappings for common data types
    fn default_era_mappings() -> HashMap<String, EraMapping> {
        let mut mappings = HashMap::new();

        // Hot data -> Modern era (NVMe)
        mappings.insert(
            "hot".to_string(),
            EraMapping {
                preferred_era: StorageEra::Modern,
                fallback_eras: vec![StorageEra::Digital],
                cross_era_replication: false,
            },
        );

        // Archive data -> Biological era (DNA)
        mappings.insert(
            "archive".to_string(),
            EraMapping {
                preferred_era: StorageEra::Biological,
                fallback_eras: vec![StorageEra::Magnetic, StorageEra::Digital],
                cross_era_replication: true,
            },
        );

        // Cold data -> Magnetic era
        mappings.insert(
            "cold".to_string(),
            EraMapping {
                preferred_era: StorageEra::Magnetic,
                fallback_eras: vec![StorageEra::Digital],
                cross_era_replication: true,
            },
        );

        mappings
    }

    /// Add a storage device
    ///
    /// # Arguments
    ///
    /// * `device` - Device to add
    pub fn add_device(&mut self, device: TemporalDevice) {
        self.devices.push(device);
    }

    /// Add era mapping
    ///
    /// # Arguments
    ///
    /// * `data_type` - Data type identifier
    /// * `mapping` - Era mapping configuration
    pub fn add_era_mapping(&mut self, data_type: String, mapping: EraMapping) {
        self.era_mappings.insert(data_type, mapping);
    }

    /// Get devices for a specific era
    ///
    /// # Arguments
    ///
    /// * `era` - Storage era to filter by
    ///
    /// # Returns
    ///
    /// Vector of devices in the specified era
    #[must_use]
    pub fn devices_for_era(&self, era: &StorageEra) -> Vec<&TemporalDevice> {
        self.devices.iter().filter(|d| &d.era == era).collect()
    }

    /// Get total storage capacity
    ///
    /// # Returns
    ///
    /// Total capacity in megabytes across all devices
    #[must_use]
    pub fn total_capacity_mb(&self) -> u64 {
        self.devices.iter().map(|d| d.capacity_mb).sum()
    }

    /// Get device count
    ///
    /// # Returns
    ///
    /// Number of registered devices
    #[must_use]
    pub const fn device_count(&self) -> usize {
        self.devices.len()
    }

    /// Get era mapping for data type
    ///
    /// # Arguments
    ///
    /// * `data_type` - Data type identifier
    ///
    /// # Returns
    ///
    /// Optional era mapping
    #[must_use]
    pub fn get_era_mapping(&self, data_type: &str) -> Option<&EraMapping> {
        self.era_mappings.get(data_type)
    }
}

impl Default for TemporalStorageSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl EraMapping {
    /// Create new era mapping
    ///
    /// # Arguments
    ///
    /// * `preferred_era` - Preferred storage era
    ///
    /// # Returns
    ///
    /// New era mapping with no fallbacks
    #[must_use]
    pub const fn new(preferred_era: StorageEra) -> Self {
        Self {
            preferred_era,
            fallback_eras: Vec::new(),
            cross_era_replication: false,
        }
    }

    /// Add fallback era
    ///
    /// # Arguments
    ///
    /// * `era` - Fallback era to add
    pub fn add_fallback(&mut self, era: StorageEra) {
        self.fallback_eras.push(era);
    }

    /// Enable cross-era replication
    pub const fn enable_cross_era_replication(&mut self) {
        self.cross_era_replication = true;
    }

    /// Check if era is acceptable (preferred or fallback)
    ///
    /// # Arguments
    ///
    /// * `era` - Era to check
    ///
    /// # Returns
    ///
    /// `true` if era is acceptable
    #[must_use]
    pub fn accepts_era(&self, era: &StorageEra) -> bool {
        &self.preferred_era == era || self.fallback_eras.contains(era)
    }
}

#[cfg(test)]
mod tests {
    use super::super::device::{
        PerformanceTier, PhysicalDimensions, StorageEra, StorageTechnology, TemporalDevice,
    };
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_system_new_empty() {
        let system = TemporalStorageSystem::new();
        assert!(system.devices.is_empty());
        assert!(system.era_mappings.is_empty());
        assert_eq!(system.device_count(), 0);
        assert_eq!(system.total_capacity_mb(), 0);
    }

    #[test]
    fn test_system_default() {
        let system = TemporalStorageSystem::default();
        assert_eq!(system.device_count(), 0);
    }

    #[test]
    fn test_system_add_device_and_devices_for_era() {
        let mut system = TemporalStorageSystem::new();
        let device = TemporalDevice {
            era: StorageEra::Modern,
            technology: StorageTechnology::NVMe,
            capacity_mb: 512,
            performance_tier: PerformanceTier::Ultra,
            physical_dimensions: PhysicalDimensions {
                width_mm: 22.0,
                height_mm: 3.5,
                depth_mm: 80.0,
            },
            supported_formats: vec!["ext4".to_string()],
            metadata: HashMap::new(),
        };
        system.add_device(device);
        assert_eq!(system.device_count(), 1);
        assert_eq!(system.total_capacity_mb(), 512);
        let modern_devices = system.devices_for_era(&StorageEra::Modern);
        assert_eq!(modern_devices.len(), 1);
    }

    #[test]
    fn test_system_with_discovery() {
        let system = TemporalStorageSystem::with_discovery().unwrap();
        assert!(system.era_mappings.contains_key("hot"));
        assert!(system.era_mappings.contains_key("archive"));
        assert!(system.era_mappings.contains_key("cold"));
    }

    #[test]
    fn test_system_add_era_mapping() {
        let mut system = TemporalStorageSystem::new();
        let mapping = EraMapping::new(StorageEra::Digital);
        system.add_era_mapping("custom".to_string(), mapping);
        let retrieved = system.get_era_mapping("custom").unwrap();
        assert_eq!(&retrieved.preferred_era, &StorageEra::Digital);
    }

    #[test]
    fn test_era_mapping_new() {
        let mapping = EraMapping::new(StorageEra::Biological);
        assert_eq!(mapping.preferred_era, StorageEra::Biological);
        assert!(mapping.fallback_eras.is_empty());
        assert!(!mapping.cross_era_replication);
    }

    #[test]
    fn test_era_mapping_add_fallback_and_accepts_era() {
        let mut mapping = EraMapping::new(StorageEra::Modern);
        mapping.add_fallback(StorageEra::Digital);
        assert_eq!(mapping.fallback_eras.len(), 1);
        assert!(mapping.accepts_era(&StorageEra::Modern));
        assert!(mapping.accepts_era(&StorageEra::Digital));
        assert!(!mapping.accepts_era(&StorageEra::Biological));
    }

    #[test]
    fn test_era_mapping_enable_cross_era_replication() {
        let mut mapping = EraMapping::new(StorageEra::Magnetic);
        mapping.enable_cross_era_replication();
        assert!(mapping.cross_era_replication);
    }

    #[test]
    fn test_default_era_mappings_content() {
        let system = TemporalStorageSystem::with_discovery().unwrap();
        let hot = system.get_era_mapping("hot").unwrap();
        assert_eq!(hot.preferred_era, StorageEra::Modern);
        let archive = system.get_era_mapping("archive").unwrap();
        assert_eq!(archive.preferred_era, StorageEra::Biological);
        assert!(archive.cross_era_replication);
    }
}

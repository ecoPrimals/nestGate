//! **System Orchestration**
//!
//! Domain: Temporal storage system coordination and era mapping
//!
//! This module handles:
//! - System-level orchestration
//! - Era mapping and transitions
//! - System initialization and lifecycle

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
    pub fn with_discovery() -> crate::Result<Self> {
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
    pub fn devices_for_era(&self, era: &StorageEra) -> Vec<&TemporalDevice> {
        self.devices.iter().filter(|d| &d.era == era).collect()
    }

    /// Get total storage capacity
    ///
    /// # Returns
    ///
    /// Total capacity in megabytes across all devices
    pub fn total_capacity_mb(&self) -> u64 {
        self.devices.iter().map(|d| d.capacity_mb).sum()
    }

    /// Get device count
    ///
    /// # Returns
    ///
    /// Number of registered devices
    pub fn device_count(&self) -> usize {
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
    pub fn new(preferred_era: StorageEra) -> Self {
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
    pub fn enable_cross_era_replication(&mut self) {
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
    pub fn accepts_era(&self, era: &StorageEra) -> bool {
        &self.preferred_era == era || self.fallback_eras.contains(era)
    }
}

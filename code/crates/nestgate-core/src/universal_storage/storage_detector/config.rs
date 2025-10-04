//
// Configuration structures and settings for storage detection behavior.

use serde::{Deserialize, Serialize};

/// Configuration for storage detection behavior
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectionConfig {
    /// Whether to include virtual/loop devices
    pub include_virtual_devices: bool,
    /// Whether to perform performance profiling (can be slow)
    pub enable_performance_profiling: bool,
    /// Whether to cache detection results
    pub enable_caching: bool,
    /// Minimum storage size to consider (bytes)
    pub minimum_storage_size: u64,
    /// Timeout for detection operations (seconds)
    pub detection_timeout_secs: u64,
    /// Whether to detect cloud storage
    pub enable_cloud_detection: bool,
    /// Whether to detect network shares
    pub enable_network_detection: bool,
    /// Whether to perform deep analysis
    pub enable_deep_analysis: bool,
}
impl Default for DetectionConfig {
    fn default() -> Self {
        Self {
            include_virtual_devices: false,
            enable_performance_profiling: true,
            enable_caching: true,
            minimum_storage_size: 100_000_000, // 100MB minimum
            detection_timeout_secs: 30,
            enable_cloud_detection: true,
            enable_network_detection: true,
            enable_deep_analysis: false,
        }
    }
}

impl DetectionConfig {
    /// Create a fast detection configuration (minimal profiling)
    #[must_use]
    pub fn fast() -> Self {
        Self {
            enable_performance_profiling: false,
            enable_deep_analysis: false,
            detection_timeout_secs: 10,
            ..Default::default()
        }
    }

    /// Create a comprehensive detection configuration (full profiling)
    #[must_use]
    pub fn comprehensive() -> Self {
        Self {
            include_virtual_devices: true,
            enable_performance_profiling: true,
            enable_deep_analysis: true,
            detection_timeout_secs: 120,
            minimum_storage_size: 10_000_000, // 10MB minimum
            ..Default::default()
        }
    }

    /// Create a cloud-only detection configuration
    #[must_use]
    pub fn cloud_only() -> Self {
        Self {
            enable_cloud_detection: true,
            enable_network_detection: false,
            include_virtual_devices: false,
            ..Default::default()
        }
    }

    /// Create a local-only detection configuration
    #[must_use]
    pub fn local_only() -> Self {
        Self {
            enable_cloud_detection: false,
            enable_network_detection: false,
            ..Default::default()
        }
    }

    /// Validate configuration settings
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub fn validate(&self) -> Result<(), String> {
        if self.minimum_storage_size == 0 {
            return Err("minimum_storage_size must be greater than 0".to_string());
        }

        if self.detection_timeout_secs == 0 {
            return Err("detection_timeout_secs must be greater than 0".to_string());
        }

        if self.detection_timeout_secs > 300 {
            return Err("detection_timeout_secs should not exceed 300 seconds".to_string());
        }

        Ok(())
    }
}

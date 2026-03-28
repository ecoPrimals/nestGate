// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Pool configuration for ZFS setup
#[derive(Debug, Clone, Serialize, Deserialize)]
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
///
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::network::config::PoolSetupConfig;
///
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::network::config::PoolSetupConfig; // Now aliases to CanonicalNetworkConfig
/// ```
///
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(
    since = "0.11.0",
    note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead"
)]
/// Configuration for PoolSetup
pub struct PoolSetupConfig {
    /// Pool name
    pub pool_name: String,
    /// Devices
    pub devices: Vec<String>,
    /// Topology
    pub topology: PoolTopology,
    /// Properties
    pub properties: HashMap<String, String>,
    /// Tier Mappings
    pub tier_mappings: HashMap<StorageTier, Vec<DeviceType>>,
    /// Redundancy
    pub redundancy: RedundancyLevel,
    /// Device Detection
    pub device_detection: DeviceDetectionConfig,
    /// Create Tiers
    pub create_tiers: bool,
}

/// ZFS pool topology options
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Pooltopology
pub enum PoolTopology {
    /// Single
    Single,
    /// Mirror
    Mirror,
    /// Raidz1
    RaidZ1,
    /// Raidz2
    RaidZ2,
    /// Raidz3
    RaidZ3,
}

/// Storage tier definitions
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
/// Storagetier
pub enum StorageTier {
    /// Hot
    Hot,
    /// Warm
    Warm,
    /// Cold
    Cold,
    /// Cache
    Cache,
}

/// Device type classifications
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
/// Types of Device
pub enum DeviceType {
    /// Optanememory
    OptaneMemory,
    /// Nvmessd
    NvmeSsd,
    /// Satassd
    SataSsd,
    /// Spinningdisk
    SpinningDisk,
}

/// Redundancy level for pool setup
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Redundancylevel
pub enum RedundancyLevel {
    /// None
    None,
    /// Single
    Single,
    /// Double
    Double,
    /// Triple
    Triple,
}

/// Pool property configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
///
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::network::config::PoolPropertyConfig;
///
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::network::config::PoolPropertyConfig; // Now aliases to CanonicalNetworkConfig
/// ```
///
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(
    since = "0.11.0",
    note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead"
)]
/// Configuration for PoolProperty
pub struct PoolPropertyConfig {
    /// Ashift
    pub ashift: u8,
    /// Autoexpand
    pub autoexpand: bool,
    /// Autotrim
    pub autotrim: bool,
    /// Compression
    pub compression: String,
    /// Recordsize
    pub recordsize: String,
}

impl Default for PoolPropertyConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            ashift: 12,
            autoexpand: true,
            autotrim: true,
            compression: "lz4".to_string(),
            recordsize: "128K".to_string(),
        }
    }
}

/// Device detection configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
///
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::network::config::DeviceDetectionConfig;
///
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::network::config::DeviceDetectionConfig; // Now aliases to CanonicalNetworkConfig
/// ```
///
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(
    since = "0.11.0",
    note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead"
)]
/// Configuration for DeviceDetection
pub struct DeviceDetectionConfig {
    /// Scan Paths
    pub scan_paths: Vec<String>,
    /// Exclude Patterns
    pub exclude_patterns: Vec<String>,
    /// Include Removable
    pub include_removable: bool,
    /// Size of min device
    pub min_device_size: u64,
    /// Size of max device
    pub max_device_size: u64,
    /// Skip Mountpoints
    pub skip_mountpoints: Vec<String>,
    /// Skip Fstypes
    pub skip_fstypes: Vec<String>,
    /// Include Loop Devices
    pub include_loop_devices: bool,
}

impl Default for DeviceDetectionConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            scan_paths: vec!["/dev".to_string()],
            exclude_patterns: vec!["loop".to_string(), "ram".to_string()],
            include_removable: false,
            min_device_size: 1024 * 1024 * 1024, // 1GB minimum
            max_device_size: 0,                  // 0 means no maximum
            skip_mountpoints: vec!["/".to_string(), "/boot".to_string(), "/home".to_string()],
            skip_fstypes: vec!["ext4".to_string(), "xfs".to_string(), "btrfs".to_string()],
            include_loop_devices: false,
        }
    }
}

impl Default for PoolSetupConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            pool_name: "rpool".to_string(),
            devices: Vec::new(),
            topology: PoolTopology::Single,
            properties: HashMap::new(),
            tier_mappings: HashMap::new(),
            redundancy: RedundancyLevel::None,
            device_detection: DeviceDetectionConfig::default(),
            create_tiers: false,
        }
    }
}

/// Safety configuration for pool operations
#[derive(Debug, Clone, Serialize, Deserialize)]
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
///
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::network::config::SafetyConfig;
///
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::network::config::SafetyConfig; // Now aliases to CanonicalNetworkConfig
/// ```
///
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(
    since = "0.11.0",
    note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead"
)]
/// Configuration for Safety
pub struct SafetyConfig {
    /// Require Confirmation
    pub require_confirmation: bool,
    /// Backup Existing
    pub backup_existing: bool,
    /// Dry Run First
    pub dry_run_first: bool,
}

impl Default for SafetyConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            require_confirmation: true,
            backup_existing: true,
            dry_run_first: true,
        }
    }
}

/// Cache threshold configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Cachethresholds
pub struct CacheThresholds {
    /// L1Arc Max
    pub l1arc_max: u64,
    /// L2Arc Max
    pub l2arc_max: u64,
    /// Metadata Ratio
    pub metadata_ratio: f64,
}

impl Default for CacheThresholds {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            l1arc_max: 8 * 1024 * 1024 * 1024,  // 8GB
            l2arc_max: 32 * 1024 * 1024 * 1024, // 32GB
            metadata_ratio: 0.25,
        }
    }
}

/// I/O threshold configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Iothresholds
pub struct IoThresholds {
    /// Queue Depth
    pub queue_depth: u32,
    /// Timeout Seconds
    pub timeout_seconds: u32,
    /// Count of retry
    pub retry_count: u32,
}

impl Default for IoThresholds {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            queue_depth: 32,
            timeout_seconds: 30,
            retry_count: 3,
        }
    }
}

/// Memory limit configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Memorylimits
pub struct MemoryLimits {
    /// Arc Max
    pub arc_max: u64,
    /// Arc Min
    pub arc_min: u64,
    /// Kmem Max
    pub kmem_max: u64,
}

impl Default for MemoryLimits {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            arc_max: 16 * 1024 * 1024 * 1024, // 16GB
            arc_min: 1024 * 1024 * 1024,      // 1GB
            kmem_max: 4 * 1024 * 1024 * 1024, // 4GB
        }
    }
}

/// Tier properties for ZFS datasets
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Tierproperties
pub struct TierProperties {
    /// Compression
    pub compression: String,
    /// Recordsize
    pub recordsize: String,
    /// Primarycache
    pub primarycache: String,
    /// Secondarycache
    pub secondarycache: String,
    /// Logbias
    pub logbias: String,
    /// Sync
    pub sync: String,
    /// Atime
    pub atime: String,
}

impl Default for TierProperties {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            compression: "lz4".to_string(),
            recordsize: "128K".to_string(),
            primarycache: "all".to_string(),
            secondarycache: "all".to_string(),
            logbias: "latency".to_string(),
            sync: "standard".to_string(),
            atime: "on".to_string(),
        }
    }
}

/// Complete ZFS configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Zfs
pub struct ZfsConfig {
    /// Pool Properties
    pub pool_properties: PoolPropertyConfig,
    /// Device Detection
    pub device_detection: DeviceDetectionConfig,
    /// Safety
    pub safety: SafetyConfig,
    /// Cache Thresholds
    pub cache_thresholds: CacheThresholds,
    /// Io Thresholds
    pub io_thresholds: IoThresholds,
    /// Memory Limits
    pub memory_limits: MemoryLimits,
    /// Tier Properties
    pub tier_properties: HashMap<StorageTier, TierProperties>,
}

impl Default for ZfsConfig {
    /// Returns the default instance
    fn default() -> Self {
        let mut tier_properties = HashMap::new();

        // Hot tier - optimized for performance
        tier_properties.insert(
            StorageTier::Hot,
            TierProperties {
                compression: "lz4".to_string(),
                recordsize: "128K".to_string(),
                primarycache: "all".to_string(),
                secondarycache: "all".to_string(),
                logbias: "latency".to_string(),
                sync: "standard".to_string(),
                atime: "on".to_string(),
            },
        );

        // Warm tier - balanced performance and compression
        tier_properties.insert(
            StorageTier::Warm,
            TierProperties {
                compression: "gzip-6".to_string(),
                recordsize: "1M".to_string(),
                primarycache: "all".to_string(),
                secondarycache: "all".to_string(),
                logbias: "throughput".to_string(),
                sync: "standard".to_string(),
                atime: "off".to_string(),
            },
        );

        // Cold tier - optimized for storage efficiency
        tier_properties.insert(
            StorageTier::Cold,
            TierProperties {
                compression: "gzip-9".to_string(),
                recordsize: "1M".to_string(),
                primarycache: "metadata".to_string(),
                secondarycache: "none".to_string(),
                logbias: "throughput".to_string(),
                sync: "disabled".to_string(),
                atime: "off".to_string(),
            },
        );

        Self {
            pool_properties: PoolPropertyConfig::default(),
            device_detection: DeviceDetectionConfig::default(),
            safety: SafetyConfig::default(),
            cache_thresholds: CacheThresholds::default(),
            io_thresholds: IoThresholds::default(),
            memory_limits: MemoryLimits::default(),
            tier_properties,
        }
    }
}

// ==================== CANONICAL TYPE ALIAS ====================
// This type now aliases to the canonical network configuration
// Original struct definition kept above for reference and backward compatibility

/// Type alias to canonical network configuration
///
/// This provides backward compatibility while migrating to unified configuration.
/// The original struct is marked as deprecated but still functional.
#[allow(deprecated)]
/// Type alias for Devicedetectionconfigcanonical
pub type DeviceDetectionConfigCanonical =
    nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using DeviceDetectionConfig (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.

// ==================== CANONICAL TYPE ALIAS ====================
// This type now aliases to the canonical network configuration
// Original struct definition kept above for reference and backward compatibility

/// Type alias to canonical network configuration
///
/// This provides backward compatibility while migrating to unified configuration.
/// The original struct is marked as deprecated but still functional.
#[allow(deprecated)]
/// Type alias for Poolpropertyconfigcanonical
pub type PoolPropertyConfigCanonical =
    nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using PoolPropertyConfig (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.

// ==================== CANONICAL TYPE ALIAS ====================
// This type now aliases to the canonical network configuration
// Original struct definition kept above for reference and backward compatibility

/// Type alias to canonical network configuration
///
/// This provides backward compatibility while migrating to unified configuration.
/// The original struct is marked as deprecated but still functional.
#[allow(deprecated)]
/// Type alias for Poolsetupconfigcanonical
pub type PoolSetupConfigCanonical =
    nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using PoolSetupConfig (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.

// ==================== CANONICAL TYPE ALIAS ====================
// This type now aliases to the canonical network configuration
// Original struct definition kept above for reference and backward compatibility

/// Type alias to canonical network configuration
///
/// This provides backward compatibility while migrating to unified configuration.
/// The original struct is marked as deprecated but still functional.
#[allow(deprecated)]
/// Type alias for Safetyconfigcanonical
pub type SafetyConfigCanonical =
    nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using SafetyConfig (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.

//! Enum definitions for the auto-configurator
//!
//! This module contains all enum types used for storage configuration,
//! including ZFS features, redundancy levels, storage use cases, and more.

use serde::{Deserialize, Serialize};

// ==================== ZFS FEATURES ====================

/// ZFS features that can be required
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ZfsFeature {
    /// Compression
    Compression,
    /// Deduplication
    Deduplication,
    /// Snapshots
    Snapshots,
    /// Checksumming
    Checksumming,
    /// Encryption
    Encryption,
    /// RAID-Z
    RaidZ,
}

// ==================== REDUNDANCY ====================

/// Redundancy level options
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RedundancyLevel {
    /// No redundancy
    None,
    /// Mirror (2-way)
    Mirror,
    /// RAID-Z1 (single parity)
    RaidZ1,
    /// RAID-Z2 (double parity)
    RaidZ2,
    /// RAID-Z3 (triple parity)
    RaidZ3,
}

/// Redundancy strategy options
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum RedundancyStrategy {
    /// No redundancy
    #[default]
    None,
    /// Mirror (2-way)
    Mirror,
    /// RAID-Z1 (single parity)
    RaidZ1,
    /// RAID-Z2 (double parity)
    RaidZ2,
    /// RAID-Z3 (triple parity)
    RaidZ3,
    /// Custom redundancy configuration
    Custom(String),
}

/// Cross-tier redundancy strategy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CrossTierRedundancyStrategy {
    /// No cross-tier redundancy
    None,
}

// ==================== STORAGE TIERS ====================

/// Performance tier classification
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PerformanceTier {
    /// High performance
    High,
    /// Medium performance
    Medium,
    /// Low performance / archival
    Low,
}

// ==================== USE CASES ====================

/// Storage use case categories
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum StorageUseCase {
    /// Home NAS
    HomeNas,
    /// Small business
    SmallBusiness,
    /// Enterprise
    Enterprise,
    /// Cloud-native
    CloudNative,
    /// High performance computing
    HighPerformance,
    /// Archive/backup
    Archive,
    /// Development/testing
    Development,
}

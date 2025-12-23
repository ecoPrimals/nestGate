//! ZFS Configuration Types
//!
//! Configuration types for datasets and snapshots.
//!
//! ⚠️ **Deprecation Notice**: `DatasetConfig` and `SnapshotConfig` are deprecated.
//! They are widely used (120+ references across 31 files) and require a coordinated
//! codebase-wide migration. Use these types for now, but plan migration to canonical
//! configuration types.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Dataset configuration
///
/// ⚠️ **DEPRECATED**: Use canonical configuration types instead.
/// This type is still functional but marked for future migration.
#[deprecated(
    since = "0.1.0",
    note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead"
)]
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Dataset
pub struct DatasetConfig {
    /// Dataset name
    pub name: String,
    /// Mount point
    pub mountpoint: Option<String>,
    /// Compression enabled
    pub compression: bool,
    /// Quota in bytes
    pub quota: Option<u64>,
    /// Reservation in bytes
    pub reservation: Option<u64>,
    /// Additional properties
    pub properties: HashMap<String, String>,
}

/// Snapshot configuration
///
/// ⚠️ **DEPRECATED**: Use canonical configuration types instead.
/// This type is still functional but marked for future migration.
#[deprecated(
    since = "0.1.0",
    note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead"
)]
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Snapshot
pub struct SnapshotConfig {
    /// Snapshot name (e.g., "pool/dataset@snapshot-name")
    pub name: String,
    /// Dataset name (e.g., "pool/dataset")
    #[serde(default)]
    /// Dataset
    pub dataset: String,
    /// Properties to set
    pub properties: HashMap<String, String>,
}

// ==================== CANONICAL TYPE ALIASES ====================

/// Type alias to canonical network configuration
///
/// This provides backward compatibility while migrating to unified configuration.
/// The original struct is marked as deprecated but still functional.
#[allow(deprecated)]
/// Type alias for Datasetconfigcanonical
pub type DatasetConfigCanonical =
    nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;

/// Type alias to canonical network configuration
///
/// This provides backward compatibility while migrating to unified configuration.
/// The original struct is marked as deprecated but still functional.
#[allow(deprecated)]
/// Type alias for Snapshotconfigcanonical
pub type SnapshotConfigCanonical =
    nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;

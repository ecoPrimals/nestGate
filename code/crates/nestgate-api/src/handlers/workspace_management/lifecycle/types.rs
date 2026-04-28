// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

use serde::{Deserialize, Serialize};

/// Backup configuration for workspace operations
#[derive(Debug, Clone, Serialize, Deserialize)]
/// DEPRECATED: This config has been consolidated into `canonical_primary`
///
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::config::BackupConfig;
///
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// ```
///
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
/// Configuration for Backup
pub struct BackupConfig {
    /// Backup name/identifier
    pub backup_name: String,
    /// Include snapshots in backup
    pub include_snapshots: bool,
    /// Compression level (0-9)
    pub compression_level: u8,
    /// Encryption enabled
    pub encryption_enabled: bool,
    /// Description of the backup
    pub description: Option<String>,
}

/// Restore configuration for workspace operations
#[derive(Debug, Clone, Serialize, Deserialize)]
/// DEPRECATED: This config has been consolidated into `canonical_primary`
///
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::network::config::RestoreConfig;
///
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::network::config::RestoreConfig; // Now aliases to CanonicalNetworkConfig
/// ```
///
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
/// Configuration for Restore
pub struct RestoreConfig {
    /// Backup to restore from
    pub backup_name: String,
    /// Target workspace ID (if different from source)
    pub target_workspace_id: Option<String>,
    /// Restore point in time (snapshot name)
    pub restore_point: Option<String>,
    /// Force restore even if target exists
    pub force: bool,
}

/// Migration configuration for workspace operations
#[derive(Debug, Clone, Serialize, Deserialize)]
/// DEPRECATED: This config has been consolidated into `canonical_primary`
///
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::config::MigrationConfig;
///
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// ```
///
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
/// Configuration for Migration
pub struct MigrationConfig {
    /// Target pool for migration
    pub target_pool: String,
    /// Target host for remote migration
    pub target_host: Option<String>,
    /// Migration strategy
    pub strategy: MigrationStrategy,
    /// Bandwidth limit in bytes per second
    pub bandwidth_limit: Option<u64>,
}

/// Migration strategy options
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
/// Migrationstrategy
pub enum MigrationStrategy {
    /// Copy data to new location, keep original
    Copy,
    /// Move data to new location, remove original
    Move,
    /// Create incremental replica
    Replicate,
}

/// Type alias to canonical network configuration
///
/// This provides backward compatibility while migrating to unified configuration.
/// The original struct is marked as deprecated but still functional.
/// Type alias for Restoreconfigcanonical
pub type RestoreConfigCanonical =
    nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;

/// Type alias to canonical network configuration
///
/// This provides backward compatibility while migrating to unified configuration.
/// The original struct is marked as deprecated but still functional.
/// Type alias for Backupconfigcanonical
pub type BackupConfigCanonical =
    nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;

/// Type alias to canonical network configuration
///
/// This provides backward compatibility while migrating to unified configuration.
/// The original struct is marked as deprecated but still functional.
/// Type alias for Migrationconfigcanonical
pub type MigrationConfigCanonical =
    nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;

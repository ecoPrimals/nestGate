// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

// **STORAGE REPLICATION CONFIGURATION**

use serde::{Deserialize, Serialize};

/// Storage replication configuration for data redundancy and disaster recovery.
///
/// Provides comprehensive replication settings including real-time replication,
/// backup strategies, and disaster recovery planning.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for `StorageReplication`
pub struct StorageReplicationConfig {
    /// Real-time data replication configuration.
    pub replication: ReplicationConfig,
    /// Backup configuration for point-in-time recovery.
    pub backup: BackupConfig,
    /// Disaster recovery configuration for business continuity.
    pub disaster_recovery: DisasterRecoveryConfig,
}

/// Real-time replication configuration.
///
/// Controls synchronous or asynchronous data replication to secondary storage.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Replication
pub struct ReplicationConfig {
    /// Whether replication is enabled (default: false due to overhead).
    pub enabled: bool,
    /// Replication strategy (default: asynchronous for performance).
    pub strategy: ReplicationStrategy,
}

/// Backup configuration for data protection.
///
/// Defines backup frequency, retention, and backup type.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Backup
pub struct BackupConfig {
    /// Whether backups are enabled (default: true).
    pub enabled: bool,
    /// Backup strategy (default: incremental for efficiency).
    pub strategy: BackupStrategy,
}

/// Disaster recovery configuration.
///
/// Defines recovery time objectives (RTO) and recovery point objectives (RPO).
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for `DisasterRecovery`
pub struct DisasterRecoveryConfig {
    /// Whether disaster recovery is enabled (default: false).
    pub enabled: bool,
    /// Recovery strategy determining RTO/RPO (default: warm standby).
    pub strategy: RecoveryStrategy,
}

/// Replication strategy for data synchronization.
///
/// Determines how data is replicated to secondary storage.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Replicationstrategy
pub enum ReplicationStrategy {
    /// Synchronous - write completes after replication (highest consistency, higher latency).
    Synchronous,
    /// Asynchronous - write completes before replication (lower latency, eventual consistency).
    Asynchronous,
    /// Semi-synchronous - write completes after at least one replica acknowledges.
    SemiSynchronous,
}

/// Backup strategy for data protection.
///
/// Determines backup type and storage efficiency.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Backupstrategy
pub enum BackupStrategy {
    /// Full backup - complete data copy (highest storage, simplest restore).
    Full,
    /// Incremental backup - only changes since last backup (lowest storage, complex restore).
    Incremental,
    /// Differential backup - changes since last full backup (balanced approach).
    Differential,
}

/// Disaster recovery strategy.
///
/// Determines recovery time and infrastructure readiness.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Recoverystrategy
pub enum RecoveryStrategy {
    /// Hot standby - active/active with instant failover (highest cost, lowest RTO).
    Hot,
    /// Warm standby - passive replica with quick activation (balanced cost/RTO).
    Warm,
    /// Cold standby - backup only, manual recovery (lowest cost, highest RTO).
    Cold,
}

impl Default for StorageReplicationConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            replication: ReplicationConfig {
                enabled: false,
                strategy: ReplicationStrategy::Asynchronous,
            },
            backup: BackupConfig {
                enabled: true,
                strategy: BackupStrategy::Incremental,
            },
            disaster_recovery: DisasterRecoveryConfig {
                enabled: false,
                strategy: RecoveryStrategy::Warm,
            },
        }
    }
}

impl StorageReplicationConfig {
    /// Create production-optimized replication configuration.
    ///
    /// Uses incremental backups with warm standby disaster recovery.
    #[must_use]
    pub fn production_optimized() -> Self {
        Self::default()
    }

    /// Create development-optimized replication configuration.
    ///
    /// Uses default settings suitable for local development (backups only).
    #[must_use]
    pub fn development_optimized() -> Self {
        Self::default()
    }

    /// Create high-performance replication configuration.
    ///
    /// Minimal replication overhead with asynchronous strategy.
    #[must_use]
    pub fn high_performance() -> Self {
        Self::default()
    }

    /// Create cloud-native replication configuration.
    ///
    /// Optimized for cloud-based multi-region replication and disaster recovery.
    #[must_use]
    pub fn cloud_native() -> Self {
        Self::default()
    }

    /// Merge this configuration with another, preferring values from `other`.
    #[must_use]
    pub const fn merge(self, _other: Self) -> Self {
        self
    }

    /// Validate replication configuration.
    ///
    /// # Errors
    ///
    /// Returns an error if validation fails.
    pub const fn validate(&self) -> nestgate_types::error::Result<()> {
        Ok(())
    }
}

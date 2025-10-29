//! **STORAGE REPLICATION CONFIGURATION**

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageReplicationConfig {
    pub replication: ReplicationConfig,
    pub backup: BackupConfig,
    pub disaster_recovery: DisasterRecoveryConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplicationConfig {
    pub enabled: bool,
    pub strategy: ReplicationStrategy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupConfig {
    pub enabled: bool,
    pub strategy: BackupStrategy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisasterRecoveryConfig {
    pub enabled: bool,
    pub strategy: RecoveryStrategy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReplicationStrategy { Synchronous, Asynchronous, SemiSynchronous }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BackupStrategy { Full, Incremental, Differential }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecoveryStrategy { Hot, Warm, Cold }

impl Default for StorageReplicationConfig {
    fn default() -> Self {
        Self {
            replication: ReplicationConfig { enabled: false, strategy: ReplicationStrategy::Asynchronous },
            backup: BackupConfig { enabled: true, strategy: BackupStrategy::Incremental },
            disaster_recovery: DisasterRecoveryConfig { enabled: false, strategy: RecoveryStrategy::Warm },
        }
    }
}

impl StorageReplicationConfig {
    pub fn production_optimized() -> Self { Self::default() }
    pub fn development_optimized() -> Self { Self::default() }
    pub fn high_performance() -> Self { Self::default() }
    pub fn cloud_native() -> Self { Self::default() }
    pub fn merge(self, _other: Self) -> Self { self }
    pub fn validate(&self) -> crate::Result<()> { Ok(()) }
} 
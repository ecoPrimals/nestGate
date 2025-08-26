//
// This module defines the core trait for enterprise storage capabilities,
// building on top of canonical storage and zero-copy storage traits.

// Removed unused Future import

use super::{
    analytics::{DeduplicationReport, DetailedMetrics, OptimizationReport},
    replication::{BackupManifest, ReplicationJob, StorageTarget},
    snapshots::SnapshotInfo,
    tiering::TieringReport,
};
use crate::error::CanonicalResult as Result;
use crate::universal_storage::{
    canonical_storage::CanonicalStorageBackend, zero_copy::ZeroCopyStorage,
};
// REMOVED: async_trait - using zero-cost native async patterns

/// Enterprise storage capabilities trait - **ZERO-COST NATIVE ASYNC**
///
/// This trait extends the canonical storage backend with enterprise-grade features
/// including snapshots, replication, analytics, and tiering.
/// **CANONICAL MODERNIZATION**: Zero-cost native async patterns
pub trait EnterpriseStorageCapabilities: CanonicalStorageBackend + ZeroCopyStorage {
    /// Create a snapshot of the current storage state
    fn create_snapshot(&self, name: &str, description: Option<&str>) -> impl std::future::Future<Output = Result<SnapshotInfo>> + Send;

    /// List all available snapshots
    fn list_snapshots(&self) -> impl std::future::Future<Output = Result<Vec<SnapshotInfo>>> + Send;

    /// Get detailed information about a specific snapshot
    fn get_snapshot_info(&self, snapshot_id: &str) -> impl std::future::Future<Output = Result<SnapshotInfo>> + Send;

    /// Restore storage state from a snapshot
    fn restore_snapshot(&self, snapshot_id: &str) -> impl std::future::Future<Output = Result<()>> + Send;

    /// Delete a snapshot
    fn delete_snapshot(&self, snapshot_id: &str) -> impl std::future::Future<Output = Result<()>> + Send;

    /// Start replication to a target
    fn replicate_to(
        &self,
        snapshot_id: &str,
        target: &StorageTarget,
    ) -> impl std::future::Future<Output = Result<ReplicationJob>> + Send;

    /// Get replication job status
    fn get_replication_status(&self, job_id: &str) -> impl std::future::Future<Output = Result<ReplicationJob>> + Send;

    /// Cancel a replication job
    fn cancel_replication(&self, job_id: &str) -> impl std::future::Future<Output = Result<()>> + Send;

    /// Create an incremental backup
    fn backup_incremental(&self, base_snapshot_id: Option<&str>) -> impl std::future::Future<Output = Result<BackupManifest>> + Send;

    /// Restore from a backup manifest
    fn restore_from_backup(&self, manifest: &BackupManifest) -> impl std::future::Future<Output = Result<()>> + Send;

    /// Get detailed performance metrics
    fn get_performance_metrics(&self) -> impl std::future::Future<Output = Result<DetailedMetrics>> + Send;

    /// Generate optimization recommendations
    fn optimize_layout(&self) -> impl std::future::Future<Output = Result<OptimizationReport>> + Send;

    /// Perform deduplication analysis
    fn analyze_deduplication(&self) -> impl std::future::Future<Output = Result<DeduplicationReport>> + Send;

    /// Execute storage tiering optimization
    fn optimize_tiering(&self) -> impl std::future::Future<Output = Result<TieringReport>> + Send;
}

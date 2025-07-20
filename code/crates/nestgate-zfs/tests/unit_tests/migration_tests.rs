//! Migration Unit Tests
//!
//! Tests for migration functionality

use std::collections::HashMap;
use std::path::PathBuf;
use std::time::SystemTime;

use nestgate_core::StorageTier as CoreStorageTier;
use nestgate_zfs::performance::TierMetrics;
use nestgate_zfs::performance::{AlertCondition, AlertMetric, AlertOperator, AlertSeverity};
use nestgate_zfs::{
    automation::{DatasetLifecycle, LifecycleRule, LifecycleStage},
    config::ZfsConfig,
    migration::{MigrationJob, MigrationPriority, MigrationStatus},
    snapshot::*,
    types::StorageTier,
};

#[cfg(test)]
mod migration_unit_tests {
    use super::*;

    #[test]
    fn test_migration_job_lifecycle() {
        let job = MigrationJob::new(
            PathBuf::from("/test/dataset"),
            StorageTier::Hot,
            StorageTier::Warm,
            MigrationPriority::Normal,
            1024 * 1024, // 1MB
        );

        assert!(!job.id.is_empty());
        assert_eq!(job.source_path, PathBuf::from("/test/dataset"));
        assert!(matches!(job.source_tier, StorageTier::Hot));
        assert!(matches!(job.target_tier, StorageTier::Warm));
        assert!(matches!(job.status, MigrationStatus::Queued));
    }

    #[test]
    fn test_migration_priority_ordering() {
        // Higher priority values should have higher precedence
        assert!(MigrationPriority::Critical as u32 > MigrationPriority::High as u32);
        assert!(MigrationPriority::High as u32 > MigrationPriority::Normal as u32);
        assert!(MigrationPriority::Normal as u32 > MigrationPriority::Low as u32);
    }

    #[test]
    fn test_migration_config_validation() {
        let config = nestgate_zfs::migration::MigrationConfig::default();

        assert!(config.max_concurrent_migrations > 0);
        assert!(config.total_bandwidth_limit > 0);
        assert!(config.max_bandwidth_per_migration > 0);
        assert!(config.batch_size > 0);
    }
} 
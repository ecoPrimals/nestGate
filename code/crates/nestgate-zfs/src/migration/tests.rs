//
// Contains comprehensive unit tests for the migration system including
// job creation, configuration, and core functionality testing.

#[cfg(test)]
mod test_suite {
    use super::super::types::*;
    use crate::types::StorageTier;
    use std::path::PathBuf;

    #[tokio::test]
    async fn test_migration_job_creation() {
        let source_path = PathBuf::from("/test/file.txt");
        let job = MigrationJob::new(
            source_path.clone(),
            StorageTier::Hot,
            StorageTier::Warm,
            MigrationPriority::Normal,
            1024,
        );

        assert_eq!(job.source_path, source_path);
        assert_eq!(job.source_tier, StorageTier::Hot);
        assert_eq!(job.target_tier, StorageTier::Warm);
        assert_eq!(job.priority, MigrationPriority::Normal);
        assert_eq!(job.file_size, 1024);
        assert_eq!(job.status, MigrationStatus::Queued);
        assert_eq!(job.progress, 0.0);
    }

    #[tokio::test]
    async fn test_migration_config_default() {
        let config = MigrationConfig::default();

        assert_eq!(config.max_concurrent_migrations, 3);
        assert_eq!(config.max_bandwidth_per_migration, 100 * 1024 * 1024);
        assert_eq!(config.total_bandwidth_limit, 200 * 1024 * 1024);
        assert_eq!(config.allowed_hours.len(), 24);
        assert_eq!(config.performance_impact_threshold, 0.05);
        assert_eq!(config.min_free_space_percent, 10.0);
    }

    #[tokio::test]
    async fn test_migration_priority_ordering() {
        assert!(MigrationPriority::Critical > MigrationPriority::High);
        assert!(MigrationPriority::High > MigrationPriority::Normal);
        assert!(MigrationPriority::Normal > MigrationPriority::Low);
    }

    #[tokio::test]
    async fn test_migration_status_transitions() {
        let mut job = MigrationJob::new(
            PathBuf::from("/test/file.txt"),
            StorageTier::Hot,
            StorageTier::Warm,
            MigrationPriority::Normal,
            1024,
        );

        assert_eq!(job.status, MigrationStatus::Queued);

        job.status = MigrationStatus::Running;
        assert_eq!(job.status, MigrationStatus::Running);

        job.status = MigrationStatus::Completed;
        assert_eq!(job.status, MigrationStatus::Completed);
    }

    #[tokio::test]
    async fn test_migration_statistics_default() {
        let stats = MigrationStatistics::default();

        assert_eq!(stats.total_jobs, 0);
        assert_eq!(stats.successful_migrations, 0);
        assert_eq!(stats.failed_migrations, 0);
        assert_eq!(stats.total_bytes_migrated, 0);
        assert_eq!(stats.average_migration_time, 0.0);
        assert_eq!(stats.average_transfer_rate, 0.0);
        assert_eq!(stats.active_migrations, 0);
        assert_eq!(stats.queued_migrations, 0);
        assert_eq!(stats.success_rate, 1.0);
    }

    #[tokio::test]
    async fn test_optimal_priority_determination() {
        // Moving to hot tier should be high priority
        let priority = MigrationJob::get_optimal_priority(&StorageTier::Warm, &StorageTier::Hot);
        assert_eq!(priority, MigrationPriority::High);

        // Moving to cold tier should be low priority
        let priority = MigrationJob::get_optimal_priority(&StorageTier::Warm, &StorageTier::Cold);
        assert_eq!(priority, MigrationPriority::Low);

        // Cache operations should be critical
        let priority = MigrationJob::get_optimal_priority(&StorageTier::Cache, &StorageTier::Hot);
        assert_eq!(priority, MigrationPriority::Critical);

        // Freeing up hot storage should be high priority
        let priority = MigrationJob::get_optimal_priority(&StorageTier::Hot, &StorageTier::Cold);
        assert_eq!(priority, MigrationPriority::High);
    }
}

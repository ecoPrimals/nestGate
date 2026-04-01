// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

// **MANAGER MODULE TESTS**
//
// Comprehensive tests for the ZFS manager functionality

#[cfg(test)]
/// Tests module
mod manager_tests {
    use super::super::*;
    use std::collections::HashMap;

    // Helper to get test endpoint URL
    /// Helper function to get test endpoint (environment-driven)
    ///
    /// Uses NESTGATE_TEST_PORT environment variable, defaults to 18080
    fn test_endpoint() -> String {
        let port = std::env::var("NESTGATE_TEST_PORT").unwrap_or_else(|_| "18080".to_string());
        format!("http://localhost:{}", port)
    }

    /// Helper function to get test health endpoint
    fn test_health_endpoint() -> String {
        format!("{}/health", test_endpoint())
    }

    // ==================== SERVICE INFO TESTS ====================

    #[test]
    fn test_service_info_creation() {
        let mut metadata = HashMap::new();
        metadata.insert("region".to_string(), "us-west-1".to_string());

        let service_info = ServiceInfo {
            name: "test-zfs-service".to_string(),
            endpoint: test_endpoint(),
            health_endpoint: test_health_endpoint(),
            capabilities: vec!["pool-management".to_string(), "snapshots".to_string()],
            metadata,
            ai_capabilities: vec!["tier-optimization".to_string()],
            monitoring_features: vec!["real-time-metrics".to_string()],
        };

        assert_eq!(service_info.name, "test-zfs-service");
        assert_eq!(service_info.capabilities.len(), 2);
        assert_eq!(service_info.ai_capabilities.len(), 1);
        assert_eq!(service_info.monitoring_features.len(), 1);
    }

    #[test]
    fn test_service_info_empty_capabilities() {
        let service_info = ServiceInfo {
            name: "minimal-service".to_string(),
            endpoint: test_endpoint(),
            health_endpoint: test_health_endpoint(),
            capabilities: vec![],
            metadata: HashMap::new(),
            ai_capabilities: vec![],
            monitoring_features: vec![],
        };

        assert!(service_info.capabilities.is_empty());
        assert!(service_info.ai_capabilities.is_empty());
        assert!(service_info.monitoring_features.is_empty());
    }

    // ==================== TIER BENEFITS TESTS ====================

    #[test]
    fn test_tier_benefits_calculation() {
        let benefits = TierBenefits {
            performance_improvement: 25.5,
            cost_savings: 15.0,
            storage_efficiency: 30.0,
        };

        assert_eq!(benefits.performance_improvement, 25.5);
        assert_eq!(benefits.cost_savings, 15.0);
        assert_eq!(benefits.storage_efficiency, 30.0);
    }

    #[test]
    fn test_tier_benefits_zero_values() {
        let benefits = TierBenefits {
            performance_improvement: 0.0,
            cost_savings: 0.0,
            storage_efficiency: 0.0,
        };

        assert_eq!(benefits.performance_improvement, 0.0);
        assert_eq!(benefits.cost_savings, 0.0);
    }

    #[test]
    fn test_tier_benefits_negative_values() {
        // In some cases, tier migration might have negative impact
        let benefits = TierBenefits {
            performance_improvement: -10.0,
            cost_savings: 5.0,
            storage_efficiency: -2.0,
        };

        assert!(benefits.performance_improvement < 0.0);
        assert!(benefits.cost_savings > 0.0);
    }

    // ==================== FILE ANALYSIS DATA TESTS ====================

    #[test]
    fn test_file_analysis_data_creation() {
        let analysis = FileAnalysisData {
            file_size: 1024 * 1024, // 1MB
            file_type: "application/pdf".to_string(),
            access_frequency: "daily".to_string(),
            last_accessed: 1234567890,
            last_modified: 1234567800,
            is_system_critical: false,
            is_frequently_accessed_dir: true,
            estimated_access_pattern: "read-heavy".to_string(),
        };

        assert_eq!(analysis.file_size, 1024 * 1024);
        assert_eq!(analysis.file_type, "application/pdf");
        assert_eq!(analysis.access_frequency, "daily");
        assert!(!analysis.is_system_critical);
        assert!(analysis.is_frequently_accessed_dir);
    }

    #[test]
    fn test_file_analysis_system_critical() {
        let analysis = FileAnalysisData {
            file_size: 4096,
            file_type: "system/config".to_string(),
            access_frequency: "constant".to_string(),
            last_accessed: 9999999999,
            last_modified: 9999999999,
            is_system_critical: true,
            is_frequently_accessed_dir: false,
            estimated_access_pattern: "read-write".to_string(),
        };

        assert!(analysis.is_system_critical);
        assert_eq!(analysis.access_frequency, "constant");
    }

    #[test]
    fn test_file_analysis_large_file() {
        let analysis = FileAnalysisData {
            file_size: 10 * 1024 * 1024 * 1024, // 10GB
            file_type: "video/mp4".to_string(),
            access_frequency: "monthly".to_string(),
            last_accessed: 1000000000,
            last_modified: 1000000000,
            is_system_critical: false,
            is_frequently_accessed_dir: false,
            estimated_access_pattern: "read-only".to_string(),
        };

        assert_eq!(analysis.file_size, 10 * 1024 * 1024 * 1024);
        assert_eq!(analysis.estimated_access_pattern, "read-only");
    }

    // ==================== CURRENT METRICS TESTS ====================

    #[test]
    fn test_current_metrics_default() {
        let metrics = CurrentMetrics::default();

        assert_eq!(metrics.operations_per_second, 0.0);
        assert_eq!(metrics.throughput_bytes_per_second, 0);
        assert_eq!(metrics.average_latency_ms, 0.0);
        assert_eq!(metrics.error_rate, 0.0);
    }

    #[test]
    fn test_current_metrics_normal_operation() {
        let metrics = CurrentMetrics {
            operations_per_second: 1000.5,
            throughput_bytes_per_second: 1024 * 1024 * 100, // 100 MB/s
            average_latency_ms: 5.2,
            error_rate: 0.001, // 0.1% error rate
        };

        assert_eq!(metrics.operations_per_second, 1000.5);
        assert_eq!(metrics.throughput_bytes_per_second, 1024 * 1024 * 100);
        assert_eq!(metrics.average_latency_ms, 5.2);
        assert_eq!(metrics.error_rate, 0.001);
    }

    #[test]
    fn test_current_metrics_high_load() {
        let metrics = CurrentMetrics {
            operations_per_second: 50000.0,
            throughput_bytes_per_second: 1024 * 1024 * 1024, // 1 GB/s
            average_latency_ms: 25.8,
            error_rate: 0.05, // 5% error rate under stress
        };

        assert!(metrics.operations_per_second > 10000.0);
        assert!(metrics.error_rate > 0.01);
    }

    #[test]
    fn test_current_metrics_idle_state() {
        let metrics = CurrentMetrics {
            operations_per_second: 0.5,
            throughput_bytes_per_second: 1024, // 1 KB/s
            average_latency_ms: 0.1,
            error_rate: 0.0,
        };

        assert!(metrics.operations_per_second < 1.0);
        assert_eq!(metrics.error_rate, 0.0);
    }

    // ==================== AI INTEGRATION STATUS TESTS ====================

    #[test]
    fn test_ai_integration_status_enabled() {
        use std::time::SystemTime;

        let status = AiIntegrationStatus {
            enabled: true,
            models_deployed: 3,
            optimization_active: true,
            last_optimization: SystemTime::now(),
            prediction_accuracy: 0.92,
        };

        assert!(status.enabled);
        assert_eq!(status.models_deployed, 3);
        assert!(status.optimization_active);
        assert_eq!(status.prediction_accuracy, 0.92);
    }

    #[test]
    fn test_ai_integration_status_disabled() {
        use std::time::UNIX_EPOCH;

        let status = AiIntegrationStatus {
            enabled: false,
            models_deployed: 0,
            optimization_active: false,
            last_optimization: UNIX_EPOCH,
            prediction_accuracy: 0.0,
        };

        assert!(!status.enabled);
        assert_eq!(status.models_deployed, 0);
        assert!(!status.optimization_active);
    }

    // ==================== POOL OVERALL STATUS TESTS ====================

    #[test]
    fn test_pool_status_healthy() {
        let status = PoolOverallStatus {
            pools_online: 3,
            pools_degraded: 0,
            total_capacity: 10 * 1024 * 1024 * 1024 * 1024, // 10 TB
            available_capacity: 5 * 1024 * 1024 * 1024 * 1024, // 5 TB
        };

        assert_eq!(status.pools_online, 3);
        assert_eq!(status.pools_degraded, 0);
        assert!(status.available_capacity > 0);
    }

    #[test]
    fn test_pool_status_degraded() {
        let status = PoolOverallStatus {
            pools_online: 3,
            pools_degraded: 2,
            total_capacity: 20 * 1024 * 1024 * 1024 * 1024,
            available_capacity: 5 * 1024 * 1024 * 1024 * 1024,
        };

        assert_eq!(status.pools_degraded, 2);
        assert_eq!(status.pools_online, 3);
    }

    #[test]
    fn test_pool_status_full() {
        let status = PoolOverallStatus {
            pools_online: 2,
            pools_degraded: 0,
            total_capacity: 5 * 1024 * 1024 * 1024 * 1024,
            available_capacity: 100_000_000_000, // Only 100 GB free
        };

        assert!(status.available_capacity < status.total_capacity / 50); // Less than 2% free
    }

    // ==================== TIER OVERALL STATUS TESTS ====================

    #[test]
    fn test_tier_status_balanced() {
        let status = TierOverallStatus {
            hot_utilization: 30.0,
            warm_utilization: 50.0,
            cold_utilization: 20.0,
            migration_queue_size: 0,
        };

        assert_eq!(status.hot_utilization, 30.0);
        assert_eq!(status.warm_utilization, 50.0);
        assert_eq!(status.cold_utilization, 20.0);
        assert_eq!(status.migration_queue_size, 0);

        let total = status.hot_utilization + status.warm_utilization + status.cold_utilization;
        assert_eq!(total, 100.0);
    }

    #[test]
    fn test_tier_status_with_migrations() {
        let status = TierOverallStatus {
            hot_utilization: 60.0,
            warm_utilization: 30.0,
            cold_utilization: 10.0,
            migration_queue_size: 12,
        };

        assert_eq!(status.hot_utilization, 60.0);
        assert_eq!(status.migration_queue_size, 12);
    }

    // ==================== MIGRATION STATUS TESTS ====================

    #[test]
    fn test_migration_status_idle() {
        let status = MigrationStatus {
            active_jobs: 0,
            queued_jobs: 0,
            completed_jobs: 15,
            failed_jobs: 0,
            total_bytes_migrated: 1024 * 1024 * 1024 * 50, // 50 GB
        };

        assert_eq!(status.active_jobs, 0);
        assert_eq!(status.completed_jobs, 15);
        assert_eq!(status.failed_jobs, 0);
    }

    #[test]
    fn test_migration_status_active() {
        let status = MigrationStatus {
            active_jobs: 3,
            queued_jobs: 7,
            completed_jobs: 42,
            failed_jobs: 2,
            total_bytes_migrated: 1024 * 1024 * 1024 * 500, // 500 GB
        };

        assert_eq!(status.active_jobs, 3);
        assert_eq!(status.queued_jobs, 7);
        assert!(status.failed_jobs > 0);
    }

    // ==================== SNAPSHOT STATUS TESTS ====================

    #[test]
    fn test_snapshot_status_healthy() {
        let status = SnapshotStatus {
            total_snapshots: 150,
            active_policies: 10,
            pending_operations: 2,
            recent_failures: 0,
        };

        assert_eq!(status.total_snapshots, 150);
        assert_eq!(status.active_policies, 10);
        assert_eq!(status.recent_failures, 0);
    }

    #[test]
    fn test_snapshot_status_with_failures() {
        let status = SnapshotStatus {
            total_snapshots: 500,
            active_policies: 15,
            pending_operations: 20,
            recent_failures: 5,
        };

        assert!(status.recent_failures > 0);
        assert!(status.pending_operations > 0);
    }

    // ==================== HEALTH STATE TESTS ====================

    #[test]
    fn test_health_state_variants() {
        let healthy = HealthState::Healthy;
        let warning = HealthState::Warning;
        let critical = HealthState::Critical;
        let unknown = HealthState::Unknown;

        // Test Debug formatting
        assert_eq!(format!("{:?}", healthy), "Healthy");
        assert_eq!(format!("{:?}", warning), "Warning");
        assert_eq!(format!("{:?}", critical), "Critical");
        assert_eq!(format!("{:?}", unknown), "Unknown");
    }

    // ==================== SERVICE INFO SERIALIZATION TESTS ====================

    #[test]
    fn test_service_info_serialization() {
        use serde_json;

        let service_info = ServiceInfo {
            name: "test-service".to_string(),
            endpoint: test_endpoint(),
            health_endpoint: test_health_endpoint(),
            capabilities: vec!["test".to_string()],
            metadata: HashMap::new(),
            ai_capabilities: vec![],
            monitoring_features: vec![],
        };

        let serialized = serde_json::to_string(&service_info);
        assert!(serialized.is_ok());

        let deserialized: Result<ServiceInfo, _> = serde_json::from_str(&serialized.unwrap());
        assert!(deserialized.is_ok());
        assert_eq!(deserialized.unwrap().name, "test-service");
    }

    // ==================== CURRENT METRICS SERIALIZATION TESTS ====================

    #[test]
    fn test_current_metrics_serialization() {
        use serde_json;

        let metrics = CurrentMetrics {
            operations_per_second: 1000.0,
            throughput_bytes_per_second: 1024 * 1024,
            average_latency_ms: 5.0,
            error_rate: 0.01,
        };

        let serialized = serde_json::to_string(&metrics);
        assert!(serialized.is_ok());

        let deserialized: Result<CurrentMetrics, _> = serde_json::from_str(&serialized.unwrap());
        assert!(deserialized.is_ok());
        let recovered = deserialized.unwrap();
        assert_eq!(recovered.operations_per_second, 1000.0);
    }
}

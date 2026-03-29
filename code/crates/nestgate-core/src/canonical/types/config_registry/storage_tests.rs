// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Unit tests for `config_registry::storage` types (defaults, serde).

use super::storage::{
    ConflictResolutionStrategy, ReplicationSyncMode, StorageBackendType, StorageConnectionConfig,
    StorageMonitoringConfig, StoragePerformanceConfig, StorageReplicationConfig,
    StorageResourceConfig, StorageSecurityConfig,
};
use std::collections::HashMap;
use std::time::Duration;

#[test]
fn storage_backend_type_default_is_filesystem() {
    assert!(matches!(
        StorageBackendType::default(),
        StorageBackendType::Filesystem
    ));
}

#[test]
fn replication_sync_mode_default() {
    assert!(matches!(
        ReplicationSyncMode::default(),
        ReplicationSyncMode::Asynchronous
    ));
}

#[test]
fn conflict_resolution_default() {
    assert!(matches!(
        ConflictResolutionStrategy::default(),
        ConflictResolutionStrategy::LastWriteWins
    ));
}

#[test]
#[expect(deprecated, reason = "legacy storage config types under migration")]
fn serde_roundtrip_storage_connection_config() {
    let c = StorageConnectionConfig {
        connection_string: Some("zfs://pool".to_string()),
        pool_size: 8,
        timeout: Duration::from_secs(12),
        retry_attempts: 2,
        retry_delay: Duration::from_millis(100),
        keep_alive: true,
    };
    let json = serde_json::to_string(&c).expect("serialize");
    let back: StorageConnectionConfig = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(back.pool_size, c.pool_size);
    assert_eq!(back.keep_alive, c.keep_alive);
}

#[test]
#[expect(deprecated, reason = "legacy storage config types under migration")]
fn serde_roundtrip_storage_replication_config() {
    let c = StorageReplicationConfig {
        enabled: true,
        replica_count: 3,
        sync_mode: ReplicationSyncMode::Synchronous,
        conflict_resolution: ConflictResolutionStrategy::Versioned,
        health_check_interval: Duration::from_secs(60),
    };
    let json = serde_json::to_string(&c).expect("serialize");
    let back: StorageReplicationConfig = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(back.replica_count, 3);
    assert!(matches!(back.sync_mode, ReplicationSyncMode::Synchronous));
}

#[test]
fn serde_roundtrip_storage_resource_config() {
    let mut limits = HashMap::new();
    limits.insert("x".to_string(), 42);
    let c = StorageResourceConfig {
        max_memory_mb: Some(512),
        max_disk_gb: None,
        max_connections: 100,
        max_concurrent_operations: 10,
        resource_limits: limits,
    };
    let json = serde_json::to_string(&c).expect("serialize");
    let back: StorageResourceConfig = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(back.max_connections, 100);
    assert_eq!(back.resource_limits.get("x"), Some(&42u64));
}

#[test]
#[expect(deprecated, reason = "legacy storage config types under migration")]
fn serde_roundtrip_storage_performance_and_security() {
    let perf = StoragePerformanceConfig {
        cache_size_mb: 128,
        io_threads: 4,
        batch_size: 32,
        prefetch_enabled: true,
        compression_enabled: false,
        max_iops: Some(1000),
        max_throughput_mbps: Some(500),
        latency_target_ms: Some(5.0),
    };
    let json = serde_json::to_string(&perf).expect("serialize perf");
    let back: StoragePerformanceConfig = serde_json::from_str(&json).expect("deserialize perf");
    assert_eq!(back.io_threads, 4);

    let sec = StorageSecurityConfig {
        encryption_enabled: true,
        encryption_algorithm: "aes-gcm".to_string(),
        key_rotation_interval: Duration::from_secs(3600),
        access_logging: true,
        permission_checks: true,
        secure_deletion: false,
    };
    let json = serde_json::to_string(&sec).expect("serialize sec");
    let back: StorageSecurityConfig = serde_json::from_str(&json).expect("deserialize sec");
    assert!(back.encryption_enabled);
}

#[test]
fn serde_roundtrip_storage_monitoring_config() {
    let mut alerts = HashMap::new();
    alerts.insert("cpu".to_string(), 0.9);
    let c = StorageMonitoringConfig {
        metrics_enabled: true,
        metrics_interval: Duration::from_secs(15),
        alert_thresholds: alerts,
        performance_tracking: true,
        usage_reporting: false,
    };
    let json = serde_json::to_string(&c).expect("serialize");
    let back: StorageMonitoringConfig = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(back.metrics_interval, Duration::from_secs(15));
}

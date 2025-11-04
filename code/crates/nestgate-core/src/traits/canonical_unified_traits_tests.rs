//! Tests for Canonical Unified Traits
//!
//! This module contains comprehensive tests for the canonical trait system.
//! Split from main file to maintain <1000 lines per file compliance.

use super::*;
use std::collections::HashMap;
use std::time::{Duration, SystemTime};

// Note: Some tests reference types that need to be defined for compilation
// These will be fixed as part of the test compilation error resolution

// **SERVICECAPABILITIES TESTS**

#[test]
fn test_service_capabilities_default() {
    let caps = ServiceCapabilities {
        can_scale: true,
        can_migrate: false,
        can_backup: true,
        supported_protocols: vec!["http".to_string(), "grpc".to_string()],
    };

    assert!(caps.can_scale);
    assert!(!caps.can_migrate);
    assert!(caps.can_backup);
    assert_eq!(caps.supported_protocols.len(), 2);
}

#[test]
fn test_service_capabilities_with_custom_capabilities() {
    let caps = ServiceCapabilities {
        can_scale: false,
        can_migrate: true,
        can_backup: false,
        supported_protocols: vec!["tcp".to_string()],
    };

    assert_eq!(caps.supported_protocols.len(), 1);
    assert_eq!(caps.supported_protocols[0], "tcp");
}

// **STORAGE BACKEND TYPE TESTS**

#[test]
fn test_storage_backend_types() {
    let fs_backend = StorageBackendType::FileSystem;
    let mem_backend = StorageBackendType::Memory;
    let zfs_backend = StorageBackendType::Zfs;

    assert!(matches!(fs_backend, StorageBackendType::FileSystem));
    assert!(matches!(mem_backend, StorageBackendType::Memory));
    assert!(matches!(zfs_backend, StorageBackendType::Zfs));
}

#[test]
fn test_storage_backend_custom() {
    let custom = StorageBackendType::Custom("MyStorage".to_string());

    if let StorageBackendType::Custom(name) = custom {
        assert_eq!(name, "MyStorage");
    } else {
        assert!(false, "Expected Custom variant");
    }
}

// **STORAGE CAPABILITY TESTS**

#[test]
fn test_storage_capabilities() {
    let basic = StorageCapability::BasicOperations;
    let batch = StorageCapability::BatchOperations;
    let snapshots = StorageCapability::Snapshots;

    assert!(matches!(basic, StorageCapability::BasicOperations));
    assert!(matches!(batch, StorageCapability::BatchOperations));
    assert!(matches!(snapshots, StorageCapability::Snapshots));
}

// **CONNECTION STATUS TESTS**

#[test]
fn test_connection_status_variants() {
    let active = ConnectionStatus::Active;
    let idle = ConnectionStatus::Idle;
    let closed = ConnectionStatus::Closed;
    let error = ConnectionStatus::Error("connection failed".to_string());

    assert!(matches!(active, ConnectionStatus::Active));
    assert!(matches!(idle, ConnectionStatus::Idle));
    assert!(matches!(closed, ConnectionStatus::Closed));

    if let ConnectionStatus::Error(msg) = error {
        assert_eq!(msg, "connection failed");
    }
}

// **HEALTH STATUS TESTS**

#[test]
fn test_health_status_variants() {
    let healthy = HealthStatus::Healthy;
    let degraded = HealthStatus::Degraded;
    let unhealthy = HealthStatus::Unhealthy;
    let unknown = HealthStatus::Unknown;

    assert!(matches!(healthy, HealthStatus::Healthy));
    assert!(matches!(degraded, HealthStatus::Degraded));
    assert!(matches!(unhealthy, HealthStatus::Unhealthy));
    assert!(matches!(unknown, HealthStatus::Unknown));
}

// **PROVIDER HEALTH TESTS**

#[test]
fn test_provider_health_default() {
    let health = ProviderHealth::default();

    assert!(!health.is_healthy);
    assert!(health.health.is_empty());
}

#[test]
fn test_provider_health_custom() {
    let health = ProviderHealth {
        is_healthy: true,
        last_check: SystemTime::now(),
        health: "All systems operational".to_string(),
    };

    assert!(health.is_healthy);
    assert_eq!(health.health, "All systems operational");
}

// **STORAGE USAGE STATS TESTS**

#[test]
fn test_storage_usage_stats() {
    let stats = StorageUsageStats {
        total_capacity: 1_000_000_000,
        used_space: 500_000_000,
        available_space: 500_000_000,
        item_count: 1000,
        last_updated: SystemTime::now(),
    };

    assert_eq!(stats.total_capacity, 1_000_000_000);
    assert_eq!(stats.used_space, 500_000_000);
    assert_eq!(stats.available_space, 500_000_000);
    assert_eq!(stats.item_count, 1000);
}

// **SNAPSHOT INFO TESTS**

#[test]
fn test_snapshot_info() {
    let snapshot = SnapshotInfo {
        id: "snap-001".to_string(),
        name: "backup-2025".to_string(),
        created_at: SystemTime::now(),
        size: 1024 * 1024 * 100, // 100 MB
    };

    assert_eq!(snapshot.id, "snap-001");
    assert_eq!(snapshot.name, "backup-2025");
    assert_eq!(snapshot.size, 104_857_600);
}

// **CONNECTION HANDLE TESTS**

#[test]
fn test_connection_handle() {
    let handle1 = ConnectionHandle(1);
    let handle2 = ConnectionHandle(2);
    let handle3 = ConnectionHandle(1);

    assert_eq!(handle1, handle3);
    assert_ne!(handle1, handle2);
}

// **SECURITY CREDENTIALS TESTS**

#[test]
fn test_security_credentials() {
    let creds = SecurityCredentials {
        username: "admin".to_string(),
        password: "secret123".to_string(),
    };

    assert_eq!(creds.username, "admin");
    assert_eq!(creds.password, "secret123");
}

// **CRON SCHEDULE TESTS**

#[test]
fn test_cron_schedule() {
    let schedule = CronSchedule {
        expression: "0 0 * * *".to_string(),
    };

    assert_eq!(schedule.expression, "0 0 * * *");
}

// **SCHEDULE INFO TESTS**

#[test]
fn test_schedule_info() {
    let schedule_id = ScheduleId {
        id: "schedule-1".to_string(),
    };

    let schedule = CronSchedule {
        expression: "0 */6 * * *".to_string(),
    };

    let info = ScheduleInfo {
        id: schedule_id,
        schedule,
        next_run: Some(SystemTime::now()),
    };

    assert_eq!(info.id.id, "schedule-1");
    assert_eq!(info.schedule.expression, "0 */6 * * *");
    assert!(info.next_run.is_some());
}

// **DATA STREAM TESTS**

#[test]
fn test_data_stream_construction() {
    let _stream = DataStream {
        _phantom: std::marker::PhantomData,
    };
    // If this compiles, DataStream is constructible
}

// **WRITE STREAM TESTS**

#[test]
fn test_write_stream_construction() {
    let _stream = WriteStream {
        _phantom: std::marker::PhantomData,
    };
    // If this compiles, WriteStream is constructible
}

// **SERIALIZATION/DESERIALIZATION TESTS**

#[test]
fn test_connection_status_serialization() {
    let status = ConnectionStatus::Active;
    let serialized = serde_json::to_string(&status).expect("Test setup failed");
    assert!(serialized.contains("Active"));

    let deserialized: ConnectionStatus = serde_json::from_str(&serialized).expect("Test setup failed");
    assert!(matches!(deserialized, ConnectionStatus::Active));
}

#[test]
fn test_health_status_serialization() {
    let status = HealthStatus::Healthy;
    let serialized = serde_json::to_string(&status).expect("Test setup failed");
    assert!(serialized.contains("Healthy"));

    let deserialized: HealthStatus = serde_json::from_str(&serialized).expect("Test setup failed");
    assert!(matches!(deserialized, HealthStatus::Healthy));
}

#[test]
fn test_service_capabilities_serialization() {
    let caps = ServiceCapabilities {
        can_scale: true,
        can_migrate: false,
        can_backup: true,
        supported_protocols: vec!["http".to_string()],
    };

    let serialized = serde_json::to_string(&caps).expect("Test setup failed");
    let deserialized: ServiceCapabilities = serde_json::from_str(&serialized).expect("Test setup failed");

    assert_eq!(caps.can_scale, deserialized.can_scale);
    assert_eq!(caps.can_migrate, deserialized.can_migrate);
    assert_eq!(caps.can_backup, deserialized.can_backup);
}

#[test]
fn test_storage_backend_type_serialization() {
    let backend = StorageBackendType::Zfs;
    let serialized = serde_json::to_string(&backend).expect("Test setup failed");
    let deserialized: StorageBackendType = serde_json::from_str(&serialized).expect("Test setup failed");

    assert_eq!(backend, deserialized);
}

#[test]
fn test_storage_capability_serialization() {
    let cap = StorageCapability::Snapshots;
    let serialized = serde_json::to_string(&cap).expect("Test setup failed");
    let deserialized: StorageCapability = serde_json::from_str(&serialized).expect("Test setup failed");

    assert_eq!(cap, deserialized);
}

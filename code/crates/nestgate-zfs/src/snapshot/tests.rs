//! Snapshot Management Tests
//!
//! Unit tests for snapshot policies, operations, and management functionality.

use super::operations::SnapshotOperationType;
use super::policy::{RetentionPolicy, ScheduleFrequency, SnapshotPolicy};
use super::types::{SnapshotOperation, SnapshotOperationStatus};
use std::time::SystemTime;

#[test]
fn test_snapshot_policy_default() {
    let policy = SnapshotPolicy::default();

    assert_eq!(policy.name, "default");
    assert!(policy.enabled);
    assert_eq!(policy.priority, 50);
    assert_eq!(policy.max_snapshots_per_run, 100);
}

#[test]
fn test_retention_policy_default() {
    let retention = RetentionPolicy::default();

    if let RetentionPolicy::Custom {
        hourly_hours,
        daily_days,
        weekly_weeks,
        monthly_months,
        yearly_years,
    } = retention
    {
        assert_eq!(hourly_hours, 24);
        assert_eq!(daily_days, 30);
        assert_eq!(weekly_weeks, 12);
        assert_eq!(monthly_months, 12);
        assert_eq!(yearly_years, 5);
    } else {
        panic!("Invalid retention policy type - expected Custom");
    }
}

#[tokio::test]
async fn test_snapshot_operation_creation() {
    let operation = SnapshotOperation {
        id: "test-123".to_string(),
        operation_type: SnapshotOperationType::Create,
        dataset: "pool/dataset".to_string(),
        snapshot_name: Some("test-snapshot".to_string()),
        status: SnapshotOperationStatus::Queued,
        created_at: SystemTime::now(),
        started_at: None,
        completed_at: None,
        error_message: None,
        policy: Some("test-policy".to_string()),
    };

    assert_eq!(operation.dataset, "pool/dataset");
    assert_eq!(operation.snapshot_name, Some("test-snapshot".to_string()));
    assert_eq!(operation.status, SnapshotOperationStatus::Queued);
}

#[test]
fn test_schedule_frequency_hours() {
    let schedule = ScheduleFrequency::Hours(6);

    match schedule {
        ScheduleFrequency::Hours(hours) => assert_eq!(hours, 6),
        _ => panic!("Expected Hours schedule frequency"),
    }
}

#[test]
fn test_schedule_frequency_daily() {
    let schedule = ScheduleFrequency::Daily(2);

    match schedule {
        ScheduleFrequency::Daily(hour) => assert_eq!(hour, 2),
        _ => panic!("Expected Daily schedule frequency"),
    }
}

#[test]
fn test_operation_types() {
    let create_op = SnapshotOperationType::Create;
    let delete_op = SnapshotOperationType::Delete;
    let clone_op = SnapshotOperationType::Clone;

    // Just verify they can be created and compared
    assert!(matches!(create_op, SnapshotOperationType::Create));
    assert!(matches!(delete_op, SnapshotOperationType::Delete));
    assert!(matches!(clone_op, SnapshotOperationType::Clone));
}

#[test]
fn test_operation_status() {
    let queued = SnapshotOperationStatus::Queued;
    let running = SnapshotOperationStatus::Running;
    let completed = SnapshotOperationStatus::Completed;
    let failed = SnapshotOperationStatus::Failed("error".to_string());
    let cancelled = SnapshotOperationStatus::Cancelled;

    assert!(matches!(queued, SnapshotOperationStatus::Queued));
    assert!(matches!(running, SnapshotOperationStatus::Running));
    assert!(matches!(completed, SnapshotOperationStatus::Completed));
    assert!(matches!(failed, SnapshotOperationStatus::Failed(_)));
    assert!(matches!(cancelled, SnapshotOperationStatus::Cancelled));
}

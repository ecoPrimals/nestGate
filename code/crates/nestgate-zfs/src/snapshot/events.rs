//
// Event logging, policy statistics, and automation status tracking
// for snapshot management operations.

use serde::{Deserialize, Serialize};
use std::time::SystemTime;

/// Snapshot event types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SnapshotEventType {
    Created,
    Deleted,
    PolicyApplied,
    RetentionApplied,
    Error,
}
/// Snapshot event for tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotEvent {
    pub event_id: String,
    pub snapshot_name: String,
    pub event_type: SnapshotEventType,
    pub timestamp: SystemTime,
    pub details: String,
    pub success: bool,
}
/// Policy execution statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyStats {
    pub policy_id: String,
    pub total_snapshots: u64,
    pub successful_snapshots: u64,
    pub failed_snapshots: u64,
    pub last_execution: SystemTime,
    pub total_size_bytes: u64,
}
/// Snapshot automation status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotAutomationStatus {
    pub enabled: bool,
    pub active_policies: u32,
    pub total_snapshots: u64,
    pub recent_failures: u32,
    pub last_automation_run: SystemTime,
}

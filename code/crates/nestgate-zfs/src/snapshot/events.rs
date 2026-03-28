// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//
// Event logging, policy statistics, and automation status tracking
// for snapshot management operations.

use serde::{Deserialize, Serialize};
use std::time::SystemTime;

/// Snapshot event types
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Types of SnapshotEvent
pub enum SnapshotEventType {
    /// Created
    Created,
    /// Deleted
    Deleted,
    /// Policyapplied
    PolicyApplied,
    /// Retentionapplied
    RetentionApplied,
    /// Error
    Error,
}
/// Snapshot event for tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Snapshotevent
pub struct SnapshotEvent {
    /// Event identifier
    pub event_id: String,
    /// Snapshot name
    pub snapshot_name: String,
    /// Event Type
    pub event_type: SnapshotEventType,
    /// Timestamp
    pub timestamp: SystemTime,
    /// Details
    pub details: String,
    /// Success
    pub success: bool,
}
/// Policy execution statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Policystats
pub struct PolicyStats {
    /// Policy identifier
    pub policy_id: String,
    /// Total Snapshots
    pub total_snapshots: u64,
    /// Successful Snapshots
    pub successful_snapshots: u64,
    /// Failed Snapshots
    pub failed_snapshots: u64,
    /// Last Execution
    pub last_execution: SystemTime,
    /// Total Size Bytes
    pub total_size_bytes: u64,
}
/// Snapshot automation status
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Snapshotautomationstatus
pub struct SnapshotAutomationStatus {
    /// Whether this feature is enabled
    pub enabled: bool,
    /// Active Policies
    pub active_policies: u32,
    /// Total Snapshots
    pub total_snapshots: u64,
    /// Recent Failures
    pub recent_failures: u32,
    /// Last Automation Run
    pub last_automation_run: SystemTime,
}

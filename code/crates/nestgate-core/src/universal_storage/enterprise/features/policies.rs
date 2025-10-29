// **STORAGE POLICY MANAGEMENT**
//! Policies functionality and utilities.
// Automated storage policy enforcement and management.

use serde::{Deserialize, Serialize};
use std::time::SystemTime;

/// Storage policy definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoragePolicy {
    pub policy_id: String,
    pub name: String,
    pub policy_type: PolicyType,
    pub conditions: Vec<PolicyCondition>,
    pub actions: Vec<String>,
    pub enabled: bool,
}
/// Types of storage policies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PolicyType {
    DataRetention,
    TierMigration,
    BackupSchedule,
    AccessControl,
    Compliance,
    Performance,
    Cost,
    Security,
    Lifecycle,
    Quota,
    Replication,
    Compression,
    Deduplication,
    Encryption,
    Monitoring,
}
/// Policy condition for automated enforcement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyCondition {
    pub field: String,
    pub operator: ComparisonOperator,
    pub value: String,
    pub logical_operator: Option<LogicalOperator>,
}
/// Comparison operators for policy conditions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComparisonOperator {
    Equals,
    NotEquals,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
    Contains,
    StartsWith,
    EndsWith,
    Matches,
}
/// Logical operators for combining conditions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogicalOperator {
    And,
    Or,
    Not,
}
/// Policy enforcement report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyReport {
    pub report_id: String,
    pub timestamp: SystemTime,
    pub policies_evaluated: u32,
    pub policies_triggered: u32,
    pub actions_executed: u32,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
} 
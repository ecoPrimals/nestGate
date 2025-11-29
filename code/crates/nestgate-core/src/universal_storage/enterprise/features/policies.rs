// **STORAGE POLICY MANAGEMENT**
//! Policies functionality and utilities.
// Automated storage policy enforcement and management.

use serde::{Deserialize, Serialize};
use std::time::SystemTime;

/// Storage policy definition
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Storagepolicy
pub struct StoragePolicy {
    /// Policy identifier
    pub policy_id: String,
    /// Name
    pub name: String,
    /// Policy Type
    pub policy_type: PolicyType,
    /// Conditions
    pub conditions: Vec<PolicyCondition>,
    /// Actions
    pub actions: Vec<String>,
    /// Whether this feature is enabled
    pub enabled: bool,
}
/// Types of storage policies
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Types of Policy
pub enum PolicyType {
    /// Dataretention
    DataRetention,
    /// Tiermigration
    TierMigration,
    /// Backupschedule
    BackupSchedule,
    /// Accesscontrol
    AccessControl,
    /// Compliance
    Compliance,
    /// Performance
    Performance,
    /// Cost
    Cost,
    /// Security
    Security,
    /// Lifecycle
    Lifecycle,
    /// Quota
    Quota,
    Replication,
    Compression,
    Deduplication,
    Encryption,
    Monitoring,
}
/// Policy condition for automated enforcement
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Policycondition
pub struct PolicyCondition {
    /// Field
    pub field: String,
    /// Operator
    pub operator: ComparisonOperator,
    /// Value
    pub value: String,
    /// Logical Operator
    pub logical_operator: Option<LogicalOperator>,
}
/// Comparison operators for policy conditions
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Comparisonoperator
pub enum ComparisonOperator {
    /// Equals
    Equals,
    /// Notequals
    NotEquals,
    /// Greaterthan
    GreaterThan,
    /// Lessthan
    LessThan,
    /// Greaterthanorequal
    GreaterThanOrEqual,
    /// Lessthanorequal
    LessThanOrEqual,
    /// Contains
    Contains,
    /// Startswith
    StartsWith,
    /// Endswith
    EndsWith,
    /// Matches
    Matches,
}
/// Logical operators for combining conditions
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Logicaloperator
pub enum LogicalOperator {
    /// And
    And,
    /// Or
    Or,
    /// Not
    Not,
}
/// Policy enforcement report
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Policyreport
pub struct PolicyReport {
    /// Report identifier
    pub report_id: String,
    /// Timestamp
    pub timestamp: SystemTime,
    /// Policies Evaluated
    pub policies_evaluated: u32,
    /// Policies Triggered
    pub policies_triggered: u32,
    /// Actions Executed
    pub actions_executed: u32,
    /// Errors
    pub errors: Vec<String>,
    /// Warnings
    pub warnings: Vec<String>,
} 
// **DISASTER RECOVERY PLANNING**
//! Disaster Recovery functionality and utilities.
// Automated disaster recovery preparation and management.

use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime};

/// Disaster recovery plan
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisasterRecoveryPlan {
    pub plan_id: String,
    pub created_at: SystemTime,
    pub recovery_strategies: Vec<RecoveryStrategy>,
    pub backup_configurations: Vec<BackupConfiguration>,
    pub estimated_rto: Duration, // Recovery Time Objective
    pub estimated_rpo: Duration, // Recovery Point Objective
    pub test_schedule: TestSchedule,
}
/// Recovery strategy definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecoveryStrategy {
    pub strategy_id: String,
    pub strategy_type: RecoveryType,
    pub priority: u32,
    pub description: String,
    pub prerequisites: Vec<String>,
    pub estimated_time: Duration,
    pub success_criteria: Vec<String>,
}
/// Types of recovery strategies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecoveryType {
    FullRestore,
    IncrementalRestore,
    PointInTimeRecovery,
    GeographicFailover,
    HotStandby,
    ColdStandby,
    CloudFailover,
    HybridRecovery,
}
/// Backup configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupConfiguration {
    pub config_id: String,
    pub backup_type: BackupType,
    pub schedule: BackupSchedule,
    pub retention_policy: RetentionPolicy,
    pub encryption_enabled: bool,
    pub compression_enabled: bool,
    pub target_location: String,
}
/// Types of backups
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BackupType {
    Full,
    Incremental,
    Differential,
    Snapshot,
    Continuous,
}
/// Backup scheduling
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupSchedule {
    pub frequency: BackupFrequency,
    pub time_of_day: String,
    pub days_of_week: Vec<u32>,
    pub timezone: String,
}
/// Backup frequency options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BackupFrequency {
    Hourly,
    Daily,
    Weekly,
    Monthly,
    Custom(Duration),
}
/// Data retention policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetentionPolicy {
    pub daily_retention_days: u32,
    pub weekly_retention_weeks: u32,
    pub monthly_retention_months: u32,
    pub yearly_retention_years: u32,
}
/// Disaster recovery testing schedule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestSchedule {
    pub frequency: TestFrequency,
    pub next_test_date: SystemTime,
    pub last_test_result: Option<TestResult>,
}
/// Test frequency for disaster recovery
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TestFrequency {
    Monthly,
    Quarterly,
    SemiAnnually,
    Annually,
}
/// Disaster recovery test result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestResult {
    pub test_date: SystemTime,
    pub success: bool,
    pub actual_rto: Duration,
    pub actual_rpo: Duration,
    pub issues_found: Vec<String>,
    pub recommendations: Vec<String>,
} 
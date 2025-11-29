// **DISASTER RECOVERY PLANNING**
//! Disaster Recovery functionality and utilities.
// Automated disaster recovery preparation and management.

use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime};

/// Disaster recovery plan
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Disasterrecoveryplan
pub struct DisasterRecoveryPlan {
    /// Plan identifier
    pub plan_id: String,
    /// Timestamp when this was created
    pub created_at: SystemTime,
    /// Recovery Strategies
    pub recovery_strategies: Vec<RecoveryStrategy>,
    /// Configuration for backup urations
    pub backup_configurations: Vec<BackupConfiguration>,
    /// Estimated Rto
    pub estimated_rto: Duration, // Recovery Time Objective
    /// Estimated Rpo
    pub estimated_rpo: Duration, // Recovery Point Objective
    /// Test Schedule
    pub test_schedule: TestSchedule,
}
/// Recovery strategy definition
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Recoverystrategy
pub struct RecoveryStrategy {
    /// Strategy identifier
    pub strategy_id: String,
    /// Strategy Type
    pub strategy_type: RecoveryType,
    /// Priority
    pub priority: u32,
    /// Human-readable description
    pub description: String,
    /// Prerequisites
    pub prerequisites: Vec<String>,
    /// Estimated Time
    pub estimated_time: Duration,
    /// Success Criteria
    pub success_criteria: Vec<String>,
}
/// Types of recovery strategies
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Types of Recovery
pub enum RecoveryType {
    /// Fullrestore
    FullRestore,
    /// Incrementalrestore
    IncrementalRestore,
    /// Pointintimerecovery
    PointInTimeRecovery,
    /// Geographicfailover
    GeographicFailover,
    /// Hotstandby
    HotStandby,
    /// Coldstandby
    ColdStandby,
    /// Cloudfailover
    CloudFailover,
    /// Hybridrecovery
    HybridRecovery,
}
/// Backup configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Backupconfiguration
pub struct BackupConfiguration {
    /// Config identifier
    pub config_id: String,
    /// Backup Type
    pub backup_type: BackupType,
    /// Schedule
    pub schedule: BackupSchedule,
    /// Retention Policy
    pub retention_policy: RetentionPolicy,
    /// Encryption Enabled
    pub encryption_enabled: bool,
    /// Compression Enabled
    pub compression_enabled: bool,
    /// Target Location
    pub target_location: String,
}
/// Types of backups
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Types of Backup
pub enum BackupType {
    /// Full
    Full,
    /// Incremental
    Incremental,
    /// Differential
    Differential,
    /// Snapshot
    Snapshot,
    /// Continuous
    Continuous,
}
/// Backup scheduling
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Backupschedule
pub struct BackupSchedule {
    /// Frequency
    pub frequency: BackupFrequency,
    /// Time Of Day
    pub time_of_day: String,
    /// Days Of Week
    pub days_of_week: Vec<u32>,
    /// Timezone
    pub timezone: String,
}
/// Backup frequency options
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Backupfrequency
pub enum BackupFrequency {
    /// Hourly
    Hourly,
    /// Daily
    Daily,
    /// Weekly
    Weekly,
    /// Monthly
    Monthly,
    Custom(Duration),
}
/// Data retention policy
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Retentionpolicy
pub struct RetentionPolicy {
    /// Daily Retention Days
    pub daily_retention_days: u32,
    /// Weekly Retention Weeks
    pub weekly_retention_weeks: u32,
    /// Monthly Retention Months
    pub monthly_retention_months: u32,
    /// Yearly Retention Years
    pub yearly_retention_years: u32,
}
/// Disaster recovery testing schedule
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Testschedule
pub struct TestSchedule {
    /// Frequency
    pub frequency: TestFrequency,
    /// Next Test Date
    pub next_test_date: SystemTime,
    /// Last Test Result
    pub last_test_result: Option<TestResult>,
}
/// Test frequency for disaster recovery
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Testfrequency
pub enum TestFrequency {
    /// Monthly
    Monthly,
    /// Quarterly
    Quarterly,
    /// Semiannually
    SemiAnnually,
    /// Annually
    Annually,
}
/// Disaster recovery test result
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Testresult
pub struct TestResult {
    /// Test Date
    pub test_date: SystemTime,
    /// Success
    pub success: bool,
    /// Actual Rto
    pub actual_rto: Duration,
    /// Actual Rpo
    pub actual_rpo: Duration,
    /// Issues Found
    pub issues_found: Vec<String>,
    /// Recommendations
    pub recommendations: Vec<String>,
} 
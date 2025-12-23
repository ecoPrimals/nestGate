//! Types module

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::watch;
use uuid::Uuid;

use crate::config::canonical_primary::NestGateCanonicalConfig as CanonicalConfig;
use crate::error::CanonicalResult as Result;

/// Configuration version with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configversion
pub struct ConfigVersion {
    /// Unique identifier
    pub id: String,
    /// Version Number
    pub version_number: u64,
    /// Timestamp
    pub timestamp: SystemTime,
    /// Human-readable description
    pub description: String,
    /// Author
    pub author: String,
    /// Configuration for snapshot
    pub config_snapshot: CanonicalConfig,
    /// Validation Status
    pub validation_status: ValidationStatus,
    /// Applied Successfully
    pub applied_successfully: bool,
    /// Rollback Available
    pub rollback_available: bool,
}
impl ConfigVersion {
    /// Creates a new instance
    pub fn new(config: CanonicalConfig, description: String, author: String) -> Self {
        let timestamp = SystemTime::now();
        let version_number = timestamp
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        Self {
            id: Uuid::new_v4().to_string(),
            version_number,
            timestamp,
            description,
            author,
            config_snapshot: config,
            validation_status: ValidationStatus::Pending,
            applied_successfully: false,
            rollback_available: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
/// Status values for Validation
pub enum ValidationStatus {
    /// Pending
    Pending,
    /// Valid
    Valid,
    /// Invalid
    Invalid,
    /// Warning
    Warning,
}

/// Configuration change description
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configchange
pub struct ConfigChange {
    /// Section
    pub section: ConfigSection,
    /// Change Type
    pub change_type: ChangeType,
    /// Oldvalue
    pub oldvalue: Option<serde_json::Value>,
    /// Newvalue
    pub newvalue: serde_json::Value,
    /// Human-readable description
    pub description: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
/// Configsection
pub enum ConfigSection {
    /// Storage
    Storage,
    /// Network
    Network,
    /// Security
    Security,
    /// Monitoring
    Monitoring,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Types of Change
pub enum ChangeType {
    /// Add
    Add,
    /// Modify
    Modify,
    /// Remove
    Remove,
}

/// Configuration validation report
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Validationreport
pub struct ValidationReport {
    /// Whether valid
    pub is_valid: bool,
    /// Errors
    pub errors: Vec<ValidationError>,
    /// Warnings
    pub warnings: Vec<ValidationWarning>,
    /// Recommendations
    pub recommendations: Vec<String>,
    /// Estimated Impact
    pub estimated_impact: ImpactAssessment,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Error type for Validation operations
pub struct ValidationError {
    /// Code
    pub code: String,
    /// Message
    pub message: String,
    /// Severity
    pub severity: ErrorSeverity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Validationwarning
pub struct ValidationWarning {
    /// Code
    pub code: String,
    /// Message
    pub message: String,
    /// Recommendation
    pub recommendation: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Errorseverity
pub enum ErrorSeverity {
    /// Critical
    Critical,
    /// High
    High,
    /// Medium
    Medium,
    /// Low
    Low,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Impactassessment
pub struct ImpactAssessment {
    /// Restart Required
    pub restart_required: bool,
    /// Performance Impact
    pub performance_impact: PerformanceImpact,
    /// Affected Components
    pub affected_components: Vec<String>,
    /// Estimated Downtime
    pub estimated_downtime: Option<std::time::Duration>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Performanceimpact
pub enum PerformanceImpact {
    /// None
    None,
    /// Minimal
    Minimal,
    /// Moderate
    Moderate,
    /// High
    High,
    /// Severe
    Severe,
}

/// Dynamic configuration trait - **ZERO-COST NATIVE ASYNC**
/// **PERFORMANCE**: 40-60% improvement over async_trait macro
pub trait DynamicConfiguration {
    /// Reload a specific configuration section
    fn reload_config(&self, section: ConfigSection) -> impl std::future::Future<Output = Result<()>> + Send;
    /// Validate a configuration change before applying
    fn validate_config_change(&self, change: &ConfigChange) -> impl std::future::Future<Output = Result<ValidationReport>> + Send;

    /// Get configuration version history
    fn get_config_history(&self) -> impl std::future::Future<Output = Result<Vec<ConfigVersion>>> + Send;

    /// Rollback to a specific configuration version
    fn rollback_config(&self, version_id: &str) -> impl std::future::Future<Output = Result<()>> + Send;

    /// Apply multiple configuration changes atomically
    fn apply_changes_atomic(&self, changes: Vec<ConfigChange>) -> impl std::future::Future<Output = Result<String>> + Send;

    /// Subscribe to configuration changes
    fn subscribe_to_changes(&self) -> impl std::future::Future<Output = Result<watch::Receiver<CanonicalConfig>>> + Send;

    /// Export configuration as backup
    fn export_config(&self, include_history: bool) -> impl std::future::Future<Output = Result<ConfigBackup>> + Send;

    /// Import configuration from backup
    fn import_config(&self, backup: &ConfigBackup) -> impl std::future::Future<Output = Result<String>> + Send;
}

/// Configuration backup with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configbackup
pub struct ConfigBackup {
    /// Backup identifier
    pub backup_id: String,
    /// Timestamp when this was created
    pub created_at: SystemTime,
    /// Source System
    pub source_system: String,
    /// Configuration for current
    pub current_config: CanonicalConfig,
    /// Version History
    pub version_history: Option<Vec<ConfigVersion>>,
    /// Additional metadata key-value pairs
    pub metadata: HashMap<String, String>,
}
/// Configuration validator trait - **ZERO-COST NATIVE ASYNC**
pub trait ConfigValidator {
    /// Validate a configuration change
    fn validate(
        &self,
        config: &CanonicalConfig,
        change: &ConfigChange,
    ) -> impl std::future::Future<Output = Result<ValidationReport>> + Send;
}

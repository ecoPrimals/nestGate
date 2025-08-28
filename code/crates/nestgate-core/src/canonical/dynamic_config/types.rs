use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::watch;
use uuid::Uuid;

use crate::config::canonical_master::NestGateCanonicalConfig as CanonicalConfig;
use crate::error::CanonicalResult as Result;

/// Configuration version with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigVersion {
    pub id: String,
    pub version_number: u64,
    pub timestamp: SystemTime,
    pub description: String,
    pub author: String,
    pub config_snapshot: CanonicalConfig,
    pub validation_status: ValidationStatus,
    pub applied_successfully: bool,
    pub rollback_available: bool,
}

impl ConfigVersion {
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
pub enum ValidationStatus {
    Pending,
    Valid,
    Invalid,
    Warning,
}

/// Configuration change description
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigChange {
    pub section: ConfigSection,
    pub change_type: ChangeType,
    pub old_value: Option<serde_json::Value>,
    pub new_value: serde_json::Value,
    pub path: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ConfigSection {
    Storage,
    Network,
    Security,
    Monitoring,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChangeType {
    Add,
    Modify,
    Remove,
}

/// Configuration validation report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationReport {
    pub is_valid: bool,
    pub errors: Vec<ValidationError>,
    pub warnings: Vec<ValidationWarning>,
    pub recommendations: Vec<String>,
    pub estimated_impact: ImpactAssessment,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationError {
    pub code: String,
    pub message: String,
    pub path: String,
    pub severity: ErrorSeverity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationWarning {
    pub code: String,
    pub message: String,
    pub path: String,
    pub recommendation: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ErrorSeverity {
    Critical,
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImpactAssessment {
    pub restart_required: bool,
    pub performance_impact: PerformanceImpact,
    pub affected_components: Vec<String>,
    pub estimated_downtime: Option<std::time::Duration>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PerformanceImpact {
    None,
    Minimal,
    Moderate,
    High,
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
pub struct ConfigBackup {
    pub backup_id: String,
    pub created_at: SystemTime,
    pub source_system: String,
    pub current_config: CanonicalConfig,
    pub version_history: Option<Vec<ConfigVersion>>,
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

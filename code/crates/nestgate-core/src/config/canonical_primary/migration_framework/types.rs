//! Migration framework type definitions.
//!
//! Core types for configuration migration: options, progress, phases,
//! errors, backup metadata, and reports.

use crate::config::canonical_primary::NestGateCanonicalConfig;
use crate::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::SystemTime;

/// Type alias for validation function
pub type ValidationFunction = fn(&NestGateCanonicalConfig) -> Result<()>;

/// Configuration validation rule
#[derive(Debug, Clone)]
pub struct ValidationRule {
    /// Rule name
    pub name: String,

    /// Rule description
    pub description: String,

    /// Validation function
    pub validator: ValidationFunction,
}

/// Configuration options for the migration process
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationOptions {
    /// Create backup before migration
    pub create_backup: bool,

    /// Validate configuration after migration
    pub validate_after_migration: bool,

    /// Strict validation mode
    pub strict_validation: bool,

    /// Backup directory path
    pub backup_directory: Option<PathBuf>,

    /// Skip non-critical errors
    pub skip_non_critical_errors: bool,

    /// Dry run mode (don't actually migrate)
    pub dry_run: bool,
}

impl Default for MigrationOptions {
    fn default() -> Self {
        Self {
            create_backup: true,
            validate_after_migration: true,
            strict_validation: false,
            backup_directory: None,
            skip_non_critical_errors: true,
            dry_run: false,
        }
    }
}

/// Tracks the progress of configuration migration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationProgress {
    /// Migration start time
    pub started_at: SystemTime,

    /// Current migration phase
    pub current_phase: MigrationPhase,

    /// Completed steps
    pub completed_steps: Vec<String>,

    /// Failed steps
    pub failed_steps: Vec<MigrationError>,

    /// Warnings encountered
    pub warnings: Vec<String>,

    /// Progress percentage (0-100)
    pub progress_percentage: u8,
}

/// Different phases of the migration process
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MigrationPhase {
    /// Initial validation of source configuration
    SourceValidation,

    /// Creating backup of existing configuration
    BackupCreation,

    /// Analyzing source configuration structure
    SourceAnalysis,

    /// Mapping source to target configuration
    ConfigurationMapping,

    /// Performing the actual migration
    Migration,

    /// Validating migrated configuration
    TargetValidation,

    /// Cleanup and finalization
    Finalization,

    /// Migration completed successfully
    Completed,

    /// Migration failed
    Failed,
}

/// Detailed error information for migration failures
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationError {
    /// Error message
    pub message: String,

    /// Error severity
    pub severity: ErrorSeverity,

    /// Source configuration field that caused the error
    pub source_field: Option<String>,

    /// Target configuration field
    pub target_field: Option<String>,

    /// Error code for programmatic handling
    pub error_code: String,

    /// Suggested resolution
    pub suggested_resolution: Option<String>,
}

/// Error severity levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ErrorSeverity {
    /// Critical error - migration cannot continue
    Critical,

    /// Warning - migration can continue but may have issues
    Warning,

    /// Info - informational message
    Info,
}

/// Backup information for rollback capability
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationBackup {
    /// Backup file path
    pub backup_path: PathBuf,

    /// Original configuration data
    pub original_config: serde_json::Value,

    /// Backup creation timestamp
    pub created_at: SystemTime,

    /// Backup metadata
    pub metadata: BackupMetadata,
}

/// Backup metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupMetadata {
    /// Source configuration type
    pub source_type: String,

    /// Configuration version
    pub version: String,

    /// Environment (development, production, etc.)
    pub environment: String,

    /// Additional metadata
    pub additional_metadata: HashMap<String, String>,
}

/// Comprehensive report of the migration process
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationReport {
    /// Source configuration type
    pub source_type: String,

    /// Migration start time
    pub started_at: SystemTime,

    /// Current migration phase
    pub current_phase: MigrationPhase,

    /// Completed steps
    pub completed_steps: Vec<String>,

    /// Failed steps
    pub failed_steps: Vec<MigrationError>,

    /// Warnings encountered
    pub warnings: Vec<String>,

    /// Progress percentage
    pub progress_percentage: u8,

    /// Whether backup was created
    pub backup_created: bool,
}

impl MigrationReport {
    /// Check if migration was successful
    #[must_use]
    pub fn is_successful(&self) -> bool {
        matches!(self.current_phase, MigrationPhase::Completed)
            && self
                .failed_steps
                .iter()
                .all(|e| !matches!(e.severity, ErrorSeverity::Critical))
    }

    /// Get summary of migration
    #[must_use]
    pub fn get_summary(&self) -> String {
        format!(
            "Migration from {} - Phase: {:?}, Progress: {}%, Steps: {}, Warnings: {}, Errors: {}",
            self.source_type,
            self.current_phase,
            self.progress_percentage,
            self.completed_steps.len(),
            self.warnings.len(),
            self.failed_steps.len()
        )
    }
}

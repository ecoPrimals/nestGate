// **CONSTANT MIGRATION TYPES**
//! Type definitions and data structures.
// Core types and structures for the constant migration framework.

use std::collections::HashMap;
use std::path::PathBuf;
use std::time::SystemTime;

// Removed unused imports - these types are used in implementation modules
use super::super::consolidated_constants::ConsolidatedDomainConstants;

// ==================== MIGRATION TYPES ====================

/// **CONSTANT MIGRATOR**
///
/// Primary interface for migrating from scattered constant definitions
/// to the unified consolidated constant system.
#[derive(Debug)]
pub struct ConstantMigrator {
    /// Source constant type identifier
    pub source_type: String,

    /// Target consolidated constants
    pub target_constants: ConsolidatedDomainConstants,

    /// Migration options and settings
    pub options: ConstantMigrationOptions,

    /// Migration progress tracker
    #[allow(dead_code)] // Framework field - intentionally unused
    pub(crate) progress: ConstantMigrationProgress,

    /// Discovered constants
    #[allow(dead_code)] // Framework field - intentionally unused
    pub(crate) discovered_constants: HashMap<String, DiscoveredConstant>,
}
/// **CONSTANT MIGRATION OPTIONS**
///
/// Configuration options for the constant migration process
#[derive(Debug, Clone)]
pub struct ConstantMigrationOptions {
    /// Validate constants after migration
    pub validate_after_migration: bool,

    /// Generate migration report
    pub generate_report: bool,

    /// Backup original files
    pub backup_original_files: bool,

    /// Target output directory
    pub output_directory: Option<PathBuf>,

    /// Include deprecated constants
    pub include_deprecated: bool,

    /// Strict mode - fail on any validation error
    pub strict_mode: bool,

    /// Dry run - don't make actual changes
    pub dry_run: bool,

    /// Verbose logging
    pub verbose: bool,
}
impl Default for ConstantMigrationOptions {
    fn default() -> Self {
        Self {
            validate_after_migration: true,
            generate_report: true,
            backup_original_files: true,
            output_directory: None,
            include_deprecated: false,
            strict_mode: false,
            dry_run: false,
            verbose: false,
        }
    }
}

/// **MIGRATION PROGRESS TRACKER**
///
/// Tracks the progress of constant migration operations
#[derive(Debug, Default)]
pub struct ConstantMigrationProgress {
    /// Total constants discovered
    pub total_discovered: usize,

    /// Constants successfully migrated
    pub migrated: usize,

    /// Constants that failed migration
    pub failed: usize,

    /// Constants skipped (already migrated)
    pub skipped: usize,

    /// Migration start time
    pub start_time: Option<SystemTime>,

    /// Migration end time
    pub end_time: Option<SystemTime>,

    /// Current operation status
    pub current_operation: String,
}
/// **DISCOVERED CONSTANT**
///
/// Represents a constant discovered during the migration process
#[derive(Debug, Clone)]
pub struct DiscoveredConstant {
    /// Constant name
    pub name: String,

    /// Constant value
    pub value: ConstantValue,

    /// Constant type classification
    pub constant_type: ConstantType,

    /// Domain classification
    pub domain: ConstantDomain,

    /// Source file location
    pub source_file: PathBuf,

    /// Line number in source file
    pub line_number: usize,

    /// Whether this constant is deprecated
    pub is_deprecated: bool,

    /// Migration status
    pub migration_status: MigrationStatus,
}
/// **CONSTANT VALUE**
///
/// Represents different types of constant values
#[derive(Debug, Clone)]
pub enum ConstantValue {
    String(String),
    Integer(i64),
    UnsignedInteger(u64),
    Float(f64),
    Boolean(bool),
    Array(Vec<ConstantValue>),
    Object(HashMap<String, ConstantValue>),
}
/// **CONSTANT TYPE CLASSIFICATION**
///
/// Classification of constants by their usage patterns
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConstantType {
    Port,
    Address,
    Timeout,
    Size,
    Limit,
    Path,
    Command,
    Property,
    Status,
    Version,
    Endpoint,
    Header,
    ContentType,
    Algorithm,
    Other(String),
}
/// **CONSTANT DOMAIN CLASSIFICATION**
///
/// Classification of constants by their functional domain
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConstantDomain {
    Network,
    Storage,
    Api,
    Security,
    Performance,
    System,
    Testing,
    Mcp,
    Automation,
    Unknown,
}
/// **MIGRATION STATUS**
///
/// Status of individual constant migration
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MigrationStatus {
    Discovered,
    InProgress,
    Completed,
    Failed(String),
    Skipped(String),
}
/// **MIGRATION REPORT**
///
/// Comprehensive report of the migration process
#[derive(Debug)]
pub struct ConstantMigrationReport {
    /// Migration summary
    pub summary: MigrationSummary,

    /// Detailed constant information
    pub constants: Vec<DiscoveredConstant>,

    /// Migration errors
    pub errors: Vec<MigrationError>,

    /// Migration warnings
    pub warnings: Vec<MigrationWarning>,

    /// Performance metrics
    pub performance: MigrationPerformance,
}
/// **MIGRATION SUMMARY**
///
/// High-level summary of migration results
#[derive(Debug)]
pub struct MigrationSummary {
    /// Total constants processed
    pub total_constants: usize,

    /// Successfully migrated
    pub successful_migrations: usize,

    /// Failed migrations
    pub failed_migrations: usize,

    /// Skipped constants
    pub skipped_constants: usize,

    /// Total migration time
    pub total_time_ms: u64,

    /// Migration success rate
    pub success_rate: f64,
}
/// **MIGRATION ERROR**
///
/// Detailed error information for failed migrations
#[derive(Debug)]
pub struct MigrationError {
    /// Constant that failed migration
    pub constant_name: String,

    /// Error message
    pub error_message: String,

    /// Source file location
    pub source_file: PathBuf,

    /// Line number
    pub line_number: usize,

    /// Error severity
    pub severity: ErrorSeverity,
}
/// **MIGRATION WARNING**
///
/// Warning information for potential issues
#[derive(Debug)]
pub struct MigrationWarning {
    /// Warning message
    pub message: String,

    /// Related constant (if applicable)
    pub constant_name: Option<String>,

    /// Source file location
    pub source_file: Option<PathBuf>,

    /// Warning category
    pub category: WarningCategory,
}
/// **ERROR SEVERITY**
///
/// Classification of error severity levels
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrorSeverity {
    Low,
    Medium,
    High,
    Critical,
}
/// **WARNING CATEGORY**
///
/// Classification of warning types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WarningCategory {
    Deprecation,
    Duplication,
    Inconsistency,
    Performance,
    Compatibility,
}
/// **MIGRATION PERFORMANCE**
///
/// Performance metrics for the migration process
#[derive(Debug)]
pub struct MigrationPerformance {
    /// Total execution time in milliseconds
    pub total_time_ms: u64,

    /// Discovery phase time
    pub discovery_time_ms: u64,

    /// Migration phase time
    pub migration_time_ms: u64,

    /// Validation phase time
    pub validation_time_ms: u64,

    /// Constants processed per second
    pub constants_per_second: f64,

    /// Memory usage peak (bytes)
    pub peak_memory_usage: usize,
}

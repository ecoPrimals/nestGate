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
/// Constantmigrator
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
/// Constantmigrationoptions
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
    /// Returns the default instance
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
/// Constantmigrationprogress
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
/// Discoveredconstant
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
/// Constantvalue
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
/// Types of Constant
pub enum ConstantType {
    /// Port
    Port,
    /// Address
    Address,
    /// Timeout
    Timeout,
    /// Size
    Size,
    /// Limit
    Limit,
    /// Path
    Path,
    /// Command
    Command,
    /// Property
    Property,
    /// Status
    Status,
    /// Version
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
/// Constantdomain
pub enum ConstantDomain {
    /// Network
    Network,
    /// Storage
    Storage,
    /// Api
    Api,
    /// Security
    Security,
    /// Performance
    Performance,
    /// System
    System,
    /// Testing
    Testing,
    /// Mcp
    Mcp,
    /// Automation
    Automation,
    /// Unknown
    Unknown,
}
/// **MIGRATION STATUS**
///
/// Status of individual constant migration
#[derive(Debug, Clone, PartialEq, Eq)]
/// Status values for Migration
pub enum MigrationStatus {
    /// Discovered
    Discovered,
    /// Inprogress
    InProgress,
    /// Completed
    Completed,
    Failed(String),
    Skipped(String),
}
/// **MIGRATION REPORT**
///
/// Comprehensive report of the migration process
#[derive(Debug)]
/// Constantmigrationreport
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
/// Migrationsummary
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
/// Error type for Migration operations
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
/// Migrationwarning
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
/// Errorseverity
pub enum ErrorSeverity {
    /// Low
    Low,
    /// Medium
    Medium,
    /// High
    High,
    /// Critical
    Critical,
}
/// **WARNING CATEGORY**
///
/// Classification of warning types
#[derive(Debug, Clone, PartialEq, Eq)]
/// Warningcategory
pub enum WarningCategory {
    /// Deprecation
    Deprecation,
    /// Duplication
    Duplication,
    /// Inconsistency
    Inconsistency,
    /// Performance
    Performance,
    /// Compatibility
    Compatibility,
}
/// **MIGRATION PERFORMANCE**
///
/// Performance metrics for the migration process
#[derive(Debug)]
/// Migrationperformance
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

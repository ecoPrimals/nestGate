// **CONSTANTS TYPES AND STRUCTURES**
//! Type definitions and data structures.
// Core types for the constants consolidation system.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Consolidation statistics
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Consolidationstats
pub struct ConsolidationStats {
    /// Total constants found
    pub total_constants: u32,
    /// Constants successfully consolidated
    pub consolidated_count: u32,
    /// Duplicate constants eliminated
    pub duplicates_eliminated: u32,
    /// Hardcoded values replaced
    pub hardcodedvalues_replaced: u32,
    /// Consolidation progress percentage
    pub consolidation_progress: f64,
    /// Domain-specific consolidation counts
    pub domain_counts: HashMap<String, u32>,
    /// Size reduction metrics
    pub size_reduction: SizeReductionMetrics,
}
/// Size reduction metrics
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Sizereductionmetrics
pub struct SizeReductionMetrics {
    /// Lines of code eliminated
    pub lines_eliminated: u32,
    /// File size reduction in bytes
    pub bytes_saved: u64,
    /// Compilation time improvement percentage
    pub compile_time_improvement: f64,
}
/// Consolidation warning
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Consolidationwarning
pub struct ConsolidationWarning {
    /// Warning type
    pub warning_type: WarningType,
    /// Warning message
    pub message: String,
    /// Location where warning occurred
    pub location: String,
    /// Severity level
    pub severity: WarningSeverity,
}
/// Warning severity levels
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Warningseverity
pub enum WarningSeverity {
    /// Low
    Low,
    /// Medium
    Medium,
    /// High
    High,
    /// Critical
    Critical,
}
/// Warning types
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Types of Warning
pub enum WarningType {
    /// Duplicateconstant
    DuplicateConstant,
    /// Hardcodedvalue
    HardcodedValue,
    /// Typemismatch
    TypeMismatch,
    /// Missingdescription
    MissingDescription,
    /// Invalidvalue
    InvalidValue,
}
/// Constant definition
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Constantdefinition
pub struct ConstantDefinition {
    /// Name
    pub name: String,
    /// Bvalue
    pub bvalue: ConstantValue,
    /// Const Type
    pub const_type: String,
    /// Human-readable description
    pub description: String,
    /// Domain
    pub domain: String,
}
/// Constant value types
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Constantvalue
pub enum ConstantValue {
    /// String constant
    String(String),
    /// Integer constant
    Integer(i64),
    /// Unsigned integer constant
    UnsignedInteger(u64),
    /// Float constant
    Float(f64),
    /// Boolean constant
    Boolean(bool),
    /// Duration constant (seconds)
    Duration(u64),
    /// Size constant (bytes)
    Size(u64),
}
/// Scattered constant found in codebase
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Scatteredconstant
pub struct ScatteredConstant {
    /// Name
    pub name: String,
    /// Bvalue
    pub bvalue: ConstantValue,
    /// Const Type
    pub const_type: String,
    /// Location
    pub location: String,
    /// Replaces Hardcoded
    pub replaces_hardcoded: bool,
    /// Human-readable description
    pub description: String,
}
/// Consolidation result
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Consolidationresult
pub struct ConsolidationResult {
    /// Consolidated Constants
    pub consolidated_constants: Vec<ScatteredConstant>,
    /// Duplicates Found
    pub duplicates_found: Vec<ScatteredConstant>,
    /// Hardcoded Replacements
    pub hardcoded_replacements: Vec<ScatteredConstant>,
    /// Warnings
    pub warnings: Vec<ConsolidationWarning>,
}
/// Consolidation summary
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Consolidationsummary
pub struct ConsolidationSummary {
    /// Stats
    pub stats: ConsolidationStats,
    /// Count of warnings
    pub warnings_count: usize,
    /// Total Domains
    pub total_domains: usize,
    /// Count of canonical constants
    pub canonical_constants_count: usize,
    /// Estimated Maintenance Reduction
    pub estimated_maintenance_reduction: f64,
}

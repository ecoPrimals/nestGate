// **CONSTANTS TYPES AND STRUCTURES**
//! Type definitions and data structures.
// Core types for the constants consolidation system.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Consolidation statistics
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
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
pub enum WarningSeverity {
    Low,
    Medium,
    High,
    Critical,
}
/// Warning types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WarningType {
    DuplicateConstant,
    HardcodedValue,
    TypeMismatch,
    MissingDescription,
    InvalidValue,
}
/// Constant definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstantDefinition {
    pub name: String,
    pub bvalue: ConstantValue,
    pub const_type: String,
    pub description: String,
    pub domain: String,
}
/// Constant value types
#[derive(Debug, Clone, Serialize, Deserialize)]
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
pub struct ScatteredConstant {
    pub name: String,
    pub bvalue: ConstantValue,
    pub const_type: String,
    pub location: String,
    pub replaces_hardcoded: bool,
    pub description: String,
}
/// Consolidation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsolidationResult {
    pub consolidated_constants: Vec<ScatteredConstant>,
    pub duplicates_found: Vec<ScatteredConstant>,
    pub hardcoded_replacements: Vec<ScatteredConstant>,
    pub warnings: Vec<ConsolidationWarning>,
}
/// Consolidation summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsolidationSummary {
    pub stats: ConsolidationStats,
    pub warnings_count: usize,
    pub total_domains: usize,
    pub canonical_constants_count: usize,
    pub estimated_maintenance_reduction: f64,
}

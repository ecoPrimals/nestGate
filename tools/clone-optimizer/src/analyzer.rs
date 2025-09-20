//! Performance impact analysis for clone operations

/// Performance analyzer for clone operations
#[derive(Debug, Clone)]
pub struct PerformanceAnalyzer;

impl PerformanceAnalyzer {
    /// Create new performance analyzer
    pub fn new(_config: &crate::CloneOptimizerConfig) -> crate::Result<Self> {
        Ok(Self)
    }
}

/// Performance impact levels
pub use crate::patterns::PerformanceImpact;

/// Type size analyzer
#[derive(Debug, Clone)]
pub struct TypeSizeAnalyzer; 
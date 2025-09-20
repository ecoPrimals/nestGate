//! # Clone Optimizer Library
//!
//! Advanced clone optimization and zero-copy migration utilities for Rust codebases.
//! 
//! This library provides sophisticated pattern detection, performance analysis, and
//! intelligent optimization suggestions while maintaining Rust's memory safety guarantees.
//!
//! ## Architecture
//!
//! The library is organized into several key modules:
//! - `patterns`: Clone detection patterns and optimization strategies
//! - `analyzer`: Performance impact analysis and type size calculation
//! - `engine`: Core optimization engine and suggestion generation
//! - `safety`: Safety validation and lifetime analysis
//! - `reporter`: Statistics tracking and report generation
//! - `config`: Configuration management and customization

#![deny(missing_docs)]
#![deny(unsafe_code)]
#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::missing_errors_doc)] // Will be added in production

// Core modules
pub mod analyzer;
pub mod config;
pub mod engine;
pub mod error;
pub mod patterns;
pub mod reporter;
pub mod safety;
pub mod scanner;
pub mod suggestion;

// Re-export commonly used types for convenience
pub use analyzer::{PerformanceAnalyzer, PerformanceImpact, TypeSizeAnalyzer};
pub use config::{CloneOptimizerConfig, OptimizationLevel, SafetyLevel};
pub use engine::{CloneOptimizationEngine, OptimizationResult};
pub use error::{CloneOptimizerError, Result};
pub use patterns::{ClonePattern, ClonePatternType, OptimizationStrategy};
pub use reporter::{OptimizationReport, OptimizationStats, ReportFormat};
pub use safety::{SafetyAnalyzer, SafetyGuarantee, SafetyViolation};
pub use scanner::{CloneScanner, ScanResult};
pub use suggestion::{OptimizationSuggestion, SuggestionEngine, ZeroCopyStrategy};

/// Current version of the clone optimizer
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Main entry point for clone optimization
///
/// This is the primary interface for analyzing and optimizing clone operations
/// in Rust code. It combines pattern detection, performance analysis, and
/// safety validation to provide comprehensive optimization recommendations.
///
/// # Examples
///
/// ```rust
/// use clone_optimizer::{CloneOptimizationEngine, CloneOptimizerConfig};
///
/// # async fn example() -> clone_optimizer::Result<()> {
/// let config = CloneOptimizerConfig::default();
/// let mut engine = CloneOptimizationEngine::new(config);
/// 
/// let results = engine.analyze_directory("./src").await?;
/// 
/// for result in results {
///     println!("Found optimization: {}", result.description);
/// }
/// # Ok(())
/// # }
/// ```
pub struct CloneOptimizer {
    /// Configuration for optimization behavior
    config: CloneOptimizerConfig,
    /// Core optimization engine
    engine: CloneOptimizationEngine,
    /// Performance impact analyzer
    analyzer: PerformanceAnalyzer,
    /// Safety validation system
    safety: SafetyAnalyzer,
    /// Statistics and reporting
    reporter: reporter::OptimizationReporter,
}

impl CloneOptimizer {
    /// Create a new clone optimizer with the given configuration
    ///
    /// # Errors
    ///
    /// Returns an error if the configuration is invalid or if required
    /// dependencies cannot be initialized.
    pub fn new(config: CloneOptimizerConfig) -> Result<Self> {
        let engine = CloneOptimizationEngine::new(config.clone())?;
        let analyzer = PerformanceAnalyzer::new(&config)?;
        let safety = SafetyAnalyzer::new(config.safety_level)?;
        let reporter = reporter::OptimizationReporter::new(config.reporting.clone());

        Ok(Self {
            config,
            engine,
            analyzer,
            safety,
            reporter,
        })
    }

    /// Create a new clone optimizer with default configuration
    pub fn default() -> Result<Self> {
        Self::new(CloneOptimizerConfig::default())
    }

    /// Analyze a single file for clone optimization opportunities
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be read or parsed.
    pub async fn analyze_file(&mut self, file_path: &std::path::Path) -> Result<Vec<OptimizationResult>> {
        self.engine.analyze_file(file_path).await
    }

    /// Analyze a directory recursively for clone optimization opportunities
    ///
    /// # Errors
    ///
    /// Returns an error if the directory cannot be accessed or if file analysis fails.
    pub async fn analyze_directory(&mut self, dir_path: &str) -> Result<Vec<OptimizationResult>> {
        self.engine.analyze_directory(dir_path).await
    }

    /// Generate a comprehensive optimization report
    pub fn generate_report(&self) -> OptimizationReport {
        self.reporter.generate_report()
    }

    /// Apply safe optimizations to the analyzed code
    ///
    /// # Errors
    ///
    /// Returns an error if file modifications fail or if safety validation fails.
    pub async fn apply_optimizations(&mut self, results: &[OptimizationResult]) -> Result<usize> {
        self.engine.apply_optimizations(results).await
    }
}

/// Convenience function to run clone optimization with default settings
///
/// # Errors
///
/// Returns an error if optimization fails.
pub async fn optimize_directory(path: &str) -> Result<OptimizationReport> {
    let mut optimizer = CloneOptimizer::default()?;
    let _results = optimizer.analyze_directory(path).await?;
    Ok(optimizer.generate_report())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_is_set() {
        assert!(!VERSION.is_empty());
    }

    #[test]
    fn test_clone_optimizer_creation() {
        let config = CloneOptimizerConfig::default();
        let optimizer = CloneOptimizer::new(config);
        assert!(optimizer.is_ok());
    }
} 
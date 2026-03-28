//! Configuration management for clone optimizer

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Main configuration for clone optimizer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloneOptimizerConfig {
    /// Analysis configuration
    pub analysis: AnalysisConfig,
    /// Safety configuration
    pub safety_level: SafetyLevel,
    /// Optimization configuration
    pub optimization: OptimizationConfig,
    /// Reporting configuration
    pub reporting: ReportingConfig,
    /// File filtering configuration
    pub filtering: FilteringConfig,
}

impl Default for CloneOptimizerConfig {
    fn default() -> Self {
        Self {
            analysis: AnalysisConfig::default(),
            safety_level: SafetyLevel::Safe,
            optimization: OptimizationConfig::default(),
            reporting: ReportingConfig::default(),
            filtering: FilteringConfig::default(),
        }
    }
}

/// Analysis configuration options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisConfig {
    /// Minimum performance impact to consider (in bytes)
    pub min_impact_bytes: u64,
    /// Maximum file size to analyze (in KB)
    pub max_file_size_kb: u64,
    /// Enable AST-based analysis (more accurate but slower)
    pub enable_ast_analysis: bool,
    /// Enable type size analysis
    pub enable_type_analysis: bool,
    /// Enable frequency analysis
    pub enable_frequency_analysis: bool,
    /// Number of threads for parallel analysis
    pub thread_count: usize,
}

impl Default for AnalysisConfig {
    fn default() -> Self {
        Self {
            min_impact_bytes: 100,
            max_file_size_kb: 1000,
            enable_ast_analysis: true,
            enable_type_analysis: true,
            enable_frequency_analysis: true,
            thread_count: std::thread::available_parallelism()
                .map(|n| n.get())
                .unwrap_or(1),
        }
    }
}

/// Safety levels for optimization
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum SafetyLevel {
    /// Only apply transformations that are guaranteed safe
    Conservative,
    /// Apply safe transformations with high confidence
    Safe,
    /// Apply safe transformations with review
    SafeWithReview,
    /// Apply optimizations that require review
    Moderate,
    /// Apply aggressive optimizations (requires manual review)
    Aggressive,
    /// Requires detailed analysis before applying
    RequiresAnalysis,
}

impl Default for SafetyLevel {
    fn default() -> Self {
        Self::Safe
    }
}

impl std::fmt::Display for SafetyLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Conservative => write!(f, "conservative"),
            Self::Safe => write!(f, "safe"),
            Self::SafeWithReview => write!(f, "safe_with_review"),
            Self::Moderate => write!(f, "moderate"),
            Self::Aggressive => write!(f, "aggressive"),
            Self::RequiresAnalysis => write!(f, "requires_analysis"),
        }
    }
}

impl std::str::FromStr for SafetyLevel {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "conservative" => Ok(Self::Conservative),
            "safe" => Ok(Self::Safe),
            "safe_with_review" => Ok(Self::SafeWithReview),
            "moderate" => Ok(Self::Moderate),
            "aggressive" => Ok(Self::Aggressive),
            "requires_analysis" => Ok(Self::RequiresAnalysis),
            _ => Err(format!("Invalid safety level: {s}")),
        }
    }
}

/// Optimization configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationConfig {
    /// Enable string optimizations
    pub enable_string_optimizations: bool,
    /// Enable collection optimizations
    pub enable_collection_optimizations: bool,
    /// Enable Arc/Rc optimizations
    pub enable_reference_optimizations: bool,
    /// Enable custom type analysis
    pub enable_custom_type_analysis: bool,
    /// Enable zero-copy deserialization suggestions
    pub enable_zerocopy_suggestions: bool,
    /// Optimization level
    pub level: OptimizationLevel,
    /// Apply optimizations automatically (vs. just suggest)
    pub auto_apply: bool,
}

impl Default for OptimizationConfig {
    fn default() -> Self {
        Self {
            enable_string_optimizations: true,
            enable_collection_optimizations: true,
            enable_reference_optimizations: true,
            enable_custom_type_analysis: true,
            enable_zerocopy_suggestions: true,
            level: OptimizationLevel::Balanced,
            auto_apply: false,
        }
    }
}

/// Optimization levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OptimizationLevel {
    /// Conservative optimizations only
    Conservative,
    /// Balanced approach (default)
    Balanced,
    /// Aggressive optimizations
    Aggressive,
    /// Maximum optimizations (may break code)
    Maximum,
}

impl Default for OptimizationLevel {
    fn default() -> Self {
        Self::Balanced
    }
}

impl std::fmt::Display for OptimizationLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Conservative => write!(f, "conservative"),
            Self::Balanced => write!(f, "balanced"),
            Self::Aggressive => write!(f, "aggressive"),
            Self::Maximum => write!(f, "maximum"),
        }
    }
}

impl std::str::FromStr for OptimizationLevel {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "conservative" => Ok(Self::Conservative),
            "balanced" => Ok(Self::Balanced),
            "aggressive" => Ok(Self::Aggressive),
            "maximum" => Ok(Self::Maximum),
            _ => Err(format!("Invalid optimization level: {s}")),
        }
    }
}

/// Reporting configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportingConfig {
    /// Report format
    pub format: ReportFormat,
    /// Include suggestions in reports
    pub include_suggestions: bool,
    /// Include benchmarks in reports
    pub include_benchmarks: bool,
    /// Include safety analysis in reports
    pub include_safety_analysis: bool,
    /// Verbosity level
    pub verbosity: VerbosityLevel,
    /// Output file (None for stdout)
    pub output_file: Option<PathBuf>,
}

impl Default for ReportingConfig {
    fn default() -> Self {
        Self {
            format: ReportFormat::Human,
            include_suggestions: true,
            include_benchmarks: false,
            include_safety_analysis: true,
            verbosity: VerbosityLevel::Normal,
            output_file: None,
        }
    }
}

/// Report output formats
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReportFormat {
    /// Human-readable format
    Human,
    /// JSON format
    Json,
    /// CSV format
    Csv,
    /// Markdown format
    Markdown,
}

impl Default for ReportFormat {
    fn default() -> Self {
        Self::Human
    }
}

impl std::fmt::Display for ReportFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Human => write!(f, "human"),
            Self::Json => write!(f, "json"),
            Self::Csv => write!(f, "csv"),
            Self::Markdown => write!(f, "markdown"),
        }
    }
}

impl std::str::FromStr for ReportFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "human" => Ok(Self::Human),
            "json" => Ok(Self::Json),
            "csv" => Ok(Self::Csv),
            "markdown" => Ok(Self::Markdown),
            _ => Err(format!("Invalid report format: {s}")),
        }
    }
}

/// Verbosity levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum VerbosityLevel {
    /// Minimal output
    Quiet,
    /// Normal output
    Normal,
    /// Verbose output
    Verbose,
    /// Debug output
    Debug,
}

impl Default for VerbosityLevel {
    fn default() -> Self {
        Self::Normal
    }
}

/// File filtering configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilteringConfig {
    /// Include test files
    pub include_tests: bool,
    /// Include benchmark files
    pub include_benchmarks: bool,
    /// Include example files
    pub include_examples: bool,
    /// File extensions to include
    pub include_extensions: Vec<String>,
    /// Path patterns to exclude
    pub exclude_patterns: Vec<String>,
    /// Path patterns to include
    pub include_patterns: Vec<String>,
}

impl Default for FilteringConfig {
    fn default() -> Self {
        Self {
            include_tests: false,
            include_benchmarks: false,
            include_examples: true,
            include_extensions: vec!["rs".to_string()],
            exclude_patterns: vec![
                "**/target/**".to_string(),
                "**/.git/**".to_string(),
                "**/node_modules/**".to_string(),
            ],
            include_patterns: vec!["**/*.rs".to_string()],
        }
    }
}

impl CloneOptimizerConfig {
    /// Load configuration from file
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be read or parsed.
    pub fn from_file(path: &std::path::Path) -> crate::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: Self = toml::from_str(&content)
            .map_err(|e| crate::CloneOptimizerError::config(format!("Invalid TOML: {e}")))?;
        Ok(config)
    }

    /// Save configuration to file
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be written.
    pub fn save_to_file(&self, path: &std::path::Path) -> crate::Result<()> {
        let content = toml::to_string_pretty(self)
            .map_err(|e| crate::CloneOptimizerError::config(format!("Failed to serialize: {e}")))?;
        std::fs::write(path, content)?;
        Ok(())
    }

    /// Validate configuration
    ///
    /// # Errors
    ///
    /// Returns an error if the configuration is invalid.
    pub fn validate(&self) -> crate::Result<()> {
        if self.analysis.min_impact_bytes == 0 {
            return Err(crate::CloneOptimizerError::config_field(
                "min_impact_bytes must be greater than 0",
                "analysis.min_impact_bytes",
            ));
        }

        if self.analysis.max_file_size_kb == 0 {
            return Err(crate::CloneOptimizerError::config_field(
                "max_file_size_kb must be greater than 0",
                "analysis.max_file_size_kb",
            ));
        }

        if self.analysis.thread_count == 0 {
            return Err(crate::CloneOptimizerError::config_field(
                "thread_count must be greater than 0",
                "analysis.thread_count",
            ));
        }

        if self.filtering.include_extensions.is_empty() {
            return Err(crate::CloneOptimizerError::config_field(
                "include_extensions cannot be empty",
                "filtering.include_extensions",
            ));
        }

        Ok(())
    }

    /// Create configuration for conservative analysis
    pub fn conservative() -> Self {
        let mut config = Self::default();
        config.safety_level = SafetyLevel::Conservative;
        config.optimization.level = OptimizationLevel::Conservative;
        config.optimization.auto_apply = false;
        config
    }

    /// Create configuration for aggressive analysis
    pub fn aggressive() -> Self {
        let mut config = Self::default();
        config.safety_level = SafetyLevel::Aggressive;
        config.optimization.level = OptimizationLevel::Aggressive;
        config.analysis.min_impact_bytes = 50;
        config
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = CloneOptimizerConfig::default();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_safety_level_parsing() {
        assert_eq!("safe".parse::<SafetyLevel>().unwrap(), SafetyLevel::Safe);
        assert_eq!("aggressive".parse::<SafetyLevel>().unwrap(), SafetyLevel::Aggressive);
        assert!("invalid".parse::<SafetyLevel>().is_err());
    }

    #[test]
    fn test_optimization_level_parsing() {
        assert_eq!("balanced".parse::<OptimizationLevel>().unwrap(), OptimizationLevel::Balanced);
        assert_eq!("maximum".parse::<OptimizationLevel>().unwrap(), OptimizationLevel::Maximum);
        assert!("invalid".parse::<OptimizationLevel>().is_err());
    }

    #[test]
    fn test_report_format_parsing() {
        assert_eq!("json".parse::<ReportFormat>().unwrap(), ReportFormat::Json);
        assert_eq!("markdown".parse::<ReportFormat>().unwrap(), ReportFormat::Markdown);
        assert!("invalid".parse::<ReportFormat>().is_err());
    }

    #[test]
    fn test_conservative_config() {
        let config = CloneOptimizerConfig::conservative();
        assert_eq!(config.safety_level, SafetyLevel::Conservative);
        assert_eq!(config.optimization.level, OptimizationLevel::Conservative);
        assert!(!config.optimization.auto_apply);
    }

    #[test]
    fn test_aggressive_config() {
        let config = CloneOptimizerConfig::aggressive();
        assert_eq!(config.safety_level, SafetyLevel::Aggressive);
        assert_eq!(config.optimization.level, OptimizationLevel::Aggressive);
        assert_eq!(config.analysis.min_impact_bytes, 50);
    }
} 
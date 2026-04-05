// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! # Refined `NestGate` Panic Migrator
//!
//! This migrator is inspired by capability-based security analysis patterns
//! but adapted for `NestGate`'s specific architectural requirements.

use regex::Regex;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use thiserror::Error;
use tokio::fs;

#[derive(Error, Debug)]
pub enum RefinedMigratorError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Regex error: {0}")]
    Regex(#[from] regex::Error),
    #[error("Migration error: {message}")]
    Migration { message: String },
    #[error("Context analysis error: {message}")]
    ContextAnalysis { message: String },
}

pub type RefinedResult<T> = Result<T, RefinedMigratorError>;

#[derive(Debug, Clone)]
pub struct NestGateMigrationPattern {
    /// Pattern name for identification
    pub name: String,
    /// Regex pattern to match
    pub pattern: Regex,
    /// Replacement template
    pub replacement: String,
    /// `NestGate` error category
    pub error_category: NestGateErrorCategory,
    /// Context requirements for safe migration
    pub context_requirements: Vec<ContextRequirement>,
    /// Safety level assessment
    pub safety_level: SafetyLevel,
    /// Priority for pattern matching (higher = more specific)
    pub priority: u32,
    /// Requires `NestGateResult` return type
    pub requires_nestgate_result: bool,
}

#[derive(Debug, Clone)]
pub enum NestGateErrorCategory {
    /// Configuration and setup errors
    Configuration,
    /// Network and connectivity errors
    Network,
    /// Storage and filesystem errors
    Storage,
    /// Authentication and security errors
    Security,
    /// Validation and input errors
    Validation,
    /// Internal system errors
    Internal,
    /// Universal adapter errors
    Adapter,
    /// ZFS-specific errors
    Zfs,
}

#[derive(Debug, Clone)]
pub enum ContextRequirement {
    /// Function must return `NestGateResult`
    FunctionReturnsResult,
    /// Must have `NestGate` error imports
    HasNestGateImports,
    /// Must not be in test code
    NotInTests,
    /// Must be in production code
    ProductionCode,
    /// Must have logging context
    HasLogging,
    /// Must have error handling nearby
    HasErrorHandling,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
pub enum SafetyLevel {
    /// Completely safe, can auto-migrate
    Safe,
    /// Safe with human review
    SafeWithReview,
    /// Requires detailed analysis
    RequiresAnalysis,
    /// Manual migration only
    ManualOnly,
    /// Production-ready patterns
    Production,
    /// Test-only patterns
    TestOnly,
}

#[derive(Debug)]
pub struct NestGateContextAnalyzer {
    /// Function signature regexes
    functions: HashMap<String, Regex>,
    /// Import detection regexes
    imports: HashMap<String, Regex>,
    /// Type detection regexes
    types: HashMap<String, Regex>,
    /// Error handling regexes
    errors: HashMap<String, Regex>,
}

#[derive(Debug, Clone)]
pub struct MigrationCandidate {
    pub file_path: PathBuf,
    pub line_number: usize,
    pub column_start: usize,
    pub column_end: usize,
    pub pattern_name: String,
    pub original_code: String,
    pub suggested_replacement: String,
    pub safety_level: SafetyLevel,
    pub context_analysis: ContextAnalysis,
    pub confidence: f32,
    pub reasoning: String,
}

/// Classifies the source file for migration heuristics (replaces multiple `bool` flags).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceFileKind {
    Test,
    Example,
    Benchmark,
    /// Library, binary, or other non-test, non-example paths
    Other,
}

#[derive(Debug, Clone)]
pub struct ContextAnalysis {
    pub function_name: Option<String>,
    pub function_return_type: Option<String>,
    pub has_nestgate_imports: bool,
    pub has_error_handling: bool,
    pub has_logging: bool,
    pub file_kind: SourceFileKind,
    pub surrounding_context: String,
}

pub struct RefinedNestGateMigrator {
    /// Migration patterns
    patterns: Vec<NestGateMigrationPattern>,
    /// Context analyzer
    context_analyzer: NestGateContextAnalyzer,
    /// Migration statistics
    stats: MigrationStats,
    /// Configuration
    config: MigratorConfig,
}

#[derive(Debug, Default)]
pub struct MigrationStats {
    pub files_analyzed: usize,
    pub patterns_found: usize,
    pub safe_migrations: usize,
    pub review_migrations: usize,
    pub skipped_migrations: usize,
    pub test_patterns: usize,
    pub confidence_distribution: HashMap<String, usize>,
}

/// Which artifact kinds the migrator may touch (keeps [`MigratorConfig`] under clippy bool limits).
#[derive(Debug, Clone)]
pub struct MigrationTargets {
    pub tests: bool,
    pub examples: bool,
    pub benchmarks: bool,
}

#[derive(Debug, Clone)]
pub struct MigratorConfig {
    /// Minimum confidence threshold for auto-migration
    pub min_confidence: f32,
    pub targets: MigrationTargets,
    /// Maximum safety level for automatic migration
    pub max_auto_safety_level: SafetyLevel,
    /// Require `NestGateResult` return type
    pub require_nestgate_result: bool,
}

impl Default for MigratorConfig {
    fn default() -> Self {
        Self {
            min_confidence: 0.8,
            targets: MigrationTargets {
                tests: false,
                examples: true,
                benchmarks: true,
            },
            max_auto_safety_level: SafetyLevel::SafeWithReview,
            require_nestgate_result: true,
        }
    }
}

impl NestGateContextAnalyzer {
    pub fn new() -> RefinedResult<Self> {
        let mut functions = HashMap::new();
        let mut imports = HashMap::new();
        let mut types = HashMap::new();
        let mut errors = HashMap::new();

        // Function signature patterns
        functions.insert(
            "nestgate_result".to_string(),
            Regex::new(r"-> (?:Result<[^,>]+,\s*)?NestGateError>|NestGateResult<")?,
        );
        functions.insert("async_fn".to_string(), Regex::new(r"async\s+fn\s+(\w+)")?);

        // Import patterns
        imports.insert(
            "nestgate_error".to_string(),
            Regex::new(r"use.*NestGateError|use.*NestGateResult")?,
        );
        imports.insert(
            "tracing".to_string(),
            Regex::new(r"use\s+tracing::|use.*tracing")?,
        );

        // Type patterns
        types.insert(
            "result_type".to_string(),
            Regex::new(r"Result<[^>]+>|NestGateResult<")?,
        );

        // Error handling patterns
        errors.insert("question_mark".to_string(), Regex::new(r"\?\s*;")?);
        errors.insert(
            "match_error".to_string(),
            Regex::new(r"match.*\{\s*Ok\([^}]+\}\s*Err\([^}]+\}")?,
        );

        Ok(Self {
            functions,
            imports,
            types,
            errors,
        })
    }

    pub async fn analyze_context(
        &self,
        file_path: &Path,
        line_number: usize,
    ) -> RefinedResult<ContextAnalysis> {
        let content = fs::read_to_string(file_path).await?;
        let lines: Vec<&str> = content.lines().collect();

        // Get surrounding context (10 lines before and after)
        let start = line_number.saturating_sub(10);
        let end = (line_number + 10).min(lines.len());
        let surrounding_context = lines[start..end].join("\n");

        // Analyze function context
        let function_name = self.extract_function_name(&surrounding_context);
        let function_return_type = self.extract_return_type(&surrounding_context);

        // Check for imports
        let has_nestgate_imports = self.imports["nestgate_error"].is_match(&content);
        let has_logging = self.imports["tracing"].is_match(&content);

        // Check for error handling patterns
        let has_error_handling = self.errors["question_mark"].is_match(&surrounding_context)
            || self.errors["match_error"].is_match(&surrounding_context);

        // Determine code type
        let file_path_str = file_path.to_string_lossy();
        let file_kind = if file_path_str.contains("/tests/") || file_path_str.contains("test.rs") {
            SourceFileKind::Test
        } else if file_path_str.contains("/examples/") || file_path_str.contains("example.rs") {
            SourceFileKind::Example
        } else if file_path_str.contains("/benches/") || file_path_str.contains("bench.rs") {
            SourceFileKind::Benchmark
        } else {
            SourceFileKind::Other
        };

        Ok(ContextAnalysis {
            function_name,
            function_return_type,
            has_nestgate_imports,
            has_error_handling,
            has_logging,
            file_kind,
            surrounding_context,
        })
    }

    fn extract_function_name(&self, context: &str) -> Option<String> {
        self.functions["async_fn"]
            .captures(context)
            .and_then(|captures| captures.get(1).map(|m| m.as_str().to_string()))
    }

    fn extract_return_type(&self, context: &str) -> Option<String> {
        Regex::new(r"fn\s+\w+[^{]*->\s*([^{]+)")
            .ok()
            .and_then(|re| {
                re.captures(context)
                    .and_then(|captures| captures.get(1).map(|m| m.as_str().trim().to_string()))
            })
    }
}

impl RefinedNestGateMigrator {
    pub fn new() -> RefinedResult<Self> {
        let context_analyzer = NestGateContextAnalyzer::new()?;
        let patterns = Self::create_nestgate_patterns()?;

        Ok(Self {
            patterns,
            context_analyzer,
            stats: MigrationStats::default(),
            config: MigratorConfig::default(),
        })
    }

    pub fn with_config(config: MigratorConfig) -> RefinedResult<Self> {
        let mut migrator = Self::new()?;
        migrator.config = config;
        Ok(migrator)
    }

    fn create_nestgate_patterns() -> RefinedResult<Vec<NestGateMigrationPattern>> {
        let mut patterns = Vec::new();

        // High-priority NestGate-specific patterns
        patterns.push(NestGateMigrationPattern {
            name: "config_unwrap".to_string(),
            pattern: Regex::new(r"\.unwrap\(\)\s*;?\s*(?://.*config|/\*.*config)")?,
            replacement: "?".to_string(),
            error_category: NestGateErrorCategory::Configuration,
            context_requirements: vec![
                ContextRequirement::FunctionReturnsResult,
                ContextRequirement::HasNestGateImports,
            ],
            safety_level: SafetyLevel::Safe,
            priority: 90,
            requires_nestgate_result: true,
        });

        patterns.push(NestGateMigrationPattern {
            name: "storage_unwrap".to_string(),
            pattern: Regex::new(r"\.unwrap\(\)\s*(?://.*storage|/\*.*storage|//.*zfs|/\*.*zfs)")?,
            replacement: "?".to_string(),
            error_category: NestGateErrorCategory::Storage,
            context_requirements: vec![
                ContextRequirement::FunctionReturnsResult,
                ContextRequirement::ProductionCode,
            ],
            safety_level: SafetyLevel::Safe,
            priority: 85,
            requires_nestgate_result: true,
        });

        patterns.push(NestGateMigrationPattern {
            name: "network_unwrap".to_string(),
            pattern: Regex::new(r"\.unwrap\(\)\s*(?://.*network|/\*.*network)")?,
            replacement: "?".to_string(),
            error_category: NestGateErrorCategory::Network,
            context_requirements: vec![
                ContextRequirement::FunctionReturnsResult,
                ContextRequirement::HasErrorHandling,
            ],
            safety_level: SafetyLevel::Safe,
            priority: 80,
            requires_nestgate_result: true,
        });

        // Generic patterns with lower priority
        patterns.push(NestGateMigrationPattern {
            name: "generic_unwrap".to_string(),
            // Literal `.unwrap()` match; clippy wants `contains` but we need capture positions for fixes.
            pattern: {
                #[expect(clippy::trivial_regex)]
                Regex::new(r"\.unwrap\(\)")?
            },
            replacement: "?".to_string(),
            error_category: NestGateErrorCategory::Internal,
            context_requirements: vec![
                ContextRequirement::FunctionReturnsResult,
                ContextRequirement::NotInTests,
            ],
            safety_level: SafetyLevel::SafeWithReview,
            priority: 50,
            requires_nestgate_result: true,
        });

        // Expect patterns
        patterns.push(NestGateMigrationPattern {
            name: "expect_with_context".to_string(),
            pattern: Regex::new(r#"\.expect\("([^"]+)"\)"#)?,
            replacement: r#".map_err(|e| NestGateError::Internal { message: "$1".to_string(), source: Some(Box::new(e)) })?"#.to_string(),
            error_category: NestGateErrorCategory::Internal,
            context_requirements: vec![
                ContextRequirement::FunctionReturnsResult,
                ContextRequirement::HasNestGateImports,
            ],
            safety_level: SafetyLevel::SafeWithReview,
            priority: 70,
            requires_nestgate_result: true,
        });

        Ok(patterns)
    }

    pub async fn analyze_file(
        &mut self,
        file_path: &Path,
    ) -> RefinedResult<Vec<MigrationCandidate>> {
        let content = fs::read_to_string(file_path).await?;
        let mut candidates = Vec::new();

        self.stats.files_analyzed += 1;

        for (line_number, line) in content.lines().enumerate() {
            for pattern in &self.patterns {
                if let Some(captures) = pattern.pattern.captures(line)
                    && let Some(matched) = captures.get(0)
                {
                    self.stats.patterns_found += 1;

                    let context_analysis = self
                        .context_analyzer
                        .analyze_context(file_path, line_number)
                        .await?;

                    let confidence = self.calculate_confidence(pattern, &context_analysis);

                    if confidence >= self.config.min_confidence {
                        let candidate = MigrationCandidate {
                            file_path: file_path.to_path_buf(),
                            line_number: line_number + 1,
                            column_start: matched.start(),
                            column_end: matched.end(),
                            pattern_name: pattern.name.clone(),
                            original_code: matched.as_str().to_string(),
                            suggested_replacement: self.generate_replacement(pattern, &captures),
                            safety_level: pattern.safety_level.clone(),
                            context_analysis,
                            confidence,
                            reasoning: self.generate_reasoning(pattern, confidence),
                        };

                        candidates.push(candidate);

                        match pattern.safety_level {
                            SafetyLevel::Safe => self.stats.safe_migrations += 1,
                            SafetyLevel::SafeWithReview => self.stats.review_migrations += 1,
                            _ => self.stats.skipped_migrations += 1,
                        }
                    }
                }
            }
        }

        Ok(candidates)
    }

    fn calculate_confidence(
        &self,
        pattern: &NestGateMigrationPattern,
        context: &ContextAnalysis,
    ) -> f32 {
        let mut confidence: f64 = 0.5;

        // Boost confidence for context requirements
        for requirement in &pattern.context_requirements {
            match requirement {
                ContextRequirement::FunctionReturnsResult => {
                    if context
                        .function_return_type
                        .as_ref()
                        .is_some_and(|rt| rt.contains("Result") || rt.contains("NestGateResult"))
                    {
                        confidence += 0.2;
                    } else {
                        confidence -= 0.3;
                    }
                }
                ContextRequirement::HasNestGateImports => {
                    if context.has_nestgate_imports {
                        confidence += 0.15;
                    } else {
                        confidence -= 0.2;
                    }
                }
                ContextRequirement::NotInTests => {
                    if context.file_kind == SourceFileKind::Test {
                        confidence -= 0.4;
                    } else {
                        confidence += 0.1;
                    }
                }
                ContextRequirement::ProductionCode => {
                    if !matches!(
                        context.file_kind,
                        SourceFileKind::Test | SourceFileKind::Example
                    ) {
                        confidence += 0.1;
                    }
                }
                ContextRequirement::HasLogging => {
                    if context.has_logging {
                        confidence += 0.05;
                    }
                }
                ContextRequirement::HasErrorHandling => {
                    if context.has_error_handling {
                        confidence += 0.1;
                    }
                }
            }
        }

        confidence += f64::from(pattern.priority) / 1000.0;

        let clamped = confidence.clamp(0.0, 1.0);
        // API uses `f32`; values are within [0,1] after clamp.
        #[expect(clippy::cast_possible_truncation)]
        {
            clamped as f32
        }
    }

    fn generate_replacement(
        &self,
        pattern: &NestGateMigrationPattern,
        captures: &regex::Captures,
    ) -> String {
        let mut replacement = pattern.replacement.clone();

        // Replace capture groups
        for (i, capture) in captures.iter().enumerate() {
            if let Some(capture) = capture {
                replacement = replacement.replace(&format!("${i}"), capture.as_str());
            }
        }

        replacement
    }

    fn generate_reasoning(&self, pattern: &NestGateMigrationPattern, confidence: f32) -> String {
        format!(
            "Pattern '{}' matched with {:.1}% confidence. Category: {:?}, Safety: {:?}",
            pattern.name,
            confidence * 100.0,
            pattern.error_category,
            pattern.safety_level
        )
    }

    #[must_use]
    pub const fn get_stats(&self) -> &MigrationStats {
        &self.stats
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[tokio::test]
    async fn test_context_analyzer() -> RefinedResult<()> {
        let analyzer = NestGateContextAnalyzer::new()?;

        let mut temp_file = NamedTempFile::new()?;
        writeln!(temp_file, "use nestgate_core::NestGateResult;")?;
        writeln!(temp_file)?;
        writeln!(
            temp_file,
            "async fn test_function() -> NestGateResult<String> {{"
        )?;
        writeln!(temp_file, "    let value = some_operation().unwrap();")?;
        writeln!(temp_file, "    Ok(value)")?;
        writeln!(temp_file, "}}")?;

        let context = analyzer.analyze_context(temp_file.path(), 3).await?;

        assert!(context.has_nestgate_imports);
        assert_eq!(context.function_name.as_deref(), Some("test_function"));
        assert_eq!(context.file_kind, SourceFileKind::Other);
        Ok(())
    }

    #[tokio::test]
    async fn test_migration_patterns() -> RefinedResult<()> {
        let mut migrator = RefinedNestGateMigrator::new()?;

        let mut temp_file = NamedTempFile::new()?;
        writeln!(
            temp_file,
            "use nestgate_core::{{NestGateResult, NestGateError}};"
        )?;
        writeln!(temp_file)?;
        writeln!(temp_file, "fn load_config() -> NestGateResult<Config> {{")?;
        writeln!(
            temp_file,
            "    let config = fs::read_to_string(\"config.toml\").unwrap(); // config"
        )?;
        writeln!(temp_file, "    Ok(parse_config(config)?)")?;
        writeln!(temp_file, "}}")?;

        let candidates = migrator.analyze_file(temp_file.path()).await?;

        assert!(!candidates.is_empty());
        assert!(candidates[0].confidence > 0.8);
        assert_eq!(candidates[0].pattern_name, "config_unwrap");
        Ok(())
    }
}

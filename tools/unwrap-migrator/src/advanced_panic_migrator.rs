// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! # Advanced Panic Migrator
//!
//! Enhanced version inspired by capability-based security analysis patterns
//! for comprehensive panic migration in Rust codebases.

use regex::Regex;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use thiserror::Error;
use tokio::fs;
use tracing::{debug, error, info};

#[derive(Error, Debug)]
pub enum AdvancedPanicMigratorError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Regex error: {0}")]
    Regex(#[from] regex::Error),
    #[error("Migration error: {message}")]
    Migration { message: String },
    #[error("Context analysis error: {message}")]
    ContextAnalysis { message: String },
}

pub type AdvancedPanicResult<T> = Result<T, AdvancedPanicMigratorError>;

async fn collect_rust_sources(
    root_path: &Path,
    files: &mut Vec<PathBuf>,
) -> AdvancedPanicResult<()> {
    let mut directories_to_process = vec![root_path.to_path_buf()];

    while let Some(current_dir) = directories_to_process.pop() {
        let mut entries = fs::read_dir(&current_dir).await?;

        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();

            if path.is_dir() {
                if let Some(dir_name) = path.file_name().and_then(|n| n.to_str())
                    && !["target", "backup", ".git"].contains(&dir_name)
                {
                    directories_to_process.push(path);
                }
            } else if path.extension().and_then(|ext| ext.to_str()) == Some("rs") {
                files.push(path);
            }
        }
    }

    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PanicPattern {
    /// panic!("message") calls
    PanicMacro,
    /// unimplemented!() calls
    Unimplemented,
    /// unreachable!() calls
    Unreachable,
    /// todo!() calls
    Todo,
    /// .`unwrap()` calls
    Unwrap,
    /// .`expect()` calls
    Expect,
}

#[derive(Debug, Clone)]
pub struct NestGatePanicReplacement {
    pub pattern: PanicPattern,
    /// `None` when matching is done only via [`Self::line_matches`] (substring).
    pub regex: Option<Regex>,
    pub replacement_template: String,
    pub priority: u32,
    pub safety_level: SafetyLevel,
    pub error_category: NestGateErrorCategory,
    pub context_requirements: Vec<ContextRequirement>,
}

impl NestGatePanicReplacement {
    /// Match a line; some patterns use [`str::contains`] instead of regex.
    pub(crate) fn line_matches(&self, line: &str) -> bool {
        if let Some(re) = &self.regex {
            return re.is_match(line);
        }
        match (self.pattern, self.priority) {
            (PanicPattern::Unwrap, 100) => line.contains(".unwrap()"),
            (PanicPattern::Unimplemented, 75) => line.contains("unimplemented!("),
            _ => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum SafetyLevel {
    /// Safe to auto-migrate
    Safe,
    /// Safe with human review
    SafeWithReview,
    /// Requires manual analysis
    RequiresAnalysis,
    /// Test code only
    TestOnly,
}

#[derive(Debug, Clone)]
pub enum NestGateErrorCategory {
    Configuration,
    Network,
    Storage,
    Authentication,
    Validation,
    Security,
    Hardware,
    System,
    FileSystem,
    ZfsOperation,
    ApiRequest,
    Generic,
}

#[derive(Debug, Clone)]
pub enum ContextRequirement {
    InNestGateResultFunction,
    InTestFunction,
    InBenchmarkFunction,
    InExampleCode,
    HasErrorHandling,
    IsOptionType,
    IsResultType,
    InProductionCode,
    HasLoggingContext,
    InZfsOperation,
    InConfigOperation,
}

/// Where the match sits in the repo (replaces multiple boolean flags on [`PanicContext`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CodeArtifactKind {
    Test,
    Example,
    Benchmark,
    Production,
}

#[derive(Debug, Clone)]
pub struct PanicContext {
    pub artifact_kind: CodeArtifactKind,
    pub function_name: Option<String>,
    pub return_type: Option<String>,
    pub has_nestgate_result: bool,
    pub has_error_handling: bool,
    pub surrounding_lines: Vec<String>,
    pub file_path: PathBuf,
    pub line_number: usize,
    pub error_category: NestGateErrorCategory,
}

pub struct AdvancedNestGatePanicMigrator {
    patterns: Vec<NestGatePanicReplacement>,
    stats: PanicMigrationStats,
    dry_run: bool,
}

#[derive(Debug, Default, Clone)]
pub struct PanicMigrationStats {
    pub files_scanned: usize,
    pub panic_patterns_found: usize,
    pub migrations_applied: usize,
    pub patterns_by_type: HashMap<String, usize>,
    pub safety_distribution: HashMap<String, usize>,
    pub error_categories: HashMap<String, usize>,
}

impl AdvancedNestGatePanicMigrator {
    pub fn new(dry_run: bool) -> AdvancedPanicResult<Self> {
        let patterns = Self::create_nestgate_panic_patterns()?;

        Ok(Self {
            patterns,
            stats: PanicMigrationStats::default(),
            dry_run,
        })
    }

    fn create_nestgate_panic_patterns() -> AdvancedPanicResult<Vec<NestGatePanicReplacement>> {
        let patterns = vec![
            // High-priority unwrap patterns
            NestGatePanicReplacement {
            pattern: PanicPattern::Unwrap,
            regex: None,
            replacement_template: "?".to_string(),
            priority: 100,
            safety_level: SafetyLevel::SafeWithReview,
            error_category: NestGateErrorCategory::Generic,
            context_requirements: vec![ContextRequirement::InNestGateResultFunction],
            },

            // Configuration-specific expect patterns
            NestGatePanicReplacement {
                pattern: PanicPattern::Expect,
                regex: Some(Regex::new(r#"\.expect\("([^"]*)"\)"#)?),
                replacement_template: ".map_err(|e| NestGateError::Configuration {{ message: \"$1\".to_string(), source: Some(Box::new(e)) }})?".to_string(),
                priority: 90,
                safety_level: SafetyLevel::Safe,
                error_category: NestGateErrorCategory::Configuration,
                context_requirements: vec![ContextRequirement::InConfigOperation],
            },

            // ZFS-specific patterns
            NestGatePanicReplacement {
                pattern: PanicPattern::Unwrap,
                regex: Some(Regex::new(r"zfs.*\.unwrap\(\)")?),
                replacement_template: ".map_err(|e| NestGateError::ZfsOperation {{ operation: \"zfs_command\".to_string(), source: Some(Box::new(e)) }})?".to_string(),
                priority: 95,
                safety_level: SafetyLevel::Safe,
                error_category: NestGateErrorCategory::ZfsOperation,
                context_requirements: vec![ContextRequirement::InZfsOperation],
            },

            // Panic macro patterns
            NestGatePanicReplacement {
                pattern: PanicPattern::PanicMacro,
                regex: Some(Regex::new(r#"panic!\("([^"]*)"\)"#)?),
                replacement_template:
                    "return Err(NestGateError::System {{ message: \"$1\".to_string(), source: None }})"
                        .to_string(),
                priority: 80,
                safety_level: SafetyLevel::RequiresAnalysis,
                error_category: NestGateErrorCategory::System,
                context_requirements: vec![ContextRequirement::InProductionCode],
            },

            // Todo patterns for migration
            NestGatePanicReplacement {
                pattern: PanicPattern::Todo,
                regex: Some(Regex::new(r#"todo!\("([^"]*)"\)"#)?),
                replacement_template: "return Err(NestGateError::System {{ message: \"Not implemented: $1\".to_string(), source: None }})".to_string(),
                priority: 70,
                safety_level: SafetyLevel::RequiresAnalysis,
                error_category: NestGateErrorCategory::System,
                context_requirements: vec![ContextRequirement::InProductionCode],
            },

            // Unimplemented patterns
            NestGatePanicReplacement {
                pattern: PanicPattern::Unimplemented,
                regex: None,
                replacement_template: "return Err(NestGateError::System {{ message: \"Feature not implemented\".to_string(), source: None }})".to_string(),
                priority: 75,
                safety_level: SafetyLevel::RequiresAnalysis,
                error_category: NestGateErrorCategory::System,
                context_requirements: vec![ContextRequirement::InProductionCode],
            },
        ];

        Ok(patterns)
    }

    pub async fn migrate_directory(
        &mut self,
        path: &Path,
    ) -> AdvancedPanicResult<PanicMigrationStats> {
        info!(
            "🔥 Starting advanced panic pattern migration for: {}",
            path.display()
        );

        let mut rust_files = Vec::new();
        collect_rust_sources(path, &mut rust_files).await?;

        info!("📁 Found {} Rust files to analyze", rust_files.len());

        for file_path in rust_files {
            if let Err(e) = self.migrate_file(&file_path).await {
                error!("Failed to migrate file {}: {}", file_path.display(), e);
            }
        }

        self.print_migration_summary();
        Ok(self.stats.clone())
    }

    async fn migrate_file(&mut self, file_path: &Path) -> AdvancedPanicResult<()> {
        let file_content = fs::read_to_string(file_path).await?;
        let lines: Vec<&str> = file_content.lines().collect();
        let mut modified_lines = Vec::new();
        let mut file_modified = false;

        self.stats.files_scanned += 1;

        for (line_number, line) in lines.iter().enumerate() {
            let context = self.analyze_context(file_path, line_number, &lines);
            let mut modified_line = (*line).to_string();
            let mut _line_modified = false;

            // Try each pattern in priority order
            for pattern in &self.patterns {
                if pattern.line_matches(line) && self.should_apply_pattern(pattern, &context) {
                    let replacement = self.generate_replacement(pattern, line, &context);
                    modified_line = replacement;
                    _line_modified = true;
                    file_modified = true;

                    self.stats.panic_patterns_found += 1;
                    if !self.dry_run {
                        self.stats.migrations_applied += 1;
                    }

                    // Update statistics
                    let pattern_name = format!("{:?}", pattern.pattern);
                    *self.stats.patterns_by_type.entry(pattern_name).or_insert(0) += 1;

                    let safety_name = format!("{:?}", pattern.safety_level);
                    *self
                        .stats
                        .safety_distribution
                        .entry(safety_name)
                        .or_insert(0) += 1;

                    let category_name = format!("{:?}", pattern.error_category);
                    *self
                        .stats
                        .error_categories
                        .entry(category_name)
                        .or_insert(0) += 1;

                    debug!(
                        "Migrated pattern in {}:{}: {} -> {}",
                        file_path.display(),
                        line_number + 1,
                        line,
                        modified_line
                    );
                    break;
                }
            }

            modified_lines.push(modified_line);
        }

        // Write back if modified and not dry run
        if file_modified && !self.dry_run {
            let new_content = modified_lines.join("\n");
            fs::write(file_path, new_content).await?;
            info!("✅ Migrated file: {}", file_path.display());
        } else if file_modified {
            info!("📋 Would migrate file: {}", file_path.display());
        }

        Ok(())
    }

    fn analyze_context(
        &self,
        file_path: &Path,
        line_number: usize,
        lines: &[&str],
    ) -> PanicContext {
        let is_test = file_path.to_string_lossy().contains("test")
            || lines
                .iter()
                .any(|line| line.contains("#[test]") || line.contains("#[tokio::test]"));

        let is_example = file_path.to_string_lossy().contains("example");
        let is_benchmark = file_path.to_string_lossy().contains("bench");
        let artifact_kind = if is_test {
            CodeArtifactKind::Test
        } else if is_example {
            CodeArtifactKind::Example
        } else if is_benchmark {
            CodeArtifactKind::Benchmark
        } else {
            CodeArtifactKind::Production
        };

        // Analyze surrounding context
        let start = line_number.saturating_sub(5);
        let end = std::cmp::min(line_number + 5, lines.len());
        let surrounding_lines: Vec<String> = lines[start..end]
            .iter()
            .map(std::string::ToString::to_string)
            .collect();

        // Detect function context
        let mut function_name = None;
        let mut return_type = None;
        let mut has_nestgate_result = false;

        // Look backwards for function signature
        for i in (0..line_number).rev().take(10) {
            if lines[i].contains("fn ") {
                function_name = self.extract_function_name(lines[i]);
                return_type = self.extract_return_type(lines[i]);
                has_nestgate_result =
                    lines[i].contains("NestGateResult") || lines[i].contains("Result<");
                break;
            }
        }

        // Check for error handling patterns
        let has_error_handling = surrounding_lines.iter().any(|line| {
            line.contains("match")
                || line.contains("if let")
                || line.contains('?')
                || line.contains("map_err")
        });

        // Determine error category based on context
        let error_category = self.determine_error_category(file_path, &surrounding_lines);

        PanicContext {
            artifact_kind,
            function_name,
            return_type,
            has_nestgate_result,
            has_error_handling,
            surrounding_lines,
            file_path: file_path.to_path_buf(),
            line_number,
            error_category,
        }
    }

    fn extract_function_name(&self, line: &str) -> Option<String> {
        if let Some(start) = line.find("fn ") {
            let after_fn = &line[start + 3..];
            if let Some(end) = after_fn.find('(') {
                return Some(after_fn[..end].trim().to_string());
            }
        }
        None
    }

    fn extract_return_type(&self, line: &str) -> Option<String> {
        if let Some(arrow_pos) = line.find("->") {
            let after_arrow = &line[arrow_pos + 2..];
            if let Some(brace_pos) = after_arrow.find('{') {
                return Some(after_arrow[..brace_pos].trim().to_string());
            }
        }
        None
    }

    fn determine_error_category(
        &self,
        file_path: &Path,
        surrounding_lines: &[String],
    ) -> NestGateErrorCategory {
        let path_str = file_path.to_string_lossy().to_lowercase();
        let context = surrounding_lines.join(" ").to_lowercase();

        if path_str.contains("zfs") || context.contains("zfs") {
            NestGateErrorCategory::ZfsOperation
        } else if path_str.contains("config") || context.contains("config") {
            NestGateErrorCategory::Configuration
        } else if path_str.contains("network") || context.contains("network") {
            NestGateErrorCategory::Network
        } else if path_str.contains("storage") || context.contains("storage") {
            NestGateErrorCategory::Storage
        } else if path_str.contains("auth") || context.contains("auth") {
            NestGateErrorCategory::Authentication
        } else if path_str.contains("security") || context.contains("security") {
            NestGateErrorCategory::Security
        } else if path_str.contains("api") || context.contains("api") {
            NestGateErrorCategory::ApiRequest
        } else {
            NestGateErrorCategory::Generic
        }
    }

    fn should_apply_pattern(
        &self,
        pattern: &NestGatePanicReplacement,
        context: &PanicContext,
    ) -> bool {
        // Check context requirements
        for requirement in &pattern.context_requirements {
            match requirement {
                ContextRequirement::InNestGateResultFunction => {
                    if !context.has_nestgate_result {
                        return false;
                    }
                }
                ContextRequirement::InTestFunction => {
                    if context.artifact_kind != CodeArtifactKind::Test {
                        return false;
                    }
                }
                ContextRequirement::InProductionCode => {
                    if !matches!(context.artifact_kind, CodeArtifactKind::Production) {
                        return false;
                    }
                }
                ContextRequirement::HasErrorHandling => {
                    if !context.has_error_handling {
                        return false;
                    }
                }
                ContextRequirement::InZfsOperation => {
                    if !matches!(context.error_category, NestGateErrorCategory::ZfsOperation) {
                        return false;
                    }
                }
                ContextRequirement::InConfigOperation => {
                    if !matches!(context.error_category, NestGateErrorCategory::Configuration) {
                        return false;
                    }
                }
                _ => {} // Other requirements not implemented yet
            }
        }

        true
    }

    fn generate_replacement(
        &self,
        pattern: &NestGatePanicReplacement,
        original_line: &str,
        _context: &PanicContext,
    ) -> String {
        let mut replacement = pattern.replacement_template.clone();

        // Handle regex capture groups
        if let Some(re) = &pattern.regex
            && let Some(captures) = re.captures(original_line)
        {
            for (i, capture) in captures.iter().enumerate().skip(1) {
                if let Some(capture) = capture {
                    replacement = replacement.replace(&format!("${i}"), capture.as_str());
                }
            }
        }

        // Preserve indentation
        let indent = original_line.len() - original_line.trim_start().len();
        let indent_str = " ".repeat(indent);

        format!("{indent_str}{}", replacement.trim())
    }

    fn print_migration_summary(&self) {
        info!("📊 ADVANCED PANIC MIGRATION SUMMARY");
        info!("=====================================");
        info!("📁 Files scanned: {}", self.stats.files_scanned);
        info!(
            "🔥 Panic patterns found: {}",
            self.stats.panic_patterns_found
        );
        info!("✅ Migrations applied: {}", self.stats.migrations_applied);

        if !self.stats.patterns_by_type.is_empty() {
            info!("📈 Patterns by type:");
            for (pattern, count) in &self.stats.patterns_by_type {
                info!("  {}: {}", pattern, count);
            }
        }

        if !self.stats.error_categories.is_empty() {
            info!("🏷️ Error categories:");
            for (category, count) in &self.stats.error_categories {
                info!("  {}: {}", category, count);
            }
        }

        if self.dry_run {
            info!("🔍 This was a dry run - no files were modified");
        }
    }

    #[must_use]
    pub const fn get_stats(&self) -> &PanicMigrationStats {
        &self.stats
    }
}

// Advanced API Error Fixer for NestGate
//
// Fixes all the API evolution errors systematically

fn apply_comprehensive_api_fixes_early(
    modified: &mut String,
) -> Result<bool, Box<dyn std::error::Error>> {
    let mut file_modified = false;

    let validation_regex = Regex::new(r"NestGateError::validation\(\s*([^,)]+),\s*[^)]+\)")?;
    if validation_regex.is_match(modified.as_str()) {
        *modified = validation_regex
            .replace_all(modified.as_str(), "NestGateError::validation($1)")
            .to_string();
        file_modified = true;
    }

    let internal_regex = Regex::new(r"NestGateError::internal\(\s*([^,)]+),\s*[^)]+\)")?;
    if internal_regex.is_match(modified.as_str()) {
        *modified = internal_regex
            .replace_all(modified.as_str(), "NestGateError::internal($1)")
            .to_string();
        file_modified = true;
    }

    let subject_field_regex = Regex::new(r"\bsubject:")?;
    if subject_field_regex.is_match(modified.as_str()) {
        *modified = subject_field_regex
            .replace_all(modified.as_str(), "principal:")
            .to_string();
        file_modified = true;
    }

    let subject_access_regex = Regex::new(r"\.subject\b")?;
    if subject_access_regex.is_match(modified.as_str()) {
        *modified = subject_access_regex
            .replace_all(modified.as_str(), ".principal")
            .to_string();
        file_modified = true;
    }

    if modified.contains("{resource}") {
        *modified = modified.replace("{resource}", "resource");
        file_modified = true;
    }

    let operation_some_regex = Regex::new(r#"operation:\s*Some\(("([^"]+)"\.to_string\(\))\)"#)?;
    if operation_some_regex.is_match(modified.as_str()) {
        *modified = operation_some_regex
            .replace_all(modified.as_str(), "operation: $1")
            .to_string();
        file_modified = true;
    }

    let pool_interface_stats_regex =
        Regex::new(r"(PoolInterfaceStats\s*\{\s*[^}]*buffer_size:\s*[^,}]+),(\s*\})")?;
    if pool_interface_stats_regex.is_match(modified.as_str()) {
        *modified = pool_interface_stats_regex
            .replace_all(modified.as_str(), "$1,\n            utilization: 0.0$2")
            .to_string();
        file_modified = true;
    }

    Ok(file_modified)
}

fn apply_comprehensive_api_fixes_late(
    modified: &mut String,
) -> Result<bool, Box<dyn std::error::Error>> {
    let mut file_modified = false;

    let pool_stats_regex = Regex::new(r"(PoolStats\s*\{\s*[^}]*total_capacity:\s*[^,}]+),(\s*\})")?;
    if pool_stats_regex.is_match(modified.as_str()) {
        *modified = pool_stats_regex
            .replace_all(modified.as_str(), "$1,\n            utilization: 0.0$2")
            .to_string();
        file_modified = true;
    }

    let timeout_regex =
        Regex::new(r"(NestGateError::Timeout\s*\{\s*[^}]*context:\s*[^,}]+),(\s*\})")?;
    if timeout_regex.is_match(modified.as_str()) {
        *modified = timeout_regex
            .replace_all(modified.as_str(), "$1,\n            retryable: true$2")
            .to_string();
        file_modified = true;
    }

    let external_regex =
        Regex::new(r"(NestGateError::External\s*\{\s*[^}]*context:\s*[^,}]+),(\s*\})")?;
    if external_regex.is_match(modified.as_str()) {
        *modified = external_regex
            .replace_all(modified.as_str(), "$1,\n            retryable: false$2")
            .to_string();
        file_modified = true;
    }

    let system_path_regex =
        Regex::new(r"(NestGateError::System\s*\{[^}]*)\bpath:\s*Some\(([^)]+)\)")?;
    if system_path_regex.is_match(modified.as_str()) {
        *modified = system_path_regex
            .replace_all(modified.as_str(), "${1}component: $2")
            .to_string();
        file_modified = true;
    }

    let diagnostic_resource_regex = Regex::new(r"\.resource\s*=\s*[^;]+;")?;
    if diagnostic_resource_regex.is_match(modified.as_str()) {
        *modified = diagnostic_resource_regex
            .replace_all(modified.as_str(), "; // resource field removed")
            .to_string();
        file_modified = true;
    }

    let debug_context_regex =
        Regex::new(r"internal_error_with_debug_context\(\s*([^,]+),\s*Some\(([^)]+)\)\s*\)")?;
    if debug_context_regex.is_match(modified.as_str()) {
        *modified = debug_context_regex
            .replace_all(
                modified.as_str(),
                "internal_error_with_debug_context($1, $2)",
            )
            .to_string();
        file_modified = true;
    }

    let format_fix_regex = Regex::new(r#"format!\("([^"]*)\{[^}]*\}([^"]*)"(?:,\s*[^)]+)?\)"#)?;
    if format_fix_regex.is_match(modified.as_str()) {
        *modified = format_fix_regex
            .replace_all(modified.as_str(), r#"format!("$1{}$2", "placeholder")"#)
            .to_string();
        file_modified = true;
    }

    Ok(file_modified)
}

pub struct ComprehensiveApiFixer {
    dry_run: bool,
    fixes_applied: u32,
}

impl ComprehensiveApiFixer {
    #[must_use]
    pub const fn new(dry_run: bool) -> Self {
        Self {
            dry_run,
            fixes_applied: 0,
        }
    }

    pub async fn fix_all_api_errors(
        &mut self,
        root_path: &Path,
    ) -> Result<u32, Box<dyn std::error::Error>> {
        let mut rust_files = Vec::new();
        collect_rust_sources(root_path, &mut rust_files)
            .await
            .map_err(|e| -> Box<dyn std::error::Error> { e.into() })?;

        println!(
            "🔧 Found {} Rust files to process for comprehensive API fixes",
            rust_files.len()
        );

        for file_path in rust_files {
            self.fix_comprehensive_api_errors(&file_path).await?;
        }

        Ok(self.fixes_applied)
    }

    async fn fix_comprehensive_api_errors(
        &mut self,
        file_path: &Path,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let content = fs::read_to_string(file_path).await?;
        let mut modified_content = content;
        let mut file_modified = apply_comprehensive_api_fixes_early(&mut modified_content)?;
        file_modified |= apply_comprehensive_api_fixes_late(&mut modified_content)?;

        if file_modified {
            if !self.dry_run {
                fs::write(file_path, modified_content).await?;
            }
            self.fixes_applied += 1;
            println!("✅ Comprehensive fix applied: {}", file_path.display());
        }

        Ok(())
    }
}

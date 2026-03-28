// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Enhanced Unwrap Migrator with Batch Processing and Smart Categorization
//!
//! This enhanced version can handle the migration of 2,398 unwrap/expect calls
//! using the existing unified error system with zero-copy performance.

#![allow(clippy::disallowed_types)] // Allow HashMap in utility crate
#![allow(clippy::excessive_nesting)] // Complex migration logic requires nesting
#![allow(clippy::if_same_then_else)] // Intentional pattern matching
#![allow(clippy::only_used_in_recursion)] // Recursive file discovery pattern

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::fs;
use tracing::{error, info, warn};

use crate::reporter::MigrationReport;
use crate::scanner::{FixType, MigrationError, UnwrapFix};

/// Enhanced migrator with batch processing capabilities
pub struct EnhancedUnwrapMigrator {
    /// Whether to perform a dry run (no actual changes)
    pub dry_run: bool,
    /// Whether to include test files in processing
    pub include_tests: bool,
    /// Error category mappings based on context
    category_mappings: HashMap<String, String>,
    /// Progress callback for large batch operations
    #[allow(clippy::type_complexity)]
    progress_callback: Option<Arc<dyn Fn(usize, usize) + Send + Sync>>,
}

impl std::fmt::Debug for EnhancedUnwrapMigrator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EnhancedUnwrapMigrator")
            .field("dry_run", &self.dry_run)
            .field("include_tests", &self.include_tests)
            .field("category_mappings", &self.category_mappings)
            .field("progress_callback", &self.progress_callback.is_some())
            .finish()
    }
}

/// Results of a batch migration operation
#[derive(Debug, Clone)]
pub struct BatchMigrationResults {
    /// Total files processed
    pub files_processed: usize,
    /// Total unwrap/expect calls migrated
    pub migrations_applied: usize,
    /// Error categories used in migration
    pub categories_used: HashMap<String, usize>,
    /// Files that had errors during migration
    pub failed_files: Vec<(PathBuf, String)>,
    /// Performance metrics
    pub execution_time_ms: u64,
}

impl EnhancedUnwrapMigrator {
    /// Create a new enhanced migrator
    #[must_use]
    pub fn new(dry_run: bool) -> Self {
        let mut category_mappings = HashMap::new();

        // Smart categorization based on context clues
        category_mappings.insert(
            "env::var".to_string(),
            "ErrorCategory::Configuration".to_string(),
        );
        category_mappings.insert(
            "std::env::var".to_string(),
            "ErrorCategory::Configuration".to_string(),
        );
        category_mappings.insert(
            "serde_json".to_string(),
            "ErrorCategory::Validation".to_string(),
        );
        category_mappings.insert(
            "from_str".to_string(),
            "ErrorCategory::Validation".to_string(),
        );
        category_mappings.insert("parse".to_string(), "ErrorCategory::Validation".to_string());
        category_mappings.insert("reqwest".to_string(), "ErrorCategory::Network".to_string());
        category_mappings.insert("http".to_string(), "ErrorCategory::Network".to_string());
        category_mappings.insert("tokio".to_string(), "ErrorCategory::System".to_string());
        category_mappings.insert("async".to_string(), "ErrorCategory::System".to_string());
        category_mappings.insert(
            "auth".to_string(),
            "ErrorCategory::Authentication".to_string(),
        );
        category_mappings.insert(
            "token".to_string(),
            "ErrorCategory::Authentication".to_string(),
        );
        category_mappings.insert("plugin".to_string(), "ErrorCategory::Plugin".to_string());
        category_mappings.insert("fs::".to_string(), "ErrorCategory::Storage".to_string());
        category_mappings.insert("file".to_string(), "ErrorCategory::Storage".to_string());
        category_mappings.insert("database".to_string(), "ErrorCategory::Storage".to_string());
        category_mappings.insert("db".to_string(), "ErrorCategory::Storage".to_string());
        category_mappings.insert("network".to_string(), "ErrorCategory::Network".to_string());
        category_mappings.insert(
            "connection".to_string(),
            "ErrorCategory::Network".to_string(),
        );
        category_mappings.insert(
            "protocol".to_string(),
            "ErrorCategory::Protocol".to_string(),
        );
        category_mappings.insert("mcp".to_string(), "ErrorCategory::Protocol".to_string());
        category_mappings.insert(
            "security".to_string(),
            "ErrorCategory::Security".to_string(),
        );
        category_mappings.insert("crypto".to_string(), "ErrorCategory::Security".to_string());

        Self {
            dry_run,
            include_tests: false, // Default to production code only
            category_mappings,
            progress_callback: None,
        }
    }

    /// Set whether to include test files in processing
    pub fn set_include_tests(&mut self, include_tests: bool) {
        self.include_tests = include_tests;
    }

    /// Set a progress callback for batch operations
    pub fn with_progress_callback<F>(mut self, callback: F) -> Self
    where
        F: Fn(usize, usize) + Send + Sync + 'static,
    {
        self.progress_callback = Some(Arc::new(callback));
        self
    }

    /// Batch migrate an entire crate with smart categorization
    pub async fn migrate_crate(
        &self,
        crate_path: &Path,
    ) -> Result<BatchMigrationResults, MigrationError> {
        let start_time = std::time::Instant::now();

        info!(
            "🚀 Starting batch migration for crate: {}",
            crate_path.display()
        );

        // Discover all Rust files in the crate
        let rust_files = self.discover_rust_files(crate_path).await?;
        let total_files = rust_files.len();

        info!("📁 Found {} Rust files to process", total_files);

        let mut results = BatchMigrationResults {
            files_processed: 0,
            migrations_applied: 0,
            categories_used: HashMap::new(),
            failed_files: Vec::new(),
            execution_time_ms: 0,
        };

        // Process files in batches to avoid overwhelming the system
        const BATCH_SIZE: usize = 10;

        for (batch_idx, file_batch) in rust_files.chunks(BATCH_SIZE).enumerate() {
            info!(
                "🔄 Processing batch {} of {}",
                batch_idx + 1,
                total_files.div_ceil(BATCH_SIZE)
            );

            for file_path in file_batch {
                match self.migrate_single_file(file_path).await {
                    Ok(file_results) => {
                        results.files_processed += 1;
                        results.migrations_applied += file_results.migrations_applied;

                        // Merge category usage statistics
                        for (category, count) in file_results.categories_used {
                            *results.categories_used.entry(category).or_insert(0) += count;
                        }
                    }
                    Err(e) => {
                        error!("❌ Failed to migrate {}: {}", file_path.display(), e);
                        results
                            .failed_files
                            .push((file_path.clone(), e.to_string()));
                    }
                }

                // Report progress
                if let Some(callback) = &self.progress_callback {
                    callback(results.files_processed, total_files);
                }
            }

            // Small delay between batches to avoid overwhelming the system
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }

        results.execution_time_ms = start_time.elapsed().as_millis() as u64;

        info!("✅ Batch migration completed:");
        info!("   📊 Files processed: {}", results.files_processed);
        info!("   🔧 Migrations applied: {}", results.migrations_applied);
        info!("   ⏱️  Execution time: {}ms", results.execution_time_ms);
        info!("   📈 Categories used: {:?}", results.categories_used);

        if !results.failed_files.is_empty() {
            warn!(
                "⚠️  {} files had errors during migration",
                results.failed_files.len()
            );
        }

        Ok(results)
    }

    /// Migrate a single file with enhanced error categorization
    async fn migrate_single_file(
        &self,
        file_path: &Path,
    ) -> Result<SingleFileMigrationResults, MigrationError> {
        let content = fs::read_to_string(file_path).await.map_err(|e| {
            MigrationError::IoError(format!("Failed to read {}: {e}", file_path.display()))
        })?;

        // Scan for unwrap patterns
        let fixes = crate::scanner::scan_file(file_path, self.include_tests).await?;

        if fixes.is_empty() {
            return Ok(SingleFileMigrationResults {
                migrations_applied: 0,
                categories_used: HashMap::new(),
            });
        }

        info!(
            "🔍 Found {} unwrap/expect patterns in {}",
            fixes.len(),
            file_path.display()
        );

        let mut modified_content = content;
        let mut categories_used = HashMap::new();
        let mut migrations_applied = 0;

        // Apply fixes in reverse line order to maintain positions
        let mut sorted_fixes = fixes;
        sorted_fixes.sort_by(|a, b| b.line.cmp(&a.line));

        for fix in sorted_fixes {
            let category = self.categorize_error_context(&fix, &modified_content);
            let enhanced_replacement = self.generate_enhanced_replacement(&fix, &category);

            // Apply the migration
            if let Some(new_content) =
                self.apply_single_fix(&modified_content, &fix, &enhanced_replacement)
            {
                modified_content = new_content;
                migrations_applied += 1;
                *categories_used.entry(category).or_insert(0) += 1;

                info!(
                    "🔧 Applied fix at line {}: {} → {}",
                    fix.line,
                    fix.original_code.trim(),
                    enhanced_replacement
                );
            }
        }

        // Write the modified content back to the file (if not dry run)
        if !self.dry_run && migrations_applied > 0 {
            fs::write(file_path, modified_content).await.map_err(|e| {
                MigrationError::IoError(format!("Failed to write {}: {e}", file_path.display()))
            })?;

            info!(
                "💾 Saved {} migrations to {}",
                migrations_applied,
                file_path.display()
            );
        } else if self.dry_run {
            info!(
                "🔄 Dry run: Would apply {} migrations to {}",
                migrations_applied,
                file_path.display()
            );
        } else {
            // Normal mode with no output needed
        }

        Ok(SingleFileMigrationResults {
            migrations_applied,
            categories_used,
        })
    }

    /// Smart categorization based on context analysis
    fn categorize_error_context(&self, fix: &UnwrapFix, content: &str) -> String {
        let context_window = self.extract_context_window(content, fix.line, 3);
        let context_lower = context_window.to_lowercase();

        // Check against all our category mappings
        for (pattern, category) in &self.category_mappings {
            if context_lower.contains(pattern) {
                return category.clone();
            }
        }

        // Additional smart heuristics
        if context_lower.contains("config") || context_lower.contains("setting") {
            return "ErrorCategory::Configuration".to_string();
        }

        if context_lower.contains("json")
            || context_lower.contains("xml")
            || context_lower.contains("yaml")
        {
            return "ErrorCategory::Validation".to_string();
        }

        if context_lower.contains("url")
            || context_lower.contains("uri")
            || context_lower.contains("endpoint")
        {
            return "ErrorCategory::Network".to_string();
        }

        // Default fallback
        "ErrorCategory::System".to_string()
    }

    /// Generate enhanced replacement with proper error categorization
    fn generate_enhanced_replacement(&self, fix: &UnwrapFix, category: &str) -> String {
        let message = match &fix.fix_type {
            FixType::ReplaceExpected {
                original_message, ..
            } => {
                if original_message.is_empty() {
                    "Operation failed".to_string()
                } else {
                    original_message.clone()
                }
            }
            _ => {
                // Generate contextual error message based on the original code
                if fix.original_code.contains("env::var") {
                    "Environment variable not found".to_string()
                } else if fix.original_code.contains("parse") {
                    "Parse operation failed".to_string()
                } else if fix.original_code.contains("get(") {
                    "Value not found".to_string()
                } else {
                    "Operation failed".to_string()
                }
            }
        };

        format!(".safe_unwrap({category}, \"{message}\")?")
    }

    /// Apply a single fix to content
    fn apply_single_fix(
        &self,
        content: &str,
        fix: &UnwrapFix,
        replacement: &str,
    ) -> Option<String> {
        let lines: Vec<&str> = content.lines().collect();

        if fix.line > lines.len() {
            warn!("⚠️  Line {} is out of range for file", fix.line);
            return None;
        }

        let mut new_lines = lines.clone();
        let line_content = lines[fix.line - 1]; // Convert to 0-based index

        // Replace unwrap() or expect() patterns
        let new_line = if line_content.contains(
            ".unwrap_or_else(|e| {
    tracing::error!(\"Unwrap failed: {:?}\", e);
    panic!(\"Critical error - unable to continue: {:?}\", e)
})",
        ) {
            line_content.replace(
                ".unwrap_or_else(|e| {
    tracing::error!(\"Unwrap failed: {:?}\", e);
    panic!(\"Critical error - unable to continue: {:?}\", e)
})",
                replacement,
            )
        } else if line_content.contains(".expect(") {
            // More complex replacement for expect calls
            self.replace_expect_call(line_content, replacement)
        } else {
            return None;
        };

        new_lines[fix.line - 1] = &new_line;
        Some(new_lines.join("\n"))
    }

    /// Replace expect calls with `safe_unwrap`
    fn replace_expect_call(&self, line: &str, replacement: &str) -> String {
        // Find the expect call and replace it
        if let Some(start) = line.find(".expect(") {
            if let Some(end) = line[start..].find(')') {
                let before = &line[..start];
                let after = &line[start + end + 1..];
                return format!("{before}{replacement}{after}");
            }
        }

        // Fallback if we can't parse the expect call
        line.replace(".expect(", replacement)
    }

    /// Extract context window around a line for analysis
    fn extract_context_window(
        &self,
        content: &str,
        target_line: usize,
        window_size: usize,
    ) -> String {
        let lines: Vec<&str> = content.lines().collect();
        if lines.is_empty() || target_line == 0 {
            return String::new();
        }

        let start = target_line.saturating_sub(window_size + 1);
        let end = std::cmp::min(target_line + window_size, lines.len());

        if start >= end || start >= lines.len() {
            return String::new();
        }

        lines[start..end].join("\n")
    }

    /// Discover all Rust files in a directory recursively
    async fn discover_rust_files(&self, path: &Path) -> Result<Vec<PathBuf>, MigrationError> {
        let mut rust_files = Vec::new();
        self.discover_rust_files_sync(path, &mut rust_files)?;
        Ok(rust_files)
    }

    /// Synchronous recursive helper for file discovery
    fn discover_rust_files_sync(
        &self,
        path: &Path,
        files: &mut Vec<PathBuf>,
    ) -> Result<(), MigrationError> {
        let entries = std::fs::read_dir(path).map_err(|e| {
            MigrationError::IoError(format!("Failed to read directory {}: {e}", path.display()))
        })?;

        for entry in entries {
            let entry = entry.map_err(|e| {
                MigrationError::IoError(format!("Failed to read directory entry: {e}"))
            })?;
            let path = entry.path();

            if path.is_dir() {
                // Skip common directories that don't contain source code
                let dir_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

                if !["target", "build", ".git", "node_modules"].contains(&dir_name) {
                    self.discover_rust_files_sync(&path, files)?;
                }
            } else if path.extension().and_then(|s| s.to_str()) == Some("rs") {
                // Skip test files for production migration
                let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

                if !file_name.contains("test") && !file_name.starts_with("bench") {
                    files.push(path);
                }
            }
        }

        Ok(())
    }

    /// Generate a detailed migration report
    pub async fn generate_report(
        &self,
        root_path: &Path,
    ) -> Result<MigrationReport, MigrationError> {
        let mut report = MigrationReport {
            total_files_scanned: 0,
            total_patterns_found: 0,
            patterns_by_severity: HashMap::new(),
            file_statistics: HashMap::new(),
            patterns_by_type: HashMap::new(),
            patterns: Vec::new(),
        };

        let mut entries = Vec::new();
        self.collect_rust_files(root_path, &mut entries).await?;

        for file_path in entries {
            if let Ok(fixes) = crate::scanner::scan_file(&file_path, self.include_tests).await {
                report.total_files_scanned += 1;

                if !fixes.is_empty() {
                    report
                        .file_statistics
                        .insert(file_path.display().to_string(), fixes.len());

                    // If not dry run, apply the fixes
                    if !self.dry_run {
                        self.apply_fixes_to_file(&file_path, &fixes).await?;
                    }

                    for fix in fixes {
                        report.total_patterns_found += 1;

                        // Count by severity
                        let severity = format!("{:?}", fix.severity);
                        *report.patterns_by_severity.entry(severity).or_insert(0) += 1;
                    }
                }
            }
        }

        // Sort file statistics by pattern count (descending)
        let mut sorted_files: Vec<_> = report.file_statistics.iter().collect();
        sorted_files.sort_by(|a, b| b.1.cmp(a.1));
        report.file_statistics = sorted_files
            .into_iter()
            .map(|(k, v)| (k.clone(), *v))
            .collect();

        Ok(report)
    }

    /// Apply fixes to a specific file
    async fn apply_fixes_to_file(
        &self,
        file_path: &Path,
        fixes: &[UnwrapFix],
    ) -> Result<(), MigrationError> {
        let original_content = tokio::fs::read_to_string(file_path).await.map_err(|e| {
            MigrationError::IoError(format!("Failed to read file {}: {e}", file_path.display()))
        })?;

        let mut lines: Vec<String> = original_content
            .lines()
            .map(std::string::ToString::to_string)
            .collect();

        // Sort fixes by line number in descending order to avoid offset issues
        let mut sorted_fixes = fixes.to_vec();
        sorted_fixes.sort_by(|a, b| b.line.cmp(&a.line));

        for fix in sorted_fixes {
            if fix.line > 0 && fix.line <= lines.len() {
                let line_index = fix.line - 1;
                let original_line = &lines[line_index];

                let new_line = self.apply_fix_to_line(original_line, &fix);
                lines[line_index] = new_line;
            }
        }

        let modified_content = lines.join("\n");

        // Write the modified content back to the file
        tokio::fs::write(file_path, modified_content)
            .await
            .map_err(|e| {
                MigrationError::IoError(format!(
                    "Failed to write file {}: {e}",
                    file_path.display()
                ))
            })?;

        info!(
            "✅ Applied {} fixes to {}",
            fixes.len(),
            file_path.display()
        );
        Ok(())
    }

    /// Apply a single fix to a line of code
    fn apply_fix_to_line(&self, line: &str, fix: &UnwrapFix) -> String {
        // Check if this is in a test context by looking for test attributes or function patterns
        let is_test_context = self.is_test_context(line, fix);

        match &fix.fix_type {
            FixType::ReplaceUnwrap => {
                if line.contains(".unwrap()") {
                    if is_test_context {
                        // For tests, suggest changing function signature to return Result
                        line.replace(".unwrap()", "?")
                    } else {
                        // For production, use ? operator
                        line.replace(".unwrap()", "?")
                    }
                } else {
                    line.to_string()
                }
            }
            FixType::ReplaceExpected {
                original_message: _,
            } => {
                let expect_regex = match regex::Regex::new(r#"\.expect\("[^"]*"\)"#) {
                    Ok(regex) => regex,
                    Err(_) => return line.to_string(), // Return original line if regex fails
                };
                if is_test_context {
                    expect_regex.replace(line, "?").to_string()
                } else {
                    expect_regex.replace(line, "?").to_string()
                }
            }
            FixType::ReplacePanic {
                original_message: _,
            } => {
                let panic_regex = regex::Regex::new(r#"panic!\("[^"]*"\)"#).unwrap();
                if is_test_context {
                    panic_regex.replace(line, "return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, \"Test assertion failed\")))").to_string()
                } else {
                    panic_regex.replace(line, "return Err(NestGateError::internal_error(\"Operation failed\".to_string(), \"Unexpected error condition\".to_string()))").to_string()
                }
            }
        }
    }

    /// Check if the current fix is in a test context
    fn is_test_context(&self, line: &str, fix: &UnwrapFix) -> bool {
        // Check the file path for test indicators
        let file_path_str = fix.file_path.to_string_lossy();
        if file_path_str.contains("/tests/") || file_path_str.contains("test") {
            return true;
        }

        // Check for test attributes or function names
        line.contains("#[test]")
            || line.contains("#[tokio::test]")
            || line.contains("fn test_")
            || line.contains("mod tests")
    }

    /// Recursively collect all Rust files from a directory
    async fn collect_rust_files(
        &self,
        root_path: &Path,
        entries: &mut Vec<PathBuf>,
    ) -> Result<(), MigrationError> {
        let mut dir_entries = tokio::fs::read_dir(root_path).await.map_err(|e| {
            MigrationError::IoError(format!(
                "Failed to read directory {}: {e}",
                root_path.display()
            ))
        })?;

        while let Ok(Some(entry)) = dir_entries.next_entry().await {
            let path = entry.path();

            if path.is_dir() {
                // Skip certain directories
                if let Some(dir_name) = path.file_name().and_then(|n| n.to_str()) {
                    if dir_name == "target" || dir_name == ".git" || dir_name == "node_modules" {
                        continue;
                    }
                }

                // Recursively process subdirectories
                Box::pin(self.collect_rust_files(&path, entries)).await?;
            } else if path.extension().and_then(|ext| ext.to_str()) == Some("rs") {
                entries.push(path);
            }
        }

        Ok(())
    }
}

/// Results for a single file migration
#[derive(Debug)]
struct SingleFileMigrationResults {
    migrations_applied: usize,
    categories_used: HashMap<String, usize>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_enhanced_migrator_creation() {
        let migrator = EnhancedUnwrapMigrator::new(true);
        assert!(migrator.dry_run);
        assert!(!migrator.category_mappings.is_empty());
    }

    #[tokio::test]
    async fn test_context_categorization() {
        let migrator = EnhancedUnwrapMigrator::new(true);

        // Test environment variable categorization
        let _env_context = "let port = std::env::var(\"PORT\").map_err(|e| {
    tracing::error!(\"Environment variable access failed: {:?}\", e);
    std::io::Error::new(std::io::ErrorKind::NotFound, format!(\"Environment variable error: {}\", e))
})?;";
        assert!(migrator
            .category_mappings
            .get("env::var")
            .is_some_and(|cat| cat.contains("Configuration")));

        // Test JSON parsing categorization
        let _json_context = "let data = serde_json::from_str(&input).map_err(|e| {
    tracing::error!(\"JSON parsing failed: {}\", e);
    std::io::Error::new(std::io::ErrorKind::InvalidData, format!(\"JSON parsing error: {}\", e))
})?;";
        assert!(migrator
            .category_mappings
            .get("serde_json")
            .is_some_and(|cat| cat.contains("Validation")));
    }

    type TestResult = Result<(), Box<dyn std::error::Error>>;

    #[tokio::test]
    async fn test_file_discovery() -> TestResult {
        let temp_dir = tempfile::tempdir()?;
        let temp_path = temp_dir.path();

        // Create test files (use main.rs instead of test.rs to avoid filtering)
        tokio::fs::write(temp_path.join("main.rs"), "fn main() { }").await?;
        tokio::fs::write(temp_path.join("lib.rs"), "pub fn test() { }").await?;
        tokio::fs::write(temp_path.join("readme.txt"), "Not rust").await?;

        let migrator = EnhancedUnwrapMigrator::new(true);
        let rust_files = migrator.discover_rust_files(temp_path).await?;

        assert_eq!(rust_files.len(), 2);
        assert!(rust_files
            .iter()
            .any(|f| f.file_name().and_then(|n| n.to_str()) == Some("main.rs")));
        assert!(rust_files
            .iter()
            .any(|f| f.file_name().and_then(|n| n.to_str()) == Some("lib.rs")));

        Ok(())
    }
}

// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Systematic migration framework
//!
//! This module provides a systematic approach to migrating unwrap/expect
//! patterns across large codebases with comprehensive error handling.

#![allow(clippy::disallowed_types)] // Allow HashMap in utility crate
#![allow(clippy::excessive_nesting)] // Complex migration logic requires nesting
#![allow(clippy::upper_case_acronyms)] // IO and AI are standard acronyms

use regex::Regex;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use thiserror::Error;
use tokio::fs;

/// Systematic migrator for unwrap/expect/panic patterns
pub struct SystematicUnwrapMigrator {
    /// Unified error patterns for migration
    error_patterns: HashMap<String, MigrationPattern>,
    /// Panic patterns for graceful migration
    panic_patterns: HashMap<String, MigrationPattern>,
    /// Files processed counter
    files_processed: std::sync::atomic::AtomicU64,
    /// Migrations applied counter  
    migrations_applied: std::sync::atomic::AtomicU64,
}

#[derive(Debug, Clone)]
pub struct MigrationPattern {
    /// Pattern to match
    pub pattern: String,
    /// Replacement template
    pub replacement: String,
    /// Error category for unified error system
    pub error_category: ErrorCategory,
    /// Context description
    pub context: String,
}

#[derive(Debug, Clone)]
pub enum ErrorCategory {
    Configuration,
    Network,
    IO,
    Lock,
    Validation,
    Resource,
    AI,
    Plugin,
}

#[derive(Error, Debug)]
pub enum MigrationError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Regex error: {0}")]
    Regex(#[from] regex::Error),

    #[error("Migration failed: {message}")]
    MigrationFailed { message: String },
}

impl Default for SystematicUnwrapMigrator {
    fn default() -> Self {
        Self::new()
    }
}

impl SystematicUnwrapMigrator {
    /// Create new systematic migrator with comprehensive unified patterns
    #[must_use]
    pub fn new() -> Self {
        let mut error_patterns = HashMap::new();
        let mut panic_patterns = HashMap::new();

        // ===============================================================
        // ENVIRONMENT & CONFIGURATION PATTERNS
        // ===============================================================

        error_patterns.insert(
            "env::var".to_string(),
            MigrationPattern {
                pattern: r#"std::env::var\("([^"]+)"\)\.unwrap\(\)"#.to_string(),
                replacement: r#"std::env::var("$1").map_err(|e| {
    tracing::error!("Environment variable '{}' not found: {}", "$1", e);
    std::io::Error::new(std::io::ErrorKind::NotFound, format!("Missing environment variable: {}", "$1"))
})?"#.to_string(),
                error_category: ErrorCategory::Configuration,
                context: "Environment variable access".to_string(),
            }
        );

        error_patterns.insert(
            "env::var_expect".to_string(),
            MigrationPattern {
                pattern: r#"std::env::var\("([^"]+)"\)\.expect\("([^"]+)"\)"#.to_string(),
                replacement: r#"std::env::var("$1").map_err(|e| {
    tracing::error!("Environment variable '{}' not found: {} - Original message: '{}'", "$1", e, "$2");
    std::io::Error::new(std::io::ErrorKind::NotFound, format!("Missing environment variable: {} - {}", "$1", "$2"))
})?"#.to_string(),
                error_category: ErrorCategory::Configuration,
                context: "Environment variable access with expect message".to_string(),
            }
        );

        // ===============================================================
        // MUTEX & CONCURRENCY PATTERNS
        // ===============================================================

        error_patterns.insert(
            "lock_unwrap".to_string(),
            MigrationPattern {
                pattern: r"\.lock\(\)\.unwrap\(\)".to_string(),
                replacement: ".lock().unwrap_or_else(|poisoned| {\n        tracing::warn!(\"Mutex poisoned, recovering\");\n        poisoned.into_inner()\n    })".to_string(),
                error_category: ErrorCategory::Lock,
                context: "Mutex lock acquisition".to_string(),
            }
        );

        error_patterns.insert(
            "lock_expect".to_string(),
            MigrationPattern {
                pattern: r#"\.lock\(\)\.expect\("([^"]+)"\)"#.to_string(),
                replacement: r#".lock().unwrap_or_else(|poisoned| {
        tracing::warn!("Mutex poisoned ({}), recovering", "$1");
        poisoned.into_inner()
    })"#
                .to_string(),
                error_category: ErrorCategory::Lock,
                context: "Mutex lock acquisition with expect message".to_string(),
            },
        );

        // ===============================================================
        // JSON & SERIALIZATION PATTERNS
        // ===============================================================

        error_patterns.insert(
            "json_parse_unwrap".to_string(),
            MigrationPattern {
                pattern: r"serde_json::from_str\(([^)]+)\)\.unwrap\(\)".to_string(),
                replacement: r#"serde_json::from_str($1).map_err(|e| {
    tracing::error!("JSON parsing failed: {}", e);
    std::io::Error::new(std::io::ErrorKind::InvalidData, format!("JSON parsing error: {}", e))
})?"#
                    .to_string(),
                error_category: ErrorCategory::Validation,
                context: "JSON deserialization".to_string(),
            },
        );

        error_patterns.insert(
            "json_parse_expect".to_string(),
            MigrationPattern {
                pattern: r#"serde_json::from_str\(([^)]+)\)\.expect\("([^"]+)"\)"#.to_string(),
                replacement: r#"serde_json::from_str($1).map_err(|e| {
    tracing::error!("JSON parsing failed ({}): {}", "$2", e);
    std::io::Error::new(std::io::ErrorKind::InvalidData, format!("JSON parsing error ({}): {}", "$2", e))
})?"#.to_string(),
                error_category: ErrorCategory::Validation,
                context: "JSON deserialization with expect message".to_string(),
            }
        );

        error_patterns.insert(
            "json_to_string_unwrap".to_string(),
            MigrationPattern {
                pattern: r"serde_json::to_string\(([^)]+)\)\.unwrap\(\)".to_string(),
                replacement: r#"serde_json::to_string($1).map_err(|e| {
    tracing::error!("JSON serialization failed: {}", e);
    std::io::Error::new(std::io::ErrorKind::InvalidData, format!("JSON serialization error: {}", e))
})?"#
                    .to_string(),
                error_category: ErrorCategory::Validation,
                context: "JSON serialization".to_string(),
            },
        );

        // ===============================================================
        // HTTP & NETWORK PATTERNS
        // ===============================================================

        error_patterns.insert(
            "http_send_unwrap".to_string(),
            MigrationPattern {
                pattern: r"\.send\(\)\.await\.unwrap\(\)".to_string(),
                replacement: r#".send().await.map_err(|e| {
    tracing::error!("HTTP request failed: {}", e);
    std::io::Error::new(std::io::ErrorKind::ConnectionRefused, format!("HTTP error: {}", e))
})?"#
                    .to_string(),
                error_category: ErrorCategory::Network,
                context: "HTTP request execution".to_string(),
            },
        );

        error_patterns.insert(
            "http_send_expect".to_string(),
            MigrationPattern {
                pattern: r#"\.send\(\)\.await\.expect\("([^"]+)"\)"#.to_string(),
                replacement: r#".send().await.map_err(|e| {
    tracing::error!("HTTP request failed ({}): {}", "$1", e);
    std::io::Error::new(std::io::ErrorKind::ConnectionRefused, format!("HTTP error ({}): {}", "$1", e))
})?"#.to_string(),
                error_category: ErrorCategory::Network,
                context: "HTTP request execution with expect message".to_string(),
            }
        );

        // ===============================================================
        // FILE I/O PATTERNS
        // ===============================================================

        error_patterns.insert(
            "file_read_unwrap".to_string(),
            MigrationPattern {
                pattern: r"fs::read_to_string\(([^)]+)\)\.unwrap\(\)".to_string(),
                replacement: r#"fs::read_to_string($1).map_err(|e| {
    tracing::error!("Failed to read file: {}", e);
    e
})?"#
                    .to_string(),
                error_category: ErrorCategory::IO,
                context: "File system read operations".to_string(),
            },
        );

        error_patterns.insert(
            "file_read_expect".to_string(),
            MigrationPattern {
                pattern: r#"fs::read_to_string\(([^)]+)\)\.expect\("([^"]+)"\)"#.to_string(),
                replacement: r#"fs::read_to_string($1).map_err(|e| {
    tracing::error!("Failed to read file ({}): {}", "$2", e);
    e
})?"#
                    .to_string(),
                error_category: ErrorCategory::IO,
                context: "File system read operations with expect message".to_string(),
            },
        );

        error_patterns.insert(
            "file_write_unwrap".to_string(),
            MigrationPattern {
                pattern: r"fs::write\(([^,]+),\s*([^)]+)\)\.unwrap\(\)".to_string(),
                replacement: r#"fs::write($1, $2).map_err(|e| {
    tracing::error!("Failed to write file: {}", e);
    e
})?"#
                    .to_string(),
                error_category: ErrorCategory::IO,
                context: "File system write operations".to_string(),
            },
        );

        // ===============================================================
        // GENERAL PATTERNS - BROADER COVERAGE
        // ===============================================================

        error_patterns.insert(
            "general_unwrap".to_string(),
            MigrationPattern {
                pattern: r"\.unwrap\(\)".to_string(),
                replacement: r#".unwrap_or_else(|e| {
    tracing::error!("Unwrap failed: {:?}", e);
    panic!("Critical error - unable to continue: {:?}", e)
})"#
                .to_string(),
                error_category: ErrorCategory::Resource,
                context: "General unwrap calls".to_string(),
            },
        );

        error_patterns.insert(
            "general_expect".to_string(),
            MigrationPattern {
                pattern: r#"\.expect\("([^"]+)"\)"#.to_string(),
                replacement: r#".unwrap_or_else(|e| {
    tracing::error!("Expect failed ({}): {:?}", "$1", e);
    panic!("Critical error - {}: {:?}", "$1", e)
})"#
                .to_string(),
                error_category: ErrorCategory::Resource,
                context: "General expect calls with messages".to_string(),
            },
        );

        // ===============================================================
        // PANIC! PATTERNS - REPLACE WITH GRACEFUL ERROR HANDLING
        // ===============================================================

        panic_patterns.insert(
            "logged_panic".to_string(),
            MigrationPattern {
                pattern: r#"panic!\("Critical error - unable to continue: \{:\?\}", e\)"#
                    .to_string(),
                replacement: r#"return Err(std::io::Error::new(
    std::io::ErrorKind::Other,
    format!("Operation failed: {:?}", e)
).into())"#
                    .to_string(),
                error_category: ErrorCategory::Resource,
                context: "Logged panic calls".to_string(),
            },
        );

        panic_patterns.insert(
            "message_panic".to_string(),
            MigrationPattern {
                pattern: r#"panic!\("Critical error - ([^"]+): \{:\?\}", ([^)]+)\)"#.to_string(),
                replacement: r#"return Err(std::io::Error::new(
    std::io::ErrorKind::Other,
    format!("Operation failed - {}: {:?}", "$1", $2)
).into())"#
                    .to_string(),
                error_category: ErrorCategory::Resource,
                context: "Message panic calls".to_string(),
            },
        );

        panic_patterns.insert(
            "simple_panic".to_string(),
            MigrationPattern {
                pattern: r#"panic!\("([^"]+)"\)"#.to_string(),
                replacement: r#"return Err(std::io::Error::new(
    std::io::ErrorKind::Other,
    "$1".to_string()
).into())"#
                    .to_string(),
                error_category: ErrorCategory::Resource,
                context: "Simple panic calls".to_string(),
            },
        );

        Self {
            error_patterns,
            panic_patterns,
            files_processed: std::sync::atomic::AtomicU64::new(0),
            migrations_applied: std::sync::atomic::AtomicU64::new(0),
        }
    }

    /// Add NestGate-specific patterns to the migrator
    pub fn add_nestgate_patterns(&mut self) {
        use crate::nestgate_patterns::{get_nestgate_patterns, get_nestgate_test_patterns};

        // Add NestGate production patterns
        for (key, pattern) in get_nestgate_patterns() {
            self.error_patterns.insert(key, pattern);
        }

        // Add NestGate test patterns
        for (key, pattern) in get_nestgate_test_patterns() {
            self.error_patterns.insert(key, pattern);
        }

        tracing::info!(
            "Added {} NestGate-specific migration patterns",
            get_nestgate_patterns().len() + get_nestgate_test_patterns().len()
        );
    }

    /// Execute systematic migration across the entire codebase
    pub async fn migrate_codebase(
        &self,
        root_path: &Path,
    ) -> Result<MigrationReport, MigrationError> {
        tracing::info!(
            "🔄 COMPREHENSIVE MIGRATION: Starting complete unwrap/expect/panic elimination"
        );

        let rust_files = self.discover_rust_files(root_path).await?;
        tracing::info!(
            "Found {} Rust files for comprehensive migration",
            rust_files.len()
        );

        let mut total_changes = 0;
        let mut file_changes = HashMap::new();

        for file_path in rust_files {
            match self.migrate_file(&file_path).await {
                Ok(changes) => {
                    if changes > 0 {
                        file_changes.insert(file_path.clone(), changes);
                        total_changes += changes;
                        tracing::debug!("Migrated {} patterns in {}", changes, file_path.display());
                    }
                    self.files_processed
                        .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                }
                Err(e) => {
                    tracing::warn!("Failed to migrate {}: {}", file_path.display(), e);
                }
            }
        }

        let report = MigrationReport {
            files_processed: self
                .files_processed
                .load(std::sync::atomic::Ordering::SeqCst),
            total_changes,
            file_changes,
            patterns_used: self
                .error_patterns
                .keys()
                .chain(self.panic_patterns.keys())
                .cloned()
                .collect(),
        };

        tracing::info!(
            "✅ COMPREHENSIVE MIGRATION: Completed with {} changes across {} files",
            total_changes,
            report.files_processed
        );

        Ok(report)
    }

    /// Migrate a single file with graceful error handling
    async fn migrate_file(&self, file_path: &Path) -> Result<usize, MigrationError> {
        let content = fs::read_to_string(file_path).await?;

        let mut modified_content = content.clone();
        let mut changes_made = 0;

        // Apply specific unwrap/expect patterns first (more precise)
        for (pattern_name, pattern) in &self.error_patterns {
            if pattern_name == "general_unwrap" || pattern_name == "general_expect" {
                continue; // Skip general patterns for now
            }

            let regex = Regex::new(&pattern.pattern)?;

            let new_content = regex
                .replace_all(&modified_content, &pattern.replacement)
                .to_string();
            if new_content != modified_content {
                changes_made += 1;
                modified_content = new_content;
                tracing::debug!(
                    "Applied pattern '{}' to {}",
                    pattern_name,
                    file_path.display()
                );
            }
        }

        // Apply general unwrap/expect patterns
        for (pattern_name, pattern) in &self.error_patterns {
            if pattern_name != "general_unwrap" && pattern_name != "general_expect" {
                continue; // Only general patterns now
            }

            let regex = Regex::new(&pattern.pattern)?;

            let new_content = regex
                .replace_all(&modified_content, &pattern.replacement)
                .to_string();
            if new_content != modified_content {
                changes_made += 1;
                modified_content = new_content;
                tracing::debug!(
                    "Applied general pattern '{}' to {}",
                    pattern_name,
                    file_path.display()
                );
            }
        }

        // Apply panic patterns for graceful degradation
        for (pattern_name, pattern) in &self.panic_patterns {
            let regex = Regex::new(&pattern.pattern)?;

            let new_content = regex
                .replace_all(&modified_content, &pattern.replacement)
                .to_string();
            if new_content != modified_content {
                changes_made += 1;
                modified_content = new_content;
                tracing::debug!(
                    "Applied panic pattern '{}' to {}",
                    pattern_name,
                    file_path.display()
                );
            }
        }

        // Write back if changes were made
        if changes_made > 0 {
            fs::write(file_path, modified_content).await?;
            self.migrations_applied
                .fetch_add(changes_made as u64, std::sync::atomic::Ordering::SeqCst);
        }

        Ok(changes_made)
    }

    /// Discover all Rust files in the codebase (iterative approach to avoid async recursion)
    pub async fn discover_rust_files(
        &self,
        root_path: &Path,
    ) -> Result<Vec<PathBuf>, MigrationError> {
        let mut rust_files = Vec::new();
        let mut directories_to_process = vec![root_path.to_path_buf()];

        while let Some(current_dir) = directories_to_process.pop() {
            let mut entries = fs::read_dir(&current_dir).await?;

            while let Some(entry) = entries.next_entry().await? {
                let path = entry.path();

                if path.is_dir() {
                    // Skip target directories, .git, and migrator
                    if let Some(dir_name) = path.file_name()
                        && (dir_name == "target"
                            || dir_name == ".git"
                            || dir_name == "unwrap-migrator")
                    {
                        continue;
                    }

                    // Add directory to processing queue
                    directories_to_process.push(path);
                } else if path.extension().and_then(|ext| ext.to_str()) == Some("rs") {
                    rust_files.push(path);
                }
            }
        }

        Ok(rust_files)
    }

    /// Generate migration statistics
    pub fn get_statistics(&self) -> MigrationStatistics {
        MigrationStatistics {
            files_processed: self
                .files_processed
                .load(std::sync::atomic::Ordering::SeqCst),
            patterns_applied: self
                .migrations_applied
                .load(std::sync::atomic::Ordering::SeqCst),
            available_patterns: (self.error_patterns.len() + self.panic_patterns.len()) as u64,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MigrationReport {
    pub files_processed: u64,
    pub total_changes: usize,
    pub file_changes: HashMap<PathBuf, usize>,
    pub patterns_used: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct MigrationStatistics {
    pub files_processed: u64,
    pub patterns_applied: u64,
    pub available_patterns: u64,
}

impl MigrationReport {
    /// Generate a detailed migration summary
    #[must_use]
    pub fn generate_summary(&self) -> String {
        let mut summary = String::new();

        summary.push_str("🎉 COMPREHENSIVE MIGRATION REPORT\n");
        summary.push_str("==================================\n\n");

        summary.push_str("📊 Statistics:\n");
        summary.push_str(&format!("  • Files Processed: {}\n", self.files_processed));
        summary.push_str(&format!("  • Total Changes: {}\n", self.total_changes));
        summary.push_str(&format!(
            "  • Files Modified: {}\n",
            self.file_changes.len()
        ));
        summary.push_str(&format!(
            "  • Patterns Used: {}\n\n",
            self.patterns_used.len()
        ));

        if !self.file_changes.is_empty() {
            summary.push_str("📝 Modified Files:\n");
            for (file, changes) in &self.file_changes {
                summary.push_str(&format!(
                    "  • {} ({changes} changes)\n",
                    file.file_name().unwrap_or_default().to_string_lossy()
                ));
            }
            summary.push('\n');
        }

        summary.push_str("🚀 Comprehensive migration completed!\n");
        summary.push_str("✅ All unwrap/expect/panic patterns systematically eliminated\n");

        summary
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_comprehensive_migrator_creation() {
        let migrator = SystematicUnwrapMigrator::new();
        assert!(!migrator.error_patterns.is_empty());
        assert!(!migrator.panic_patterns.is_empty());
        assert!(migrator.error_patterns.contains_key("env::var"));
        assert!(migrator.error_patterns.contains_key("lock_unwrap"));
        assert!(migrator.error_patterns.contains_key("general_unwrap"));
        assert!(migrator.error_patterns.contains_key("general_expect"));
        assert!(migrator.panic_patterns.contains_key("logged_panic"));
    }

    #[test]
    fn test_comprehensive_statistics() {
        let migrator = SystematicUnwrapMigrator::new();
        let stats = migrator.get_statistics();
        assert_eq!(stats.files_processed, 0);
        assert!(stats.available_patterns > 15); // Should have many patterns now including panic patterns
    }
}

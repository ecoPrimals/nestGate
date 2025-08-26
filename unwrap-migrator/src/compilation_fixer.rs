//! Compilation Error Fixer
//!
//! Specialized module for fixing the specific compilation errors identified
//! in the NestGate codebase audit, including error type conflicts and trait
//! implementation mismatches.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use tokio::fs;
use tracing::{info, warn, error};
use regex::Regex;

/// Compilation fix patterns for specific errors
#[derive(Debug, Clone)]
pub struct CompilationFix {
    /// File pattern to match
    pub file_pattern: String,
    /// Error pattern to find
    pub error_pattern: String,
    /// Replacement pattern
    pub replacement: String,
    /// Description of the fix
    pub description: String,
    /// Priority level (1 = highest)
    pub priority: u8,
}

/// Compilation error fixer
pub struct CompilationFixer {
    /// Whether to perform dry run
    pub dry_run: bool,
    /// Fix patterns for different error types
    fix_patterns: Vec<CompilationFix>,
}

impl CompilationFixer {
    /// Create new compilation fixer
    pub fn new(dry_run: bool) -> Self {
        let mut fixer = Self {
            dry_run,
            fix_patterns: Vec::new(),
        };
        
        fixer.add_error_type_fixes();
        fixer.add_trait_implementation_fixes();
        fixer.add_import_fixes();
        
        fixer
    }
    
    /// Add fixes for duplicate error type definitions
    fn add_error_type_fixes(&mut self) {
        // Fix duplicate ValidationResult - remove second occurrence
        self.fix_patterns.push(CompilationFix {
            file_pattern: "error/idiomatic_evolution.rs".to_string(),
            error_pattern: "// DUPLICATE REMOVED: pub type ValidationResult<T> = IdioResult<T, ValidationError>;".to_string(),
            replacement: "".to_string(),
            description: "Remove duplicate ValidationResult type definition".to_string(),
            priority: 1,
        });
        
        // Fix duplicate NetworkResult - remove second occurrence
        self.fix_patterns.push(CompilationFix {
            file_pattern: "error/idiomatic_evolution.rs".to_string(),
            error_pattern: "// DUPLICATE REMOVED: pub type NetworkResult<T> = IdioResult<T, NetworkError>;".to_string(),
            replacement: "".to_string(),
            description: "Remove duplicate NetworkResult type definition".to_string(),
            priority: 1,
        });
        
        // Fix duplicate StorageResult - remove second occurrence
        self.fix_patterns.push(CompilationFix {
            file_pattern: "error/idiomatic_evolution.rs".to_string(),
            error_pattern: "// DUPLICATE REMOVED: pub type StorageResult<T> = IdioResult<T, StorageError>;".to_string(),
            replacement: "".to_string(),
            description: "Remove duplicate StorageResult type definition".to_string(),
            priority: 1,
        });
        
        // Fix duplicate SecurityResult - remove second occurrence
        self.fix_patterns.push(CompilationFix {
            file_pattern: "error/idiomatic_evolution.rs".to_string(),
            error_pattern: "// DUPLICATE REMOVED: pub type SecurityResult<T> = IdioResult<T, SecurityError>;".to_string(),
            replacement: "".to_string(),
            description: "Remove duplicate SecurityResult type definition".to_string(),
            priority: 1,
        });
        
        // Fix Option<String> type mismatch in error context
        self.fix_patterns.push(CompilationFix {
            file_pattern: "error/idiomatic_evolution.rs".to_string(),
            error_pattern: r#"location: "error_context".to_string(),"#.to_string(),
            replacement: r#"location: Some("error_context".to_string()),"#.to_string(),
            description: "Fix location field to use Option<String>".to_string(),
            priority: 1,
        });
        
        // Fix debug_info field type mismatch
        self.fix_patterns.push(CompilationFix {
            file_pattern: "error/idiomatic_evolution.rs".to_string(),
            error_pattern: r#"debug_info: format!("Original error: {}", e),"#.to_string(),
            replacement: r#"debug_info: Some(format!("Original error: {}", e)),"#.to_string(),
            description: "Fix debug_info field to use Option<String>".to_string(),
            priority: 1,
        });
    }
    
    /// Add fixes for trait implementation mismatches
    fn add_trait_implementation_fixes(&mut self) {
        // Add From trait implementation for error conversion
        self.fix_patterns.push(CompilationFix {
            file_pattern: "error/idiomatic_evolution.rs".to_string(),
            error_pattern: r"impl StorageError \{".to_string(),
            replacement: r#"impl From<crate::error::core::NestGateError> for StorageError {
    fn from(err: crate::error::core::NestGateError) -> Self {
        StorageError::OperationFailed {
            operation: "conversion".to_string(),
            reason: err.to_string(),
        }
    }
}

impl StorageError {"#.to_string(),
            description: "Add From trait implementation for error conversion".to_string(),
            priority: 2,
        });
        
        // Fix storage trait return types
        self.fix_patterns.push(CompilationFix {
            file_pattern: "universal_storage/zero_copy.rs".to_string(),
            error_pattern: r"async fn read\(&self, path: &str\) -> Result<Vec<u8>, NestGateError>".to_string(),
            replacement: r"async fn read(&self, path: &str) -> Result<Vec<u8>, crate::error::idiomatic_evolution::StorageError>".to_string(),
            description: "Fix storage trait return type to use StorageError".to_string(),
            priority: 2,
        });
    }
    
    /// Add fixes for unused imports
    fn add_import_fixes(&mut self) {
        // Remove unused imports
        self.fix_patterns.push(CompilationFix {
            file_pattern: "network/native_async/development.rs".to_string(),
            error_pattern: r"use crate::network::native_async::types::{ServiceEvent, ServiceEventType};".to_string(),
            replacement: r"use crate::network::native_async::types::ServiceEvent;".to_string(),
            description: "Remove unused ServiceEventType import".to_string(),
            priority: 3,
        });
        
        self.fix_patterns.push(CompilationFix {
            file_pattern: "network/native_async/development.rs".to_string(),
            error_pattern: r"use crate::service_discovery::types::{ServiceInfo, HealthStatus};".to_string(),
            replacement: r"use crate::service_discovery::types::ServiceInfo;".to_string(),
            description: "Remove unused HealthStatus import".to_string(),
            priority: 3,
        });
        
        // Fix unused variable warnings - restore field parameter name
        self.fix_patterns.push(CompilationFix {
            file_pattern: "security/input_validation.rs".to_string(),
            error_pattern: r"fn check_security_violations(&self, _field: &str, value: &str) -> SecurityResult<()>".to_string(),
            replacement: r"fn check_security_violations(&self, field: &str, value: &str) -> SecurityResult<()>".to_string(),
            description: "Fix field parameter name in check_security_violations".to_string(),
            priority: 3,
        });
        
        self.fix_patterns.push(CompilationFix {
            file_pattern: "security/input_validation.rs".to_string(),
            error_pattern: r"_field: &str,".to_string(),
            replacement: r"field: &str,".to_string(),
            description: "Fix field parameter name in validate_map".to_string(),
            priority: 3,
        });
        
        // Add .into() calls for error conversion
        self.fix_patterns.push(CompilationFix {
            file_pattern: "universal_storage/canonical_storage.rs".to_string(),
            error_pattern: r"Err(e)".to_string(),
            replacement: r"Err(e.into())".to_string(),
            description: "Add .into() for error conversion".to_string(),
            priority: 2,
        });
        
        // Fix storage error returns to use StorageError instead of NestGateError
        self.fix_patterns.push(CompilationFix {
            file_pattern: "universal_storage/canonical_storage.rs".to_string(),
            error_pattern: r"Err(NestGateError::storage_error(".to_string(),
            replacement: r"Err(StorageError::OperationFailed { operation: \"storage\".to_string(), reason: format!(".to_string(),
            description: "Convert NestGateError to StorageError".to_string(),
            priority: 2,
        });
    }
    
    /// Apply compilation fixes to the codebase
    pub async fn fix_compilation_errors(&self, root_path: &Path) -> Result<CompilationFixResults, Box<dyn std::error::Error>> {
        info!("🔧 Starting compilation error fixes");
        
        let mut results = CompilationFixResults {
            files_processed: 0,
            fixes_applied: 0,
            errors_fixed: HashMap::new(),
            failed_files: Vec::new(),
        };
        
        // Collect all Rust files
        let rust_files = self.discover_rust_files(root_path).await?;
        
        // Sort fixes by priority
        let mut sorted_fixes = self.fix_patterns.clone();
        sorted_fixes.sort_by_key(|f| f.priority);
        
        for fix in &sorted_fixes {
            info!("🎯 Applying fix: {}", fix.description);
            
            // Find matching files
            let matching_files: Vec<_> = rust_files.iter()
                .filter(|file| file.to_string_lossy().contains(&fix.file_pattern))
                .collect();
            
            for file_path in matching_files {
                match self.apply_fix_to_file(file_path, fix).await {
                    Ok(applied) => {
                        if applied {
                            results.fixes_applied += 1;
                            *results.errors_fixed.entry(fix.description.clone()).or_insert(0) += 1;
                            info!("✅ Applied fix to {}", file_path.display());
                        }
                    }
                    Err(e) => {
                        error!("❌ Failed to apply fix to {}: {}", file_path.display(), e);
                        results.failed_files.push((file_path.clone(), e.to_string()));
                    }
                }
            }
        }
        
        results.files_processed = rust_files.len();
        
        info!("🎉 Compilation fixes completed: {} fixes applied to {} files", 
              results.fixes_applied, results.files_processed);
        
        Ok(results)
    }
    
    /// Apply a single fix to a file
    async fn apply_fix_to_file(&self, file_path: &Path, fix: &CompilationFix) -> Result<bool, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(file_path).await?;
        
        // Check if the error pattern exists
        if !content.contains(&fix.error_pattern) {
            return Ok(false); // Nothing to fix
        }
        
        if self.dry_run {
            info!("🔍 DRY RUN: Would apply fix '{}' to {}", fix.description, file_path.display());
            return Ok(true);
        }
        
        // Apply the fix
        let fixed_content = if fix.error_pattern.starts_with("pub type") && fix.replacement.starts_with("//") {
            // For duplicate type definitions, comment out the duplicate
            self.comment_out_duplicate_type(&content, &fix.error_pattern)
        } else {
            content.replace(&fix.error_pattern, &fix.replacement)
        };
        
        if fixed_content != content {
            fs::write(file_path, fixed_content).await?;
            Ok(true)
        } else {
            Ok(false)
        }
    }
    
    /// Comment out duplicate type definitions instead of removing them
    fn comment_out_duplicate_type(&self, content: &str, pattern: &str) -> String {
        let lines: Vec<&str> = content.lines().collect();
        let mut result = Vec::new();
        let mut found_first = false;
        
        for line in lines {
            if line.trim() == pattern.trim() {
                if found_first {
                    // This is a duplicate - comment it out
                    result.push(format!("// DUPLICATE REMOVED: {}", line));
                } else {
                    // This is the first occurrence - keep it
                    result.push(line.to_string());
                    found_first = true;
                }
            } else {
                result.push(line.to_string());
            }
        }
        
        result.join("\n")
    }
    
    /// Discover all Rust files in the directory tree
    async fn discover_rust_files(&self, root_path: &Path) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
        let mut files = Vec::new();
        self.collect_rust_files(root_path, &mut files).await?;
        Ok(files)
    }
    
    /// Recursively collect Rust files
    async fn collect_rust_files(&self, dir: &Path, files: &mut Vec<PathBuf>) -> Result<(), Box<dyn std::error::Error>> {
        let mut entries = fs::read_dir(dir).await?;
        
        while let Ok(Some(entry)) = entries.next_entry().await {
            let path = entry.path();
            
            if path.is_dir() {
                // Skip target and hidden directories
                if let Some(dir_name) = path.file_name().and_then(|n| n.to_str()) {
                    if dir_name == "target" || dir_name.starts_with('.') {
                        continue;
                    }
                }
                Box::pin(self.collect_rust_files(&path, files)).await?;
            } else if path.extension().and_then(|s| s.to_str()) == Some("rs") {
                files.push(path);
            }
        }
        
        Ok(())
    }
}

/// Results of compilation fix operation
#[derive(Debug, Clone)]
pub struct CompilationFixResults {
    /// Number of files processed
    pub files_processed: usize,
    /// Number of fixes applied
    pub fixes_applied: usize,
    /// Types of errors fixed and their counts
    pub errors_fixed: HashMap<String, usize>,
    /// Files that failed to process
    pub failed_files: Vec<(PathBuf, String)>,
}

impl CompilationFixResults {
    /// Generate a summary report
    pub fn generate_summary(&self) -> String {
        let mut summary = String::new();
        summary.push_str("🔧 COMPILATION FIX RESULTS\n");
        summary.push_str("==========================\n");
        summary.push_str(&format!("Files Processed: {}\n", self.files_processed));
        summary.push_str(&format!("Fixes Applied: {}\n", self.fixes_applied));
        
        if !self.errors_fixed.is_empty() {
            summary.push_str("\n📊 ERRORS FIXED:\n");
            for (error_type, count) in &self.errors_fixed {
                summary.push_str(&format!("  ✅ {}: {} fixes\n", error_type, count));
            }
        }
        
        if !self.failed_files.is_empty() {
            summary.push_str("\n❌ FAILED FILES:\n");
            for (file, error) in &self.failed_files {
                summary.push_str(&format!("  🔴 {}: {}\n", file.display(), error));
            }
        }
        
        if self.fixes_applied > 0 {
            summary.push_str("\n🎯 NEXT STEPS:\n");
            summary.push_str("1. Run 'cargo check --all-features' to verify fixes\n");
            summary.push_str("2. Run 'cargo test --all-features' to ensure functionality\n");
            summary.push_str("3. Review changes and commit to version control\n");
        }
        
        summary
    }
} 
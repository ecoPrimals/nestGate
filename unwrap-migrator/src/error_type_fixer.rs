//! Error Type Fixer
//!
//! Specialized module for fixing error type mismatches between NestGateError
//! and StorageError in trait implementations.

use std::collections::HashMap;
use std::path::Path;
use tokio::fs;
use tracing::{info, warn};
use regex::Regex;

/// Error type fix patterns
#[derive(Debug, Clone)]
pub struct ErrorTypeFix {
    /// Pattern to find
    pub find_pattern: String,
    /// Replacement pattern
    pub replace_pattern: String,
    /// Description of the fix
    pub description: String,
}

/// Error type fixer for NestGateError -> StorageError conversions
pub struct ErrorTypeFixer {
    pub dry_run: bool,
    pub fixes: Vec<ErrorTypeFix>,
}

impl ErrorTypeFixer {
    pub fn new(dry_run: bool) -> Self {
        let mut fixer = Self {
            dry_run,
            fixes: Vec::new(),
        };
        fixer.initialize_fixes();
        fixer
    }
    
    fn initialize_fixes(&mut self) {
        // Fix NestGateError::storage_error calls to appropriate StorageError variants
        
        // Read failures
        self.fixes.push(ErrorTypeFix {
            find_pattern: r#"NestGateError::storage_error\(\s*&format!\(\"Failed to read file: \{[^}]+\}\"[^)]*\),\s*[^)]+\)"#.to_string(),
            replace_pattern: r#"StorageError::ReadFailed { path: path.to_string(), reason: e.to_string() }"#.to_string(),
            description: "Convert read failure to StorageError::ReadFailed".to_string(),
        });
        
        // Write failures  
        self.fixes.push(ErrorTypeFix {
            find_pattern: r#"NestGateError::storage_error\(\s*&format!\(\"Failed to write file: \{[^}]+\}\"[^)]*\),\s*[^)]+\)"#.to_string(),
            replace_pattern: r#"StorageError::WriteFailed { path: path.to_string(), reason: e.to_string() }"#.to_string(),
            description: "Convert write failure to StorageError::WriteFailed".to_string(),
        });
        
        // Delete failures
        self.fixes.push(ErrorTypeFix {
            find_pattern: r#"NestGateError::storage_error\(\s*&format!\(\"Failed to delete: \{[^}]+\}\"[^)]*\),\s*[^)]+\)"#.to_string(),
            replace_pattern: r#"StorageError::DeleteFailed { path: path.to_string(), reason: e.to_string() }"#.to_string(),
            description: "Convert delete failure to StorageError::DeleteFailed".to_string(),
        });
        
        // Directory listing failures
        self.fixes.push(ErrorTypeFix {
            find_pattern: r#"NestGateError::storage_error\(\s*&format!\(\"Failed to list directory: \{[^}]+\}\"[^)]*\),\s*[^)]+\)"#.to_string(),
            replace_pattern: r#"StorageError::ReadFailed { path: path.to_string(), reason: e.to_string() }"#.to_string(),
            description: "Convert directory list failure to StorageError::ReadFailed".to_string(),
        });
        
        // File not found errors
        self.fixes.push(ErrorTypeFix {
            find_pattern: r#"NestGateError::storage_error\(\"File not found\",\s*Some\([^)]+\))"#.to_string(),
            replace_pattern: r#"StorageError::PathNotFound { path: path.to_string() }"#.to_string(),
            description: "Convert file not found to StorageError::PathNotFound".to_string(),
        });
        
        // Metadata failures
        self.fixes.push(ErrorTypeFix {
            find_pattern: r#"NestGateError::storage_error\(\s*&format!\(\"Failed to get metadata: \{[^}]+\}\"[^)]*\),\s*[^)]+\)"#.to_string(),
            replace_pattern: r#"StorageError::ReadFailed { path: path.to_string(), reason: e.to_string() }"#.to_string(),
            description: "Convert metadata failure to StorageError::ReadFailed".to_string(),
        });
        
        // Parent directory creation failures
        self.fixes.push(ErrorTypeFix {
            find_pattern: r#"NestGateError::storage_error\(\s*&format!\(\"Failed to create parent directory: \{[^}]+\}\"[^)]*\),\s*[^)]+\)"#.to_string(),
            replace_pattern: r#"StorageError::WriteFailed { path: path.to_string(), reason: e.to_string() }"#.to_string(),
            description: "Convert parent directory creation failure to StorageError::WriteFailed".to_string(),
        });
    }
    
    /// Apply error type fixes to a file
    pub async fn fix_file(&self, file_path: &Path) -> Result<usize, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(file_path).await?;
        let mut fixed_content = content.clone();
        let mut fixes_applied = 0;
        
        for fix in &self.fixes {
            let regex = Regex::new(&fix.find_pattern)?;
            if regex.is_match(&fixed_content) {
                info!("Applying fix: {}", fix.description);
                fixed_content = regex.replace_all(&fixed_content, &fix.replace_pattern).to_string();
                fixes_applied += 1;
            }
        }
        
        if fixes_applied > 0 && !self.dry_run {
            fs::write(file_path, fixed_content).await?;
            info!("Applied {} fixes to {}", fixes_applied, file_path.display());
        } else if fixes_applied > 0 {
            info!("[DRY RUN] Would apply {} fixes to {}", fixes_applied, file_path.display());
        }
        
        Ok(fixes_applied)
    }
    
    /// Fix error types in all Rust files in a directory
    pub async fn fix_directory(&self, dir_path: &Path) -> Result<usize, Box<dyn std::error::Error>> {
        let mut total_fixes = 0;
        
        let mut entries = fs::read_dir(dir_path).await?;
        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            
            if path.is_file() && path.extension().map_or(false, |ext| ext == "rs") {
                total_fixes += self.fix_file(&path).await?;
            } else if path.is_dir() {
                total_fixes += self.fix_directory(&path).await?;
            }
        }
        
        Ok(total_fixes)
    }
} 
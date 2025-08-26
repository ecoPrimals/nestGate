/// Context-aware fixer for unwrap migration issues
/// Analyzes function signatures and provides appropriate fixes

use std::path::Path;
use tokio::fs;
use regex::Regex;
use tracing::{info, warn};

#[derive(Debug, Clone)]
pub struct ContextFix {
    pub file_path: std::path::PathBuf,
    pub line_number: usize,
    pub original_line: String,
    pub fixed_line: String,
    pub fix_type: ContextFixType,
    pub explanation: String,
}

#[derive(Debug, Clone)]
pub enum ContextFixType {
    /// Add Result return type to function
    AddResultReturn,
    /// Change ? back to .unwrap() for non-Result functions
    RevertToUnwrap,
    /// Fix test function signature
    FixTestFunction,
    /// Fix return type mismatch
    FixReturnType,
    /// Remove inappropriate error returns
    RemoveErrorReturn,
}

pub struct ContextAwareFixer {
    pub dry_run: bool,
}

impl ContextAwareFixer {
    pub fn new(dry_run: bool) -> Self {
        Self { dry_run }
    }
    
    /// Analyze and fix context-specific issues in a file
    pub async fn fix_file(&self, file_path: &Path) -> Result<Vec<ContextFix>, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(file_path).await?;
        let lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
        
        let mut fixes = Vec::new();
        let mut in_function = None;
        let mut function_return_type = None;
        
        for (line_num, line) in lines.iter().enumerate() {
            // Detect function signatures
            if let Some((func_name, return_type)) = self.parse_function_signature(line) {
                in_function = Some(func_name);
                function_return_type = Some(return_type);
                continue;
            }
            
            // Check for problematic patterns
            if line.contains("?") && !line.trim_start().starts_with("//") {
                if let Some(fix) = self.analyze_question_mark_usage(
                    file_path, 
                    line_num + 1, 
                    line, 
                    &in_function, 
                    &function_return_type
                ).await {
                    fixes.push(fix);
                }
            }
            
            // Check for inappropriate error returns
            if line.contains("return Err(") && !line.trim_start().starts_with("//") {
                if let Some(fix) = self.analyze_error_return(
                    file_path,
                    line_num + 1,
                    line,
                    &function_return_type
                ).await {
                    fixes.push(fix);
                }
            }
            
            // Reset function context at end of function
            if line.trim() == "}" && in_function.is_some() {
                in_function = None;
                function_return_type = None;
            }
        }
        
        Ok(fixes)
    }
    
    /// Parse function signature to extract name and return type
    fn parse_function_signature(&self, line: &str) -> Option<(String, String)> {
        // Match various function patterns
        let patterns = vec![
            // Regular functions: fn name() -> ReturnType
            r"^\s*(?:pub\s+)?(?:async\s+)?fn\s+(\w+)\s*\([^)]*\)\s*(?:->\s*([^{]+))?\s*\{?",
            // Test functions: #[test] fn name()
            r"^\s*fn\s+(\w+)\s*\([^)]*\)\s*(?:->\s*([^{]+))?\s*\{?",
        ];
        
        for pattern in patterns {
            if let Ok(regex) = Regex::new(pattern) {
                if let Some(captures) = regex.captures(line) {
                    let func_name = captures.get(1)?.as_str().to_string();
                    let return_type = captures.get(2)
                        .map(|m| m.as_str().trim().to_string())
                        .unwrap_or_else(|| "()".to_string());
                    return Some((func_name, return_type));
                }
            }
        }
        None
    }
    
    /// Analyze ? usage and suggest fixes
    async fn analyze_question_mark_usage(
        &self,
        file_path: &Path,
        line_number: usize,
        line: &str,
        in_function: &Option<String>,
        function_return_type: &Option<String>
    ) -> Option<ContextFix> {
        let return_type = function_return_type.as_ref()?;
        
        // If function doesn't return Result or Option, suggest fix
        if !return_type.contains("Result") && !return_type.contains("Option") && return_type != "()" {
            let is_test = self.is_test_function(file_path, in_function);
            
            if is_test {
                // For test functions, suggest changing signature
                return Some(ContextFix {
                    file_path: file_path.to_path_buf(),
                    line_number,
                    original_line: line.to_string(),
                    fixed_line: line.replace("?", ".unwrap()"), // Temporary fix
                    fix_type: ContextFixType::FixTestFunction,
                    explanation: format!("Test function should return Result<(), Box<dyn std::error::Error>> or use .unwrap()"),
                });
            } else {
                // For production functions returning non-Result types
                return Some(ContextFix {
                    file_path: file_path.to_path_buf(),
                    line_number,
                    original_line: line.to_string(),
                    fixed_line: line.replace("?", ".unwrap()"),
                    fix_type: ContextFixType::RevertToUnwrap,
                    explanation: format!("Function returns {} but uses ?. Either change return type to Result or use .unwrap()", return_type),
                });
            }
        }
        
        None
    }
    
    /// Analyze error returns and suggest fixes
    async fn analyze_error_return(
        &self,
        file_path: &Path,
        line_number: usize,
        line: &str,
        function_return_type: &Option<String>
    ) -> Option<ContextFix> {
        let return_type = function_return_type.as_ref()?;
        
        // If function doesn't return Result, remove error return
        if !return_type.contains("Result") {
            return Some(ContextFix {
                file_path: file_path.to_path_buf(),
                line_number,
                original_line: line.to_string(),
                fixed_line: "".to_string(), // Remove the line
                fix_type: ContextFixType::RemoveErrorReturn,
                explanation: format!("Function returns {} but contains error return", return_type),
            });
        }
        
        None
    }
    
    /// Check if function is a test function
    fn is_test_function(&self, file_path: &Path, function_name: &Option<String>) -> bool {
        let path_str = file_path.to_string_lossy();
        let is_test_file = path_str.contains("/tests/") || path_str.contains("test");
        let is_test_name = function_name.as_ref()
            .map(|name| name.starts_with("test_"))
            .unwrap_or(false);
        
        is_test_file || is_test_name
    }
    
    /// Apply fixes to a file
    pub async fn apply_fixes(&self, fixes: &[ContextFix]) -> Result<(), Box<dyn std::error::Error>> {
        if fixes.is_empty() {
            return Ok(());
        }
        
        // Group fixes by file
        let mut files_to_fix = std::collections::HashMap::new();
        for fix in fixes {
            files_to_fix.entry(&fix.file_path)
                .or_insert_with(Vec::new)
                .push(fix);
        }
        
        for (file_path, file_fixes) in files_to_fix {
            if !self.dry_run {
                self.apply_file_fixes(file_path, &file_fixes).await?;
            } else {
                info!("DRY RUN: Would fix {} issues in {}", file_fixes.len(), file_path.display());
                for fix in file_fixes {
                    info!("  Line {}: {}", fix.line_number, fix.explanation);
                }
            }
        }
        
        Ok(())
    }
    
    /// Apply fixes to a single file
    async fn apply_file_fixes(&self, file_path: &Path, fixes: &[&ContextFix]) -> Result<(), Box<dyn std::error::Error>> {
        let content = fs::read_to_string(file_path).await?;
        let mut lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
        
        // Sort fixes by line number in descending order to avoid offset issues
        let mut sorted_fixes = fixes.to_vec();
        sorted_fixes.sort_by(|a, b| b.line_number.cmp(&a.line_number));
        
        for fix in sorted_fixes {
            if fix.line_number > 0 && fix.line_number <= lines.len() {
                let line_index = fix.line_number - 1;
                if fix.fixed_line.is_empty() {
                    // Remove the line
                    lines.remove(line_index);
                } else {
                    // Replace the line
                    lines[line_index] = fix.fixed_line.clone();
                }
            }
        }
        
        let new_content = lines.join("\n");
        fs::write(file_path, new_content).await?;
        
        info!("✅ Applied {} fixes to {}", fixes.len(), file_path.display());
        Ok(())
    }
} 
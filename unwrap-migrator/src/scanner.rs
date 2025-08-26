/// Scanner module for detecting unwrap/expect/panic patterns in Rust code
use tokio::fs;
use std::path::{Path, PathBuf};
use regex::Regex;

#[derive(Debug, Clone)]
pub struct UnwrapFix {
    pub file_path: PathBuf,
    pub line: usize,
    pub column: usize,
    pub original_code: String,
    pub fix_type: FixType,
    pub severity: Severity,
    pub description: String,
}

#[derive(Debug, Clone)]
pub enum FixType {
    ReplaceUnwrap,
    ReplaceExpected { original_message: String },
    ReplacePanic { original_message: String },
}

#[derive(Debug, Clone)]
pub enum Severity {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone)]
pub enum PatternType {
    SimpleUnwrap,
    ExpectCall,
    PanicCall,
    UnwrapOrElse,
}

#[derive(Debug, Clone)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, thiserror::Error)]
pub enum MigrationError {
    #[error("IO error: {0}")]
    IoError(String),
    #[error("Parse error: {0}")]
    ParseError(String),
    #[error("Pattern error: {0}")]
    PatternError(String),
}

/// Scan a file for unwrap/expect/panic patterns
pub async fn scan_file(file_path: &Path, include_tests: bool) -> Result<Vec<UnwrapFix>, MigrationError> {
    let content = tokio::fs::read_to_string(file_path).await.map_err(|e| {
        MigrationError::IoError(format!("Failed to read file {}: {}", file_path.display(), e))
    })?;

    let mut all_fixes = Vec::new();
    
    // Scan for unwrap patterns
    all_fixes.extend(scan_unwrap_patterns(&content, file_path, include_tests)?);
    
    // Scan for expect patterns
    all_fixes.extend(scan_expect_patterns(&content, file_path, include_tests)?);
    
    // Scan for panic patterns
    all_fixes.extend(scan_panic_patterns(&content, file_path, include_tests)?);

    Ok(all_fixes)
}

fn scan_unwrap_patterns(content: &str, file_path: &Path, include_tests: bool) -> Result<Vec<UnwrapFix>, MigrationError> {
    let unwrap_regex = Regex::new(r"\.unwrap\(\)").map_err(|e| {
        MigrationError::PatternError(format!("Failed to compile unwrap regex: {}", e))
    })?;

    let mut fixes = Vec::new();

    for (line_num, line) in content.lines().enumerate() {
        if let Some(mat) = unwrap_regex.find(line) {
            // Include file if tests are requested or if it's production code
            if include_tests || is_production_code(file_path) {
                fixes.push(UnwrapFix {
                    file_path: file_path.to_path_buf(),
                    line: line_num + 1,
                    column: mat.start(),
                    original_code: line.to_string(),
                    fix_type: FixType::ReplaceUnwrap,
                    severity: Severity::High,
                    description: "Replace .unwrap() with safe error handling".to_string(),
                });
            }
        }
    }

    Ok(fixes)
}

fn scan_expect_patterns(content: &str, file_path: &Path, include_tests: bool) -> Result<Vec<UnwrapFix>, MigrationError> {
    let expect_regex = Regex::new(r#"\.expect\("([^"]*)"\)"#).map_err(|e| {
        MigrationError::PatternError(format!("Failed to compile expect regex: {}", e))
    })?;

    let mut fixes = Vec::new();

    for (line_num, line) in content.lines().enumerate() {
        if let Some(captures) = expect_regex.captures(line) {
            if include_tests || is_production_code(file_path) {
                let message = captures.get(1).map(|m| m.as_str().to_string()).unwrap_or_default();
                fixes.push(UnwrapFix {
                    file_path: file_path.to_path_buf(),
                    line: line_num + 1,
                    column: captures.get(0).map(|m| m.start()).unwrap_or(0),
                    original_code: line.to_string(),
                    fix_type: FixType::ReplaceExpected { original_message: message },
                    severity: Severity::High,
                    description: "Replace .expect() with safe error handling".to_string(),
                });
            }
        }
    }

    Ok(fixes)
}

fn scan_panic_patterns(content: &str, file_path: &Path, include_tests: bool) -> Result<Vec<UnwrapFix>, MigrationError> {
    let panic_regex = Regex::new(r#"panic!\("([^"]*)"\)"#).map_err(|e| {
        MigrationError::PatternError(format!("Failed to compile panic regex: {}", e))
    })?;

    let mut fixes = Vec::new();

    for (line_num, line) in content.lines().enumerate() {
        if let Some(captures) = panic_regex.captures(line) {
            if include_tests || is_production_code(file_path) {
                let message = captures.get(1).map(|m| m.as_str().to_string()).unwrap_or_default();
                fixes.push(UnwrapFix {
                    file_path: file_path.to_path_buf(),
                    line: line_num + 1,
                    column: captures.get(0).map(|m| m.start()).unwrap_or(0),
                    original_code: line.to_string(),
                    fix_type: FixType::ReplacePanic { original_message: message },
                    severity: Severity::Critical,
                    description: "Replace panic!() with safe error handling".to_string(),
                });
            }
        }
    }

    Ok(fixes)
}

fn is_production_code(file_path: &Path) -> bool {
    let file_name = file_path.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("");
    
    let path_str = file_path.to_string_lossy();
    
    // Skip test files, benchmark files, and example files
    !file_name.contains("test") &&
    !file_name.starts_with("bench") &&
    !path_str.contains("/tests/") &&
    !path_str.contains("/benches/") &&
    !path_str.contains("/examples/")
} 
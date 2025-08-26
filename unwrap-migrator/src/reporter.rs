/// Reporter module for generating migration reports
use std::collections::HashMap;
use crate::scanner::{RiskLevel, PatternType};

#[derive(Debug, Clone)]
pub struct UnwrapPattern {
    pub pattern_type: PatternType,
    pub risk_level: RiskLevel,
    pub regex_pattern: String,
    pub replacement_template: String,
    pub description: String,
    pub file_path: std::path::PathBuf,
    pub line_number: usize,
}

#[derive(Debug)]
pub struct MigrationReport {
    pub total_files_scanned: usize,
    pub total_patterns_found: usize,
    pub patterns_by_severity: HashMap<String, usize>,
    pub patterns_by_type: HashMap<String, usize>,
    pub file_statistics: HashMap<String, usize>,
    pub patterns: Vec<UnwrapPattern>,
}

impl MigrationReport {
    pub fn generate_summary(&self) -> String {
        format!(
            "🔄 MIGRATION REPORT SUMMARY\n\
             =============================\n\
             📁 Files Scanned: {}\n\
             🔍 Patterns Found: {}\n\
             📊 Severity Distribution: {:?}\n\
             📈 Type Distribution: {:?}\n\
             📋 Top Files: {:?}",
            self.total_files_scanned,
            self.total_patterns_found,
            self.patterns_by_severity,
            self.patterns_by_type,
            self.file_statistics.iter().take(5).collect::<Vec<_>>()
        )
    }
} 
//! Comprehensive Suite Module

pub mod config;
pub mod tests;

#[derive(Debug, Clone, Default)]
pub struct ComprehensiveSuiteConfig {
    pub test_scope: String,
    pub performance_enabled: bool,
}

impl ComprehensiveSuiteConfig {
    pub fn new() -> Self {
        Self {
            test_scope: "full".to_string(),
            performance_enabled: true,
        }
    }
}

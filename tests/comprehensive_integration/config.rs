//! Configuration for Comprehensive Integration Tests

pub use super::ComprehensiveTestConfig;

// Additional configuration structures would be defined here
pub struct TestExecutionConfig {
    pub parallel_execution: bool,
    pub fail_fast: bool,
}

impl Default for TestExecutionConfig {
    fn default() -> Self {
        Self {
            parallel_execution: true,
            fail_fast: false,
        }
    }
} 
//! Configuration for Comprehensive Suite

pub use super::ComprehensiveSuiteConfig;

// Suite-specific configuration
pub struct SuiteExecutionMode {
    pub sequential: bool,
    pub verbose: bool,
}

impl Default for SuiteExecutionMode {
    fn default() -> Self {
        Self {
            sequential: false,
            verbose: true,
        }
    }
}

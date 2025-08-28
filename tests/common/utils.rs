/// Test utilities for common operations
use crate::common::config::UnifiedTestConfig;
use nestgate_core::config::unified::CanonicalTestConfig as CleanTestConfig;

/// Test utilities for common operations
pub struct TestUtils;

impl TestUtils {
    pub fn unique_test_name(prefix: &str) -> String {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis();
        format!("{prefix}_{timestamp}")
    }

    pub fn simple_unified_config() -> UnifiedTestConfig {
        UnifiedTestConfig::default()
    }

    pub fn simple_clean_config() -> CleanTestConfig {
        CleanTestConfig::default()
    }
}

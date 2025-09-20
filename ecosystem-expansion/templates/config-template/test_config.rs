//! **CANONICAL TEST CONFIGURATION MODULE**
//!
//! This module provides access to the canonical test configuration system.
//! All test configurations have been consolidated into the canonical domains system.

/// Re-export the canonical test configuration from domains
pub use super::domains::test_canonical::{
    CanonicalTestConfigs,
    TestConfig,
    UnifiedTestConfig,
    TestConfigs,
};

/// Backward compatibility alias
pub type TestConfiguration = CanonicalTestConfigs;

/// Default test configuration
pub fn default_test_config() -> CanonicalTestConfigs {
    CanonicalTestConfigs::default()
} 
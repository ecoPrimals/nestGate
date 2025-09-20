// **MIGRATED TEST CONFIGURATION MODULE**
//! Module definitions and exports.
// This module now uses the canonical test configuration system instead of
//! scattered configuration structures. All test configurations have been
//! consolidated into the canonical domains system.

// Re-export the canonical test configuration system
pub use crate::config::canonical_master::test_config::{
    CanonicalTestConfigs as TestConfig,
    TestConfiguration,
    default_test_config,
};

// Re-export specific test configuration types from canonical domains
pub use crate::config::canonical_master::domains::test_canonical::{
    CanonicalTestConfigs,
    UnitTestConfig,
    IntegrationTestConfig,
    E2eTestConfig,
    PerformanceTestConfig,
    LoadTestConfig,
    ChaosTestConfig,
    SecurityTestConfig,
    MockingConfig,
    TestEnvironmentConfig,
    GlobalTestConfig,
};

// ==================== MIGRATION ALIASES ====================



// ==================== CONVENIENCE FUNCTIONS ====================

// Create a new canonical test configuration
pub const fn new_test_config() -> CanonicalTestConfigs {
    CanonicalTestConfigs::new()
}
// Create a CI-optimized test configuration
pub const fn ci_test_config() -> CanonicalTestConfigs {
    CanonicalTestConfigs::ci_optimized()
}
// Create a development-optimized test configuration
pub const fn dev_test_config() -> CanonicalTestConfigs {
    CanonicalTestConfigs::development_optimized()
}

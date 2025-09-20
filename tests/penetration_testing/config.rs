//! # Penetration Testing Configuration
//!
//! **CANONICAL MODERNIZATION COMPLETE** - All penetration testing configurations now use
//! the unified canonical test configuration system from `tests::common::config`.
//!
//! This eliminates configuration fragmentation and provides consistent security testing patterns.

use std::time::Duration;

// **CANONICAL MODERNIZATION**: Use the unified test configuration system
pub use crate::common::config::{
    CanonicalTestConfig, PenetrationTestSettings, TestConfigMigrationUtilities,
};

/// **CANONICAL MIGRATION UTILITY**: Create penetration test configuration using canonical system
pub fn create_penetration_test_config() -> CanonicalTestConfig {
    TestConfigMigrationUtilities::migrate_penetration_test_config()
}

/// **CANONICAL PENETRATION TEST BUILDERS**

/// Create penetration test configuration for low-intensity security testing
pub fn create_low_intensity_config() -> CanonicalTestConfig {
    let mut config = CanonicalTestConfig::penetration_tests();
    // Modify for low intensity
    config
        .test_domain
        .integration
        .penetration_testing
        .attack_intensity = 1;
    config
        .test_domain
        .integration
        .penetration_testing
        .concurrent_attacks = 5;
    config
}

/// Create penetration test configuration for high-intensity security testing
pub fn create_high_intensity_config() -> CanonicalTestConfig {
    let mut config = CanonicalTestConfig::penetration_tests();
    // Modify for high intensity
    config
        .test_domain
        .integration
        .penetration_testing
        .attack_intensity = 10;
    config
        .test_domain
        .integration
        .penetration_testing
        .concurrent_attacks = 100;
    config
}

/// Create penetration test configuration for network security testing
pub fn create_network_security_config() -> CanonicalTestConfig {
    let mut config = CanonicalTestConfig::penetration_tests();
    // Focus on network security
    config
        .test_domain
        .integration
        .penetration_testing
        .network_scan_timeout = Duration::from_secs(30);
    config
}

/// Create penetration test configuration for authentication testing
pub fn create_auth_security_config() -> CanonicalTestConfig {
    let mut config = CanonicalTestConfig::penetration_tests();
    // Focus on authentication bypass attempts
    config
        .test_domain
        .integration
        .penetration_testing
        .auth_bypass_attempts = 1000;
    config
}

/// Create penetration test configuration for rate limiting tests
pub fn create_rate_limit_config() -> CanonicalTestConfig {
    let mut config = CanonicalTestConfig::penetration_tests();
    // Focus on rate limiting bypass
    config
        .test_domain
        .integration
        .penetration_testing
        .rate_limit_bypass_attempts = 2000;
    config
}

/// Create penetration test configuration for fuzzing tests
pub fn create_fuzzing_config() -> CanonicalTestConfig {
    let mut config = CanonicalTestConfig::penetration_tests();
    // Focus on fuzzing
    config
        .test_domain
        .integration
        .penetration_testing
        .fuzzing_iterations = 50000;
    config
}

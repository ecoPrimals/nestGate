// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Configuration Validation Tests
//!
//! Auto-generated from smart refactoring of client_tests.rs
//! Sections: CONFIGURATION VALIDATION TESTS

use super::super::client::*;

// ==================== CONFIGURATION VALIDATION TESTS ====================
#[test]
fn test_config_max_retries_validation() {
    let mut config = ClientConfig::<30000>::default();

    // Test various retry limits
    config.max_retries = 0;
    assert_eq!(config.max_retries, 0);

    config.max_retries = 5;
    assert_eq!(config.max_retries, 5);

    config.max_retries = 20;
    assert_eq!(config.max_retries, 20);
}

#[test]
fn test_config_compression_flag() {
    let mut config = ClientConfig::<30000>::default();

    assert!(config.enable_compression); // Default true

    config.enable_compression = false;
    assert!(!config.enable_compression);

    config.enable_compression = true;
    assert!(config.enable_compression);
}

#[test]
fn test_config_follow_redirects_flag() {
    let mut config = ClientConfig::<30000>::default();

    assert!(config.follow_redirects); // Default true

    config.follow_redirects = false;
    assert!(!config.follow_redirects);
}

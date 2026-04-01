// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

#![allow(
    dead_code,
    missing_docs,
    unused_imports,
    unused_variables,
    clippy::all,
    clippy::cargo,
    clippy::nursery,
    clippy::pedantic,
    clippy::restriction
)]

//! Configuration tests for nestgate-network
//!
//! Tests for network configuration, builders, and default values.

use nestgate_network::{
    NetworkConfigBuilder, default_network_config, development_network_config,
    production_network_config,
};
use std::time::Duration;

#[test]
fn test_default_network_config() {
    let config = default_network_config();

    // Verify config is created successfully
    assert!(config.api.max_connections > 0);
    assert!(config.api.connection_timeout.as_secs() > 0);
}

#[test]
fn test_production_network_config() {
    let config = production_network_config();

    // Production should have higher limits
    assert_eq!(config.api.max_connections, 2000);
    assert_eq!(config.api.connection_timeout, Duration::from_secs(10));
}

#[test]
fn test_development_network_config() {
    let config = development_network_config();

    // Development should have lower limits and longer timeouts
    assert_eq!(config.api.max_connections, 100);
    assert_eq!(config.api.connection_timeout, Duration::from_secs(30));
}

#[test]
fn test_production_vs_development_config() {
    let prod = production_network_config();
    let dev = development_network_config();

    // Production should have more connections
    assert!(prod.api.max_connections > dev.api.max_connections);

    // Development should have longer timeout
    assert!(dev.api.connection_timeout > prod.api.connection_timeout);
}

#[test]
fn test_network_config_builder() {
    let config = NetworkConfigBuilder::new().build();

    // Builder should create valid config
    assert!(config.api.max_connections > 0);
}

#[test]
fn test_network_config_debug() {
    let config = default_network_config();
    let debug_str = format!("{:?}", config);

    // Debug output should contain meaningful information
    assert!(!debug_str.is_empty());
}

#[test]
fn test_network_config_clone() {
    let original = default_network_config();
    let cloned = original.clone();

    // Cloned config should have same values
    assert_eq!(original.api.max_connections, cloned.api.max_connections);
    assert_eq!(
        original.api.connection_timeout,
        cloned.api.connection_timeout
    );
}

#[test]
fn test_config_timeout_values() {
    let configs = vec![
        ("default", default_network_config()),
        ("production", production_network_config()),
        ("development", development_network_config()),
    ];

    for (name, config) in configs {
        // All configs should have reasonable timeout values
        let timeout_secs = config.api.connection_timeout.as_secs();
        assert!(
            timeout_secs >= 5,
            "{} timeout too short: {}",
            name,
            timeout_secs
        );
        assert!(
            timeout_secs <= 120,
            "{} timeout too long: {}",
            name,
            timeout_secs
        );
    }
}

#[test]
fn test_config_connection_limits() {
    let configs = vec![
        ("default", default_network_config()),
        ("production", production_network_config()),
        ("development", development_network_config()),
    ];

    for (name, config) in configs {
        // All configs should have reasonable connection limits
        let max_conns = config.api.max_connections;
        assert!(
            max_conns >= 10,
            "{} max_connections too low: {}",
            name,
            max_conns
        );
        assert!(
            max_conns <= 10000,
            "{} max_connections too high: {}",
            name,
            max_conns
        );
    }
}

#[test]
fn test_network_config_creation_multiple_times() {
    // Ensure configs can be created multiple times without issues
    for _ in 0..100 {
        let _ = default_network_config();
        let _ = production_network_config();
        let _ = development_network_config();
    }
}

#[test]
fn test_network_config_builder_multiple_builds() {
    // Create two separate builders with identical configuration
    let builder1 = NetworkConfigBuilder::new();
    let builder2 = NetworkConfigBuilder::new();

    // Each builder can build once
    let config1 = builder1.build();
    let config2 = builder2.build();

    assert_eq!(config1.api.max_connections, config2.api.max_connections);
}

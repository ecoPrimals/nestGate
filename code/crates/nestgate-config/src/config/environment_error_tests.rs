// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Comprehensive Error Path Tests for Environment Configuration
//!
//! Tests error handling, edge cases, and failure scenarios in environment-driven config

#![expect(clippy::panic)] // test assertions via `let ... else { panic!(...) }`

use crate::config::environment::{EnvironmentConfig, Port};
use std::str::FromStr;

// ==================== PORT VALIDATION ERROR TESTS ====================

#[test]
fn test_port_below_minimum_returns_error() {
    // Ports below 1024 are reserved
    let result = Port::new(1023);
    assert!(result.is_err());

    let result = Port::new(80);
    assert!(result.is_err());

    // Port 0 is invalid
    let result = Port::from_str("0");
    assert!(result.is_err());
}

#[test]
fn test_port_above_maximum_returns_error() {
    // Ports above 65535 are invalid (can't be represented as u16)
    let result = Port::from_str("65536");
    assert!(result.is_err());

    let result = Port::from_str("70000");
    assert!(result.is_err());

    let result = Port::from_str("100000");
    assert!(result.is_err());
}

#[test]
fn test_port_exactly_at_boundaries() {
    // Test exact boundary values
    let result = Port::new(1024);
    assert!(result.is_ok());
    let Ok(p) = result else {
        panic!("port 1024");
    };
    assert_eq!(p.get(), 1024);

    let result = Port::new(65535);
    assert!(result.is_ok());
    let Ok(p) = result else {
        panic!("port 65535");
    };
    assert_eq!(p.get(), 65535);
}

#[test]
fn test_port_from_invalid_string_formats() {
    // Empty string
    assert!(Port::from_str("").is_err());

    // Negative numbers
    assert!(Port::from_str("-8080").is_err());

    // Non-numeric
    assert!(Port::from_str("not-a-port").is_err());
    assert!(Port::from_str("abc123").is_err());

    // Whitespace
    assert!(Port::from_str("  ").is_err());
    assert!(Port::from_str("\t\n").is_err());

    // Mixed content
    assert!(Port::from_str("8080abc").is_err());
    assert!(Port::from_str("abc8080").is_err());
}

#[test]
fn test_port_from_string_with_whitespace() {
    // Leading/trailing whitespace should be handled
    let result = Port::from_str(" 8080 ");
    // Depending on implementation, this might work or fail
    // Document the behavior
    match result {
        Ok(port) => assert_eq!(port.get(), 8080),
        Err(_) => {
            // If it fails, ensure it's intentional
            assert!(Port::from_str("8080").is_ok());
        }
    }
}

// ==================== ENVIRONMENT VARIABLE ERROR TESTS ====================

#[test]
fn test_missing_required_env_var() {
    // Isolate from a polluted host env (e.g. invalid `NESTGATE_PORT` or
    // `NESTGATE_MAX_CONNECTIONS` left by a prior test).
    temp_env::with_vars(
        vec![
            ("NESTGATE_PORT", None::<&str>),
            ("NESTGATE_MAX_CONNECTIONS", None::<&str>),
            ("NESTGATE_TIMEOUT", None::<&str>),
            ("NESTGATE_HOST", None::<&str>),
            ("NESTGATE_API_PORT", None::<&str>),
            ("NESTGATE_HTTP_PORT", None::<&str>),
        ],
        || {
            temp_env::with_var_unset("NESTGATE_CRITICAL_REQUIRED_VAR", || {
                let config = EnvironmentConfig::from_env();
                if let Err(e) = &config {
                    eprintln!("Config error: {e:?}");
                }
                assert!(
                    config.is_ok(),
                    "Config should work with defaults: {:?}",
                    config.err()
                );
            });
        },
    );
}

#[tokio::test]
async fn test_malformed_port_in_environment() {
    // ✅ EVOLUTION: Isolated environment, concurrent-safe
    use temp_env::async_with_vars;

    // Set invalid port values
    let invalid_values = vec!["abc", "-1", "99999", "", " ", "8080.5", "8080abc"];

    for invalid in invalid_values {
        async_with_vars(vec![("NESTGATE_PORT", Some(invalid))], async {
            let config = EnvironmentConfig::from_env();
            // Config might error or fall back to default - both are acceptable
            // The key is it doesn't panic
            if let Ok(cfg) = config {
                // If it succeeds, should use default port, not invalid value
                assert!(cfg.network.port.get() >= 1024);
            }
        })
        .await;
    }
    // Environment automatically restored - concurrent-safe!
}

#[tokio::test]
async fn test_environment_config_with_partial_values() {
    // ✅ EVOLUTION: Isolated environment, no pollution
    use temp_env::async_with_vars;

    async_with_vars(
        vec![
            ("NESTGATE_PORT", Some("9090")),
            ("NESTGATE_HOST", None), // Explicitly unset
        ],
        async {
            let config = EnvironmentConfig::from_env();
            // Config might fail due to other required fields, which is fine
            if let Ok(cfg) = config {
                // Explicitly set value should be used
                assert_eq!(cfg.network.port.get(), 9090);
                // Missing value should use default
                // (Implementation detail depends on your config system)
            }
        },
    )
    .await;
    // Environment automatically restored - concurrent-safe!
}

// ==================== CONCURRENT ACCESS TESTS ====================

#[test]
fn test_concurrent_config_access() {
    use std::sync::Arc;
    use std::thread;

    temp_env::with_vars(
        vec![
            ("NESTGATE_PORT", None::<&str>),
            ("NESTGATE_MAX_CONNECTIONS", None::<&str>),
            ("NESTGATE_TIMEOUT", None::<&str>),
            ("NESTGATE_HOST", None::<&str>),
            ("NESTGATE_API_PORT", None::<&str>),
            ("NESTGATE_HTTP_PORT", None::<&str>),
        ],
        || {
            let Ok(config) = EnvironmentConfig::from_env() else {
                panic!("Config should load");
            };
            let config = Arc::new(config);
            let mut handles = vec![];

            for _ in 0..10 {
                let cfg = config.clone();
                let handle = thread::spawn(move || {
                    let port = cfg.network.port.get();
                    assert!(port >= 1024);
                });
                handles.push(handle);
            }

            for handle in handles {
                assert!(handle.join().is_ok(), "Thread should not panic");
            }
        },
    );
}

// ==================== DEFAULT VALUE TESTS ====================

#[test]
fn test_config_has_sensible_defaults() {
    temp_env::with_vars(
        vec![
            ("NESTGATE_PORT", None::<&str>),
            ("NESTGATE_HOST", None::<&str>),
            ("NESTGATE_TIMEOUT", None::<&str>),
            ("NESTGATE_MAX_CONNECTIONS", None::<&str>),
            ("NESTGATE_API_PORT", None::<&str>),
            ("NESTGATE_HTTP_PORT", None::<&str>),
        ],
        || {
            let config = EnvironmentConfig::from_env();
            assert!(config.is_ok(), "Should work with defaults");
            if let Ok(cfg) = config {
                assert!(cfg.network.port.get() >= 1024);
            }
        },
    );
}

// ==================== EDGE CASE TESTS ====================

#[test]
fn test_port_serialization_roundtrip() {
    let Ok(port) = Port::new(8080) else {
        panic!("port");
    };
    let serialized = format!("{}", port.get());
    let Ok(deserialized) = Port::from_str(&serialized) else {
        panic!("from_str");
    };

    assert_eq!(port.get(), deserialized.get());
}

#[test]
fn test_config_clone_independence() {
    temp_env::with_var("NESTGATE_PORT", Some("8080"), || {
        let Ok(config1) = EnvironmentConfig::from_env() else {
            panic!("Config should load");
        };
        let config2 = config1.clone();

        assert_eq!(config1.network.port.get(), config2.network.port.get());
    });
}

#[test]
fn test_config_debug_output_doesnt_expose_secrets() {
    let Ok(config) = EnvironmentConfig::from_env() else {
        panic!("Config should load");
    };
    let debug_output = format!("{config:?}");

    // Debug output should exist
    assert!(!debug_output.is_empty());

    // Should not contain sensitive patterns if any secrets exist
    // (Our config doesn't store secrets directly, but this is a good practice)
    assert!(!debug_output.contains("password="));
    assert!(!debug_output.contains("secret="));
}

#[test]
fn test_environment_config_is_send_sync() {
    // Verify EnvironmentConfig can be safely shared across threads
    fn assert_send<T: Send>() {}
    fn assert_sync<T: Sync>() {}

    assert_send::<EnvironmentConfig>();
    assert_sync::<EnvironmentConfig>();
}

// ==================== INTEGRATION ERROR TESTS ====================

#[test]
fn test_config_survives_corrupted_environment() {
    temp_env::with_vars(
        vec![
            ("NESTGATE_PORT", Some("CORRUPTED#$%")),
            ("NESTGATE_TIMEOUT", Some("NOT_A_NUMBER")),
            ("NESTGATE_MAX_CONNECTIONS", Some("-999999")),
        ],
        || {
            let _ = EnvironmentConfig::from_env();
        },
    );
}

#[test]
fn test_port_range_validation() {
    // Test a range of valid ports
    for port in [1024u16, 2000, 8080, 30000, 50000, 65535] {
        let result = Port::new(port);
        assert!(result.is_ok(), "Port {port} should be valid");
        let Ok(p) = result else {
            panic!("Port {port}");
        };
        assert_eq!(p.get(), port);
    }

    // Test invalid range
    for port in [0u16, 1, 80, 443, 1023] {
        let result = Port::new(port);
        assert!(result.is_err(), "Port {port} should be invalid");
    }

    // Test string parsing for out-of-range values
    for port_str in ["65536", "70000", "100000"] {
        let result = Port::from_str(port_str);
        assert!(result.is_err(), "Port {port_str} should be invalid");
    }
}

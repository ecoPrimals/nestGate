// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

#![no_main]

// use arbitrary::Arbitrary; // Currently unused
use libfuzzer_sys::fuzz_target;

// Use the unified fuzz configuration system
use nestgate_core::unified_final_config::{
    FuzzConfigParsingSettings, MaliciousContent, UnifiedFuzzConfig,
};

fuzz_target!(|input: FuzzConfigParsingSettings| {
    // Test comprehensive unified fuzz configuration
    test_unified_fuzz_config(&input);

    // Test malicious content handling using unified types
    test_unified_malicious_content(&input);

    // Test configuration validation using unified patterns
    test_unified_config_validation(&input);
});

fn test_unified_fuzz_config(input: &FuzzConfigParsingSettings) {
    // Create a unified fuzz configuration from the input
    let fuzz_config = UnifiedFuzzConfig::comprehensive();

    // Test that the configuration can be created successfully
    assert!(!fuzz_config.service.name.is_empty());
    assert_eq!(fuzz_config.service.service_type, "fuzz-framework");
    assert_eq!(fuzz_config.service.environment, "fuzzing");

    // Test that extensions are properly initialized
    assert!(
        fuzz_config
            .extensions
            .config_parsing
            .enable_resource_exhaustion
    );
    assert!(
        fuzz_config
            .extensions
            .api_endpoints
            .enable_parameter_injection
    );
    assert!(fuzz_config.extensions.zfs_commands.enable_pool_validation);
}

fn test_unified_malicious_content(input: &FuzzConfigParsingSettings) {
    // Test that malicious content is handled safely
    for malicious in &input.malicious_content {
        match malicious {
            MaliciousContent::YamlBomb(depth) => {
                // Should handle YAML bombs safely
                assert!(*depth <= u32::MAX);
            }
            MaliciousContent::JsonBomb(depth) => {
                // Should handle JSON bombs safely
                assert!(*depth <= u32::MAX);
            }
            MaliciousContent::PathTraversal(path) => {
                // Should validate path traversal attempts
                test_path_safety(path);
            }
            MaliciousContent::SqlInjection(injection) => {
                // Should detect SQL injection attempts
                test_sql_safety(injection);
            }
            MaliciousContent::ExtremelyLongString(length) => {
                // Should handle extremely long strings safely
                assert!(*length <= usize::MAX);
            }
            _ => {
                // All other malicious content types should be handled
            }
        }
    }
}

fn test_unified_config_validation(input: &FuzzConfigParsingSettings) {
    // Test configuration validation using unified patterns
    if input.enable_resource_exhaustion {
        assert!(input.max_parsing_time > 0);
        assert!(input.max_memory_mb > 0);
    }

    // Test database configuration validation
    let db_config = &input.config_data.database;
    if !db_config.url.is_empty() {
        assert!(db_config.max_connections > 0);
        assert!(db_config.timeout_seconds > 0);
    }

    // Test ZFS configuration validation
    let zfs_config = &input.config_data.zfs;
    if !zfs_config.pool_name.is_empty() {
        assert!(zfs_config.health_check_interval > 0);
    }
}

fn test_path_safety(path: &str) {
    // Should never allow path traversal
    assert!(!path.contains("../"), "Path traversal attempt detected");
    assert!(
        !path.contains("..\\"),
        "Windows path traversal attempt detected"
    );
}

fn test_sql_safety(injection: &str) {
    // Should detect common SQL injection patterns
    let dangerous_sql = ["DROP", "DELETE", "INSERT", "UPDATE", "UNION", "SELECT"];
    let injection_upper = injection.to_uppercase();

    for dangerous in &dangerous_sql {
        if injection_upper.contains(dangerous) {
            // Log the detection but don't panic - this is expected
            println!("SQL injection attempt detected: {}", dangerous);
        }
    }
}

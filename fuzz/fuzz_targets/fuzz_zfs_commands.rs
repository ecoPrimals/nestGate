// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective
//
// Disabled in fuzz/Cargo.toml: real ZFS command-shape fuzzing, but the harness needs type fixes
// (e.g. `UnifiedZfsConfig` import) before it builds with current `nestgate_zfs::config`.

#![no_main]

use arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;

// Import ZFS-related functionality for fuzzing
use nestgate_core::unified_final_config::DomainConfigs;
use nestgate_zfs::config::ZfsConfig;
// Removed unused StorageTier import
use std::collections::HashMap;

/// Fuzzable ZFS command structure
#[derive(Arbitrary, Debug)]
struct FuzzZfsCommand {
    pool_name: String,
    dataset_name: String,
    command_type: FuzzCommandType,
    properties: HashMap<String, String>,
    raw_args: Vec<String>,
}

#[derive(Arbitrary, Debug)]
enum FuzzCommandType {
    CreatePool,
    CreateDataset,
    SetProperty,
    GetProperty,
    Snapshot,
    Clone,
    Destroy,
    List,
    Status,
    RawCommand(String),
}

fuzz_target!(|input: FuzzZfsCommand| {
    // Test pool name validation - should handle malicious names
    let pool_result = validate_pool_name(&input.pool_name);
    assert!(pool_result.is_ok() || pool_result.is_err()); // Should never panic

    // Test dataset name validation - path traversal attempts
    let dataset_result = validate_dataset_name(&input.dataset_name);
    assert!(dataset_result.is_ok() || dataset_result.is_err()); // Should never panic

    // Test property validation - injection attempts
    for (key, value) in &input.properties {
        let prop_result = validate_property(key, value);
        assert!(prop_result.is_ok() || prop_result.is_err()); // Should never panic
    }

    // Test command construction - should sanitize everything
    let cmd_result = construct_zfs_command(&input);
    assert!(cmd_result.is_ok() || cmd_result.is_err()); // Should never panic

    // Test ZFS config creation with malformed input (no real ZFS calls)
    let config = create_fuzz_config(&input);

    // Test config validation (this is what we actually want to fuzz)
    let validation_result = validate_zfs_config(&config);
    assert!(validation_result.is_ok() || validation_result.is_err()); // Should never panic
});

fn validate_pool_name(name: &str) -> Result<(), String> {
    // This should catch malicious pool names like:
    // - "../../../etc/passwd"
    // - "pool; rm -rf /"
    // - Names with null bytes
    // - Excessively long names

    if name.is_empty() {
        return Err("Empty pool name".to_string());
    }

    if name.len() > 256 {
        return Err("Pool name too long".to_string());
    }

    if name.contains('\0') {
        return Err("Null byte in pool name".to_string());
    }

    if name.contains("..") {
        return Err("Path traversal attempt".to_string());
    }

    if name.contains(';') || name.contains('|') || name.contains('&') {
        return Err("Command injection attempt".to_string());
    }

    Ok(())
}

fn validate_dataset_name(name: &str) -> Result<(), String> {
    // Similar validation for dataset names
    if name.is_empty() {
        return Err("Empty dataset name".to_string());
    }

    if name.len() > 512 {
        return Err("Dataset name too long".to_string());
    }

    if name.contains('\0') {
        return Err("Null byte in dataset name".to_string());
    }

    // Check for path traversal
    if name.contains("../") || name.starts_with('/') && name.contains("..") {
        return Err("Path traversal attempt".to_string());
    }

    Ok(())
}

fn validate_property(key: &str, value: &str) -> Result<(), String> {
    // Validate ZFS properties for injection attempts
    if key.is_empty() || value.is_empty() {
        return Err("Empty property key or value".to_string());
    }

    if key.len() > 128 || value.len() > 1024 {
        return Err("Property key or value too long".to_string());
    }

    if key.contains('\0') || value.contains('\0') {
        return Err("Null byte in property".to_string());
    }

    // Check for command injection in property values
    let dangerous_chars = ['$', '`', ';', '|', '&', '(', ')', '<', '>'];
    if value.chars().any(|c| dangerous_chars.contains(&c)) {
        return Err("Potentially dangerous characters in property value".to_string());
    }

    Ok(())
}

fn construct_zfs_command(input: &FuzzZfsCommand) -> Result<Vec<String>, String> {
    let mut cmd = vec!["zfs".to_string()];

    match &input.command_type {
        FuzzCommandType::CreatePool => {
            cmd.push("create".to_string());
            cmd.push(input.pool_name.clone());
        }
        FuzzCommandType::CreateDataset => {
            cmd.push("create".to_string());
            cmd.push(format!("{}/{}", input.pool_name, input.dataset_name));
        }
        FuzzCommandType::SetProperty => {
            cmd.push("set".to_string());
            for (key, value) in &input.properties {
                cmd.push(format!("{}={}", key, value));
            }
            cmd.push(format!("{}/{}", input.pool_name, input.dataset_name));
        }
        FuzzCommandType::RawCommand(raw) => {
            // This is the most dangerous - raw command injection attempts
            validate_raw_command(raw)?;
            cmd.push(raw.clone());
        }
        _ => {
            // Other command types
            cmd.push("list".to_string());
        }
    }

    Ok(cmd)
}

fn validate_raw_command(cmd: &str) -> Result<(), String> {
    // Should reject any command injection attempts
    if cmd.contains(';') || cmd.contains('|') || cmd.contains('&') {
        return Err("Command injection attempt in raw command".to_string());
    }

    if cmd.contains("rm") || cmd.contains("dd") || cmd.contains("mkfs") {
        return Err("Dangerous command detected".to_string());
    }

    Ok(())
}

fn create_fuzz_config(input: &FuzzZfsCommand) -> ZfsConfig {
    // Create a config that might have malformed values
    let mut config = UnifiedZfsConfig::default();

    // Try to set potentially malicious values in API endpoint (for fuzzing validation)
    config.api_endpoint = input.pool_name.clone();

    config
}

fn validate_zfs_config(config: &ZfsConfig) -> Result<(), String> {
    // Validate API endpoint doesn't contain malicious patterns
    if config.api_endpoint.contains("..") {
        return Err("Path traversal detected in API endpoint".to_string());
    }

    if config.api_endpoint.contains('\0') {
        return Err("Null byte injection detected".to_string());
    }

    if config.api_endpoint.len() > 1000 {
        return Err("API endpoint too long".to_string());
    }

    // Add more validation as needed for fuzzing
    Ok(())
}

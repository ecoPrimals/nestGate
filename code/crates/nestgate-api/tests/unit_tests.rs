// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

#![cfg_attr(
    test,
    allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::too_many_lines,
        clippy::cognitive_complexity,
    )
)]
#![allow(
    deprecated,
    missing_docs,
    dead_code,
    unfulfilled_lint_expectations,
    unused_doc_comments,
    unused_imports,
    unused_variables,
    unused_comparisons,
    unused_must_use,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::doc_markdown,
    clippy::module_name_repetitions,
    clippy::struct_excessive_bools,
    clippy::struct_field_names,
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap,
    clippy::must_use_candidate,
    clippy::return_self_not_must_use,
    clippy::unnecessary_wraps,
    clippy::unused_self,
    clippy::unused_async,
    clippy::needless_pass_by_value,
    clippy::option_if_let_else,
    clippy::too_long_first_doc_paragraph,
    clippy::inline_always,
    clippy::redundant_closure,
    clippy::redundant_closure_for_method_calls,
    clippy::collapsible_if,
    clippy::single_char_pattern,
    clippy::implicit_hasher,
    clippy::float_cmp,
    clippy::uninlined_format_args,
    clippy::similar_names,
    clippy::items_after_statements,
    clippy::empty_line_after_doc_comments,
    clippy::unreadable_literal,
    clippy::manual_clamp,
    clippy::pub_underscore_fields,
    clippy::case_sensitive_file_extension_comparisons,
    clippy::wildcard_in_or_patterns,
    clippy::type_complexity,
    clippy::field_reassign_with_default,
    clippy::module_inception,
    clippy::unnecessary_get_then_check,
    clippy::cmp_null,
    clippy::redundant_clone,
    clippy::absurd_extreme_comparisons,
    clippy::no_effect_underscore_binding,
    clippy::default_constructed_unit_structs,
    clippy::manual_string_new,
    clippy::assertions_on_constants,
    clippy::unnecessary_unwrap,
    clippy::needless_collect,
    clippy::drop_non_drop,
    clippy::zero_sized_map_values,
    clippy::match_single_binding,
    clippy::match_same_arms,
    clippy::overly_complex_bool_expr,
    clippy::needless_character_iteration,
    clippy::manual_range_contains,
    clippy::bool_assert_comparison,
    clippy::single_component_path_imports,
    clippy::used_underscore_binding
)]

//! Unit Tests for `NestGate` API
//!
//! This test suite validates individual API components, handlers, and utilities
//! in isolation using proper mocks and test doubles.

use nestgate_api::handlers::zfs::types::*;
use nestgate_core::response::api_response::ApiResponse;
use std::collections::HashMap;

#[test]
fn test_api_response_success() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let response = ApiResponse::success("test data".to_string());
    assert!(response.success);
    assert_eq!(response.data, Some("test data".to_string()));
    assert!(response.error.is_none());
    assert!(!response.timestamp.to_string().is_empty());
    Ok(())
}

#[test]
fn test_api_response_error() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let response = ApiResponse::<()>::error("Test error message".to_string());

    assert!(!response.success);
    assert!(response.data.is_none());
    assert_eq!(response.error, Some("Test error message".to_string()));
    assert!(!response.timestamp.to_string().is_empty());
    Ok(())
}

#[test]
fn test_create_pool_request_serialization() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let request = CreatePoolRequest {
        name: "test_pool".to_string(),
        _devices: vec!["/dev/sda".to_string(), "/dev/sdb".to_string()],
        config: Some(PoolConfig {
            raid_level: Some("mirror".to_string()),
            compression: Some("lz4".to_string()),
            dedup: Some(false),
            encryption: Some(true),
        }),
    };

    let json_str = serde_json::to_string(&request)?;
    assert!(!json_str.is_empty());

    let parsed: CreatePoolRequest = serde_json::from_str(&json_str)?;
    assert_eq!(parsed.name, "test_pool");
    assert_eq!(parsed._devices.len(), 2);

    if let Some(config) = parsed.config {
        assert_eq!(config.raid_level, Some("mirror".to_string()));
        assert_eq!(config.compression, Some("lz4".to_string()));
        assert_eq!(config.dedup, Some(false));
        assert_eq!(config.encryption, Some(true));
    } else {
        panic!("Config should be present");
    }

    Ok(())
}

#[test]
fn test_create_dataset_request_serialization() -> std::result::Result<(), Box<dyn std::error::Error>>
{
    use nestgate_core::canonical_types::StorageTier;

    let request = CreateDatasetRequest {
        name: "test_dataset".to_string(),
        parent: "test_pool".to_string(),
        tier: StorageTier::Hot,
        properties: Some(HashMap::from([
            ("compression".to_string(), "lz4".to_string()),
            ("mountpoint".to_string(), "/mnt/test".to_string()),
        ])),
    };

    let json_str = serde_json::to_string(&request)?;
    assert!(!json_str.is_empty());

    let parsed: CreateDatasetRequest = serde_json::from_str(&json_str)?;
    assert_eq!(parsed.name, "test_dataset");
    assert_eq!(parsed.parent, "test_pool");

    if let Some(props) = parsed.properties {
        assert_eq!(props.get("compression"), Some(&"lz4".to_string()));
        assert_eq!(props.get("mountpoint"), Some(&"/mnt/test".to_string()));
    } else {
        panic!("Properties should be present");
    }

    Ok(())
}

#[test]
fn test_pool_config_defaults() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let minimal_config = PoolConfig {
        raid_level: None,
        compression: None,
        dedup: None,
        encryption: None,
    };

    let json_str = serde_json::to_string(&minimal_config)?;
    let parsed: PoolConfig = serde_json::from_str(&json_str)?;

    assert!(parsed.raid_level.is_none());
    assert!(parsed.compression.is_none());
    assert!(parsed.dedup.is_none());
    assert!(parsed.encryption.is_none());

    Ok(())
}

#[test]
fn test_zfs_api_state_structure() -> std::result::Result<(), Box<dyn std::error::Error>> {
    // Test that ZfsApiState can be constructed
    // Note: We can't easily test the actual state without dependencies
    // This test validates the structure exists and is accessible

    // This is a structural test - just ensure the types are available
    let _type_check = std::marker::PhantomData::<ZfsApiState>;

    Ok(())
}

#[test]
fn test_request_validation_edge_cases() -> std::result::Result<(), Box<dyn std::error::Error>> {
    // Test empty pool name
    let empty_name_request = CreatePoolRequest {
        name: String::new(),
        _devices: vec!["/dev/sda".to_string()],
        config: None,
    };

    let json = serde_json::to_string(&empty_name_request)?;
    let parsed: CreatePoolRequest = serde_json::from_str(&json)?;
    assert_eq!(parsed.name, "");

    // Test empty devices list
    let empty_devices_request = CreatePoolRequest {
        name: "test_pool".to_string(),
        _devices: vec![],
        config: None,
    };

    let json = serde_json::to_string(&empty_devices_request)?;
    let parsed: CreatePoolRequest = serde_json::from_str(&json)?;
    assert!(parsed._devices.is_empty());

    Ok(())
}

#[test]
fn test_dataset_request_minimal() -> std::result::Result<(), Box<dyn std::error::Error>> {
    use nestgate_core::canonical_types::StorageTier;

    let minimal_request = CreateDatasetRequest {
        name: "dataset".to_string(),
        parent: "pool".to_string(),
        tier: StorageTier::Hot,
        properties: None,
    };

    let json = serde_json::to_string(&minimal_request)?;
    let parsed: CreateDatasetRequest = serde_json::from_str(&json)?;

    assert_eq!(parsed.name, "dataset");
    assert_eq!(parsed.parent, "pool");
    assert!(parsed.properties.is_none());

    Ok(())
}

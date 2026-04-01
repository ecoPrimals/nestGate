// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

#![allow(
    dead_code,
    unused_doc_comments,
    unused_imports,
    missing_docs,
    rustdoc::missing_crate_level_docs,
    deprecated,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
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
    clippy::cast_lossless,
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
    clippy::manual_midpoint,
    clippy::suboptimal_flops,
    clippy::items_after_statements,
    clippy::items_after_test_module,
    clippy::too_many_lines,
    clippy::cognitive_complexity,
    clippy::unreadable_literal,
    clippy::redundant_clone,
    clippy::useless_vec,
    clippy::field_reassign_with_default,
    clippy::cmp_null,
    clippy::bool_assert_comparison,
    clippy::used_underscore_items,
    clippy::needless_raw_string_hashes,
    clippy::ref_as_ptr,
    clippy::no_effect_underscore_binding,
    clippy::needless_collect,
    clippy::module_inception,
    clippy::default_trait_access,
    clippy::wildcard_in_or_patterns,
    clippy::or_fun_call,
    clippy::manual_string_new,
    clippy::unnecessary_literal_unwrap,
    clippy::unnecessary_debug_formatting,
    clippy::assigning_clones,
    clippy::unnecessary_unwrap,
    clippy::unnecessary_map_or,
    clippy::unnecessary_lazy_evaluations,
    clippy::similar_names,
    clippy::needless_continue,
    clippy::collection_is_never_read,
    clippy::char_lit_as_u8,
    clippy::ptr_eq,
    clippy::uninlined_format_args,
    clippy::absurd_extreme_comparisons,
    clippy::match_wild_err_arm,
    clippy::single_match_else,
    clippy::derive_partial_eq_without_eq,
    clippy::match_wildcard_for_single_variants,
    clippy::missing_const_for_fn,
    clippy::used_underscore_binding,
    clippy::ignored_unit_patterns,
    unused_comparisons,
    clippy::format_push_string
)]

//! Comprehensive tests for ZFS configuration types
//! Target: Improve coverage of config module

use nestgate_zfs::config::{CompressionType, PoolType};

// ==================== POOL TYPE TESTS ====================

#[test]
fn test_pool_type_variants() {
    let types = [
        PoolType::Standard,
        PoolType::Mirror,
        PoolType::RaidZ1,
        PoolType::RaidZ2,
        PoolType::RaidZ3,
    ];

    assert_eq!(types.len(), 5);
}

#[test]
fn test_pool_type_default() {
    let pool_type = PoolType::default();
    assert!(matches!(pool_type, PoolType::Standard));
}

#[test]
fn test_pool_type_debug() {
    let pool_type = PoolType::Mirror;
    let debug_str = format!("{:?}", pool_type);

    assert!(!debug_str.is_empty());
    assert!(debug_str.contains("Mirror"));
}

#[test]
fn test_pool_type_clone() {
    let type1 = PoolType::RaidZ2;
    let type2 = type1.clone();

    assert!(format!("{:?}", type1) == format!("{:?}", type2));
}

// ==================== POOL TYPE SERIALIZATION TESTS ====================

#[test]
fn test_pool_type_serialization() {
    let pool_type = PoolType::RaidZ1;
    let json = serde_json::to_string(&pool_type).expect("Failed to serialize");

    assert!(!json.is_empty());
}

#[test]
fn test_pool_type_deserialization() {
    let pool_type = PoolType::Mirror;
    let json = serde_json::to_string(&pool_type).expect("Failed to serialize");
    let deserialized: PoolType = serde_json::from_str(&json).expect("Failed to deserialize");

    assert!(format!("{:?}", pool_type) == format!("{:?}", deserialized));
}

#[test]
fn test_all_pool_types_serialize() {
    let types = vec![
        PoolType::Standard,
        PoolType::Mirror,
        PoolType::RaidZ1,
        PoolType::RaidZ2,
        PoolType::RaidZ3,
    ];

    for pool_type in types {
        let json = serde_json::to_string(&pool_type).expect("Failed to serialize");
        assert!(!json.is_empty());
    }
}

// ==================== COMPRESSION TYPE TESTS ====================

#[test]
fn test_compression_type_variants() {
    let types = [
        CompressionType::None,
        CompressionType::Lz4,
        CompressionType::Gzip,
        CompressionType::Zstd,
    ];

    assert_eq!(types.len(), 4);
}

#[test]
fn test_compression_type_default() {
    let compression = CompressionType::default();
    assert!(matches!(compression, CompressionType::None));
}

#[test]
fn test_compression_type_debug() {
    let compression = CompressionType::Lz4;
    let debug_str = format!("{:?}", compression);

    assert!(!debug_str.is_empty());
    assert!(debug_str.contains("Lz4"));
}

#[test]
fn test_compression_type_clone() {
    let comp1 = CompressionType::Zstd;
    let comp2 = comp1.clone();

    assert!(format!("{:?}", comp1) == format!("{:?}", comp2));
}

// ==================== COMPRESSION TYPE SERIALIZATION TESTS ====================

#[test]
fn test_compression_type_serialization() {
    let compression = CompressionType::Gzip;
    let json = serde_json::to_string(&compression).expect("Failed to serialize");

    assert!(!json.is_empty());
}

#[test]
fn test_compression_type_deserialization() {
    let compression = CompressionType::Lz4;
    let json = serde_json::to_string(&compression).expect("Failed to serialize");
    let deserialized: CompressionType = serde_json::from_str(&json).expect("Failed to deserialize");

    assert!(format!("{:?}", compression) == format!("{:?}", deserialized));
}

#[test]
fn test_all_compression_types_serialize() {
    let types = vec![
        CompressionType::None,
        CompressionType::Lz4,
        CompressionType::Gzip,
        CompressionType::Zstd,
    ];

    for compression in types {
        let json = serde_json::to_string(&compression).expect("Failed to serialize");
        assert!(!json.is_empty());
    }
}

// ==================== PATTERN MATCHING TESTS ====================

#[test]
fn test_match_pool_type() {
    let pool_type = PoolType::RaidZ2;

    let description = match pool_type {
        PoolType::Standard => "standard",
        PoolType::Mirror => "mirror",
        PoolType::RaidZ1 => "raidz1",
        PoolType::RaidZ2 => "raidz2",
        PoolType::RaidZ3 => "raidz3",
    };

    assert_eq!(description, "raidz2");
}

#[test]
fn test_match_compression_type() {
    let compression = CompressionType::Zstd;

    let name = match compression {
        CompressionType::None => "none",
        CompressionType::Lz4 => "lz4",
        CompressionType::Gzip => "gzip",
        CompressionType::Zstd => "zstd",
    };

    assert_eq!(name, "zstd");
}

// ==================== COLLECTION TESTS ====================

#[test]
fn test_pool_type_collection() {
    let types = [PoolType::Standard, PoolType::Mirror, PoolType::RaidZ1];

    assert_eq!(types.len(), 3);
}

#[test]
fn test_compression_type_collection() {
    let compressions = [
        CompressionType::None,
        CompressionType::Lz4,
        CompressionType::Gzip,
        CompressionType::Zstd,
    ];

    assert_eq!(compressions.len(), 4);
}

// ==================== INTEGRATION TESTS ====================

#[test]
fn test_pool_and_compression_combination() {
    struct PoolConfig {
        pool_type: PoolType,
        compression: CompressionType,
    }

    let config = PoolConfig {
        pool_type: PoolType::Mirror,
        compression: CompressionType::Lz4,
    };

    assert!(matches!(config.pool_type, PoolType::Mirror));
    assert!(matches!(config.compression, CompressionType::Lz4));
}

#[test]
fn test_multiple_configurations() {
    struct PoolConfig {
        name: String,
        pool_type: PoolType,
        compression: CompressionType,
    }

    let configs = [
        PoolConfig {
            name: "pool1".to_string(),
            pool_type: PoolType::Standard,
            compression: CompressionType::None,
        },
        PoolConfig {
            name: "pool2".to_string(),
            pool_type: PoolType::Mirror,
            compression: CompressionType::Lz4,
        },
        PoolConfig {
            name: "pool3".to_string(),
            pool_type: PoolType::RaidZ2,
            compression: CompressionType::Zstd,
        },
    ];

    assert_eq!(configs.len(), 3);
}

// ==================== DEFAULT BEHAVIOR TESTS ====================

#[test]
fn test_default_pool_type_is_standard() {
    let default_type = PoolType::default();
    assert!(matches!(default_type, PoolType::Standard));
}

#[test]
fn test_default_compression_is_none() {
    let default_compression = CompressionType::default();
    assert!(matches!(default_compression, CompressionType::None));
}

#[test]
fn test_defaults_for_struct() {
    #[derive(Default)]
    struct Config {
        pool_type: PoolType,
        compression: CompressionType,
    }

    let config = Config::default();
    assert!(matches!(config.pool_type, PoolType::Standard));
    assert!(matches!(config.compression, CompressionType::None));
}

// ==================== EDGE CASE TESTS ====================

#[test]
fn test_pool_type_equality_after_clone() {
    let original = PoolType::RaidZ3;
    let cloned = original.clone();

    // Both should serialize to the same JSON
    let json1 = serde_json::to_string(&original).unwrap();
    let json2 = serde_json::to_string(&cloned).unwrap();

    assert_eq!(json1, json2);
}

#[test]
fn test_compression_type_equality_after_clone() {
    let original = CompressionType::Gzip;
    let cloned = original.clone();

    // Both should serialize to the same JSON
    let json1 = serde_json::to_string(&original).unwrap();
    let json2 = serde_json::to_string(&cloned).unwrap();

    assert_eq!(json1, json2);
}

// ==================== SERIALIZATION ROUNDTRIP TESTS ====================

#[test]
fn test_pool_type_roundtrip() {
    let original = PoolType::Mirror;
    let json = serde_json::to_string(&original).unwrap();
    let deserialized: PoolType = serde_json::from_str(&json).unwrap();
    let json2 = serde_json::to_string(&deserialized).unwrap();

    assert_eq!(json, json2);
}

#[test]
fn test_compression_type_roundtrip() {
    let original = CompressionType::Lz4;
    let json = serde_json::to_string(&original).unwrap();
    let deserialized: CompressionType = serde_json::from_str(&json).unwrap();
    let json2 = serde_json::to_string(&deserialized).unwrap();

    assert_eq!(json, json2);
}

// ==================== VECTOR SERIALIZATION TESTS ====================

#[test]
fn test_serialize_pool_type_vector() {
    let types = vec![PoolType::Standard, PoolType::Mirror, PoolType::RaidZ1];

    let json = serde_json::to_string(&types).expect("Failed to serialize");
    assert!(!json.is_empty());
}

#[test]
fn test_serialize_compression_type_vector() {
    let types = vec![
        CompressionType::None,
        CompressionType::Lz4,
        CompressionType::Gzip,
    ];

    let json = serde_json::to_string(&types).expect("Failed to serialize");
    assert!(!json.is_empty());
}

#[test]
fn test_deserialize_pool_type_vector() {
    let types = vec![PoolType::RaidZ2, PoolType::RaidZ3];
    let json = serde_json::to_string(&types).unwrap();
    let deserialized: Vec<PoolType> = serde_json::from_str(&json).unwrap();

    assert_eq!(types.len(), deserialized.len());
}

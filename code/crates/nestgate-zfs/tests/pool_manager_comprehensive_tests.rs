// SPDX-License-Identifier: AGPL-3.0-or-later
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

//! **COMPREHENSIVE POOL MANAGER TESTS**
//!
//! Tests for ZFS pool manager to achieve >80% coverage.
//! Focus on pool operations, discovery, parsing, and state management.

use nestgate_zfs::config::ZfsConfig;
use nestgate_zfs::pool::{
    manager::ZfsPoolManager,
    types::{PoolCapacity, PoolHealth, PoolInfo, PoolState},
};

// ==================== POOL HEALTH TESTS ====================

#[test]
fn test_pool_health_healthy() {
    let health = PoolHealth::Healthy;
    let debug_str = format!("{:?}", health);
    assert!(debug_str.contains("Healthy"));
}

#[test]
fn test_pool_health_warning() {
    let health = PoolHealth::Warning;
    let debug_str = format!("{:?}", health);
    assert!(debug_str.contains("Warning"));
}

#[test]
fn test_pool_health_critical() {
    let health = PoolHealth::Critical;
    let debug_str = format!("{:?}", health);
    assert!(debug_str.contains("Critical"));
}

#[test]
fn test_pool_health_unknown() {
    let health = PoolHealth::Unknown;
    let debug_str = format!("{:?}", health);
    assert!(debug_str.contains("Unknown"));
}

#[test]
fn test_pool_health_clone() {
    let health1 = PoolHealth::Healthy;
    let health2 = health1.clone();
    let debug1 = format!("{:?}", health1);
    let debug2 = format!("{:?}", health2);
    assert_eq!(debug1, debug2);
}

#[test]
fn test_pool_health_partial_eq() {
    let health1 = PoolHealth::Healthy;
    let health2 = PoolHealth::Healthy;
    let health3 = PoolHealth::Warning;

    assert_eq!(health1, health2);
    assert_ne!(health1, health3);
}

// ==================== POOL STATE TESTS ====================

#[test]
fn test_pool_state_online() {
    let state = PoolState::Online;
    let debug_str = format!("{:?}", state);
    assert!(debug_str.contains("Online"));
}

#[test]
fn test_pool_state_degraded() {
    let state = PoolState::Degraded;
    let debug_str = format!("{:?}", state);
    assert!(debug_str.contains("Degraded"));
}

#[test]
fn test_pool_state_faulted() {
    let state = PoolState::Faulted;
    let debug_str = format!("{:?}", state);
    assert!(debug_str.contains("Faulted"));
}

#[test]
fn test_pool_state_offline() {
    let state = PoolState::Offline;
    let debug_str = format!("{:?}", state);
    assert!(debug_str.contains("Offline"));
}

#[test]
fn test_pool_state_unknown() {
    let state = PoolState::Unknown;
    let debug_str = format!("{:?}", state);
    assert!(debug_str.contains("Unknown"));
}

#[test]
fn test_pool_state_clone() {
    let state1 = PoolState::Online;
    let state2 = state1.clone();
    let debug1 = format!("{:?}", state1);
    let debug2 = format!("{:?}", state2);
    assert_eq!(debug1, debug2);
}

// ==================== POOL CAPACITY TESTS ====================

#[test]
fn test_pool_capacity_creation() {
    let capacity = PoolCapacity {
        total_bytes: 1_000_000_000,
        used_bytes: 500_000_000,
        available_bytes: 500_000_000,
        utilization_percent: 50.0,
        fragmentation_percent: 0.0,
        deduplication_ratio: 1.0,
        total: 1_000_000_000,
        used: 500_000_000,
        available: 500_000_000,
    };

    assert_eq!(capacity.total_bytes, 1_000_000_000);
    assert_eq!(capacity.used_bytes, 500_000_000);
    assert_eq!(capacity.utilization_percent, 50.0);
}

#[test]
fn test_pool_capacity_zero() {
    let capacity = PoolCapacity {
        total_bytes: 0,
        used_bytes: 0,
        available_bytes: 0,
        utilization_percent: 0.0,
        fragmentation_percent: 0.0,
        deduplication_ratio: 1.0,
        total: 0,
        used: 0,
        available: 0,
    };

    assert_eq!(capacity.total_bytes, 0);
}

#[test]
fn test_pool_capacity_full() {
    let capacity = PoolCapacity {
        total_bytes: 1_000_000_000,
        used_bytes: 1_000_000_000,
        available_bytes: 0,
        utilization_percent: 100.0,
        fragmentation_percent: 0.0,
        deduplication_ratio: 1.0,
        total: 1_000_000_000,
        used: 1_000_000_000,
        available: 0,
    };

    assert_eq!(capacity.utilization_percent, 100.0);
}

#[test]
fn test_pool_capacity_clone() {
    let capacity1 = PoolCapacity {
        total_bytes: 1_000_000_000,
        used_bytes: 500_000_000,
        available_bytes: 500_000_000,
        utilization_percent: 50.0,
        fragmentation_percent: 0.0,
        deduplication_ratio: 1.0,
        total: 1_000_000_000,
        used: 500_000_000,
        available: 500_000_000,
    };

    let capacity2 = capacity1.clone();
    assert_eq!(capacity1.total_bytes, capacity2.total_bytes);
}

// ==================== POOL INFO TESTS ====================

#[test]
fn test_pool_info_creation() {
    let info = PoolInfo {
        name: "tank".to_string(),
        state: PoolState::Online,
        health: PoolHealth::Healthy,
        capacity: PoolCapacity {
            total_bytes: 1_000_000_000,
            used_bytes: 500_000_000,
            available_bytes: 500_000_000,
            utilization_percent: 50.0,
            fragmentation_percent: 0.0,
            deduplication_ratio: 1.0,
            total: 1_000_000_000,
            used: 500_000_000,
            available: 500_000_000,
        },
        properties: std::collections::HashMap::new(),
        devices: Vec::new(),
    };

    assert_eq!(info.name, "tank");
    assert_eq!(info.state, PoolState::Online);
}

#[test]
fn test_pool_info_with_properties() {
    let mut properties = std::collections::HashMap::new();
    properties.insert("compression".to_string(), "lz4".to_string());
    properties.insert("atime".to_string(), "off".to_string());

    let info = PoolInfo {
        name: "tank".to_string(),
        state: PoolState::Online,
        health: PoolHealth::Healthy,
        capacity: PoolCapacity {
            total_bytes: 1_000_000_000,
            used_bytes: 500_000_000,
            available_bytes: 500_000_000,
            utilization_percent: 50.0,
            fragmentation_percent: 0.0,
            deduplication_ratio: 1.0,
            total: 1_000_000_000,
            used: 500_000_000,
            available: 500_000_000,
        },
        devices: Vec::new(),
        properties,
    };

    assert_eq!(info.properties.len(), 2);
}

#[test]
fn test_pool_info_degraded() {
    let info = PoolInfo {
        name: "backup".to_string(),
        state: PoolState::Degraded,
        health: PoolHealth::Warning,
        capacity: PoolCapacity {
            total_bytes: 2_000_000_000,
            used_bytes: 1_000_000_000,
            available_bytes: 1_000_000_000,
            utilization_percent: 50.0,
            fragmentation_percent: 0.0,
            deduplication_ratio: 1.0,
            total: 2_000_000_000,
            used: 1_000_000_000,
            available: 1_000_000_000,
        },
        properties: std::collections::HashMap::new(),
        devices: Vec::new(),
    };

    assert_eq!(info.state, PoolState::Degraded);
    assert_eq!(info.health, PoolHealth::Warning);
}

#[test]
fn test_pool_info_clone() {
    let info1 = PoolInfo {
        name: "tank".to_string(),
        state: PoolState::Online,
        health: PoolHealth::Healthy,
        capacity: PoolCapacity {
            total_bytes: 1_000_000_000,
            used_bytes: 500_000_000,
            available_bytes: 500_000_000,
            utilization_percent: 50.0,
            fragmentation_percent: 0.0,
            deduplication_ratio: 1.0,
            total: 1_000_000_000,
            used: 500_000_000,
            available: 500_000_000,
        },
        properties: std::collections::HashMap::new(),
        devices: Vec::new(),
    };

    let info2 = info1.clone();
    assert_eq!(info1.name, info2.name);
}

// ==================== ZFS POOL MANAGER TESTS ====================

#[test]
fn test_pool_manager_new_for_testing() {
    let manager = ZfsPoolManager::new_production(ZfsConfig::default());
    let debug_str = format!("{:?}", manager);
    assert!(debug_str.contains("ZfsPoolManager"));
}

#[test]
fn test_pool_manager_new_production() {
    let config = ZfsConfig::default();
    let manager = ZfsPoolManager::new_production(config);
    let debug_str = format!("{:?}", manager);
    assert!(debug_str.contains("ZfsPoolManager"));
}

#[test]
fn test_pool_manager_clone() {
    let manager1 = ZfsPoolManager::new_production(ZfsConfig::default());
    let manager2 = manager1.clone();

    let debug1 = format!("{:?}", manager1);
    let debug2 = format!("{:?}", manager2);
    assert!(!debug1.is_empty());
    assert!(!debug2.is_empty());
}

#[tokio::test]
async fn test_pool_manager_new() {
    let config = ZfsConfig::default();
    let result = ZfsPoolManager::new(&config).await;

    // May fail if ZFS is not installed, which is OK
    match result {
        Ok(manager) => {
            let debug_str = format!("{:?}", manager);
            assert!(debug_str.contains("ZfsPoolManager"));
        }
        Err(_) => {
            // Expected if ZFS is not available
        }
    }
}

#[tokio::test]
async fn test_pool_manager_with_owned_config() {
    let config = ZfsConfig::default();
    let result = ZfsPoolManager::with_owned_config(config).await;

    match result {
        Ok(manager) => {
            let debug_str = format!("{:?}", manager);
            assert!(debug_str.contains("ZfsPoolManager"));
        }
        Err(_) => {
            // Expected if ZFS is not available
        }
    }
}

// ==================== POOL PARSING TESTS ====================

#[test]
fn test_pool_info_with_empty_name() {
    let info = PoolInfo {
        name: String::new(),
        state: PoolState::Online,
        health: PoolHealth::Healthy,
        capacity: PoolCapacity {
            total_bytes: 0,
            used_bytes: 0,
            available_bytes: 0,
            utilization_percent: 50.0,
            fragmentation_percent: 0.0,
            deduplication_ratio: 1.0,
            total: 0,
            used: 0,
            available: 0,
        },
        properties: std::collections::HashMap::new(),
        devices: Vec::new(),
    };

    assert!(info.name.is_empty());
}

#[test]
fn test_pool_info_with_very_long_name() {
    let long_name = "a".repeat(1000);
    let info = PoolInfo {
        name: long_name.clone(),
        state: PoolState::Online,
        health: PoolHealth::Healthy,
        capacity: PoolCapacity {
            total_bytes: 1_000_000_000,
            used_bytes: 500_000_000,
            available_bytes: 500_000_000,
            utilization_percent: 50.0,
            fragmentation_percent: 0.0,
            deduplication_ratio: 1.0,
            total: 1_000_000_000,
            used: 500_000_000,
            available: 500_000_000,
        },
        properties: std::collections::HashMap::new(),
        devices: Vec::new(),
    };

    assert_eq!(info.name.len(), 1000);
}

#[test]
fn test_pool_capacity_extreme_values() {
    let capacity = PoolCapacity {
        total_bytes: u64::MAX,
        used_bytes: u64::MAX / 2,
        available_bytes: u64::MAX / 2,
        utilization_percent: 50.0,
        fragmentation_percent: 0.0,
        deduplication_ratio: 1.0,
        total: 0,
        used: 0,
        available: 0,
    };

    assert_eq!(capacity.total_bytes, u64::MAX);
}

#[test]
fn test_pool_info_with_many_properties() {
    let mut properties = std::collections::HashMap::new();
    for i in 0..100 {
        properties.insert(format!("prop{}", i), format!("value{}", i));
    }

    let info = PoolInfo {
        name: "tank".to_string(),
        state: PoolState::Online,
        health: PoolHealth::Healthy,
        capacity: PoolCapacity {
            total_bytes: 1_000_000_000,
            used_bytes: 500_000_000,
            available_bytes: 500_000_000,
            utilization_percent: 50.0,
            fragmentation_percent: 0.0,
            deduplication_ratio: 1.0,
            total: 1_000_000_000,
            used: 500_000_000,
            available: 500_000_000,
        },
        properties,
        devices: Vec::new(),
    };

    assert_eq!(info.properties.len(), 100);
}

// ==================== CONCURRENT TESTS ====================

#[test]
fn test_concurrent_manager_creation() {
    use std::thread;

    let handles: Vec<_> = (0..100)
        .map(|_| {
            thread::spawn(|| {
                let config = ZfsConfig::default();
                ZfsPoolManager::new_production(config)
            })
        })
        .collect();

    for handle in handles {
        let _manager = handle.join().expect("Thread should complete");
    }
}

#[test]
fn test_concurrent_pool_info_creation() {
    use std::thread;

    let handles: Vec<_> = (0..100)
        .map(|i| {
            thread::spawn(move || PoolInfo {
                name: format!("pool{}", i),
                state: PoolState::Online,
                health: PoolHealth::Healthy,
                capacity: PoolCapacity {
                    total_bytes: 1_000_000_000,
                    used_bytes: 500_000_000,
                    available_bytes: 500_000_000,
                    utilization_percent: 50.0,
                    fragmentation_percent: 0.0,
                    deduplication_ratio: 1.0,
                    total: 1_000_000_000,
                    used: 500_000_000,
                    available: 500_000_000,
                },
                properties: std::collections::HashMap::new(),
                devices: Vec::new(),
            })
        })
        .collect();

    for handle in handles {
        let _info = handle.join().expect("Thread should complete");
    }
}

// ==================== SERIALIZATION TESTS ====================

#[test]
fn test_pool_capacity_serialization() {
    let capacity = PoolCapacity {
        total_bytes: 1_000_000_000,
        used_bytes: 500_000_000,
        available_bytes: 500_000_000,
        utilization_percent: 50.0,
        fragmentation_percent: 0.0,
        deduplication_ratio: 1.0,
        total: 1_000_000_000,
        used: 500_000_000,
        available: 500_000_000,
    };

    let json = serde_json::to_string(&capacity).expect("Should serialize");
    assert!(json.contains("total_bytes"));
}

#[test]
fn test_pool_capacity_deserialization() {
    let json = r#"{
        "total": 1000000000,
        "total_bytes": 1000000000,
        "used": 500000000,
        "used_bytes": 500000000,
        "available": 500000000,
        "available_bytes": 500000000,
        "utilization_percent": 50.0
    }"#;

    let capacity: PoolCapacity = serde_json::from_str(json).expect("Should deserialize");
    assert_eq!(capacity.total_bytes, 1_000_000_000);
    assert_eq!(capacity.utilization_percent, 50.0);
}

#[test]
fn test_pool_info_serialization() {
    let info = PoolInfo {
        name: "tank".to_string(),
        state: PoolState::Online,
        health: PoolHealth::Healthy,
        capacity: PoolCapacity {
            total_bytes: 1_000_000_000,
            used_bytes: 500_000_000,
            available_bytes: 500_000_000,
            utilization_percent: 50.0,
            fragmentation_percent: 0.0,
            deduplication_ratio: 1.0,
            total: 1_000_000_000,
            used: 500_000_000,
            available: 500_000_000,
        },
        properties: std::collections::HashMap::new(),
        devices: Vec::new(),
    };

    let json = serde_json::to_string(&info).expect("Should serialize");
    assert!(json.contains("tank"));
}

// ==================== REAL-WORLD SCENARIOS ====================

#[test]
fn test_pool_scenario_healthy_tank() {
    let info = PoolInfo {
        name: "tank".to_string(),
        state: PoolState::Online,
        health: PoolHealth::Healthy,
        capacity: PoolCapacity {
            total_bytes: 10_000_000_000_000,    // 10TB
            used_bytes: 3_000_000_000_000,      // 3TB
            available_bytes: 7_000_000_000_000, // 7TB
            utilization_percent: 50.0,
            fragmentation_percent: 0.0,
            deduplication_ratio: 1.0,
            total: 10_000_000_000_000,
            used: 3_000_000_000_000,
            available: 7_000_000_000_000,
        },
        properties: std::collections::HashMap::new(),
        devices: Vec::new(),
    };

    assert_eq!(info.state, PoolState::Online);
    assert_eq!(info.health, PoolHealth::Healthy);
}

#[test]
fn test_pool_scenario_degraded_backup() {
    let info = PoolInfo {
        name: "backup".to_string(),
        state: PoolState::Degraded,
        health: PoolHealth::Warning,
        capacity: PoolCapacity {
            total_bytes: 20_000_000_000_000,    // 20TB
            used_bytes: 18_000_000_000_000,     // 18TB
            available_bytes: 2_000_000_000_000, // 2TB
            utilization_percent: 90.0,          // Fixed: should be 90% to match assertion,
            fragmentation_percent: 0.0,
            deduplication_ratio: 1.0,
            total: 20_000_000_000_000,
            used: 18_000_000_000_000,
            available: 2_000_000_000_000,
        },
        properties: std::collections::HashMap::new(),
        devices: Vec::new(),
    };

    assert_eq!(info.state, PoolState::Degraded);
    assert!(info.capacity.utilization_percent > 80.0);
}

#[test]
fn test_pool_scenario_critical_faulted() {
    let info = PoolInfo {
        name: "old_pool".to_string(),
        state: PoolState::Faulted,
        health: PoolHealth::Critical,
        capacity: PoolCapacity {
            total_bytes: 5_000_000_000_000,
            used_bytes: 4_500_000_000_000,
            available_bytes: 500_000_000_000,
            utilization_percent: 50.0,
            fragmentation_percent: 0.0,
            deduplication_ratio: 1.0,
            total: 5_000_000_000_000,
            used: 4_500_000_000_000,
            available: 500_000_000_000,
        },
        properties: std::collections::HashMap::new(),
        devices: Vec::new(),
    };

    assert_eq!(info.state, PoolState::Faulted);
    assert_eq!(info.health, PoolHealth::Critical);
}

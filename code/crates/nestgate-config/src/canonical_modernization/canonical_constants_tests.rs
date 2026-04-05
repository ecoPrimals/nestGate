// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Unit tests for canonical constant modules (values, relationships, `Duration` helpers).

// These tests intentionally assert relationships between `const` values (compile-time invariants).
#![expect(clippy::assertions_on_constants)]

use std::time::Duration;

#[test]
fn performance_buffer_sizes_align_with_hardcoding() {
    use crate::canonical_modernization::performance;
    use crate::constants::hardcoding::limits;

    assert_eq!(
        performance::DEFAULT_BUFFER_SIZE_BYTES,
        limits::BUFFER_SIZE_DEFAULT
    );
    assert_eq!(performance::MAX_BUFFER_SIZE_BYTES, limits::BUFFER_SIZE_MAX);
    assert_eq!(performance::MAX_RETRY_ATTEMPTS, limits::MAX_RETRIES);
    assert_eq!(
        performance::MAX_CONCURRENT_CONNECTIONS,
        limits::MAX_CONNECTIONS
    );
}

#[test]
fn storage_byte_scale_is_consistent() {
    use crate::canonical_modernization::storage;

    assert_eq!(storage::KB, 1024);
    assert_eq!(storage::MB, 1024 * storage::KB);
    assert_eq!(storage::GB, 1024 * storage::MB);
    assert_eq!(storage::TB, 1024 * storage::GB);
    assert_eq!(storage::DEFAULT_MAX_FILE_SIZE, 100 * storage::MB);
}

#[test]
fn storage_tier_and_compression_labels() {
    use crate::canonical_modernization::storage;

    assert_eq!(storage::TIER_HOT, "hot");
    assert_eq!(storage::TIER_WARM, "warm");
    assert_eq!(storage::TIER_COLD, "cold");
    assert_eq!(storage::COMPRESSION_LZ4, "lz4");
    assert_eq!(storage::COMPRESSION_ZSTD, "zstd");
}

#[test]
fn network_defaults_match_documented_ports() {
    use crate::canonical_modernization::network;

    assert_eq!(network::DEFAULT_API_PORT, 8080);
    assert_eq!(network::REQUEST_TIMEOUT_SECS, 30);
    assert_eq!(network::CONNECTION_TIMEOUT_SECS, 10);
}

#[test]
fn security_password_and_lockout_bounds() {
    use crate::canonical_modernization::security;

    assert_eq!(security::MIN_PASSWORD_LENGTH, 8);
    assert_eq!(security::MAX_LOGIN_ATTEMPTS, 3);
    assert_eq!(security::LOCKOUT_DURATION_SECS, 300);
}

#[test]
fn timeouts_duration_constants_match_secs() {
    use crate::canonical_modernization::timeouts;

    assert_eq!(
        timeouts::REQUEST_TIMEOUT,
        Duration::from_secs(timeouts::REQUEST_TIMEOUT_SECS)
    );
    assert_eq!(
        timeouts::CONNECTION_TIMEOUT,
        Duration::from_secs(timeouts::CONNECTION_TIMEOUT_SECS)
    );
    assert!(timeouts::MIN_TIMEOUT_SECS < timeouts::MAX_TIMEOUT_SECS);
}

#[test]
fn limits_recursion_bounds_are_positive() {
    use crate::canonical_modernization::limits;

    assert!(limits::MAX_RECURSION_DEPTH > 0);
    assert!(limits::MAX_FILE_DEPTH >= limits::ZFS_DISCOVERY_MAX_DEPTH);
}

#[test]
fn testing_constants_port_is_ephemeral_range() {
    use crate::canonical_modernization::testing;

    assert!(testing::TEST_API_PORT >= 1024);
}

#[test]
fn simd_widths_and_alignment() {
    use crate::canonical_modernization::simd;

    assert_eq!(simd::AVX2_WIDTH, 32);
    assert_eq!(simd::SSE2_WIDTH, 16);
    assert_eq!(simd::SIMD_ALIGNMENT, 32);
    assert!(simd::MIN_SIMD_SIZE >= simd::SIMD_BATCH_SIZE);
    assert_eq!(simd::CRC_TABLE_SIZE, 256);
}

#[test]
fn zero_cost_defaults_are_positive() {
    use crate::canonical_modernization::zero_cost;

    assert!(zero_cost::DEFAULT_MAX_CONCURRENT > 0);
    assert!(zero_cost::DEFAULT_BUFFER_SIZE > 0);
    assert!(zero_cost::DEFAULT_OPERATION_TIMEOUT_SECS > 0);
}

#[test]
fn zfs_operations_limits_ordering() {
    use crate::canonical_modernization::zfs_operations;

    assert!(zfs_operations::MAX_DATASETS >= zfs_operations::MAX_POOLS);
    assert!(zfs_operations::MAX_SNAPSHOTS > zfs_operations::MAX_DATASETS);
}

#[test]
fn service_limits_nonzero() {
    use crate::canonical_modernization::service_limits;

    assert!(service_limits::MAX_SERVICES > 0);
    assert!(service_limits::MAX_CONCURRENT_REQUESTS >= service_limits::MAX_SERVICES);
}

#[test]
fn development_defaults_use_localhost() {
    use crate::canonical_modernization::development;

    assert_eq!(development::DEFAULT_DEV_PORT, 8080);
    assert!(!development::DEFAULT_DEV_HOST.is_empty());
}

#[test]
fn const_access_macro_network_and_storage() {
    assert_eq!(
        crate::const_access!(network::DEFAULT_API_PORT),
        crate::canonical_modernization::network::DEFAULT_API_PORT
    );
    assert_eq!(
        crate::const_access!(storage::KB),
        crate::canonical_modernization::storage::KB
    );
}

#[test]
fn monitoring_alert_thresholds_are_percentages() {
    use crate::canonical_modernization::monitoring;

    assert!(monitoring::CPU_ALERT_THRESHOLD > 0.0 && monitoring::CPU_ALERT_THRESHOLD <= 100.0);
    assert!(monitoring::DISK_ALERT_THRESHOLD > monitoring::MEMORY_ALERT_THRESHOLD * 0.5);
}

#[test]
fn api_max_request_smaller_than_max_response() {
    use crate::canonical_modernization::api;

    assert!(api::MAX_REQUEST_SIZE < api::MAX_RESPONSE_SIZE);
}

#[test]
fn cache_ttl_relationships() {
    use crate::canonical_modernization::cache;

    assert!(cache::FILE_CACHE_TTL_SECONDS >= cache::DEFAULT_TTL_SECONDS);
}

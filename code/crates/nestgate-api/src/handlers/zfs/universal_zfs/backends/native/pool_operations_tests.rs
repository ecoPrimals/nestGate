// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Comprehensive tests for Native ZFS Pool Operations
//!
//! Tests cover pool parsing, state/health detection, and status parsing
//! to ensure reliable ZFS pool management in production.

use super::parse_pool_status;
use crate::handlers::zfs::universal_zfs_types::{PoolHealth, PoolState};

// ==================== POOL STATUS PARSING TESTS ====================

#[test]
fn test_parse_pool_status_online() {
    let output = r"  pool: tank
 state: ONLINE
config:
    NAME        STATE     READ WRITE CKSUM
    tank        ONLINE       0     0     0
";

    let result = parse_pool_status(output);
    assert!(result.is_ok());

    let pool_info = result.unwrap();
    assert_eq!(pool_info.name, "tank");
    assert_eq!(pool_info.state, PoolState::Active);
    assert_eq!(pool_info.health, PoolHealth::Online);
}

#[test]
fn test_parse_pool_status_degraded() {
    let output = r"  pool: mypool
 state: DEGRADED
config:
    NAME        STATE     READ WRITE CKSUM
    mypool      DEGRADED     0     0     0
";

    let result = parse_pool_status(output);
    assert!(result.is_ok());

    let pool_info = result.unwrap();
    assert_eq!(pool_info.name, "mypool");
    assert_eq!(pool_info.state, PoolState::Active);
    assert_eq!(pool_info.health, PoolHealth::Degraded);
}

#[test]
fn test_parse_pool_status_faulted() {
    let output = r"  pool: badpool
 state: FAULTED
config:
    NAME        STATE     READ WRITE CKSUM
    badpool     FAULTED      0     0     0
";

    let result = parse_pool_status(output);
    assert!(result.is_ok());

    let pool_info = result.unwrap();
    assert_eq!(pool_info.name, "badpool");
    assert_eq!(pool_info.state, PoolState::Active);
    assert_eq!(pool_info.health, PoolHealth::Faulted);
}

#[test]
fn test_parse_pool_status_offline() {
    let output = r"  pool: offlinepool
 state: OFFLINE
config:
    NAME          STATE     READ WRITE CKSUM
    offlinepool   OFFLINE      0     0     0
";

    let result = parse_pool_status(output);
    assert!(result.is_ok());

    let pool_info = result.unwrap();
    assert_eq!(pool_info.name, "offlinepool");
    assert_eq!(pool_info.state, PoolState::Suspended);
    assert_eq!(pool_info.health, PoolHealth::Offline);
}

#[test]
fn test_parse_pool_status_exported() {
    let output = r"  pool: exportedpool
 state: EXPORTED
config:
    NAME           STATE     READ WRITE CKSUM
    exportedpool   EXPORTED     0     0     0
";

    let result = parse_pool_status(output);
    assert!(result.is_ok());

    let pool_info = result.unwrap();
    assert_eq!(pool_info.name, "exportedpool");
    assert_eq!(pool_info.state, PoolState::Exported);
}

#[test]
fn test_parse_pool_status_empty_output() {
    let output = "";
    let result = parse_pool_status(output);
    assert!(result.is_err());
}

#[test]
fn test_parse_pool_status_minimal_valid() {
    let output = "  pool: minimal\n state: ONLINE\n";
    let result = parse_pool_status(output);
    assert!(result.is_ok());

    let pool_info = result.unwrap();
    assert_eq!(pool_info.name, "minimal");
}

#[test]
fn test_parse_pool_status_with_various_names() {
    let test_names = vec![
        "simple",
        "pool-with-dashes",
        "pool_with_underscores",
        "pool123",
        "UPPERCASE",
        "MixedCase",
    ];

    for name in test_names {
        let output = format!("  pool: {name}\n state: ONLINE\n");
        let result = parse_pool_status(&output);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().name, name);
    }
}

// ==================== POOL STATE MAPPING TESTS ====================

#[test]
fn test_pool_state_active_mappings() {
    let active_states = vec!["ONLINE", "DEGRADED", "FAULTED"];

    for state_str in active_states {
        let output = format!("  pool: test\n state: {state_str}\n");
        let result = parse_pool_status(&output);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().state, PoolState::Active);
    }
}

#[test]
fn test_pool_state_suspended_mapping() {
    let output = "  pool: test\n state: OFFLINE\n";
    let result = parse_pool_status(output);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().state, PoolState::Suspended);
}

#[test]
fn test_pool_state_exported_mapping() {
    let output = "  pool: test\n state: EXPORTED\n";
    let result = parse_pool_status(output);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().state, PoolState::Exported);
}

#[test]
fn test_pool_state_unknown_mapping() {
    let output = "  pool: test\n state: WEIRD_STATE\n";
    let result = parse_pool_status(output);
    assert!(result.is_ok());
    let pool_info = result.unwrap();
    assert_eq!(pool_info.state, PoolState::Unknown);
    assert_eq!(pool_info.health, PoolHealth::Unknown);
}

// ==================== POOL HEALTH MAPPING TESTS ====================

#[test]
fn test_pool_health_online_mapping() {
    let output = "  pool: test\n state: ONLINE\n";
    let result = parse_pool_status(output);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().health, PoolHealth::Online);
}

#[test]
fn test_pool_health_degraded_mapping() {
    let output = "  pool: test\n state: DEGRADED\n";
    let result = parse_pool_status(output);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().health, PoolHealth::Degraded);
}

#[test]
fn test_pool_health_faulted_mapping() {
    let output = "  pool: test\n state: FAULTED\n";
    let result = parse_pool_status(output);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().health, PoolHealth::Faulted);
}

#[test]
fn test_pool_health_offline_mapping() {
    let output = "  pool: test\n state: OFFLINE\n";
    let result = parse_pool_status(output);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().health, PoolHealth::Offline);
}

// ==================== COMPLEX POOL STATUS SCENARIOS ====================

#[test]
fn test_parse_pool_status_with_detailed_config() {
    let output = r"  pool: production
 state: ONLINE
  scan: scrub repaired 0B in 02:34:56 with 0 errors on Sun Nov 10 03:34:56 2025
config:

    NAME                        STATE     READ WRITE CKSUM
    production                  ONLINE       0     0     0
      raidz2-0                  ONLINE       0     0     0
        sda                     ONLINE       0     0     0
        sdb                     ONLINE       0     0     0
        sdc                     ONLINE       0     0     0
        sdd                     ONLINE       0     0     0

errors: No known data errors
";

    let result = parse_pool_status(output);
    assert!(result.is_ok());

    let pool_info = result.unwrap();
    assert_eq!(pool_info.name, "production");
    assert_eq!(pool_info.state, PoolState::Active);
    assert_eq!(pool_info.health, PoolHealth::Online);
}

#[test]
fn test_parse_pool_status_multiline_whitespace() {
    let output = "  pool: test\n\n state: ONLINE\n\nconfig:\n    test    ONLINE\n";
    let result = parse_pool_status(output);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().name, "test");
}

#[test]
fn test_parse_pool_status_case_sensitivity() {
    // State values should be case-sensitive
    let output = "  pool: test\n state: online\n";
    let result = parse_pool_status(output);
    assert!(result.is_ok());
    // Lowercase "online" won't match, so it should be Unknown
    let pool_info = result.unwrap();
    assert_eq!(pool_info.state, PoolState::Unknown);
    assert_eq!(pool_info.health, PoolHealth::Unknown);
}

// ==================== ERROR HANDLING TESTS ====================

#[test]
fn test_parse_pool_status_malformed_pool_line() {
    let output = "pool: missing_spaces\n state: ONLINE\n";
    let result = parse_pool_status(output);
    // Should still succeed but pool name will be empty
    assert!(result.is_ok());
}

#[test]
fn test_parse_pool_status_missing_state_line() {
    let output = "  pool: testpool\n";
    let result = parse_pool_status(output);
    assert!(result.is_ok());
    // State and health should be Unknown
    let pool_info = result.unwrap();
    assert_eq!(pool_info.state, PoolState::Unknown);
    assert_eq!(pool_info.health, PoolHealth::Unknown);
}

#[test]
fn test_parse_pool_status_extra_whitespace() {
    let output = "  pool:    spacedpool   \n state:   ONLINE   \n";
    let result = parse_pool_status(output);
    assert!(result.is_ok());
}

// ==================== REAL-WORLD ZFS OUTPUT TESTS ====================

#[test]
fn test_typical_single_disk_pool() {
    let output = r"  pool: rpool
 state: ONLINE
  scan: none requested
config:

    NAME        STATE     READ WRITE CKSUM
    rpool       ONLINE       0     0     0
      sda1      ONLINE       0     0     0

errors: No known data errors
";

    let result = parse_pool_status(output);
    assert!(result.is_ok());
    let pool_info = result.unwrap();
    assert_eq!(pool_info.name, "rpool");
    assert_eq!(pool_info.health, PoolHealth::Online);
}

#[test]
fn test_typical_mirror_pool() {
    let output = r"  pool: storage
 state: ONLINE
config:

    NAME          STATE     READ WRITE CKSUM
    storage       ONLINE       0     0     0
      mirror-0    ONLINE       0     0     0
        sdb       ONLINE       0     0     0
        sdc       ONLINE       0     0     0

errors: No known data errors
";

    let result = parse_pool_status(output);
    assert!(result.is_ok());
    let pool_info = result.unwrap();
    assert_eq!(pool_info.name, "storage");
    assert_eq!(pool_info.state, PoolState::Active);
}

#[test]
fn test_typical_raidz_pool() {
    let output = r"  pool: backup
 state: ONLINE
config:

    NAME          STATE     READ WRITE CKSUM
    backup        ONLINE       0     0     0
      raidz1-0    ONLINE       0     0     0
        sdd       ONLINE       0     0     0
        sde       ONLINE       0     0     0
        sdf       ONLINE       0     0     0

errors: No known data errors
";

    let result = parse_pool_status(output);
    assert!(result.is_ok());
    let pool_info = result.unwrap();
    assert_eq!(pool_info.name, "backup");
}

// ==================== CONCURRENT SAFETY TESTS ====================

#[test]
fn test_parse_pool_status_is_pure() {
    let output = "  pool: test\n state: ONLINE\n";

    let result1 = parse_pool_status(output);
    let result2 = parse_pool_status(output);

    assert_eq!(result1.is_ok(), result2.is_ok());

    let pool1 = result1.unwrap();
    let pool2 = result2.unwrap();

    assert_eq!(pool1.name, pool2.name);
    assert_eq!(pool1.state, pool2.state);
    assert_eq!(pool1.health, pool2.health);
}

#[test]
fn test_multiple_pool_parsing() {
    let pools = vec![
        ("pool1", "ONLINE"),
        ("pool2", "DEGRADED"),
        ("pool3", "OFFLINE"),
    ];

    for (name, state) in pools {
        let output = format!("  pool: {name}\n state: {state}\n");
        let result = parse_pool_status(&output);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().name, name);
    }
}

// ==================== EDGE CASES AND BOUNDARY CONDITIONS ====================

#[test]
fn test_very_long_pool_name() {
    let long_name = "a".repeat(100);
    let output = format!("  pool: {long_name}\n state: ONLINE\n");
    let result = parse_pool_status(&output);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().name, long_name);
}

#[test]
fn test_pool_name_with_numbers() {
    let output = "  pool: pool2024_backup_v2\n state: ONLINE\n";
    let result = parse_pool_status(output);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().name, "pool2024_backup_v2");
}

#[test]
fn test_unicode_in_output() {
    // ZFS shouldn't have unicode in pool names, but test robustness
    let output = "  pool: normal\n state: ONLINE\n errors: ✓ No errors\n";
    let result = parse_pool_status(output);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().name, "normal");
}

#[test]
fn test_windows_line_endings() {
    let output = "  pool: test\r\n state: ONLINE\r\n";
    let result = parse_pool_status(output);
    assert!(result.is_ok());
}

// ==================== SCRUB STATUS TESTS ====================

#[test]
fn test_pool_with_scrub_info() {
    let output = r"  pool: tank
 state: ONLINE
  scan: scrub repaired 0B in 01:23:45 with 0 errors on Wed Nov 13 12:00:00 2025
";

    let result = parse_pool_status(output);
    assert!(result.is_ok());
    let pool_info = result.unwrap();
    assert!(pool_info.scrub.is_some());
}

#[test]
fn test_pool_without_scrub_info() {
    let output = "  pool: tank\n state: ONLINE\n";
    let result = parse_pool_status(output);
    assert!(result.is_ok());
    let pool_info = result.unwrap();
    assert!(pool_info.scrub.is_some()); // Default is Some(ScrubStatus::None)
}

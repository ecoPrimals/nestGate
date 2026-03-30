// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Unit tests for iostat parsing and tier queue depth helpers.

use crate::performance::types::ZfsPerformanceMonitor;
use crate::types::StorageTier;

#[test]
fn parse_iostat_bandwidth_units_and_edge_cases() {
    assert_eq!(
        ZfsPerformanceMonitor::test_parse_iostat_bandwidth("").unwrap(),
        0
    );
    assert_eq!(
        ZfsPerformanceMonitor::test_parse_iostat_bandwidth("-").unwrap(),
        0
    );
    assert_eq!(
        ZfsPerformanceMonitor::test_parse_iostat_bandwidth("  42  ").unwrap(),
        42
    );
    assert_eq!(
        ZfsPerformanceMonitor::test_parse_iostat_bandwidth("2K").unwrap(),
        2048
    );
    assert_eq!(
        ZfsPerformanceMonitor::test_parse_iostat_bandwidth("1.5M").unwrap(),
        (1.5 * (1024.0 * 1024.0)) as u64
    );
    assert_eq!(
        ZfsPerformanceMonitor::test_parse_iostat_bandwidth("3G").unwrap(),
        3 * 1024 * 1024 * 1024
    );
}

#[test]
fn get_real_queue_depth_all_tiers() {
    assert_eq!(
        ZfsPerformanceMonitor::test_get_real_queue_depth(&StorageTier::Hot).unwrap(),
        32.0
    );
    assert_eq!(
        ZfsPerformanceMonitor::test_get_real_queue_depth(&StorageTier::Warm).unwrap(),
        16.0
    );
    assert_eq!(
        ZfsPerformanceMonitor::test_get_real_queue_depth(&StorageTier::Cold).unwrap(),
        8.0
    );
    assert_eq!(
        ZfsPerformanceMonitor::test_get_real_queue_depth(&StorageTier::Cache).unwrap(),
        64.0
    );
    assert_eq!(
        ZfsPerformanceMonitor::test_get_real_queue_depth(&StorageTier::Archive).unwrap(),
        4.0
    );
}

#[test]
fn parse_zpool_iostat_skips_short_lines() {
    let out = "only three fields here\n";
    let s = ZfsPerformanceMonitor::test_parse_zpool_iostat(out).expect("parse");
    assert_eq!(s.read_ops, 0);
    assert_eq!(s.write_ops, 0);
}

#[test]
fn parse_zpool_iostat_sums_multiple_data_rows() {
    let out = "a 0 0 1 2 3 4\nb 0 0 10 20 5 6\n";
    let s = ZfsPerformanceMonitor::test_parse_zpool_iostat(out).expect("parse");
    assert_eq!(s.read_ops, 11);
    assert_eq!(s.write_ops, 22);
}

#[test]
fn parse_zpool_get_pool_properties_tab_lines() {
    let out = "tank\tfragmentation\t12%\t-\n\
               tank\tcompressratio\t1.50x\t-\n\
               tank\tdedupratio\t1.00x\t-\n";
    let p = ZfsPerformanceMonitor::test_parse_zpool_get_pool_properties(out);
    assert!((p.fragmentation - 12.0).abs() < f64::EPSILON);
    assert!((p.compression_ratio - 1.5).abs() < f64::EPSILON);
    assert!((p.dedup_ratio - 1.0).abs() < f64::EPSILON);
}

#[test]
fn parse_zpool_get_pool_properties_malformed_skipped() {
    let out = "not enough fields\n";
    let p = ZfsPerformanceMonitor::test_parse_zpool_get_pool_properties(out);
    assert_eq!(p.fragmentation, 0.0);
    assert_eq!(p.compression_ratio, 1.0);
    assert_eq!(p.dedup_ratio, 1.0);
}

#[test]
fn parse_zpool_iostat_skips_pool_header_and_dash_lines() {
    // Lines containing the substring `pool` are ignored (filters header rows).
    let out = "-\nzpool alloc free read write read write\n\
                tank 0 0 2 3 1024 2048\n";
    let s = ZfsPerformanceMonitor::test_parse_zpool_iostat(out).expect("parse");
    assert_eq!(s.read_ops, 2);
    assert_eq!(s.write_ops, 3);
}

#[test]
fn parse_iostat_bandwidth_invalid_number_errors() {
    assert!(ZfsPerformanceMonitor::test_parse_iostat_bandwidth("not_a_number").is_err());
}

#[test]
fn parse_zpool_get_pool_properties_unknown_property_name_is_ignored() {
    let out = "tank\tnot_a_known_prop\t1.0\t-\n";
    let p = ZfsPerformanceMonitor::test_parse_zpool_get_pool_properties(out);
    assert_eq!(p.fragmentation, 0.0);
    assert_eq!(p.compression_ratio, 1.0);
    assert_eq!(p.dedup_ratio, 1.0);
}

#[test]
fn parse_zpool_get_pool_properties_invalid_fragmentation_percent_skipped() {
    let out = "tank\tfragmentation\tnot_a_number%\t-\n";
    let p = ZfsPerformanceMonitor::test_parse_zpool_get_pool_properties(out);
    assert_eq!(p.fragmentation, 0.0);
}

#[test]
fn parse_zpool_get_pool_properties_invalid_compressratio_skipped() {
    let out = "tank\tcompressratio\tbadx\t-\n";
    let p = ZfsPerformanceMonitor::test_parse_zpool_get_pool_properties(out);
    assert_eq!(p.compression_ratio, 1.0);
}

#[test]
fn parse_zpool_get_pool_properties_invalid_dedupratio_skipped() {
    let out = "tank\tdedupratio\tbadx\t-\n";
    let p = ZfsPerformanceMonitor::test_parse_zpool_get_pool_properties(out);
    assert_eq!(p.dedup_ratio, 1.0);
}

#[test]
fn parse_zpool_iostat_line_with_seven_fields_but_non_numeric_ops_skipped() {
    let out = "tank 0 0 abc def 1 2\n";
    let s = ZfsPerformanceMonitor::test_parse_zpool_iostat(out).expect("parse");
    assert_eq!(s.read_ops, 0);
    assert_eq!(s.write_ops, 0);
}

#[test]
fn parse_zpool_iostat_skips_lines_containing_pool_substring() {
    let out = "mypoolname 0 0 1 2 3 4\n";
    let s = ZfsPerformanceMonitor::test_parse_zpool_iostat(out).expect("parse");
    assert_eq!(s.read_ops, 0);
}

#[test]
fn parse_zpool_iostat_line_starting_with_dash_skipped() {
    let out = "- 0 0 9 9 1 1\ntank 0 0 2 2 1 1\n";
    let s = ZfsPerformanceMonitor::test_parse_zpool_iostat(out).expect("parse");
    assert_eq!(s.read_ops, 2);
}

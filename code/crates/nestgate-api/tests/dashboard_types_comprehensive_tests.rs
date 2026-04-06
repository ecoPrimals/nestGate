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

//! Comprehensive tests for dashboard types

use nestgate_api::handlers::dashboard_types::*;
use std::time::{Duration, SystemTime};

// =====================================================
// DASHBOARD TIME RANGE TESTS
// =====================================================

#[test]
fn test_dashboard_time_range_creation() {
    let start = SystemTime::now();
    let end = start + Duration::from_secs(3600); // 1 hour later
    let granularity = Duration::from_secs(60); // 1 minute

    let range = DashboardTimeRange::new(start, end, granularity);

    assert_eq!(range.start, start);
    assert_eq!(range.end, end);
    assert_eq!(range.granularity, granularity);
}

#[test]
fn test_dashboard_time_range_last_hours() {
    let range = DashboardTimeRange::last_hours(24);

    assert!(range.start < range.end);
    assert_eq!(range.granularity, Duration::from_secs(300)); // 5 minutes

    // Duration should be approximately 24 hours
    let duration = range.duration();
    assert!((duration.as_secs() as i64 - 24 * 3600).abs() < 10); // Within 10 seconds
}

#[test]
fn test_dashboard_time_range_last_days() {
    let range = DashboardTimeRange::last_days(7);

    assert!(range.start < range.end);
    assert_eq!(range.granularity, Duration::from_secs(3600)); // 1 hour

    // Duration should be approximately 7 days
    let duration = range.duration();
    assert!((duration.as_secs() as i64 - 7 * 24 * 3600).abs() < 10); // Within 10 seconds
}

#[test]
fn test_dashboard_time_range_duration() {
    let start = SystemTime::now();
    let end = start + Duration::from_secs(7200); // 2 hours
    let granularity = Duration::from_secs(60);

    let range = DashboardTimeRange::new(start, end, granularity);
    let duration = range.duration();

    assert_eq!(duration.as_secs(), 7200);
}

#[test]
fn test_dashboard_time_range_is_valid() {
    let start = SystemTime::now();
    let end = start + Duration::from_secs(3600);
    let granularity = Duration::from_secs(60);

    let valid_range = DashboardTimeRange::new(start, end, granularity);
    assert!(valid_range.is_valid());
}

#[test]
fn test_dashboard_time_range_invalid_when_start_after_end() {
    let end = SystemTime::now();
    let start = end + Duration::from_secs(3600); // Start after end!
    let granularity = Duration::from_secs(60);

    let invalid_range = DashboardTimeRange::new(start, end, granularity);
    assert!(!invalid_range.is_valid());
}

#[test]
fn test_dashboard_time_range_invalid_when_zero_granularity() {
    let start = SystemTime::now();
    let end = start + Duration::from_secs(3600);
    let granularity = Duration::ZERO; // Invalid!

    let invalid_range = DashboardTimeRange::new(start, end, granularity);
    assert!(!invalid_range.is_valid());
}

#[test]
fn test_dashboard_time_range_data_points() {
    let start = SystemTime::now();
    let end = start + Duration::from_secs(3600); // 1 hour
    let granularity = Duration::from_secs(60); // 1 minute

    let range = DashboardTimeRange::new(start, end, granularity);
    let points = range.data_points();

    assert_eq!(points, 60); // 60 minutes in an hour
}

#[test]
fn test_dashboard_time_range_data_points_zero_when_invalid() {
    let end = SystemTime::now();
    let start = end + Duration::from_secs(3600); // Invalid: start > end
    let granularity = Duration::from_secs(60);

    let range = DashboardTimeRange::new(start, end, granularity);
    let points = range.data_points();

    assert_eq!(points, 0); // Should be 0 for invalid range
}

#[test]
fn test_dashboard_time_range_intervals() {
    let start = SystemTime::now();
    let end = start + Duration::from_secs(300); // 5 minutes
    let granularity = Duration::from_secs(60); // 1 minute

    let range = DashboardTimeRange::new(start, end, granularity);
    let intervals = range.intervals();

    assert_eq!(intervals.len(), 5); // Should have 5 intervals

    // First interval should start at range start
    assert_eq!(intervals[0].0, start);

    // Last interval should end at range end
    assert_eq!(intervals[intervals.len() - 1].1, end);
}

#[test]
fn test_dashboard_time_range_intervals_with_partial_final_interval() {
    let start = SystemTime::now();
    let end = start + Duration::from_secs(250); // 4 minutes 10 seconds
    let granularity = Duration::from_secs(60); // 1 minute

    let range = DashboardTimeRange::new(start, end, granularity);
    let intervals = range.intervals();

    assert_eq!(intervals.len(), 5); // Should have 5 intervals (partial last one)

    // Last interval should end at range end (not at full granularity)
    assert_eq!(intervals[intervals.len() - 1].1, end);
}

#[test]
fn test_dashboard_time_range_clone() {
    let start = SystemTime::now();
    let end = start + Duration::from_secs(3600);
    let granularity = Duration::from_secs(60);

    let range = DashboardTimeRange::new(start, end, granularity);
    let cloned = range.clone();

    assert_eq!(cloned.start, range.start);
    assert_eq!(cloned.end, range.end);
    assert_eq!(cloned.granularity, range.granularity);
}

#[test]
fn test_dashboard_time_range_serialization() {
    let start = SystemTime::now();
    let end = start + Duration::from_secs(3600);
    let granularity = Duration::from_secs(60);

    let range = DashboardTimeRange::new(start, end, granularity);

    let serialized = serde_json::to_string(&range);
    assert!(serialized.is_ok(), "DashboardTimeRange should serialize");
}

#[test]
fn test_dashboard_time_range_deserialization() {
    let start = SystemTime::now();
    let end = start + Duration::from_secs(3600);
    let granularity = Duration::from_secs(60);

    let range = DashboardTimeRange::new(start, end, granularity);
    let json = serde_json::to_string(&range).expect("Failed to serialize");

    let deserialized: std::result::Result<DashboardTimeRange, _> = serde_json::from_str(&json);
    assert!(
        deserialized.is_ok(),
        "DashboardTimeRange should deserialize"
    );
}

// =====================================================
// EDGE CASE TESTS
// =====================================================

#[test]
fn test_dashboard_time_range_very_short_duration() {
    let start = SystemTime::now();
    let end = start + Duration::from_secs(1); // 1 second
    let granularity = Duration::from_millis(100); // 100ms

    let range = DashboardTimeRange::new(start, end, granularity);

    assert!(range.is_valid());
    assert_eq!(range.duration().as_secs(), 1);
}

#[test]
fn test_dashboard_time_range_very_long_duration() {
    let start = SystemTime::now();
    let end = start + Duration::from_secs(365 * 24 * 3600); // 1 year
    let granularity = Duration::from_secs(24 * 3600); // 1 day

    let range = DashboardTimeRange::new(start, end, granularity);

    assert!(range.is_valid());
    let points = range.data_points();
    assert_eq!(points, 365); // 365 days in a year
}

#[test]
fn test_dashboard_time_range_last_hours_zero() {
    let range = DashboardTimeRange::last_hours(0);

    // Should handle zero gracefully
    assert!(range.start <= range.end);
}

#[test]
fn test_dashboard_time_range_last_days_zero() {
    let range = DashboardTimeRange::last_days(0);

    // Should handle zero gracefully
    assert!(range.start <= range.end);
}

#[test]
fn test_dashboard_time_range_large_granularity() {
    let start = SystemTime::now();
    let end = start + Duration::from_secs(300); // 5 minutes
    let granularity = Duration::from_secs(3600); // 1 hour (larger than range!)

    let range = DashboardTimeRange::new(start, end, granularity);

    assert!(range.is_valid());
    let points = range.data_points();
    assert_eq!(points, 1); // Should have at least 1 data point
}

#[test]
fn test_dashboard_time_range_intervals_empty_when_invalid() {
    let end = SystemTime::now();
    let start = end + Duration::from_secs(3600); // Invalid
    let granularity = Duration::from_secs(60);

    let range = DashboardTimeRange::new(start, end, granularity);
    let intervals = range.intervals();

    assert!(
        intervals.is_empty(),
        "Invalid range should have no intervals"
    );
}

// =====================================================
// PERFORMANCE TESTS
// =====================================================

#[test]
fn test_dashboard_time_range_intervals_performance() {
    let start = SystemTime::now();
    let end = start + Duration::from_secs(86400); // 24 hours
    let granularity = Duration::from_secs(1); // 1 second

    let range = DashboardTimeRange::new(start, end, granularity);

    let start_time = std::time::Instant::now();
    let intervals = range.intervals();
    let elapsed = start_time.elapsed();

    // Should compute 86,400 intervals quickly (< 100ms)
    assert!(
        elapsed.as_millis() < 100,
        "Interval computation took too long: {elapsed:?}"
    );
    assert_eq!(intervals.len(), 86400);
}

#[test]
fn test_dashboard_time_range_data_points_performance() {
    let start = SystemTime::now();
    let end = start + Duration::from_secs(31_536_000); // 1 year
    let granularity = Duration::from_secs(1); // 1 second

    let range = DashboardTimeRange::new(start, end, granularity);

    let start_time = std::time::Instant::now();
    let points = range.data_points();
    let elapsed = start_time.elapsed();

    // Should compute quickly even for large ranges
    assert!(
        elapsed.as_micros() < 1000,
        "Data points computation took too long: {elapsed:?}"
    );
    assert_eq!(points, 31_536_000);
}

// =====================================================
// REAL-WORLD SCENARIO TESTS
// =====================================================

#[test]
fn test_dashboard_time_range_real_time_monitoring() {
    // Scenario: Real-time monitoring of last 15 minutes with 5-second granularity
    let _range = DashboardTimeRange::last_hours(0); // Effectively 0 hours
    let end = SystemTime::now();
    let start = end - Duration::from_secs(15 * 60); // 15 minutes ago
    let granularity = Duration::from_secs(5); // 5 seconds

    let real_time_range = DashboardTimeRange::new(start, end, granularity);

    assert!(real_time_range.is_valid());
    assert_eq!(real_time_range.data_points(), 180); // 15 * 60 / 5 = 180 points
}

#[test]
fn test_dashboard_time_range_hourly_report() {
    // Scenario: Hourly report with minute-level granularity
    let range = DashboardTimeRange::last_hours(1);

    assert!(range.is_valid());
    assert_eq!(range.data_points(), 12); // 1 hour / 5 minutes = 12 points
}

#[test]
fn test_dashboard_time_range_daily_summary() {
    // Scenario: Daily summary with hour-level granularity
    let range = DashboardTimeRange::last_days(1);

    assert!(range.is_valid());
    assert_eq!(range.data_points(), 24); // 24 hours in a day
}

#[test]
fn test_dashboard_time_range_weekly_analysis() {
    // Scenario: Weekly analysis with hour-level granularity
    let range = DashboardTimeRange::last_days(7);

    assert!(range.is_valid());
    assert_eq!(range.data_points(), 168); // 7 * 24 = 168 hours
}

#[test]
fn test_dashboard_time_range_monthly_trends() {
    // Scenario: Monthly trends with daily granularity
    let end = SystemTime::now();
    let start = end - Duration::from_secs(30 * 24 * 3600); // 30 days
    let granularity = Duration::from_secs(24 * 3600); // 1 day

    let monthly_range = DashboardTimeRange::new(start, end, granularity);

    assert!(monthly_range.is_valid());
    assert_eq!(monthly_range.data_points(), 30); // 30 days
}

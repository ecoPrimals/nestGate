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

//! Round 5: `CommandResult` parsing branches and dry-run command paths.

use nestgate_zfs::command::{CommandResult, ZfsCommand};

#[test]
fn command_result_stdout_stderr_empty_lines() {
    let r = CommandResult {
        success: true,
        stdout: String::new(),
        stderr: String::new(),
        exit_code: 0,
    };
    assert!(r.stdout_lines().is_empty());
    assert!(r.stderr_lines().is_empty());
}

#[test]
fn command_result_parse_properties_skips_comments_and_empty() {
    let r = CommandResult {
        success: true,
        stdout: "\n# c\n\t\nkey1\tval1\nkey2 val2\n".into(),
        stderr: String::new(),
        exit_code: 0,
    };
    let m = r.parse_properties().expect("parse");
    assert_eq!(m.get("key1"), Some(&"val1".into()));
    assert_eq!(m.get("key2"), Some(&"val2".into()));
}

#[test]
fn command_result_parse_properties_no_separator_line_ignored() {
    let r = CommandResult {
        success: true,
        stdout: "noseparatorline\n".into(),
        stderr: String::new(),
        exit_code: 0,
    };
    let m = r.parse_properties().expect("parse");
    assert!(m.is_empty());
}

#[test]
fn command_result_parse_table_empty_stdout() {
    let r = CommandResult {
        success: false,
        stdout: String::new(),
        stderr: "err".into(),
        exit_code: 1,
    };
    let t = r.parse_table().expect("table");
    assert!(t.is_empty());
}

#[test]
fn command_result_parse_table_headers_only() {
    let r = CommandResult {
        success: true,
        stdout: "NAME SIZE\n".into(),
        stderr: String::new(),
        exit_code: 0,
    };
    let t = r.parse_table().expect("table");
    assert!(t.is_empty());
}

#[test]
fn command_result_parse_table_malformed_row_skipped() {
    let r = CommandResult {
        success: true,
        stdout: "A B\nshort\nx y\n".into(),
        stderr: String::new(),
        exit_code: 0,
    };
    let t = r.parse_table().expect("table");
    assert_eq!(t.len(), 1);
    assert_eq!(t[0].get("A"), Some(&"x".into()));
    assert_eq!(t[0].get("B"), Some(&"y".into()));
}

#[test]
fn command_result_parse_table_valid_rows() {
    let r = CommandResult {
        success: true,
        stdout: "A B\nx y\na b\n".into(),
        stderr: String::new(),
        exit_code: 0,
    };
    let t = r.parse_table().expect("table");
    assert_eq!(t.len(), 2);
}

#[test]
fn command_result_is_success_and_exit_code() {
    let r = CommandResult {
        success: false,
        stdout: String::new(),
        stderr: "e".into(),
        exit_code: -1,
    };
    assert!(!r.is_success());
    assert_eq!(r.exit_code, -1);
}

#[tokio::test]
async fn zfs_command_dry_run_short_circuits() {
    let cmd = ZfsCommand::new().with_dry_run(true);
    let out = cmd.zpool(&["list"]).await.expect("dry");
    assert!(out.success);
    assert!(out.stdout.contains("DRY RUN"));
    assert!(out.stderr.is_empty());
}

#[tokio::test]
async fn zfs_command_dry_run_zfs() {
    let cmd = ZfsCommand::new().with_dry_run(true);
    let out = cmd.zfs(&["list"]).await.expect("dry zfs");
    assert!(out.success);
    assert!(out.stdout.contains("zfs"));
}

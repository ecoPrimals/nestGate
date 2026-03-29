// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

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

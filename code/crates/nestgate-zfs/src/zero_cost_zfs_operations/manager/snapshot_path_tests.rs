// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Stub-based tests for [`super::snapshot_ops`] snapshot create / list flows.

use std::collections::HashMap;

use super::TestingZfsManager;
use super::test_zfs_stub::ZfsCommandStubGuard;
use crate::error::{ZfsOperation, create_zfs_error};
use crate::zero_cost_zfs_operations::types::ZeroCostDatasetInfo;
use nestgate_core::Result;
use nestgate_core::canonical_types::StorageTier;
use serial_test::serial;
use std::fmt::Write;
use std::time::SystemTime;

fn test_dataset() -> ZeroCostDatasetInfo {
    ZeroCostDatasetInfo {
        name: "ds".into(),
        pool: "tank".into(),
        tier: StorageTier::Warm,
        size: 0,
        used: 0,
        properties: HashMap::default(),
        mount_point: None,
        created_at: SystemTime::UNIX_EPOCH,
    }
}

fn stub_snapshot_and_get() -> impl Fn(&[&str]) -> Result<String> + Send + Sync + 'static {
    |args: &[&str]| -> Result<String> {
        match args.first().copied() {
            Some("snapshot") => Ok(String::new()),
            Some("get") => Ok(String::from("used\t2048\n")),
            _ => Err(create_zfs_error(
                "Unexpected command in stub",
                ZfsOperation::Command,
            )),
        }
    }
}

#[tokio::test]
#[serial]
async fn snapshot_create_executes_snapshot_and_get() {
    let _g = ZfsCommandStubGuard::set(Box::new(stub_snapshot_and_get()));
    let m = TestingZfsManager::new();
    let ds = test_dataset();
    let snap = m.snapshot_create(&ds, "s1").await.expect("snapshot create");
    assert_eq!(snap.name, "s1");
    assert_eq!(snap.dataset, "tank/ds");
    assert_eq!(snap.size, 2048);
}

#[tokio::test]
#[serial]
async fn snapshot_create_errors_when_snapshot_command_fails() {
    let _g = ZfsCommandStubGuard::set(Box::new(|args: &[&str]| -> Result<String> {
        if args.first() == Some(&"snapshot") {
            return Err(create_zfs_error(
                "ZFS command failed: busy",
                ZfsOperation::Command,
            ));
        }
        Ok(String::new())
    }));
    let m = TestingZfsManager::new();
    let err = m
        .snapshot_create(&test_dataset(), "bad")
        .await
        .expect_err("expected error");
    assert!(err.to_string().contains("ZFS command failed") || err.to_string().contains("busy"));
}

#[tokio::test]
#[serial]
async fn snapshot_create_errors_when_at_capacity() {
    let _g = ZfsCommandStubGuard::set(Box::new(|_args: &[&str]| -> Result<String> {
        Ok(String::new())
    }));
    let m = TestingZfsManager::new();
    for i in 0..100 {
        m.test_insert_snapshot_entry(format!("snap{i}")).await;
    }
    let err = m
        .snapshot_create(&test_dataset(), "overflow")
        .await
        .expect_err("capacity");
    assert!(err.to_string().contains("maximum snapshots"));
    assert!(format!("{err:?}").to_lowercase().contains("systemcheck"));
}

#[tokio::test]
#[serial]
async fn snapshot_list_returns_parsed_rows() {
    let _g = ZfsCommandStubGuard::set(Box::new(|args: &[&str]| -> Result<String> {
        if args.first() == Some(&"list") {
            return Ok(String::from("tank/ds@a\t100\ntank/ds@b\t200\n"));
        }
        Ok(String::new())
    }));
    let m = TestingZfsManager::new();
    let rows = m.snapshot_list(&test_dataset()).await.expect("list");
    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0].name, "a");
    assert_eq!(rows[1].name, "b");
}

#[tokio::test]
#[serial]
async fn snapshot_list_stops_at_max_snapshots() {
    let _g = ZfsCommandStubGuard::set(Box::new(|args: &[&str]| -> Result<String> {
        if args.first() == Some(&"list") {
            let mut out = String::new();
            for i in 0..150 {
                let _ = writeln!(out, "tank/ds@s{i}\t{i}");
            }
            return Ok(out);
        }
        Ok(String::new())
    }));
    let m = TestingZfsManager::new();
    let rows = m.snapshot_list(&test_dataset()).await.expect("list");
    assert_eq!(rows.len(), 100);
}

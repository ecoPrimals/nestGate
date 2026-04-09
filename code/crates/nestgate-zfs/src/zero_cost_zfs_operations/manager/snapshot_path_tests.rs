// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

#![allow(unsafe_code)] // `PATH` swap for mock `zfs` (Rust 2024: `set_var` is `unsafe`)

//! Path-mocked tests for [`super::snapshot_ops`] snapshot create / list flows.

use std::fmt::Write;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

use crate::path_cli_test_lock;

use super::TestingZfsManager;
use crate::zero_cost_zfs_operations::types::ZeroCostDatasetInfo;
use nestgate_core::canonical_types::StorageTier;

struct PathEnvGuard {
    previous: Option<std::ffi::OsString>,
}

impl PathEnvGuard {
    fn prepend_bin_dir(bin_dir: &Path) -> Self {
        let previous = std::env::var_os("PATH");
        let mut new_path = bin_dir.as_os_str().to_owned();
        new_path.push(":");
        if let Some(ref p) = previous {
            new_path.push(p);
        }
        // SAFETY: test-only `PATH` prepend; guarded tests use `#[serial]`.
        unsafe {
            std::env::set_var("PATH", new_path);
        }
        Self { previous }
    }
}

impl Drop for PathEnvGuard {
    fn drop(&mut self) {
        // SAFETY: restores `PATH` saved in `prepend_bin_dir`.
        unsafe {
            match &self.previous {
                Some(p) => std::env::set_var("PATH", p),
                None => std::env::remove_var("PATH"),
            }
        }
    }
}

fn write_executable(dir: &Path, name: &str, body: &str) {
    let p = dir.join(name);
    fs::write(&p, body).expect("write mock");
    let mut perm = fs::metadata(&p).expect("meta").permissions();
    perm.set_mode(0o755);
    fs::set_permissions(&p, perm).expect("chmod");
}

fn test_dataset() -> ZeroCostDatasetInfo {
    ZeroCostDatasetInfo {
        name: "ds".into(),
        pool: "tank".into(),
        tier: StorageTier::Warm,
        size: 0,
        used: 0,
        properties: Default::default(),
        mount_point: None,
        created_at: std::time::SystemTime::UNIX_EPOCH,
    }
}

#[tokio::test]
async fn snapshot_create_executes_snapshot_and_get() {
    let _lock = path_cli_test_lock::acquire().await;
    let tmp = tempfile::tempdir().expect("tempdir");
    write_executable(
        tmp.path(),
        "zfs",
        concat!(
            "#!/bin/sh\nset -eu\nif [ \"",
            "${",
            "1:-",
            "}",
            "\" = \"snapshot\" ]; then exit 0; fi\nif [ \"",
            "${",
            "1:-",
            "}",
            "\" = \"get\" ]; then\n  echo \"used\t2048\"\n  exit 0\nfi\nexit 1\n"
        ),
    );
    let _g = PathEnvGuard::prepend_bin_dir(tmp.path());

    let m = TestingZfsManager::new();
    let ds = test_dataset();
    let snap = m.snapshot_create(&ds, "s1").await.expect("snapshot create");
    assert_eq!(snap.name, "s1");
    assert_eq!(snap.dataset, "tank/ds");
    assert_eq!(snap.size, 2048);
}

#[tokio::test]
async fn snapshot_create_errors_when_snapshot_command_fails() {
    let _lock = path_cli_test_lock::acquire().await;
    let tmp = tempfile::tempdir().expect("tempdir");
    write_executable(
        tmp.path(),
        "zfs",
        r#"#!/bin/sh
echo "busy" >&2
exit 1
"#,
    );
    let _g = PathEnvGuard::prepend_bin_dir(tmp.path());

    let m = TestingZfsManager::new();
    let err = m
        .snapshot_create(&test_dataset(), "bad")
        .await
        .expect_err("expected error");
    assert!(err.to_string().contains("ZFS command failed") || err.to_string().contains("busy"));
}

#[tokio::test]
async fn snapshot_create_errors_when_at_capacity() {
    let _lock = path_cli_test_lock::acquire().await;
    let tmp = tempfile::tempdir().expect("tempdir");
    write_executable(tmp.path(), "zfs", "#!/bin/sh\nexit 0\n");
    let _g = PathEnvGuard::prepend_bin_dir(tmp.path());

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
async fn snapshot_list_returns_parsed_rows() {
    let _lock = path_cli_test_lock::acquire().await;
    let tmp = tempfile::tempdir().expect("tempdir");
    write_executable(
        tmp.path(),
        "zfs",
        concat!(
            "#!/bin/sh\nif [ \"",
            "${",
            "1:-",
            "}",
            "\" = \"list\" ]; then\n  printf '%s\\n' 'tank/ds@a\t100' 'tank/ds@b\t200'\n  exit 0\nfi\nexit 1\n"
        ),
    );
    let _g = PathEnvGuard::prepend_bin_dir(tmp.path());

    let m = TestingZfsManager::new();
    let rows = m.snapshot_list(&test_dataset()).await.expect("list");
    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0].name, "a");
    assert_eq!(rows[1].name, "b");
}

#[tokio::test]
async fn snapshot_list_stops_at_max_snapshots() {
    let _lock = path_cli_test_lock::acquire().await;
    let tmp = tempfile::tempdir().expect("tempdir");
    let mut body = String::from(concat!(
        "#!/bin/sh\nset -eu\nif [ \"",
        "${",
        "1:-",
        "}",
        "\" = \"list\" ]; then\n"
    ));
    for i in 0..150 {
        let _ = writeln!(body, "  echo 'tank/ds@s{i}\t{i}'");
    }
    body.push_str("  exit 0\nfi\nexit 1\n");
    write_executable(tmp.path(), "zfs", &body);
    let _g = PathEnvGuard::prepend_bin_dir(tmp.path());

    let m = TestingZfsManager::new();
    let rows = m.snapshot_list(&test_dataset()).await.expect("list");
    assert_eq!(rows.len(), 100);
}

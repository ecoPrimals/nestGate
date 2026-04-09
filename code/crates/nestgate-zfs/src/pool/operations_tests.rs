// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

#![allow(unsafe_code)] // `PATH` swap for mock `zpool` (Rust 2024: `set_var` is `unsafe`)

//! Tests for [`super::operations`] using mock `zpool` on `PATH`.

use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

use crate::path_cli_test_lock;

use super::manager::ZfsPoolManager;
use crate::pool::types::{PoolCapacity, PoolHealth, PoolInfo, PoolState};

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

fn sample_pool(name: &str) -> PoolInfo {
    PoolInfo {
        name: name.to_string(),
        state: PoolState::Online,
        health: PoolHealth::Healthy,
        capacity: PoolCapacity {
            total_bytes: 1_000_000,
            used_bytes: 500_000,
            available_bytes: 500_000,
            utilization_percent: 50.0,
            fragmentation_percent: 0.0,
            deduplication_ratio: 1.0,
            total: 1_000_000,
            used: 500_000,
            available: 500_000,
        },
        devices: vec![],
        properties: Default::default(),
    }
}

#[tokio::test]
async fn destroy_pool_removes_cached_pool_on_success() {
    let _lock = path_cli_test_lock::acquire().await;
    let tmp = tempfile::tempdir().expect("tempdir");
    write_executable(
        tmp.path(),
        "zpool",
        concat!(
            "#!/bin/sh\nif [ \"",
            "${",
            "1:-",
            "}",
            "\" = \"destroy\" ]; then exit 0; fi\nexit 1\n"
        ),
    );
    let _g = PathEnvGuard::prepend_bin_dir(tmp.path());

    let m = ZfsPoolManager::new_for_testing();
    m.insert_pool_for_testing(sample_pool("gone")).await;
    m.destroy_pool("gone").await.expect("destroy");
    let list = m.list_pools().await.expect("list");
    assert!(list.is_empty());
}

#[tokio::test]
async fn destroy_pool_returns_error_on_nonzero_exit() {
    let _lock = path_cli_test_lock::acquire().await;
    let tmp = tempfile::tempdir().expect("tempdir");
    write_executable(
        tmp.path(),
        "zpool",
        r#"#!/bin/sh
echo "cannot destroy" >&2
exit 3
"#,
    );
    let _g = PathEnvGuard::prepend_bin_dir(tmp.path());

    let m = ZfsPoolManager::new_for_testing();
    m.insert_pool_for_testing(sample_pool("p1")).await;
    assert!(m.destroy_pool("p1").await.is_err());
}

#[tokio::test]
async fn scrub_pool_succeeds_when_zpool_returns_ok() {
    let _lock = path_cli_test_lock::acquire().await;
    let tmp = tempfile::tempdir().expect("tempdir");
    write_executable(
        tmp.path(),
        "zpool",
        concat!(
            "#!/bin/sh\nif [ \"",
            "${",
            "1:-",
            "}",
            "\" = \"scrub\" ]; then exit 0; fi\nexit 1\n"
        ),
    );
    let _g = PathEnvGuard::prepend_bin_dir(tmp.path());

    let m = ZfsPoolManager::new_for_testing();
    m.scrub_pool("tank").await.expect("scrub");
}

#[tokio::test]
async fn scrub_pool_maps_stderr_on_failure() {
    let _lock = path_cli_test_lock::acquire().await;
    let tmp = tempfile::tempdir().expect("tempdir");
    write_executable(
        tmp.path(),
        "zpool",
        r#"#!/bin/sh
echo "scrub failed" >&2
exit 1
"#,
    );
    let _g = PathEnvGuard::prepend_bin_dir(tmp.path());

    let m = ZfsPoolManager::new_for_testing();
    let err = m.scrub_pool("bad").await.expect_err("err");
    assert!(err.to_string().contains("scrub"));
}

#[tokio::test]
async fn create_pool_maps_zpool_create_failure() {
    let _lock = path_cli_test_lock::acquire().await;
    let tmp = tempfile::tempdir().expect("tempdir");
    write_executable(
        tmp.path(),
        "zpool",
        concat!(
            "#!/bin/sh\nif [ \"",
            "${",
            "1:-",
            "}",
            "\" = \"create\" ]; then\n  echo \"no vdevs\" >&2\n  exit 1\nfi\nexit 0\n"
        ),
    );
    let _g = PathEnvGuard::prepend_bin_dir(tmp.path());

    let m = ZfsPoolManager::new_for_testing();
    let err = m
        .create_pool("newpool", &["/dev/null".into()])
        .await
        .expect_err("create should fail");
    assert!(err.to_string().contains("zpool create"));
}

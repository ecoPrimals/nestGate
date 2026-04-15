// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Tests for [`super::RealZfsOperations`] CLI wrappers using mock `zfs` / `zpool` on `PATH`.

use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

use crate::path_cli_test_lock;
use nestgate_types::MapEnv;
use std::sync::Arc;

use super::RealZfsOperations;

fn path_with_bin_prepended(bin_dir: &Path) -> String {
    let mut new_path = bin_dir.as_os_str().to_owned();
    new_path.push(":");
    if let Some(p) = std::env::var_os("PATH") {
        new_path.push(p);
    }
    new_path.to_string_lossy().into_owned()
}

fn write_executable(dir: &Path, name: &str, body: &str) {
    let p = dir.join(name);
    fs::write(&p, body).expect("write mock");
    let mut perm = fs::metadata(&p).expect("meta").permissions();
    perm.set_mode(0o755);
    fs::set_permissions(&p, perm).expect("chmod");
}

fn install_ops_mocks(dir: &Path, zpool_body: &str, zfs_body: &str) {
    write_executable(dir, "zpool", zpool_body);
    write_executable(dir, "zfs", zfs_body);
}

#[tokio::test]
async fn get_pool_status_parses_json_array() {
    let _lock = path_cli_test_lock::acquire().await;
    let tmp = tempfile::tempdir().expect("tempdir");
    let zpool = concat!(
        "#!/bin/sh\nif [ \"",
        "${",
        "1:-",
        "}",
        "\" = \"status\" ] && echo \"$*\" | grep -q -- \"-j\"; then\n  echo '[{\"name\":\"tank\",\"state\":\"ONLINE\",\"size\":\"100G\",\"allocated\":\"40G\",\"free\":\"60G\"}]'\n  exit 0\nfi\nexit 1\n"
    );
    let zfs = "#!/bin/sh\nexit 0\n";
    install_ops_mocks(tmp.path(), zpool, zfs);
    let path = path_with_bin_prepended(tmp.path());
    temp_env::async_with_vars([("PATH", Some(path.as_str()))], async {
        let ops = RealZfsOperations::default();
        let resp = ops.get_pool_status(None).await.expect("pool status");
        match resp {
            crate::handlers::ZfsResponse::PoolStatus { pools } => {
                assert_eq!(pools.len(), 1);
                assert_eq!(pools[0].name, "tank");
                assert_eq!(pools[0].state, "ONLINE");
            }
            other => panic!("unexpected response: {other:?}"),
        }
    })
    .await;
}

#[tokio::test]
async fn get_pool_status_propagates_command_failure() {
    let _lock = path_cli_test_lock::acquire().await;
    let tmp = tempfile::tempdir().expect("tempdir");
    let zpool = r#"#!/bin/sh
echo "boom" >&2
exit 7
"#;
    let zfs = "#!/bin/sh\nexit 0\n";
    install_ops_mocks(tmp.path(), zpool, zfs);
    let path = path_with_bin_prepended(tmp.path());
    temp_env::async_with_vars([("PATH", Some(path.as_str()))], async {
        let ops = RealZfsOperations::default();
        let err = ops
            .get_pool_status(None)
            .await
            .expect_err("expected failure");
        let msg = err.to_string();
        assert!(
            msg.contains("ZFS pool status") || msg.contains("boom"),
            "unexpected message: {msg}"
        );
    })
    .await;
}

#[tokio::test]
async fn get_pool_status_invalid_json_errors() {
    let _lock = path_cli_test_lock::acquire().await;
    let tmp = tempfile::tempdir().expect("tempdir");
    let zpool = concat!(
        "#!/bin/sh\nif [ \"",
        "${",
        "1:-",
        "}",
        "\" = \"status\" ]; then\n  echo 'not-json-at-all'\n  exit 0\nfi\nexit 1\n"
    );
    install_ops_mocks(tmp.path(), zpool, "#!/bin/sh\nexit 0\n");
    let path = path_with_bin_prepended(tmp.path());
    temp_env::async_with_vars([("PATH", Some(path.as_str()))], async {
        let ops = RealZfsOperations::default();
        assert!(ops.get_pool_status(None).await.is_err());
    })
    .await;
}

#[tokio::test]
async fn get_dataset_list_parses_tab_lines() {
    let _lock = path_cli_test_lock::acquire().await;
    let tmp = tempfile::tempdir().expect("tempdir");
    let zpool = "#!/bin/sh\nexit 0\n";
    let zfs = concat!(
        "#!/bin/sh\nif [ \"",
        "${",
        "1:-",
        "}",
        "\" = \"list\" ]; then\n  printf '%s\\n' 'd0/a\t10G\t20G\t5G\t/mnt/x'\n  exit 0\nfi\nexit 1\n"
    );
    install_ops_mocks(tmp.path(), zpool, zfs);
    let path = path_with_bin_prepended(tmp.path());
    temp_env::async_with_vars([("PATH", Some(path.as_str()))], async {
        let ops = RealZfsOperations::default();
        let resp = ops.get_dataset_list(None).await.expect("datasets");
        match resp {
            crate::handlers::ZfsResponse::DatasetList { datasets } => {
                assert_eq!(datasets.len(), 1);
                assert_eq!(datasets[0].name, "d0/a");
            }
            other => panic!("unexpected: {other:?}"),
        }
    })
    .await;
}

#[tokio::test]
async fn get_dataset_list_maps_command_failure() {
    let _lock = path_cli_test_lock::acquire().await;
    let tmp = tempfile::tempdir().expect("tempdir");
    install_ops_mocks(
        tmp.path(),
        "#!/bin/sh\nexit 0\n",
        r#"#!/bin/sh
echo "zfs list failed" >&2
exit 2
"#,
    );
    let path = path_with_bin_prepended(tmp.path());
    temp_env::async_with_vars([("PATH", Some(path.as_str()))], async {
        let ops = RealZfsOperations::default();
        assert!(ops.get_dataset_list(None).await.is_err());
    })
    .await;
}

#[tokio::test]
async fn get_snapshot_list_parses_lines() {
    let _lock = path_cli_test_lock::acquire().await;
    let tmp = tempfile::tempdir().expect("tempdir");
    install_ops_mocks(
        tmp.path(),
        "#!/bin/sh\nexit 0\n",
        concat!(
            "#!/bin/sh\nif [ \"",
            "${",
            "1:-",
            "}",
            "\" = \"list\" ] && echo \"$*\" | grep -q snapshot; then\n  printf '%s\\n' 'pool/ds@s1\t4096\t2020-01-01'\n  exit 0\nfi\nexit 1\n"
        ),
    );
    let path = path_with_bin_prepended(tmp.path());
    temp_env::async_with_vars([("PATH", Some(path.as_str()))], async {
        let ops = RealZfsOperations::default();
        let resp = ops
            .get_snapshot_list(Some("pool/ds".into()))
            .await
            .expect("snapshots");
        match resp {
            crate::handlers::ZfsResponse::SnapshotList { snapshots } => {
                assert_eq!(snapshots.len(), 1);
                assert_eq!(snapshots[0].name, "pool/ds@s1");
            }
            other => panic!("unexpected: {other:?}"),
        }
    })
    .await;
}

#[tokio::test]
async fn validate_security_secure_mode_requires_encryption_command() {
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
            "\" = \"get\" ] && echo \"$*\" | grep -q encryption; then\n  exit 0\nfi\nexit 1\n"
        ),
    );
    write_executable(tmp.path(), "zpool", "#!/bin/sh\nexit 1\n");
    let path = path_with_bin_prepended(tmp.path());
    temp_env::async_with_vars([("PATH", Some(path.as_str()))], async {
        use super::ProductionReadinessValidator;
        let v = ProductionReadinessValidator::new_with_env(Arc::new(MapEnv::from([(
            "NESTGATE_SECURE_MODE",
            "true",
        )])));
        assert!(
            v.validate_security()
                .expect("ok when encryption probe succeeds")
        );
    })
    .await;
}

#[tokio::test]
async fn validate_security_secure_mode_fails_when_encryption_probe_fails() {
    let _lock = path_cli_test_lock::acquire().await;
    let tmp = tempfile::tempdir().expect("tempdir");
    write_executable(tmp.path(), "zfs", "#!/bin/sh\nexit 1\n");
    let path = path_with_bin_prepended(tmp.path());
    temp_env::async_with_vars([("PATH", Some(path.as_str()))], async {
        use super::ProductionReadinessValidator;
        let v = ProductionReadinessValidator::new_with_env(Arc::new(MapEnv::from([(
            "NESTGATE_SECURE_MODE",
            "true",
        )])));
        assert!(!v.validate_security().expect("bool result"));
    })
    .await;
}

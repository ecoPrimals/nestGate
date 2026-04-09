// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

use super::*;
use axum::extract::{Json, Path};
use axum::http::StatusCode;
use nestgate_types::MapEnv;
use serde_json::json;

#[test]
fn parse_size_numeric_and_none() {
    assert_eq!(parse_size("4096"), 4096);
    assert_eq!(parse_size("none"), 0);
    assert_eq!(parse_size("-"), 0);
    assert!(parse_size("1M") >= 1024 * 1024);
}

#[test]
fn parse_size_suffixes_k_g_t_p() {
    assert_eq!(parse_size("2G"), 2 * 1024 * 1024 * 1024);
    assert_eq!(parse_size("1K"), 1024);
    let one_t = parse_size("1T");
    assert!(one_t >= 1024_u64 * 1024 * 1024 * 1024);
    let one_p = parse_size("1P");
    assert!(one_p > one_t);
}

#[test]
fn parse_size_whitespace_and_invalid() {
    assert_eq!(parse_size("  3G  "), 3 * 1024 * 1024 * 1024);
    assert_eq!(parse_size(""), 0);
    assert_eq!(parse_size("Z"), 0);
}

#[test]
fn parse_size_unknown_suffix_uses_multiplier_one() {
    assert_eq!(parse_size("1X"), 1);
    assert_eq!(parse_size("4X"), 4);
}

#[test]
fn parse_size_invalid_number_before_suffix() {
    assert_eq!(parse_size("abcG"), 0);
    assert_eq!(parse_size("not5M"), 0);
}

#[test]
fn parse_size_decimal_k_and_m() {
    assert_eq!(parse_size("1.5K"), 1536);
    let m = parse_size("2.5M");
    assert!(m >= 2 * 1024 * 1024);
}

#[test]
fn parse_size_single_char_no_suffix_parse() {
    assert_eq!(parse_size("9"), 9);
}

#[tokio::test]
async fn update_workspace_config_rejects_empty_or_slash_id() {
    let bad = update_workspace_config(Path(String::new()), Json(json!({ "quota": "1G" }))).await;
    assert!(matches!(bad, Err(StatusCode::BAD_REQUEST)));
    let bad2 = update_workspace_config(Path("a/b".to_string()), Json(json!({}))).await;
    assert!(matches!(bad2, Err(StatusCode::BAD_REQUEST)));
}

#[tokio::test]
async fn delete_workspace_rejects_invalid_id() {
    let r = delete_workspace(Path(String::new())).await;
    assert!(matches!(r, Err(StatusCode::BAD_REQUEST)));
    let r2 = delete_workspace(Path("../escape".to_string())).await;
    assert!(matches!(r2, Err(StatusCode::BAD_REQUEST)));
}

/// Creates a temp dir with an executable `zfs` script; returns `(dir, absolute path to zfs)`.
#[cfg(unix)]
fn fake_zfs_dir_and_path(script: &str) -> (tempfile::TempDir, String) {
    use std::os::unix::fs::PermissionsExt;
    let dir = tempfile::tempdir().expect("tempdir");
    let bin = dir.path().join("zfs");
    std::fs::write(&bin, script).expect("write fake zfs");
    let mut perms = std::fs::metadata(&bin).expect("metadata").permissions();
    perms.set_mode(0o755);
    std::fs::set_permissions(&bin, perms).expect("chmod");
    (dir, bin.to_string_lossy().into_owned())
}

/// Covers `get_workspaces` success path, `get_workspace_properties`, and list parsing.
#[cfg(unix)]
const FAKE_ZFS_GET_WORKSPACES: &str = r#"#!/usr/bin/env bash
cmd="${1:-}"
case "$cmd" in
  list)
if [[ "$*" == *"zfspool/workspaces"* ]]; then
  echo -e "zfspool/workspaces/ws-active\t1048576\t10485760\t1048576\t/mnt\tJan 1 2020"
  echo -e "zfspool/workspaces/ws-inactive\t1048576\t10485760\t1048576\t/mnt\tJan 1 2020"
  exit 0
fi
if [[ "$*" == *"-t snapshot"* ]]; then
  echo "pool@snap"
  exit 0
fi
if [[ "$*" == *"rpool/workspaces"* ]] && [[ "$*" == *"-o name"* ]]; then
  exit 1
fi
exit 1
;;
  get)
if [[ "$*" == *"property,value"* ]]; then
  dataset="${@: -1}"
  if [[ "$dataset" == *"crit"* ]]; then
    echo -e "used\t95"; echo -e "available\t5"; echo -e "referenced\t95"
    echo -e "quota\t100"; echo -e "compression\tlz4"; echo -e "recordsize\t128K"
    echo -e "mountpoint\t/tmp"; echo -e "creation\tJan 1 2020"
    echo -e "org.nestgate:workspace_name\tCrit"
  elif [[ "$dataset" == *"warn"* ]]; then
    echo -e "used\t85"; echo -e "available\t15"; echo -e "referenced\t85"
    echo -e "quota\t100"; echo -e "compression\tlz4"; echo -e "recordsize\t128K"
    echo -e "mountpoint\t/tmp"; echo -e "creation\tJan 1 2020"
    echo -e "org.nestgate:workspace_name\tWarn"
  elif [[ "$dataset" == *"fallback"* ]]; then
    echo -e "used\t10"; echo -e "available\t90"; echo -e "referenced\t10"
    echo -e "quota\t100"; echo -e "compression\tlz4"; echo -e "recordsize\t128K"
    echo -e "mountpoint\t/tmp"; echo -e "creation\tJan 1 2020"
  else
    echo -e "used\t10"; echo -e "available\t90"; echo -e "referenced\t10"
    echo -e "quota\t100"; echo -e "compression\tlz4"; echo -e "recordsize\t128K"
    echo -e "mountpoint\t/tmp"; echo -e "creation\tJan 1 2020"
    echo -e "org.nestgate:workspace_name\tNamed WS"
  fi
  exit 0
fi
if [[ "$*" == *"used,available,quota,compression"* ]] && [[ "$*" == *"workspaces"* ]]; then
  echo "1"; echo "2"; echo "3"; echo "4"
  exit 0
fi
if [[ "$*" == *"compression,quota,mounted"* ]]; then
  dataset="${@: -1}"
  if [[ "$dataset" == *"inactive"* ]]; then
    echo "lz4"; echo "10G"; echo "no"
  else
    echo "lz4"; echo "10G"; echo "yes"
  fi
  exit 0
fi
exit 1
;;
  create)
exit 0
;;
  set)
if [[ "$2" == "quota=bad"* ]] || [[ "$2" == "compression=bad"* ]] || [[ "$2" == org.nestgate:workspace_name=bad* ]]; then
  echo fail >&2
  exit 1
fi
exit 0
;;
  destroy)
exit 0
;;
  *)
exit 1
;;
esac
"#;

#[cfg(unix)]
#[tokio::test]
async fn get_workspaces_fake_zfs_lists_datasets() {
    let (_dir, zfs_path) = fake_zfs_dir_and_path(FAKE_ZFS_GET_WORKSPACES);
    let env = MapEnv::from([("NESTGATE_ZFS_BINARY", zfs_path.as_str())]);
    let Json(v) = get_workspaces_from_env_source(&env).await.expect("ok");
    assert_eq!(v["status"], "success");
    assert_eq!(v["pool"], "zfspool");
    let arr = v["workspaces"].as_array().expect("array");
    assert_eq!(arr.len(), 2);
    assert_eq!(arr[0]["id"], "ws-active");
    assert_eq!(arr[0]["status"], "active");
    assert_eq!(arr[1]["id"], "ws-inactive");
    assert_eq!(arr[1]["status"], "inactive");
}

#[cfg(unix)]
#[tokio::test]
async fn get_workspace_fake_zfs_healthy_warning_critical_and_name_fallback() {
    let (_dir, zfs_path) = fake_zfs_dir_and_path(FAKE_ZFS_GET_WORKSPACES);
    let env = MapEnv::from([("NESTGATE_ZFS_BINARY", zfs_path.as_str())]);
    let Json(h) = get_workspace_from_env_source(&env, Path("ws-healthy".to_string()))
        .await
        .expect("healthy");
    assert_eq!(h["workspace"]["health_status"], "healthy");
    assert_eq!(h["workspace"]["name"], "Named WS");

    let Json(w) = get_workspace_from_env_source(&env, Path("ws-warn-test".to_string()))
        .await
        .expect("warn");
    assert_eq!(w["workspace"]["health_status"], "warning");

    let Json(c) = get_workspace_from_env_source(&env, Path("ws-crit-test".to_string()))
        .await
        .expect("crit");
    assert_eq!(c["workspace"]["health_status"], "critical");

    let Json(f) = get_workspace_from_env_source(&env, Path("fallback-ws-id".to_string()))
        .await
        .expect("fallback");
    assert_eq!(f["workspace"]["name"], "fallback ws id");
}

#[cfg(unix)]
#[tokio::test]
async fn create_workspace_fake_zfs_succeeds() {
    let (_dir, zfs_path) = fake_zfs_dir_and_path(FAKE_ZFS_GET_WORKSPACES);
    let env = MapEnv::from([("NESTGATE_ZFS_BINARY", zfs_path.as_str())]);
    let req = json!({
        "name": "ok-ws",
        "quota": "20G",
        "compression": "zstd",
        "recordsize": "256K"
    });
    let Json(v) = create_workspace_from_env_source(&env, Json(req))
        .await
        .expect("create ok");
    assert_eq!(v["status"], "success");
    assert!(v.get("workspace_id").is_some());
    assert_eq!(v["name"], "ok-ws");
}

#[cfg(unix)]
#[tokio::test]
async fn update_workspace_config_fake_zfs_success_empty_and_partial() {
    let (_dir, zfs_path) = fake_zfs_dir_and_path(FAKE_ZFS_GET_WORKSPACES);
    let env = MapEnv::from([("NESTGATE_ZFS_BINARY", zfs_path.as_str())]);
    let Json(empty) =
        update_workspace_config_from_env_source(&env, Path("ws-1".to_string()), Json(json!({})))
            .await
            .expect("empty ok");
    assert_eq!(empty["status"], "success");
    let up: Vec<String> =
        serde_json::from_value(empty["updated_properties"].clone()).expect("updated_properties");
    assert!(up.is_empty());

    let Json(ok) = update_workspace_config_from_env_source(
        &env,
        Path("ws-2".to_string()),
        Json(json!({ "quota": "10G", "compression": "lz4", "name": "n" })),
    )
    .await
    .expect("full ok");
    assert_eq!(ok["status"], "success");

    let Json(partial) = update_workspace_config_from_env_source(
        &env,
        Path("ws-3".to_string()),
        Json(json!({ "quota": "badquota", "compression": "lz4", "name": "still-ok" })),
    )
    .await
    .expect("partial");
    assert_eq!(partial["status"], "partial_success");
    let errs = partial["errors"].as_array().expect("errors");
    assert!(!errs.is_empty());

    let all_bad = update_workspace_config_from_env_source(
        &env,
        Path("ws-4".to_string()),
        Json(json!({
            "quota": "badquota",
            "compression": "badcompression",
            "name": "badname"
        })),
    )
    .await;
    assert!(matches!(all_bad, Err(StatusCode::BAD_REQUEST)));
}

#[cfg(unix)]
#[tokio::test]
async fn delete_workspace_fake_zfs_dataset_missing_is_not_found() {
    let (_dir, zfs_path) = fake_zfs_dir_and_path(FAKE_ZFS_GET_WORKSPACES);
    let env = MapEnv::from([("NESTGATE_ZFS_BINARY", zfs_path.as_str())]);
    let r = delete_workspace_from_env_source(&env, Path("missing-ws".to_string())).await;
    assert!(
        matches!(r, Err(StatusCode::NOT_FOUND)),
        "expected NOT_FOUND for missing dataset, got {r:?}"
    );
}

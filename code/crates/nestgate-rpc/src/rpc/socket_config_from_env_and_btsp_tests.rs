// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! [`MapEnv`] / [`ProcessEnv`] parsing, additional `resolve()` edge cases, and BTSP guard +
//! family-scoped socket naming tests.

use super::*;
use nestgate_types::{MapEnv, ProcessEnv};
use std::path::PathBuf;

// ========================================================================
// from_env_source(MapEnv) — no process env mutation (parallel-safe)
// ========================================================================

#[test]
fn from_env_source_respects_nestgate_socket() {
    let root = tempfile::tempdir().expect("tempdir");
    let sock = root.path().join("from-env-explicit.sock");
    let sock_s = sock.to_string_lossy().into_owned();
    let env = MapEnv::from([
        ("NESTGATE_SOCKET", sock_s.as_str()),
        ("NESTGATE_FAMILY_ID", "fam-env"),
        ("NESTGATE_NODE_ID", "node-env"),
    ]);
    let cfg = SocketConfig::from_env_source(&env).expect("from_env_source");
    assert_eq!(cfg.socket_path, sock);
    assert_eq!(cfg.family_id, "fam-env");
    assert_eq!(cfg.node_id, "node-env");
    assert_eq!(cfg.source, SocketConfigSource::Environment);
}

#[test]
fn from_env_source_default_family_standalone_when_unset() {
    let env = MapEnv::new();
    let cfg = SocketConfig::from_env_source(&env).expect("from_env_source");
    assert_eq!(cfg.family_id, "standalone");
    assert!(!cfg.node_id.is_empty());
}

#[test]
fn from_env_source_biomeos_dir_when_no_socket_override() {
    let root = tempfile::tempdir().expect("tempdir");
    let expected_sock = root.path().join("nestgate-bf.sock");
    let dir = root.path().to_string_lossy().into_owned();
    let env = MapEnv::from([
        ("BIOMEOS_SOCKET_DIR", dir.as_str()),
        ("NESTGATE_FAMILY_ID", "bf"),
    ]);
    let cfg = SocketConfig::from_env_source(&env).expect("from_env_source");
    assert_eq!(cfg.socket_path, expected_sock);
    assert_eq!(cfg.source, SocketConfigSource::BiomeOSDirectory);
}

#[test]
fn from_env_source_xdg_runtime_when_dir_exists() {
    let dir = tempfile::tempdir().expect("tempdir");
    let rt = dir.path().to_string_lossy().into_owned();
    let env = MapEnv::from([("XDG_RUNTIME_DIR", rt.as_str())]);
    let cfg = SocketConfig::from_env_source(&env).expect("from_env_source");
    assert_eq!(cfg.source, SocketConfigSource::XdgRuntime);
    assert!(cfg.socket_path.ends_with("biomeos/nestgate.sock"));
}

#[test]
fn from_env_source_socket_beats_biomeos_and_xdg() {
    let xdg = tempfile::tempdir().expect("tempdir");
    let only = tempfile::tempdir().expect("tempdir");
    let only_sock = only.path().join("only-this.sock");
    let ignored = tempfile::tempdir().expect("tempdir");
    let only_s = only_sock.to_string_lossy().into_owned();
    let biome_s = ignored.path().to_string_lossy().into_owned();
    let xdg_s = xdg.path().to_string_lossy().into_owned();
    let env = MapEnv::from([
        ("NESTGATE_SOCKET", only_s.as_str()),
        ("BIOMEOS_SOCKET_DIR", biome_s.as_str()),
        ("XDG_RUNTIME_DIR", xdg_s.as_str()),
    ]);
    let cfg = SocketConfig::from_env_source(&env).expect("from_env_source");
    assert_eq!(cfg.socket_path, only_sock);
    assert_eq!(cfg.source, SocketConfigSource::Environment);
}

/// Smoke: [`ProcessEnv`] implements [`EnvSource`] the same way production `from_environment()` does.
#[test]
fn from_env_source_accepts_process_env() {
    let _ = SocketConfig::from_env_source(&ProcessEnv);
}

// ========================================================================
// Additional resolve / edge cases
// ========================================================================

#[test]
fn resolve_empty_string_override_still_tier1_environment() {
    let cfg = SocketConfig::resolve(
        "a".to_string(),
        "b".to_string(),
        Some(String::new()),
        None,
        None,
    )
    .expect("resolve");
    assert_eq!(cfg.source, SocketConfigSource::Environment);
    assert_eq!(cfg.socket_path, PathBuf::from(""));
}

#[test]
fn resolve_xdg_nonexistent_path_skips_tier3() {
    let cfg = SocketConfig::resolve(
        "fam".to_string(),
        "nod".to_string(),
        None,
        None,
        Some("/nonexistent/nestgate-xdg-999999999999999".to_string()),
    )
    .expect("resolve");
    assert_eq!(cfg.source, SocketConfigSource::TempDirectory);
    assert!(
        cfg.socket_path
            .to_string_lossy()
            .contains("nestgate-fam-nod.sock")
    );
}

#[test]
fn socket_path_str_empty_when_non_utf8() {
    use std::ffi::OsString;
    use std::os::unix::ffi::OsStringExt;

    let bytes = b"/tmp/\xFF\xFE.sock".to_vec();
    let os = OsString::from_vec(bytes);
    let cfg = SocketConfig {
        socket_path: PathBuf::from(os),
        family_id: "x".to_string(),
        node_id: "y".to_string(),
        source: SocketConfigSource::TempDirectory,
    };
    assert_eq!(cfg.socket_path_str(), "");
}

// ========================================================================
// BTSP Phase 1 — INSECURE guard + family-scoped socket naming
// ========================================================================

#[test]
fn btsp_guard_rejects_family_id_plus_insecure() {
    let env = MapEnv::from([
        ("NESTGATE_FAMILY_ID", "my-family"),
        ("BIOMEOS_INSECURE", "1"),
    ]);
    let result = SocketConfig::from_env_source(&env);
    assert!(result.is_err(), "FAMILY_ID + BIOMEOS_INSECURE=1 must fail");
    let msg = format!("{}", result.unwrap_err());
    assert!(
        msg.contains("BTSP guard"),
        "error should mention BTSP guard: {msg}"
    );
}

#[test]
fn btsp_guard_allows_insecure_without_family_id() {
    let env = MapEnv::from([("BIOMEOS_INSECURE", "1")]);
    let result = SocketConfig::from_env_source(&env);
    assert!(
        result.is_ok(),
        "INSECURE without FAMILY_ID should be fine (dev mode)"
    );
}

#[test]
fn btsp_guard_allows_insecure_with_standalone_family() {
    let env = MapEnv::from([
        ("NESTGATE_FAMILY_ID", "standalone"),
        ("BIOMEOS_INSECURE", "1"),
    ]);
    let result = SocketConfig::from_env_source(&env);
    assert!(
        result.is_ok(),
        "INSECURE with 'standalone' family should be fine (dev mode)"
    );
}

#[test]
fn btsp_guard_allows_insecure_with_default_family() {
    let env = MapEnv::from([("NESTGATE_FAMILY_ID", "default"), ("BIOMEOS_INSECURE", "1")]);
    let result = SocketConfig::from_env_source(&env);
    assert!(
        result.is_ok(),
        "INSECURE with 'default' family should be fine (dev mode)"
    );
}

#[test]
fn btsp_guard_allows_family_id_without_insecure() {
    let env = MapEnv::from([("NESTGATE_FAMILY_ID", "production-fam")]);
    let result = SocketConfig::from_env_source(&env);
    assert!(result.is_ok(), "FAMILY_ID alone (no INSECURE) should work");
}

#[test]
fn family_scoped_socket_name_in_biomeos_dir() {
    let root = tempfile::tempdir().expect("tempdir");
    let biomeos_dir = root.path().to_str().expect("utf8");
    let config = SocketConfig::resolve(
        "production-fam".to_string(),
        "node1".to_string(),
        None,
        Some(biomeos_dir.to_string()),
        None,
    )
    .expect("resolve");
    assert_eq!(
        config.socket_path.file_name().unwrap().to_str().unwrap(),
        "nestgate-production-fam.sock",
        "family-scoped naming per BTSP §Socket Naming"
    );
}

#[test]
fn standalone_uses_simple_socket_name_in_biomeos_dir() {
    let root = tempfile::tempdir().expect("tempdir");
    let biomeos_dir = root.path().to_str().expect("utf8");
    let config = SocketConfig::resolve(
        "standalone".to_string(),
        "node1".to_string(),
        None,
        Some(biomeos_dir.to_string()),
        None,
    )
    .expect("resolve");
    assert_eq!(
        config.socket_path.file_name().unwrap().to_str().unwrap(),
        "nestgate.sock",
        "dev default keeps simple name"
    );
}

#[test]
fn family_scoped_socket_name_in_xdg_tier() {
    let dir = tempfile::tempdir().expect("tempdir");
    let xdg_path = dir.path().to_str().expect("utf8");
    let config = SocketConfig::resolve(
        "my-family".to_string(),
        "node1".to_string(),
        None,
        None,
        Some(xdg_path.to_string()),
    )
    .expect("resolve");
    assert_eq!(
        config.socket_path.file_name().unwrap().to_str().unwrap(),
        "nestgate-my-family.sock",
        "XDG tier also uses family-scoped name"
    );
}

#[test]
fn generic_family_id_also_accepted() {
    let env = MapEnv::from([("FAMILY_ID", "eco-family"), ("BIOMEOS_INSECURE", "1")]);
    let result = SocketConfig::from_env_source(&env);
    assert!(
        result.is_err(),
        "generic FAMILY_ID + BIOMEOS_INSECURE must also be caught"
    );
}

#[test]
fn nestgate_family_id_takes_precedence_over_generic() {
    let env = MapEnv::from([
        ("NESTGATE_FAMILY_ID", "nestgate-specific"),
        ("FAMILY_ID", "generic-family"),
    ]);
    let config = SocketConfig::from_env_source(&env).expect("resolve");
    assert_eq!(config.family_id, "nestgate-specific");
}

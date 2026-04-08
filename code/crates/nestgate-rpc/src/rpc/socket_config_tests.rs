// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

use super::*;
use nestgate_types::{EnvSource, MapEnv, ProcessEnv};
use std::fs;
use std::os::unix::net::UnixListener;
use std::path::{Path, PathBuf};

// ========================================================================
// UNIT TESTS - Pure Logic via resolve() (no env var races)
// ========================================================================

#[test]
fn test_explicit_socket_path_has_highest_priority() {
    // Uses resolve() directly - no env var pollution between parallel tests
    let config = SocketConfig::resolve(
        "test".to_string(),
        "default".to_string(),
        Some("/tmp/explicit.sock".to_string()),
        None,
        None,
    )
    .unwrap();

    assert_eq!(config.socket_path, PathBuf::from("/tmp/explicit.sock"));
    assert_eq!(config.family_id, "test");
    assert_eq!(config.source, SocketConfigSource::Environment);
}

#[test]
fn test_biomeos_dir_second_priority() {
    let config = SocketConfig::resolve(
        "biotest".to_string(),
        "default".to_string(),
        None,
        Some("/tmp/biomeos-test-dir".to_string()),
        None,
    )
    .unwrap();

    assert_eq!(
        config.socket_path,
        PathBuf::from("/tmp/biomeos-test-dir/nestgate-biotest.sock")
    );
    assert_eq!(config.source, SocketConfigSource::BiomeOSDirectory);
}

#[test]
fn test_fallback_without_overrides() {
    // No socket override, no biomeOS dir -> falls through to XDG or /tmp
    let config = SocketConfig::resolve(
        "fallback".to_string(),
        "node42".to_string(),
        None,
        None,
        None,
    )
    .unwrap();

    let path_str = config.socket_path.to_str().unwrap();
    assert!(
        path_str.contains("nestgate"),
        "Socket path should contain 'nestgate'"
    );
    assert_eq!(config.family_id, "fallback");
    assert_eq!(config.node_id, "node42");
}

#[test]
fn test_explicit_override_beats_biomeos_dir() {
    // Both provided - explicit should win
    let config = SocketConfig::resolve(
        "test".to_string(),
        "default".to_string(),
        Some("/tmp/override.sock".to_string()),
        Some("/tmp/biomeos-dir".to_string()),
        None,
    )
    .unwrap();

    assert_eq!(config.socket_path, PathBuf::from("/tmp/override.sock"));
    assert_eq!(config.source, SocketConfigSource::Environment);
}

#[test]
fn test_multi_instance_unique_sockets() {
    // Pure logic test - no env vars
    let config1 = SocketConfig::resolve(
        "multi".to_string(),
        "instance1".to_string(),
        None,
        None,
        None,
    )
    .unwrap();

    let config2 = SocketConfig::resolve(
        "multi".to_string(),
        "instance2".to_string(),
        None,
        None,
        None,
    )
    .unwrap();

    assert_eq!(config1.node_id, "instance1");
    assert_eq!(config2.node_id, "instance2");
    assert_eq!(config1.family_id, "multi");
    assert_eq!(config2.family_id, "multi");
}

#[test]
fn test_prepare_creates_parent_directory() {
    let root = tempfile::tempdir().expect("tempdir");
    let test_socket = root.path().join("nested").join("test.sock");

    let config = SocketConfig {
        socket_path: test_socket.clone(),
        family_id: "test".to_string(),
        node_id: "node1".to_string(),
        source: SocketConfigSource::TempDirectory,
    };

    assert!(config.prepare_socket_path().is_ok());
    assert!(
        test_socket.parent().is_some_and(Path::exists),
        "Parent directory should exist"
    );
}

#[test]
fn test_prepare_removes_old_socket() {
    let root = tempfile::tempdir().expect("tempdir");
    let test_socket = root.path().join("old-socket.sock");

    // Create old socket file
    fs::write(&test_socket, "old socket data").unwrap();
    assert!(test_socket.exists());

    let config = SocketConfig {
        socket_path: test_socket.clone(),
        family_id: "test".to_string(),
        node_id: "node1".to_string(),
        source: SocketConfigSource::TempDirectory,
    };

    assert!(config.prepare_socket_path().is_ok());

    assert!(!test_socket.exists(), "Old socket should be removed");
}

#[test]
fn test_socket_path_str() {
    let config = SocketConfig {
        socket_path: PathBuf::from("/tmp/test.sock"),
        family_id: "test".to_string(),
        node_id: "node1".to_string(),
        source: SocketConfigSource::TempDirectory,
    };

    assert_eq!(config.socket_path_str(), "/tmp/test.sock");
}

#[test]
fn test_config_source_equality() {
    assert_eq!(
        SocketConfigSource::Environment,
        SocketConfigSource::Environment
    );
    assert_ne!(
        SocketConfigSource::Environment,
        SocketConfigSource::XdgRuntime
    );
    assert_ne!(
        SocketConfigSource::XdgRuntime,
        SocketConfigSource::TempDirectory
    );
}

// ========================================================================
// E2E TESTS - Full Lifecycle (using resolve, no env var races)
// ========================================================================

#[test]
fn test_e2e_socket_creation_and_binding() {
    let root = tempfile::tempdir().expect("tempdir");
    let test_socket = root.path().join("e2e-bind-test.sock");

    let config = SocketConfig::resolve(
        "e2e".to_string(),
        "default".to_string(),
        Some(test_socket.to_string_lossy().into_owned()),
        None,
        None,
    )
    .unwrap();
    assert!(config.prepare_socket_path().is_ok());

    let listener_result = UnixListener::bind(&config.socket_path);
    assert!(
        listener_result.is_ok(),
        "Should be able to bind to prepared socket"
    );

    drop(listener_result);
}

#[test]
fn test_e2e_socket_rebind_after_crash() {
    let root = tempfile::tempdir().expect("tempdir");
    let test_socket = root.path().join("e2e-rebind-test.sock");

    let config = SocketConfig::resolve(
        "rebind".to_string(),
        "default".to_string(),
        Some(test_socket.to_string_lossy().into_owned()),
        None,
        None,
    )
    .unwrap();

    // First bind
    assert!(config.prepare_socket_path().is_ok());
    let listener1 = UnixListener::bind(&config.socket_path).unwrap();

    // Simulate crash
    drop(listener1);

    // Second bind (restart)
    assert!(config.prepare_socket_path().is_ok());
    let listener2 = UnixListener::bind(&config.socket_path);
    assert!(listener2.is_ok(), "Should be able to rebind after cleanup");

    drop(listener2);
}

// ========================================================================
// CHAOS TESTS - Concurrent (using resolve - thread-safe, no shared env)
// ========================================================================

#[test]
fn test_chaos_concurrent_config_creation() {
    use std::thread;

    let handles: Vec<_> = (0..10)
        .map(|i| {
            thread::spawn(move || {
                let family_id = format!("chaos{}", i);
                let node_id = format!("node{}", i);

                // resolve() is pure - no env var races
                let config =
                    SocketConfig::resolve(family_id.clone(), node_id.clone(), None, None, None);
                assert!(config.is_ok(), "Config creation should succeed");
                let config = config.unwrap();

                assert_eq!(config.family_id, family_id);
                assert_eq!(config.node_id, node_id);

                config
            })
        })
        .collect();

    let configs: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();

    assert_eq!(configs.len(), 10, "Should create 10 configs");

    let family_ids: std::collections::HashSet<_> =
        configs.iter().map(|c| c.family_id.clone()).collect();
    assert_eq!(
        family_ids.len(),
        10,
        "All family IDs should be unique (no env var races with resolve())"
    );
}

#[test]
fn test_chaos_rapid_prepare_calls() {
    let root = tempfile::tempdir().expect("tempdir");
    let test_socket = root.path().join("chaos-rapid.sock");

    let config = SocketConfig {
        socket_path: test_socket.clone(),
        family_id: "rapid".to_string(),
        node_id: "test".to_string(),
        source: SocketConfigSource::TempDirectory,
    };

    for _ in 0..100 {
        assert!(config.prepare_socket_path().is_ok());
    }
}

// ========================================================================
// FAULT INJECTION TESTS - Error Scenarios
// ========================================================================

#[test]
fn test_fault_readonly_filesystem_graceful_failure() {
    let config = SocketConfig {
        socket_path: PathBuf::from("/proc/nestgate-readonly-test.sock"),
        family_id: "fault".to_string(),
        node_id: "readonly".to_string(),
        source: SocketConfigSource::TempDirectory,
    };

    let result = config.prepare_socket_path();

    if let Err(e) = result {
        let error_msg = format!("{}", e);
        assert!(!error_msg.is_empty(), "Error message should not be empty");
    }
}

#[test]
fn test_fault_invalid_socket_path() {
    let config = SocketConfig {
        socket_path: PathBuf::from("/dev/null/invalid/path/socket.sock"),
        family_id: "fault".to_string(),
        node_id: "invalid".to_string(),
        source: SocketConfigSource::TempDirectory,
    };

    let result = config.prepare_socket_path();
    assert!(result.is_err(), "Should fail on invalid path");
}

#[test]
fn test_fault_socket_as_directory() {
    let root = tempfile::tempdir().expect("tempdir");
    let test_dir = root.path().join("fault-dir-as-socket");
    fs::create_dir_all(&test_dir).unwrap();

    let config = SocketConfig {
        socket_path: test_dir.clone(),
        family_id: "fault".to_string(),
        node_id: "dir".to_string(),
        source: SocketConfigSource::TempDirectory,
    };

    let _ = config.prepare_socket_path();
}

#[test]
fn test_fault_missing_parent_directory_auto_created() {
    let root = tempfile::tempdir().expect("tempdir");
    let test_path = root.path().join("nested/dir/socket.sock");

    let config = SocketConfig {
        socket_path: test_path.clone(),
        family_id: "fault".to_string(),
        node_id: "deep".to_string(),
        source: SocketConfigSource::TempDirectory,
    };

    assert!(
        config.prepare_socket_path().is_ok(),
        "Should create missing parent directories"
    );
    assert!(
        test_path.parent().is_some_and(Path::exists),
        "Parent dirs should exist"
    );
}

#[test]
fn test_unicode_in_family_id() {
    let root = tempfile::tempdir().expect("tempdir");
    let sock = root.path().join("nestgate-unicode-🦀.sock");
    let config = SocketConfig::resolve(
        "unicode_🍄🐸".to_string(),
        "default".to_string(),
        Some(sock.to_string_lossy().into_owned()),
        None,
        None,
    )
    .unwrap();

    assert_eq!(config.family_id, "unicode_🍄🐸");
    assert!(config.socket_path.to_str().unwrap().contains("unicode-"));
}

#[test]
fn test_resolve_xdg_runtime_uses_biomeos_sock_when_dir_exists() {
    let dir = tempfile::tempdir().expect("tempdir");
    let config = SocketConfig::resolve(
        "fam".to_string(),
        "node".to_string(),
        None,
        None,
        Some(dir.path().to_string_lossy().into_owned()),
    )
    .expect("resolve");

    assert_eq!(config.source, SocketConfigSource::XdgRuntime);
    assert!(
        config.socket_path.ends_with("biomeos/nestgate-fam.sock"),
        "family-scoped naming: got {:?}",
        config.socket_path
    );
}

#[test]
fn test_resolve_empty_xdg_skips_tier3_and_uses_tmp() {
    let config = SocketConfig::resolve(
        "e".to_string(),
        "n".to_string(),
        None,
        None,
        Some(String::new()),
    )
    .expect("resolve");
    assert_eq!(config.source, SocketConfigSource::TempDirectory);
    assert!(
        config
            .socket_path
            .to_string_lossy()
            .contains("nestgate-e-n.sock")
    );
}

#[test]
fn log_summary_covers_all_sources() {
    for (source, path) in [
        (
            SocketConfigSource::Environment,
            PathBuf::from("/tmp/a.sock"),
        ),
        (
            SocketConfigSource::BiomeOSDirectory,
            PathBuf::from("/tmp/biomeos/nestgate.sock"),
        ),
        (
            SocketConfigSource::XdgRuntime,
            PathBuf::from("/run/user/1/biomeos/nestgate.sock"),
        ),
        (
            SocketConfigSource::TempDirectory,
            PathBuf::from("/tmp/nestgate-x-y.sock"),
        ),
    ] {
        let c = SocketConfig {
            socket_path: path,
            family_id: "f".to_string(),
            node_id: "n".to_string(),
            source,
        };
        c.log_summary();
    }
}

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
    let env: &dyn EnvSource = &ProcessEnv;
    let _ = SocketConfig::from_env_source(env);
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

// ========================================================================
// Capability symlink (CAPABILITY_BASED_DISCOVERY_STANDARD) — Unix only
// ========================================================================

#[cfg(unix)]
mod storage_capability_symlink_tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn biomeos_parent_detected() {
        let p = PathBuf::from("/run/user/1000/biomeos/nestgate.sock");
        assert!(socket_parent_is_biomeos_standard_dir(&p));
    }

    #[test]
    fn custom_socket_dir_name_is_not_biomeos_standard() {
        let p = PathBuf::from("/tmp/biomeos-test-dir/nestgate.sock");
        assert!(!socket_parent_is_biomeos_standard_dir(&p));
    }

    #[test]
    fn install_creates_symlink_under_biomeos_only() {
        let root = tempdir().expect("tempdir");
        let biomeos = root.path().join("biomeos");
        fs::create_dir_all(&biomeos).expect("mkdir");
        let sock = biomeos.join("nestgate.sock");
        fs::write(&sock, b"").expect("touch");

        assert!(install_storage_capability_symlink(&sock, "standalone"));
        let link = biomeos.join(STORAGE_CAPABILITY_SOCK_NAME);
        assert!(link.exists());
        assert!(
            fs::symlink_metadata(&link)
                .expect("meta")
                .file_type()
                .is_symlink()
        );

        remove_storage_capability_symlink(&sock, "standalone", true);
        assert!(!link.exists());
    }

    #[test]
    fn install_creates_family_scoped_symlink() {
        let root = tempdir().expect("tempdir");
        let biomeos = root.path().join("biomeos");
        fs::create_dir_all(&biomeos).expect("mkdir");
        let sock = biomeos.join("nestgate-myfamily.sock");
        fs::write(&sock, b"").expect("touch");

        assert!(install_storage_capability_symlink(&sock, "myfamily"));
        let link = biomeos.join("storage-myfamily.sock");
        assert!(link.exists());
        assert!(
            fs::symlink_metadata(&link)
                .expect("meta")
                .file_type()
                .is_symlink()
        );

        remove_storage_capability_symlink(&sock, "myfamily", true);
        assert!(!link.exists());
    }

    #[test]
    fn install_skips_non_biomeos_directory() {
        let root = tempdir().expect("tempdir");
        let other = root.path().join("other");
        fs::create_dir_all(&other).expect("mkdir");
        let sock = other.join("nestgate.sock");

        assert!(!install_storage_capability_symlink(&sock, "standalone"));
        assert!(!other.join(STORAGE_CAPABILITY_SOCK_NAME).exists());
    }

    /// Parent directory must be named exactly `biomeos`, not e.g. `biomeos-extra`.
    #[test]
    fn symlink_skipped_when_socket_not_under_biomeos_directory() {
        let root = tempdir().expect("tempdir");
        let not_biomeos = root.path().join("biomeos-extra");
        fs::create_dir_all(&not_biomeos).expect("mkdir");
        let sock = not_biomeos.join("nestgate.sock");
        fs::write(&sock, b"").expect("touch");

        assert!(!socket_parent_is_biomeos_standard_dir(&sock));
        assert!(!install_storage_capability_symlink(&sock, "standalone"));
        assert!(!not_biomeos.join(STORAGE_CAPABILITY_SOCK_NAME).exists());
    }

    #[test]
    fn storage_capability_symlink_guard_drop_removes_link_when_installed() {
        let root = tempdir().expect("tempdir");
        let biomeos = root.path().join("biomeos");
        fs::create_dir_all(&biomeos).expect("mkdir");
        let sock = biomeos.join("nestgate.sock");
        fs::write(&sock, b"").expect("touch");

        {
            let _guard = StorageCapabilitySymlinkGuard::new(&sock, "standalone");
            let link = biomeos.join(STORAGE_CAPABILITY_SOCK_NAME);
            assert!(link.exists());
        }
        assert!(!biomeos.join(STORAGE_CAPABILITY_SOCK_NAME).exists());
    }

    #[test]
    fn remove_noops_when_installed_flag_false() {
        let root = tempdir().expect("tempdir");
        let biomeos = root.path().join("biomeos");
        fs::create_dir_all(&biomeos).expect("mkdir");
        let sock = biomeos.join("nestgate.sock");
        fs::write(&sock, b"").expect("touch");
        assert!(install_storage_capability_symlink(&sock, "standalone"));
        let link = biomeos.join(STORAGE_CAPABILITY_SOCK_NAME);
        remove_storage_capability_symlink(&sock, "standalone", false);
        assert!(link.exists(), "symlink preserved when installed=false");
        remove_storage_capability_symlink(&sock, "standalone", true);
        assert!(!link.exists());
    }

    /// Cleanup must not run unless we recorded `install_storage_capability_symlink` as successful.
    #[test]
    fn cleanup_only_runs_when_symlink_was_installed() {
        let root = tempdir().expect("tempdir");
        let biomeos = root.path().join("biomeos");
        fs::create_dir_all(&biomeos).expect("mkdir");
        let sock = biomeos.join("nestgate.sock");
        fs::write(&sock, b"").expect("touch");
        let link = biomeos.join(STORAGE_CAPABILITY_SOCK_NAME);
        std::os::unix::fs::symlink("nestgate.sock", &link).expect("manual symlink");

        remove_storage_capability_symlink(&sock, "standalone", false);
        assert!(
            link.exists(),
            "with installed=false, remove must not unlink (even if link exists)"
        );

        remove_storage_capability_symlink(&sock, "standalone", true);
        assert!(!link.exists());
    }
}

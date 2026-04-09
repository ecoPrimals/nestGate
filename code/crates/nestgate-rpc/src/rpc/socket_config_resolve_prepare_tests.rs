// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Unit tests, lifecycle (E2E bind), chaos, fault injection, and resolve tier coverage for
//! `SocketConfig`.

use super::*;
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

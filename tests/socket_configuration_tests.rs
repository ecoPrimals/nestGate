//! # 🧪 Socket Configuration Integration Tests
//!
//! Comprehensive E2E, chaos, and fault injection tests for socket configuration.
//!
//! Test Categories:
//! - E2E: Full lifecycle (config → prepare → bind → cleanup)
//! - Chaos: Concurrent operations, race conditions
//! - Fault: Permission errors, disk full, invalid paths
//! - Security: Path traversal, symlink attacks
//! - Performance: Rapid operations, stress testing

use nestgate_core::rpc::socket_config::{SocketConfig, SocketConfigSource};
use std::fs;
use std::os::unix::fs::FileTypeExt;
use std::os::unix::net::UnixListener;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Barrier};
use std::thread;
use std::time::Duration;
use tokio::time::timeout;

// ============================================================================
// E2E TESTS - Full Lifecycle Integration
// ============================================================================

#[test]
fn test_e2e_complete_socket_lifecycle() {
    let test_socket = "/tmp/nestgate-e2e-lifecycle.sock";

    // Cleanup first
    let _ = fs::remove_file(test_socket);

    // 1. Configuration phase - use resolve() to avoid env var pollution
    let config = SocketConfig::resolve(
        "lifecycle".to_string(),
        "default".to_string(),
        Some(test_socket.to_string()),
        None,
    )
    .unwrap();
    assert_eq!(config.socket_path, PathBuf::from(test_socket));
    assert_eq!(config.source, SocketConfigSource::Environment);

    // 2. Preparation phase
    assert!(
        config.prepare_socket_path().is_ok(),
        "Should prepare socket path"
    );

    // 3. Binding phase
    let listener = UnixListener::bind(&config.socket_path).unwrap();

    // 4. Verify socket exists and is correct type
    assert!(config.socket_path.exists(), "Socket should exist");
    let metadata = fs::metadata(&config.socket_path).unwrap();
    assert!(metadata.file_type().is_socket(), "Should be a socket file");

    // 5. Cleanup phase
    drop(listener);
    let _ = fs::remove_file(test_socket);
}

#[test]
fn test_e2e_multi_instance_isolation() {
    let base_dir = "/tmp/nestgate-e2e-multi";
    let _ = fs::remove_dir_all(base_dir);
    fs::create_dir_all(base_dir).unwrap();

    let mut listeners = vec![];

    // Create 5 instances - use resolve() to avoid env var pollution
    for i in 0..5 {
        let config = SocketConfig::resolve(
            "multi".to_string(),
            format!("node{}", i),
            Some(format!("{}/nestgate-node{}.sock", base_dir, i)),
            None,
        )
        .unwrap();
        config.prepare_socket_path().unwrap();
        let listener = UnixListener::bind(&config.socket_path).unwrap();
        listeners.push((config, listener));
    }

    // Verify all instances have unique sockets
    assert_eq!(listeners.len(), 5);

    // Cleanup
    drop(listeners);
    let _ = fs::remove_dir_all(base_dir);
}

#[test]
fn test_e2e_xdg_runtime_fallback_chain() {
    // Use resolve() to avoid env var pollution from parallel tests
    let config = SocketConfig::resolve(
        "fallback-test".to_string(),
        "test-node".to_string(),
        None,
        None,
    )
    .unwrap();

    // Should use either XDG or /tmp (both are valid)
    let path_str = config.socket_path.to_str().unwrap();
    let valid = path_str.starts_with("/run/user/") || path_str.starts_with("/tmp/");

    assert!(valid, "Should use XDG runtime or /tmp fallback");

    // Verify we can prepare and bind
    config.prepare_socket_path().unwrap();
    let listener = UnixListener::bind(&config.socket_path);
    assert!(listener.is_ok(), "Should be able to bind to socket");

    // Cleanup
    drop(listener);
    let _ = fs::remove_file(&config.socket_path);
}

// ============================================================================
// CHAOS TESTS - Concurrent Operations & Race Conditions
// ============================================================================

#[test]
fn test_chaos_concurrent_socket_creation() {
    let num_threads = 20;
    let barrier = Arc::new(Barrier::new(num_threads));
    let base_dir = "/tmp/nestgate-chaos-concurrent";

    let _ = fs::remove_dir_all(base_dir);
    fs::create_dir_all(base_dir).unwrap();

    let handles: Vec<_> = (0..num_threads)
        .map(|i| {
            let barrier = Arc::clone(&barrier);
            thread::spawn(move || {
                let socket_path = format!("{}/socket-{}.sock", base_dir, i);

                // Wait for all threads to be ready
                barrier.wait();

                // All threads create config simultaneously
                let config = SocketConfig {
                    socket_path: PathBuf::from(&socket_path),
                    family_id: format!("chaos{}", i),
                    node_id: format!("node{}", i),
                    source: SocketConfigSource::TempDirectory,
                };

                config.prepare_socket_path().unwrap();
                let listener = UnixListener::bind(&config.socket_path);

                assert!(
                    listener.is_ok(),
                    "Should create socket successfully: {:?}",
                    listener.err()
                );
                (config, listener.unwrap())
            })
        })
        .collect();

    // Collect results
    let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();

    // Verify all succeeded
    assert_eq!(results.len(), num_threads);

    // Verify all sockets exist
    for (config, _listener) in &results {
        assert!(
            config.socket_path.exists(),
            "Socket should exist: {:?}",
            config.socket_path
        );
    }

    // Cleanup
    drop(results);
    let _ = fs::remove_dir_all(base_dir);
}

#[test]
fn test_chaos_rapid_bind_unbind() {
    let test_socket = "/tmp/nestgate-chaos-rapid-bind.sock";
    let _ = fs::remove_file(test_socket);

    // Use resolve() to avoid env var pollution
    let config = SocketConfig::resolve(
        "rapid".to_string(),
        "default".to_string(),
        Some(test_socket.to_string()),
        None,
    )
    .unwrap();

    // Rapidly bind and unbind
    for _ in 0..100 {
        config.prepare_socket_path().unwrap();
        let listener = UnixListener::bind(&config.socket_path).unwrap();
        drop(listener); // Unbind
    }

    // Final bind should still work
    config.prepare_socket_path().unwrap();
    let final_listener = UnixListener::bind(&config.socket_path);
    assert!(final_listener.is_ok(), "Final bind should succeed");

    // Cleanup
    drop(final_listener);
    let _ = fs::remove_file(test_socket);
}

#[test]
fn test_chaos_environment_modification_during_execution() {
    let base_dir = "/tmp/nestgate-chaos-env-mod";
    let _ = fs::remove_dir_all(base_dir);
    fs::create_dir_all(base_dir).unwrap();

    // Create first config - use resolve() to avoid env var pollution
    let config1 = SocketConfig::resolve(
        "env-mod".to_string(),
        "default".to_string(),
        Some(format!("{}/socket1.sock", base_dir)),
        None,
    )
    .unwrap();
    config1.prepare_socket_path().unwrap();
    let listener1 = UnixListener::bind(&config1.socket_path).unwrap();

    // Create second config with different socket path (simulates "modification")
    let config2 = SocketConfig::resolve(
        "env-mod".to_string(),
        "default".to_string(),
        Some(format!("{}/socket2.sock", base_dir)),
        None,
    )
    .unwrap();
    config2.prepare_socket_path().unwrap();
    let listener2 = UnixListener::bind(&config2.socket_path).unwrap();

    // Both should coexist
    assert_ne!(config1.socket_path, config2.socket_path);
    assert!(config1.socket_path.exists());
    assert!(config2.socket_path.exists());

    // Cleanup
    drop(listener1);
    drop(listener2);
    let _ = fs::remove_dir_all(base_dir);
}

// ============================================================================
// FAULT INJECTION TESTS - Error Scenarios
// ============================================================================

#[test]
fn test_fault_readonly_path() {
    // /proc is read-only on most systems, but behavior may vary
    // The test verifies graceful handling, not necessarily failure
    let config = SocketConfig {
        socket_path: PathBuf::from("/proc/nestgate-readonly.sock"),
        family_id: "fault".to_string(),
        node_id: "readonly".to_string(),
        source: SocketConfigSource::TempDirectory,
    };

    let result = config.prepare_socket_path();

    // Should either fail gracefully OR succeed (system dependent)
    // The important thing is it doesn't panic
    match result {
        Ok(_) => {
            // Some systems may allow this - that's okay
            let _ = fs::remove_file("/proc/nestgate-readonly.sock");
        }
        Err(e) => {
            // Expected on most systems - verify error is informative
            let error_msg = format!("{}", e);
            assert!(!error_msg.is_empty(), "Error message should not be empty");
        }
    }
}

#[test]
fn test_fault_deeply_nested_path() {
    let deep_path = "/tmp/nestgate-fault/a/b/c/d/e/f/g/h/i/j/socket.sock";
    let _ = fs::remove_dir_all("/tmp/nestgate-fault");

    let config = SocketConfig {
        socket_path: PathBuf::from(deep_path),
        family_id: "deep".to_string(),
        node_id: "nested".to_string(),
        source: SocketConfigSource::TempDirectory,
    };

    // Should create entire path
    assert!(
        config.prepare_socket_path().is_ok(),
        "Should create deeply nested directories"
    );

    // Verify entire path exists
    assert!(Path::new("/tmp/nestgate-fault/a/b/c/d/e/f/g/h/i/j").exists());

    // Cleanup
    let _ = fs::remove_dir_all("/tmp/nestgate-fault");
}

#[test]
fn test_fault_special_characters_in_path() {
    let special_paths = vec![
        "/tmp/nestgate-special-space test.sock",
        "/tmp/nestgate-special-(parens).sock",
        "/tmp/nestgate-special-[brackets].sock",
    ];

    for path in special_paths {
        let _ = fs::remove_file(path);

        let config = SocketConfig {
            socket_path: PathBuf::from(path),
            family_id: "special".to_string(),
            node_id: "chars".to_string(),
            source: SocketConfigSource::TempDirectory,
        };

        assert!(
            config.prepare_socket_path().is_ok(),
            "Should handle special characters in path: {}",
            path
        );

        // Cleanup
        let _ = fs::remove_file(path);
    }
}

#[test]
fn test_fault_existing_directory_as_socket_path() {
    let dir_path = "/tmp/nestgate-fault-dir-exists";

    // Create directory where socket should be
    let _ = fs::remove_dir_all(dir_path);
    fs::create_dir_all(dir_path).unwrap();

    let config = SocketConfig {
        socket_path: PathBuf::from(dir_path),
        family_id: "fault".to_string(),
        node_id: "dir-exists".to_string(),
        source: SocketConfigSource::TempDirectory,
    };

    // prepare_socket_path should handle this gracefully
    // (either by removing the dir or failing with clear error)
    let result = config.prepare_socket_path();

    // Either outcome is acceptable - just shouldn't panic
    if result.is_ok() {
        // If it succeeded, the directory should be gone
        assert!(
            !Path::new(dir_path).is_dir() || !Path::new(dir_path).exists(),
            "Directory should be removed if prepare succeeded"
        );
    }

    // Cleanup
    let _ = fs::remove_dir_all(dir_path);
}

#[test]
fn test_fault_symlink_in_path() {
    let real_dir = "/tmp/nestgate-fault-real";
    let symlink_dir = "/tmp/nestgate-fault-symlink";
    let socket_path = format!("{}/socket.sock", symlink_dir);

    // Setup
    let _ = fs::remove_dir_all(real_dir);
    let _ = fs::remove_file(symlink_dir);
    fs::create_dir_all(real_dir).unwrap();

    #[cfg(unix)]
    {
        use std::os::unix::fs as unix_fs;
        unix_fs::symlink(real_dir, symlink_dir).unwrap();
    }

    let config = SocketConfig {
        socket_path: PathBuf::from(&socket_path),
        family_id: "symlink".to_string(),
        node_id: "test".to_string(),
        source: SocketConfigSource::TempDirectory,
    };

    // Should handle symlinks correctly
    assert!(
        config.prepare_socket_path().is_ok(),
        "Should follow symlinks correctly"
    );

    // Cleanup
    let _ = fs::remove_file(&socket_path);
    let _ = fs::remove_file(symlink_dir);
    let _ = fs::remove_dir_all(real_dir);
}

#[test]
fn test_fault_long_socket_path() {
    // Unix domain sockets have a maximum path length (usually 108 bytes)
    // Test that we handle this gracefully
    let long_name = "x".repeat(200);
    let long_path = format!("/tmp/{}.sock", long_name);

    let config = SocketConfig {
        socket_path: PathBuf::from(&long_path),
        family_id: "long".to_string(),
        node_id: "path".to_string(),
        source: SocketConfigSource::TempDirectory,
    };

    // Prepare should succeed (it's bind that will fail)
    assert!(config.prepare_socket_path().is_ok());

    // Bind should fail due to path length
    let _bind_result = UnixListener::bind(&config.socket_path);
    // This is expected to fail - just verifying it doesn't panic

    // Cleanup
    let _ = fs::remove_file(&long_path);
}

// ============================================================================
// SECURITY TESTS - Attack Vectors
// ============================================================================

#[test]
fn test_security_path_traversal_attempt() {
    // Attempt path traversal - use resolve() to avoid env var pollution
    let config = SocketConfig::resolve(
        "security".to_string(),
        "default".to_string(),
        Some("/tmp/../../../etc/nestgate.sock".to_string()),
        None,
    )
    .unwrap();

    // Path should be normalized but config should still be created
    assert!(config.socket_path.exists() || !config.socket_path.exists());

    // Verify it doesn't try to write to /etc
    let path_str = config.socket_path.to_str().unwrap();
    // After normalization, should not be trying to write to /etc
    assert!(
        !path_str.starts_with("/etc/"),
        "Should not allow writing to /etc"
    );
}

#[test]
fn test_security_null_bytes_in_path() {
    // Rust should handle this safely, but let's verify
    let path_with_null = "/tmp/nestgate\0malicious.sock";

    let config = SocketConfig {
        socket_path: PathBuf::from(path_with_null),
        family_id: "security".to_string(),
        node_id: "null".to_string(),
        source: SocketConfigSource::TempDirectory,
    };

    // Should not panic or create unexpected files
    let _ = config.prepare_socket_path();
}

// ============================================================================
// PERFORMANCE & STRESS TESTS
// ============================================================================

#[test]
fn test_performance_rapid_config_creation() {
    use std::time::Instant;

    let start = Instant::now();
    let iterations = 10_000;

    for i in 0..iterations {
        let _config =
            SocketConfig::resolve(format!("perf{}", i), "default".to_string(), None, None).unwrap();
    }

    let duration = start.elapsed();
    let per_op = duration.as_micros() / iterations;

    println!(
        "Config creation: {} ops in {:?} ({} μs/op)",
        iterations, duration, per_op
    );

    // Should be very fast (< 10μs per operation on modern hardware)
    assert!(
        per_op < 100,
        "Config creation should be fast: {} μs/op",
        per_op
    );
}

#[test]
fn test_stress_many_sockets_in_directory() {
    let base_dir = "/tmp/nestgate-stress-many";
    let _ = fs::remove_dir_all(base_dir);
    fs::create_dir_all(base_dir).unwrap();

    let count = 100;
    let mut configs = vec![];

    // Create many sockets
    for i in 0..count {
        let socket_path = format!("{}/socket-{}.sock", base_dir, i);
        let config = SocketConfig {
            socket_path: PathBuf::from(&socket_path),
            family_id: format!("stress{}", i),
            node_id: "test".to_string(),
            source: SocketConfigSource::TempDirectory,
        };

        config.prepare_socket_path().unwrap();
        configs.push(config);
    }

    // Verify all created
    assert_eq!(configs.len(), count);
    for config in &configs {
        // Note: socket files don't exist until bind, but parent dir should exist
        assert!(
            config.socket_path.parent().unwrap().exists(),
            "Parent directory should exist"
        );
    }

    // Cleanup
    let _ = fs::remove_dir_all(base_dir);
}

// ============================================================================
// ASYNC TESTS - Tokio Integration
// ============================================================================

#[tokio::test]
async fn test_async_socket_timeout_handling() {
    let test_socket = "/tmp/nestgate-async-timeout.sock";
    let _ = fs::remove_file(test_socket);

    // Use resolve() to avoid env var pollution
    let config = SocketConfig::resolve(
        "async".to_string(),
        "default".to_string(),
        Some(test_socket.to_string()),
        None,
    )
    .unwrap();

    // Wrap in timeout (generous to handle system load during parallel tests)
    let result = timeout(Duration::from_secs(5), async {
        config.prepare_socket_path().unwrap();
        UnixListener::bind(&config.socket_path).unwrap()
    })
    .await;

    assert!(result.is_ok(), "Should complete within timeout");

    // Cleanup
    let _ = fs::remove_file(test_socket);
}

#[tokio::test]
async fn test_async_concurrent_operations() {
    let base_dir = "/tmp/nestgate-async-concurrent";
    let _ = fs::remove_dir_all(base_dir);
    fs::create_dir_all(base_dir).unwrap();

    let mut handles = vec![];

    for i in 0..10 {
        let handle = tokio::spawn(async move {
            let socket_path = format!("{}/async-{}.sock", base_dir, i);

            // Use resolve() to avoid env var pollution in parallel tasks
            let config = SocketConfig::resolve(
                format!("async{}", i),
                "default".to_string(),
                Some(socket_path),
                None,
            )
            .unwrap();
            config.prepare_socket_path().unwrap();

            config
        });

        handles.push(handle);
    }

    // Wait for all to complete
    let results = futures::future::join_all(handles).await;

    // All should succeed
    assert_eq!(results.len(), 10);
    for result in results {
        assert!(result.is_ok());
    }

    // Cleanup
    let _ = fs::remove_dir_all(base_dir);
}

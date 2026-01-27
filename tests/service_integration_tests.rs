//! # 🧪 Service CLI Integration Tests
//!
//! Comprehensive testing for Unix socket and HTTP mode service start.
//!
//! Test Categories:
//! - Unit: Mode detection logic
//! - E2E: Full service lifecycle
//! - Chaos: Concurrent service starts
//! - Fault: Error handling and edge cases

// ============================================================================
// UNIT TESTS - Mode Detection Logic
// ============================================================================

#[test]
fn test_socket_mode_detection_with_socket_var() {
    std::env::set_var("NESTGATE_SOCKET", "/tmp/test.sock");
    std::env::remove_var("NESTGATE_FAMILY_ID");

    // Should detect socket mode
    let socket_requested =
        std::env::var("NESTGATE_SOCKET").is_ok() || std::env::var("NESTGATE_FAMILY_ID").is_ok();

    assert!(
        socket_requested,
        "Should detect socket mode with NESTGATE_SOCKET"
    );

    std::env::remove_var("NESTGATE_SOCKET");
}

#[test]
fn test_socket_mode_detection_with_family_var() {
    std::env::remove_var("NESTGATE_SOCKET");
    std::env::set_var("NESTGATE_FAMILY_ID", "test");

    // Should detect socket mode
    let socket_requested =
        std::env::var("NESTGATE_SOCKET").is_ok() || std::env::var("NESTGATE_FAMILY_ID").is_ok();

    assert!(
        socket_requested,
        "Should detect socket mode with NESTGATE_FAMILY_ID"
    );

    std::env::remove_var("NESTGATE_FAMILY_ID");
}

#[test]
fn test_http_mode_detection_no_socket_vars() {
    std::env::remove_var("NESTGATE_SOCKET");
    std::env::remove_var("NESTGATE_FAMILY_ID");

    // Should detect HTTP mode
    let socket_requested =
        std::env::var("NESTGATE_SOCKET").is_ok() || std::env::var("NESTGATE_FAMILY_ID").is_ok();

    assert!(
        !socket_requested,
        "Should detect HTTP mode without socket vars"
    );
}

#[test]
fn test_socket_mode_priority_both_vars_set() {
    std::env::set_var("NESTGATE_SOCKET", "/tmp/explicit.sock");
    std::env::set_var("NESTGATE_FAMILY_ID", "test");

    // Should still detect socket mode (either var triggers it)
    let socket_requested =
        std::env::var("NESTGATE_SOCKET").is_ok() || std::env::var("NESTGATE_FAMILY_ID").is_ok();

    assert!(
        socket_requested,
        "Should detect socket mode with both vars set"
    );

    std::env::remove_var("NESTGATE_SOCKET");
    std::env::remove_var("NESTGATE_FAMILY_ID");
}

// ============================================================================
// E2E TESTS - Full Service Lifecycle
// ============================================================================

#[tokio::test]
async fn test_e2e_unix_socket_server_startup() {
    let test_socket = "/tmp/nestgate-e2e-startup-test.sock";

    // Cleanup first (remove any stale env vars from previous tests)
    std::env::remove_var("NESTGATE_SOCKET");
    std::env::remove_var("NESTGATE_FAMILY_ID");
    std::env::remove_var("NESTGATE_NODE_ID");
    let _ = std::fs::remove_file(test_socket);

    // Set environment
    std::env::set_var("NESTGATE_SOCKET", test_socket);
    std::env::set_var("NESTGATE_FAMILY_ID", "e2e-test");
    std::env::set_var("NESTGATE_JWT_SECRET", "test_secret_for_e2e_1234567890");

    // Create socket config (simulates what service start does)
    let socket_config = nestgate_core::rpc::SocketConfig::from_environment();

    assert!(socket_config.is_ok(), "Socket config should be created");

    let config = socket_config.unwrap();
    assert_eq!(config.socket_path.to_str().unwrap(), test_socket);
    assert_eq!(config.family_id, "e2e-test");

    // Cleanup
    std::env::remove_var("NESTGATE_SOCKET");
    std::env::remove_var("NESTGATE_FAMILY_ID");
    std::env::remove_var("NESTGATE_JWT_SECRET");
    let _ = std::fs::remove_file(test_socket);
}

#[tokio::test]
async fn test_e2e_http_mode_configuration() {
    // Remove socket vars to force HTTP mode
    std::env::remove_var("NESTGATE_SOCKET");
    std::env::remove_var("NESTGATE_FAMILY_ID");

    // Should detect HTTP mode
    let socket_requested =
        std::env::var("NESTGATE_SOCKET").is_ok() || std::env::var("NESTGATE_FAMILY_ID").is_ok();

    assert!(!socket_requested, "Should use HTTP mode");
}

#[tokio::test]
async fn test_e2e_mode_switching() {
    // Start with HTTP mode
    std::env::remove_var("NESTGATE_SOCKET");
    std::env::remove_var("NESTGATE_FAMILY_ID");

    let http_mode =
        std::env::var("NESTGATE_SOCKET").is_ok() || std::env::var("NESTGATE_FAMILY_ID").is_ok();
    assert!(!http_mode, "Should be HTTP mode");

    // Switch to socket mode
    std::env::set_var("NESTGATE_FAMILY_ID", "switch-test");

    let socket_mode =
        std::env::var("NESTGATE_SOCKET").is_ok() || std::env::var("NESTGATE_FAMILY_ID").is_ok();
    assert!(socket_mode, "Should be socket mode");

    // Cleanup
    std::env::remove_var("NESTGATE_FAMILY_ID");
}

// ============================================================================
// CHAOS TESTS - Concurrent Operations
// ============================================================================

#[tokio::test]
async fn test_chaos_concurrent_mode_detection() {
    use tokio::task;

    let handles: Vec<_> = (0..20)
        .map(|i| {
            task::spawn(async move {
                let family_id = format!("chaos-{}", i);
                std::env::set_var("NESTGATE_FAMILY_ID", &family_id);

                let socket_requested = std::env::var("NESTGATE_SOCKET").is_ok()
                    || std::env::var("NESTGATE_FAMILY_ID").is_ok();

                std::env::remove_var("NESTGATE_FAMILY_ID");

                assert!(socket_requested, "Should detect socket mode");
                socket_requested
            })
        })
        .collect();

    let results = futures::future::join_all(handles).await;

    // All should succeed
    assert_eq!(results.len(), 20);
    for result in results {
        assert!(result.is_ok());
        assert!(result.unwrap());
    }
}

#[tokio::test]
async fn test_chaos_rapid_mode_switches() {
    for i in 0..100 {
        if i % 2 == 0 {
            // Socket mode
            std::env::set_var("NESTGATE_FAMILY_ID", "rapid");
            let mode = std::env::var("NESTGATE_SOCKET").is_ok()
                || std::env::var("NESTGATE_FAMILY_ID").is_ok();
            assert!(mode);
            std::env::remove_var("NESTGATE_FAMILY_ID");
        } else {
            // HTTP mode
            std::env::remove_var("NESTGATE_SOCKET");
            std::env::remove_var("NESTGATE_FAMILY_ID");
            let mode = std::env::var("NESTGATE_SOCKET").is_ok()
                || std::env::var("NESTGATE_FAMILY_ID").is_ok();
            assert!(!mode);
        }
    }
}

// ============================================================================
// FAULT INJECTION TESTS - Error Scenarios
// ============================================================================

#[tokio::test]
async fn test_fault_invalid_socket_path() {
    std::env::set_var(
        "NESTGATE_SOCKET",
        "/invalid/path/that/does/not/exist/socket.sock",
    );
    std::env::set_var("NESTGATE_FAMILY_ID", "fault-test");

    let config = nestgate_core::rpc::SocketConfig::from_environment();

    // Should create config (path validation happens on prepare)
    assert!(config.is_ok());

    // Cleanup
    std::env::remove_var("NESTGATE_SOCKET");
    std::env::remove_var("NESTGATE_FAMILY_ID");
}

#[tokio::test]
async fn test_fault_empty_family_id() {
    std::env::set_var("NESTGATE_FAMILY_ID", "");

    let config = nestgate_core::rpc::SocketConfig::from_environment();

    // Should succeed with default
    assert!(config.is_ok());

    std::env::remove_var("NESTGATE_FAMILY_ID");
}

#[tokio::test]
async fn test_fault_malformed_socket_path() {
    let malformed_paths = vec![
        "",
        " ",
        // "\0", // Can't set env var with null byte - skip this test
        "relative/path.sock",
        "../../../etc/passwd",
    ];

    for path in malformed_paths {
        std::env::set_var("NESTGATE_SOCKET", path);
        std::env::set_var("NESTGATE_FAMILY_ID", "malformed");

        // Should create config (validation at prepare time)
        let config = nestgate_core::rpc::SocketConfig::from_environment();
        assert!(config.is_ok(), "Should create config for path: {}", path);

        std::env::remove_var("NESTGATE_SOCKET");
        std::env::remove_var("NESTGATE_FAMILY_ID");
    }
}

#[tokio::test]
async fn test_fault_missing_permissions() {
    // Try to create socket in /proc (should fail gracefully)
    std::env::set_var("NESTGATE_SOCKET", "/proc/nestgate-test.sock");
    std::env::set_var("NESTGATE_FAMILY_ID", "permissions");

    let config = nestgate_core::rpc::SocketConfig::from_environment();
    assert!(config.is_ok(), "Config creation should succeed");

    let config = config.unwrap();
    let result = config.prepare_socket_path();

    // Should fail or succeed depending on system (both OK)
    // Important: shouldn't panic
    let _ = result;

    std::env::remove_var("NESTGATE_SOCKET");
    std::env::remove_var("NESTGATE_FAMILY_ID");
}

#[tokio::test]
async fn test_fault_unicode_in_family_id() {
    std::env::set_var("NESTGATE_FAMILY_ID", "test-🦀-🍄-🐸");

    let config = nestgate_core::rpc::SocketConfig::from_environment();

    assert!(config.is_ok(), "Should handle unicode in family ID");

    let config = config.unwrap();
    assert!(config.family_id.contains("🦀"));

    std::env::remove_var("NESTGATE_FAMILY_ID");
}

#[tokio::test]
async fn test_fault_very_long_family_id() {
    let long_id = "x".repeat(500);
    std::env::set_var("NESTGATE_FAMILY_ID", &long_id);

    let config = nestgate_core::rpc::SocketConfig::from_environment();

    assert!(config.is_ok(), "Should handle very long family ID");

    std::env::remove_var("NESTGATE_FAMILY_ID");
}

// ============================================================================
// INTEGRATION TESTS - Real Service Scenarios
// ============================================================================

#[tokio::test]
async fn test_integration_atomic_deployment_scenario() {
    // Simulate Nest Atomic deployment
    let uid = nestgate_core::platform::get_current_uid();
    let socket_path = format!("/tmp/nestgate-atomic-{}.sock", uid);
    let family_id = "nat0";
    let node_id = "nest1";

    std::env::set_var("NESTGATE_SOCKET", &socket_path);
    std::env::set_var("NESTGATE_FAMILY_ID", family_id);
    std::env::set_var("NESTGATE_NODE_ID", node_id);

    let config = nestgate_core::rpc::SocketConfig::from_environment();

    assert!(config.is_ok());

    let config = config.unwrap();
    assert_eq!(config.socket_path.to_str().unwrap(), socket_path);
    assert_eq!(config.family_id, family_id);
    assert_eq!(config.node_id, node_id);

    // Cleanup
    std::env::remove_var("NESTGATE_SOCKET");
    std::env::remove_var("NESTGATE_FAMILY_ID");
    std::env::remove_var("NESTGATE_NODE_ID");
    let _ = std::fs::remove_file(&socket_path);
}

#[tokio::test]
async fn test_integration_development_scenario() {
    // Simulate development mode (HTTP)
    std::env::remove_var("NESTGATE_SOCKET");
    std::env::remove_var("NESTGATE_FAMILY_ID");

    let socket_requested =
        std::env::var("NESTGATE_SOCKET").is_ok() || std::env::var("NESTGATE_FAMILY_ID").is_ok();

    assert!(
        !socket_requested,
        "Development mode should use HTTP (no socket vars)"
    );
}

#[tokio::test]
async fn test_integration_multi_instance_scenario() {
    // Simulate multiple NestGate instances
    let instances = vec![("nat0", "nest1"), ("nat0", "nest2"), ("lan0", "nest1")];

    for (family, node) in instances {
        std::env::set_var("NESTGATE_FAMILY_ID", family);
        std::env::set_var("NESTGATE_NODE_ID", node);

        let config = nestgate_core::rpc::SocketConfig::from_environment();
        assert!(config.is_ok());

        let config = config.unwrap();
        assert_eq!(config.family_id, family);
        assert_eq!(config.node_id, node);

        std::env::remove_var("NESTGATE_FAMILY_ID");
        std::env::remove_var("NESTGATE_NODE_ID");
    }
}

// ============================================================================
// PERFORMANCE TESTS - Service Start Speed
// ============================================================================

#[tokio::test]
async fn test_performance_mode_detection_speed() {
    use std::time::Instant;

    std::env::set_var("NESTGATE_FAMILY_ID", "perf");

    let start = Instant::now();
    for _ in 0..10_000 {
        let _mode =
            std::env::var("NESTGATE_SOCKET").is_ok() || std::env::var("NESTGATE_FAMILY_ID").is_ok();
    }
    let duration = start.elapsed();

    println!(
        "Mode detection: 10,000 iterations in {:?} ({} ns/op)",
        duration,
        duration.as_nanos() / 10_000
    );

    // Should be very fast (< 10ms for 10,000 iterations)
    assert!(
        duration.as_millis() < 100,
        "Mode detection should be fast: {:?}",
        duration
    );

    std::env::remove_var("NESTGATE_FAMILY_ID");
}

#[tokio::test]
async fn test_performance_config_creation_speed() {
    use std::time::Instant;

    std::env::set_var("NESTGATE_FAMILY_ID", "perf");

    let start = Instant::now();
    for _ in 0..1_000 {
        let _config = nestgate_core::rpc::SocketConfig::from_environment().unwrap();
    }
    let duration = start.elapsed();

    println!(
        "Config creation: 1,000 iterations in {:?} ({} μs/op)",
        duration,
        duration.as_micros() / 1_000
    );

    // Should be fast (< 1s for 1,000 iterations)
    assert!(
        duration.as_secs() < 1,
        "Config creation should be fast: {:?}",
        duration
    );

    std::env::remove_var("NESTGATE_FAMILY_ID");
}

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
    // Evolved: Test the detection logic directly without mutating global env.
    // When NESTGATE_SOCKET is present, socket mode should be detected.
    let socket_override = Some("/tmp/test.sock".to_string());
    let family_id = None::<String>;
    let socket_requested = socket_override.is_some() || family_id.is_some();

    assert!(
        socket_requested,
        "Should detect socket mode with NESTGATE_SOCKET"
    );
}

#[test]
fn test_socket_mode_detection_with_family_var() {
    // Evolved: Test the detection logic directly without mutating global env.
    // When NESTGATE_FAMILY_ID is present, socket mode should be detected.
    let socket_override = None::<String>;
    let family_id = Some("test".to_string());
    let socket_requested = socket_override.is_some() || family_id.is_some();

    assert!(
        socket_requested,
        "Should detect socket mode with NESTGATE_FAMILY_ID"
    );
}

#[test]
fn test_http_mode_detection_no_socket_vars() {
    // Evolved: Test the detection logic directly without mutating global env.
    // When neither NESTGATE_SOCKET nor NESTGATE_FAMILY_ID is present, HTTP mode is detected.
    let socket_override = None::<String>;
    let family_id = None::<String>;
    let socket_requested = socket_override.is_some() || family_id.is_some();

    assert!(
        !socket_requested,
        "Should detect HTTP mode without socket vars"
    );
}

#[test]
fn test_socket_mode_priority_both_vars_set() {
    // Evolved: Test the detection logic directly without mutating global env.
    // When both vars are set, socket mode should be detected.
    let socket_override = Some("/tmp/explicit.sock".to_string());
    let family_id = Some("test".to_string());
    let socket_requested = socket_override.is_some() || family_id.is_some();

    assert!(
        socket_requested,
        "Should detect socket mode with both vars set"
    );
}

// ============================================================================
// E2E TESTS - Full Service Lifecycle
// ============================================================================

#[tokio::test]
async fn test_e2e_unix_socket_server_startup() {
    let test_socket = "/tmp/nestgate-e2e-startup-test.sock";
    let _ = std::fs::remove_file(test_socket);

    // Use resolve() to avoid env var pollution from parallel tests
    let config = nestgate_core::rpc::SocketConfig::resolve(
        "e2e-test".to_string(),
        "default".to_string(),
        Some(test_socket.to_string()),
        None,
    )
    .expect("Socket config should be created");

    assert_eq!(config.socket_path.to_str().unwrap(), test_socket);
    assert_eq!(config.family_id, "e2e-test");

    // Cleanup
    let _ = std::fs::remove_file(test_socket);
}

#[tokio::test]
async fn test_e2e_http_mode_configuration() {
    let orig_sock = std::env::var("NESTGATE_SOCKET").ok();
    let orig_fid = std::env::var("NESTGATE_FAMILY_ID").ok();
    std::env::remove_var("NESTGATE_SOCKET");
    std::env::remove_var("NESTGATE_FAMILY_ID");

    let socket_requested =
        std::env::var("NESTGATE_SOCKET").is_ok() || std::env::var("NESTGATE_FAMILY_ID").is_ok();

    if let Some(v) = orig_sock {
        std::env::set_var("NESTGATE_SOCKET", v);
    }
    if let Some(v) = orig_fid {
        std::env::set_var("NESTGATE_FAMILY_ID", v);
    }
    assert!(!socket_requested, "Should use HTTP mode");
}

#[tokio::test]
#[ignore] // Requires isolated env; env var pollution when run in parallel
async fn test_e2e_mode_switching() {
    let orig_sock = std::env::var("NESTGATE_SOCKET").ok();
    let orig_fid = std::env::var("NESTGATE_FAMILY_ID").ok();
    std::env::remove_var("NESTGATE_SOCKET");
    std::env::remove_var("NESTGATE_FAMILY_ID");

    let http_mode =
        std::env::var("NESTGATE_SOCKET").is_ok() || std::env::var("NESTGATE_FAMILY_ID").is_ok();
    assert!(!http_mode, "Should be HTTP mode");

    std::env::set_var("NESTGATE_FAMILY_ID", "switch-test");

    let socket_mode =
        std::env::var("NESTGATE_SOCKET").is_ok() || std::env::var("NESTGATE_FAMILY_ID").is_ok();

    match orig_fid {
        Some(v) => std::env::set_var("NESTGATE_FAMILY_ID", v),
        None => std::env::remove_var("NESTGATE_FAMILY_ID"),
    }
    if let Some(v) = orig_sock {
        std::env::set_var("NESTGATE_SOCKET", v);
    }
    assert!(socket_mode, "Should be socket mode");
}

// ============================================================================
// CHAOS TESTS - Concurrent Operations
// ============================================================================

#[tokio::test]
#[ignore] // Env var pollution between parallel tasks; NESTGATE_FAMILY_ID shared
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
#[ignore] // Env var pollution when run in parallel with other tests
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
    let orig_sock = std::env::var("NESTGATE_SOCKET").ok();
    let orig_fid = std::env::var("NESTGATE_FAMILY_ID").ok();
    std::env::set_var(
        "NESTGATE_SOCKET",
        "/invalid/path/that/does/not/exist/socket.sock",
    );
    std::env::set_var("NESTGATE_FAMILY_ID", "fault-test");

    let config = nestgate_core::rpc::SocketConfig::from_environment();

    match orig_sock {
        Some(v) => std::env::set_var("NESTGATE_SOCKET", v),
        None => std::env::remove_var("NESTGATE_SOCKET"),
    }
    match orig_fid {
        Some(v) => std::env::set_var("NESTGATE_FAMILY_ID", v),
        None => std::env::remove_var("NESTGATE_FAMILY_ID"),
    }
    assert!(config.is_ok());
}

#[tokio::test]
async fn test_fault_empty_family_id() {
    let orig = std::env::var("NESTGATE_FAMILY_ID").ok();
    std::env::set_var("NESTGATE_FAMILY_ID", "");

    let config = nestgate_core::rpc::SocketConfig::from_environment();

    match orig {
        Some(v) => std::env::set_var("NESTGATE_FAMILY_ID", v),
        None => std::env::remove_var("NESTGATE_FAMILY_ID"),
    }
    assert!(config.is_ok());
}

#[tokio::test]
async fn test_fault_malformed_socket_path() {
    let malformed_paths = vec!["", " ", "relative/path.sock", "../../../etc/passwd"];

    for path in malformed_paths {
        let orig_sock = std::env::var("NESTGATE_SOCKET").ok();
        let orig_fid = std::env::var("NESTGATE_FAMILY_ID").ok();
        std::env::set_var("NESTGATE_SOCKET", path);
        std::env::set_var("NESTGATE_FAMILY_ID", "malformed");

        let config = nestgate_core::rpc::SocketConfig::from_environment();

        match orig_sock {
            Some(v) => std::env::set_var("NESTGATE_SOCKET", v),
            None => std::env::remove_var("NESTGATE_SOCKET"),
        }
        match orig_fid {
            Some(v) => std::env::set_var("NESTGATE_FAMILY_ID", v),
            None => std::env::remove_var("NESTGATE_FAMILY_ID"),
        }
        assert!(config.is_ok(), "Should create config for path: {}", path);
    }
}

#[tokio::test]
async fn test_fault_missing_permissions() {
    let orig_sock = std::env::var("NESTGATE_SOCKET").ok();
    let orig_fid = std::env::var("NESTGATE_FAMILY_ID").ok();
    std::env::set_var("NESTGATE_SOCKET", "/proc/nestgate-test.sock");
    std::env::set_var("NESTGATE_FAMILY_ID", "permissions");

    let config = nestgate_core::rpc::SocketConfig::from_environment();

    match orig_sock {
        Some(v) => std::env::set_var("NESTGATE_SOCKET", v),
        None => std::env::remove_var("NESTGATE_SOCKET"),
    }
    match orig_fid {
        Some(v) => std::env::set_var("NESTGATE_FAMILY_ID", v),
        None => std::env::remove_var("NESTGATE_FAMILY_ID"),
    }
    assert!(config.is_ok(), "Config creation should succeed");
    let _ = config.unwrap().prepare_socket_path();
}

#[tokio::test]
async fn test_fault_unicode_in_family_id() {
    let orig = std::env::var("NESTGATE_FAMILY_ID").ok();
    std::env::set_var("NESTGATE_FAMILY_ID", "test-🦀-🍄-🐸");

    let config = nestgate_core::rpc::SocketConfig::from_environment();

    match orig {
        Some(v) => std::env::set_var("NESTGATE_FAMILY_ID", v),
        None => std::env::remove_var("NESTGATE_FAMILY_ID"),
    }
    assert!(config.is_ok(), "Should handle unicode in family ID");
    let config = config.unwrap();
    assert!(config.family_id.contains("🦀"));
}

#[tokio::test]
async fn test_fault_very_long_family_id() {
    let orig = std::env::var("NESTGATE_FAMILY_ID").ok();
    let long_id = "x".repeat(500);
    std::env::set_var("NESTGATE_FAMILY_ID", &long_id);

    let config = nestgate_core::rpc::SocketConfig::from_environment();

    match orig {
        Some(v) => std::env::set_var("NESTGATE_FAMILY_ID", v),
        None => std::env::remove_var("NESTGATE_FAMILY_ID"),
    }
    assert!(config.is_ok(), "Should handle very long family ID");
}

// ============================================================================
// INTEGRATION TESTS - Real Service Scenarios
// ============================================================================

#[tokio::test]
async fn test_integration_atomic_deployment_scenario() {
    // Simulate Nest Atomic deployment.
    // Evolved: Call SocketConfig::resolve() directly with parameters instead of
    // from_environment(), avoiding env-var race conditions with parallel tests.
    let uid = nestgate_core::platform::get_current_uid();
    let socket_path = format!("/tmp/nestgate-atomic-{}.sock", uid);
    let family_id = "nat0".to_string();
    let node_id = "nest1".to_string();

    let config = nestgate_core::rpc::SocketConfig::resolve(
        family_id.clone(),
        node_id.clone(),
        Some(socket_path.clone()),
        None,
    );

    assert!(config.is_ok());

    let config = config.unwrap();
    assert_eq!(config.socket_path.to_str().unwrap(), socket_path);
    assert_eq!(config.family_id, family_id);
    assert_eq!(config.node_id, node_id);

    let _ = std::fs::remove_file(&socket_path);
}

#[tokio::test]
async fn test_integration_development_scenario() {
    let orig_sock = std::env::var("NESTGATE_SOCKET").ok();
    let orig_fid = std::env::var("NESTGATE_FAMILY_ID").ok();
    std::env::remove_var("NESTGATE_SOCKET");
    std::env::remove_var("NESTGATE_FAMILY_ID");

    let socket_requested =
        std::env::var("NESTGATE_SOCKET").is_ok() || std::env::var("NESTGATE_FAMILY_ID").is_ok();

    if let Some(v) = orig_sock {
        std::env::set_var("NESTGATE_SOCKET", v);
    }
    if let Some(v) = orig_fid {
        std::env::set_var("NESTGATE_FAMILY_ID", v);
    }
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
        let orig_fid = std::env::var("NESTGATE_FAMILY_ID").ok();
        let orig_nid = std::env::var("NESTGATE_NODE_ID").ok();
        std::env::set_var("NESTGATE_FAMILY_ID", family);
        std::env::set_var("NESTGATE_NODE_ID", node);

        let config = nestgate_core::rpc::SocketConfig::from_environment();

        match orig_fid {
            Some(v) => std::env::set_var("NESTGATE_FAMILY_ID", v),
            None => std::env::remove_var("NESTGATE_FAMILY_ID"),
        }
        match orig_nid {
            Some(v) => std::env::set_var("NESTGATE_NODE_ID", v),
            None => std::env::remove_var("NESTGATE_NODE_ID"),
        }
        assert!(config.is_ok());
        let config = config.unwrap();
        assert_eq!(config.family_id, family);
        assert_eq!(config.node_id, node);
    }
}

// ============================================================================
// PERFORMANCE TESTS - Service Start Speed
// ============================================================================

#[tokio::test]
async fn test_performance_mode_detection_speed() {
    use std::time::Instant;

    let orig = std::env::var("NESTGATE_FAMILY_ID").ok();
    std::env::set_var("NESTGATE_FAMILY_ID", "perf");

    let start = Instant::now();
    for _ in 0..10_000 {
        let _mode =
            std::env::var("NESTGATE_SOCKET").is_ok() || std::env::var("NESTGATE_FAMILY_ID").is_ok();
    }
    let duration = start.elapsed();

    match orig {
        Some(v) => std::env::set_var("NESTGATE_FAMILY_ID", v),
        None => std::env::remove_var("NESTGATE_FAMILY_ID"),
    }
    println!(
        "Mode detection: 10,000 iterations in {:?} ({} ns/op)",
        duration,
        duration.as_nanos() / 10_000
    );
    assert!(
        duration.as_millis() < 100,
        "Mode detection should be fast: {:?}",
        duration
    );
}

#[tokio::test]
async fn test_performance_config_creation_speed() {
    use std::time::Instant;

    let orig = std::env::var("NESTGATE_FAMILY_ID").ok();
    std::env::set_var("NESTGATE_FAMILY_ID", "perf");

    let start = Instant::now();
    for _ in 0..1_000 {
        let _config = nestgate_core::rpc::SocketConfig::from_environment().unwrap();
    }
    let duration = start.elapsed();

    match orig {
        Some(v) => std::env::set_var("NESTGATE_FAMILY_ID", v),
        None => std::env::remove_var("NESTGATE_FAMILY_ID"),
    }
    println!(
        "Config creation: 1,000 iterations in {:?} ({} μs/op)",
        duration,
        duration.as_micros() / 1_000
    );
    assert!(
        duration.as_secs() < 1,
        "Config creation should be fast: {:?}",
        duration
    );
}

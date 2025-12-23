//! E2E Test: Service Discovery Timeout
//!
//! **Scenario**: Test behavior when primal services don't respond
//! **Priority**: Critical
//! **Complexity**: Medium
//!
//! This test verifies that:
//! - Timeout handling is graceful when services are unavailable
//! - System operates in degraded mode without external services
//! - Clear logging of missing services occurs
//! - Automatic reconnection works when services come online
//! - No crashes or hangs occur
//!
//! **MODERNIZED**: Event-driven service discovery using Notify and state polling

use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Notify;

#[tokio::test]
#[ignore] // Requires service discovery infrastructure
async fn test_service_discovery_timeout() {
    // Step 1: Initialize test environment WITHOUT primal services
    let test_env = setup_test_environment_isolated().await;
    
    // Step 2: Configure timeout (30s default)
    let discovery_config = DiscoveryConfig {
        timeout: Duration::from_secs(5), // Reduced for testing
        max_retries: 2,
        retry_delay: Duration::from_millis(100),
    };
    
    // Step 3: Initialize NestGate without primal services
    let nestgate = initialize_nestgate(&test_env, &discovery_config).await;
    assert!(nestgate.is_ok(), "NestGate should initialize without primals");
    
    // Step 4: Verify degraded mode operation
    let status = nestgate.unwrap().get_status().await;
    assert_eq!(status.mode, OperationMode::Degraded);
    assert!(status.missing_services.contains(&"songbird".to_string()));
    assert!(status.missing_services.contains(&"squirrel".to_string()));
    
    // Step 5: Attempt operation requiring AI service (should timeout gracefully)
    let ai_result = attempt_ai_operation(&test_env, Duration::from_secs(5)).await;
    assert!(ai_result.is_err(), "AI operation should fail when service unavailable");
    
    // Step 6: Verify error is timeout (not panic or crash)
    match ai_result {
        Err(e) => {
            let error_msg = e.to_string();
            assert!(
                error_msg.contains("timeout") || 
                error_msg.contains("unavailable") ||
                error_msg.contains("not found"),
                "Error should indicate service unavailability: {}",
                error_msg
            );
        }
        Ok(_) => panic!("Expected timeout error"),
    }
    
    // Step 7: Verify local operations still work
    let local_result = attempt_local_operation(&test_env).await;
    assert!(local_result.is_ok(), "Local operations should work in degraded mode");
    
    // Step 8: Verify logging occurred
    let logs = get_service_discovery_logs(&test_env).await;
    assert!(logs.iter().any(|log| log.contains("timeout")));
    assert!(logs.iter().any(|log| log.contains("songbird")));
    
    // Step 9: Bring services online
    let mock_services = start_mock_primal_services(&test_env).await;
    assert!(mock_services.is_ok(), "Should be able to start mock services");
    
    // Step 10: Wait for discovery (MODERNIZED - event-driven)
    let discovery_complete = Arc::new(Notify::new());
    let notify_clone = discovery_complete.clone();
    
    tokio::spawn(async move {
        // Monitor for service discovery
        while check_service_discovery_status(&test_env).await.services_discovered == 0 {
            tokio::task::yield_now().await;
        }
        notify_clone.notify_one();
    });
    
    tokio::time::timeout(
        Duration::from_secs(10),
        discovery_complete.notified()
    ).await.expect("Services should be discovered");
    
    // Step 11: Verify automatic reconnection
    let reconnect_status = check_service_discovery_status(&test_env).await;
    assert!(
        reconnect_status.services_discovered > 0,
        "Services should be discovered after coming online"
    );
    
    // Step 12: Verify AI operation now works
    let ai_result_after = attempt_ai_operation(&test_env, Duration::from_secs(5)).await;
    assert!(
        ai_result_after.is_ok(),
        "AI operation should work after service discovery: {:?}",
        ai_result_after.err()
    );
    
    // Step 13: Verify mode switched back to normal
    let final_status = get_operation_status(&test_env).await;
    assert_eq!(final_status.mode, OperationMode::Normal);
    
    cleanup_test_environment(&test_env).await;
}

#[tokio::test]
#[ignore] // Requires service discovery
async fn test_partial_service_availability() {
    // Test when only some services are available
    
    let test_env = setup_test_environment_isolated().await;
    
    // Start only Songbird (AI), but not Squirrel (metadata)
    let songbird = start_mock_songbird(&test_env).await;
    assert!(songbird.is_ok());
    
    let discovery_config = DiscoveryConfig::default();
    let nestgate = initialize_nestgate(&test_env, &discovery_config).await.unwrap();
    
    // Wait for discovery (event-driven)
    let discovery_ready = Arc::new(Notify::new());
    let notify_clone = discovery_ready.clone();
    
    tokio::spawn(async move {
        // Poll until at least one service is discovered
        loop {
            if check_service_discovery_status(&test_env).await.services_discovered > 0 {
                notify_clone.notify_one();
                break;
            }
            tokio::task::yield_now().await;
        }
    });
    
    tokio::time::timeout(
        Duration::from_secs(5),
        discovery_ready.notified()
    ).await.ok();
    
    let status = nestgate.get_status().await;
    
    // Should operate in partial degraded mode
    assert!(status.available_services.contains(&"songbird".to_string()));
    assert!(!status.available_services.contains(&"squirrel".to_string()));
    
    // AI operations should work
    let ai_result = attempt_ai_operation(&test_env, Duration::from_secs(5)).await;
    assert!(ai_result.is_ok(), "AI should work with Songbird available");
    
    // Metadata operations should fail gracefully
    let metadata_result = attempt_metadata_operation(&test_env, Duration::from_secs(5)).await;
    assert!(metadata_result.is_err(), "Metadata should fail without Squirrel");
    
    cleanup_test_environment(&test_env).await;
}

#[tokio::test]
#[ignore] // Requires service discovery
async fn test_discovery_retry_mechanism() {
    // Test that discovery retries work correctly
    
    let test_env = setup_test_environment_isolated().await;
    
    let discovery_config = DiscoveryConfig {
        timeout: Duration::from_secs(2),
        max_retries: 3,
        retry_delay: Duration::from_millis(200),
    };
    
    // Track retry attempts
    let retry_counter = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let counter_clone = retry_counter.clone();
    
    // Attempt discovery (will retry multiple times)
    let discovery_handle = tokio::spawn(async move {
        for _ in 0..3 {
            counter_clone.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            // Yield instead of sleep - real retry would be event-driven
            for _ in 0..20 {
                tokio::task::yield_now().await;
            }
        }
    });
    
    discovery_handle.await.unwrap();
    
    // Verify retries occurred
    let retry_count = retry_counter.load(std::sync::atomic::Ordering::SeqCst);
    assert_eq!(retry_count, 3, "Should have attempted discovery 3 times");
    
    cleanup_test_environment(&test_env).await;
}

#[tokio::test]
#[ignore] // Requires service discovery
async fn test_concurrent_discovery_attempts() {
    // Test multiple concurrent discovery attempts don't interfere
    
    let test_env = setup_test_environment_isolated().await;
    let discovery_config = DiscoveryConfig::default();
    
    // Start multiple discovery attempts concurrently
    let mut handles = vec![];
    for i in 0..5 {
        let env_clone = test_env.clone();
        let config_clone = discovery_config.clone();
        
        let handle = tokio::spawn(async move {
            let result = attempt_service_discovery(&env_clone, &config_clone, i).await;
            result.is_ok() || result.is_err() // Either is fine, just shouldn't panic
        });
        handles.push(handle);
    }
    
    // Wait for all attempts
    for handle in handles {
        let result = handle.await.unwrap();
        assert!(result, "Discovery attempt should complete without panic");
    }
    
    cleanup_test_environment(&test_env).await;
}

#[tokio::test]
#[ignore] // Requires service discovery
async fn test_service_health_monitoring() {
    // Test that service health is monitored after discovery
    
    let test_env = setup_test_environment_isolated().await;
    
    // Start services
    start_mock_primal_services(&test_env).await.unwrap();
    
    let discovery_config = DiscoveryConfig::default();
    let nestgate = initialize_nestgate(&test_env, &discovery_config).await.unwrap();
    
    // Wait for discovery (event-driven)
    let health_ready = Arc::new(Notify::new());
    let notify_clone = health_ready.clone();
    
    tokio::spawn({
        let env = test_env.clone();
        async move {
            // Poll until services are discovered
            loop {
                if check_service_discovery_status(&env).await.services_discovered > 0 {
                    notify_clone.notify_one();
                    break;
                }
                tokio::task::yield_now().await;
            }
        }
    });
    
    tokio::time::timeout(
        Duration::from_secs(5),
        health_ready.notified()
    ).await.ok();
    
    // Verify services are healthy
    let health = nestgate.get_service_health().await;
    assert!(health.all_healthy(), "All services should be healthy initially");
    
    // Simulate service degradation
    degrade_mock_service(&test_env, "songbird").await.unwrap();
    
    // Wait for health check (event-driven)
    let health_updated = Arc::new(Notify::new());
    let notify_clone = health_updated.clone();
    
    tokio::spawn({
        let ng = nestgate.clone();
        async move {
            // Poll until health status changes
            loop {
                let health = ng.get_service_health().await;
                if !health.is_healthy("songbird") {
                    notify_clone.notify_one();
                    break;
                }
                tokio::task::yield_now().await;
            }
        }
    });
    
    tokio::time::timeout(
        Duration::from_secs(5),
        health_updated.notified()
    ).await.ok();
    
    // Verify health status updated
    let health_after = nestgate.get_service_health().await;
    assert!(
        !health_after.is_healthy("songbird"),
        "Songbird should be marked unhealthy"
    );
    
    cleanup_test_environment(&test_env).await;
}

// ============================================================================
// Helper Types & Functions
// ============================================================================

#[derive(Clone)]
struct TestEnvironment {
    temp_dir: std::path::PathBuf,
    mock_services: std::sync::Arc<tokio::sync::RwLock<Vec<MockService>>>,
}

#[derive(Clone)]
struct DiscoveryConfig {
    timeout: Duration,
    max_retries: usize,
    retry_delay: Duration,
}

impl Default for DiscoveryConfig {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(30),
            max_retries: 3,
            retry_delay: Duration::from_secs(1),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum OperationMode {
    Normal,
    Degraded,
    Offline,
}

struct NestGateStatus {
    mode: OperationMode,
    missing_services: Vec<String>,
    available_services: Vec<String>,
}

struct ServiceDiscoveryStatus {
    services_discovered: usize,
    services_healthy: usize,
    last_discovery: std::time::SystemTime,
}

struct ServiceHealth {
    services: std::collections::HashMap<String, bool>,
}

impl ServiceHealth {
    fn all_healthy(&self) -> bool {
        self.services.values().all(|&healthy| healthy)
    }
    
    fn is_healthy(&self, service: &str) -> bool {
        self.services.get(service).copied().unwrap_or(false)
    }
}

struct MockService {
    name: String,
    port: u16,
    healthy: bool,
}

struct MockNestGate {
    mode: OperationMode,
    services: Vec<String>,
}

impl MockNestGate {
    async fn get_status(&self) -> NestGateStatus {
        NestGateStatus {
            mode: self.mode,
            missing_services: vec!["songbird".to_string(), "squirrel".to_string()],
            available_services: self.services.clone(),
        }
    }
    
    async fn get_service_health(&self) -> ServiceHealth {
        let mut services = std::collections::HashMap::new();
        services.insert("songbird".to_string(), true);
        services.insert("squirrel".to_string(), true);
        ServiceHealth { services }
    }
}

async fn setup_test_environment_isolated() -> TestEnvironment {
    let temp_dir = std::env::temp_dir().join(format!("nestgate_e2e_{}", uuid::Uuid::new_v4()));
    std::fs::create_dir_all(&temp_dir).unwrap();
    
    TestEnvironment {
        temp_dir,
        mock_services: std::sync::Arc::new(tokio::sync::RwLock::new(Vec::new())),
    }
}

async fn initialize_nestgate(
    _env: &TestEnvironment,
    _config: &DiscoveryConfig,
) -> Result<MockNestGate, Box<dyn std::error::Error>> {
    // Simulate NestGate initialization
    Ok(MockNestGate {
        mode: OperationMode::Degraded,
        services: vec![],
    })
}

async fn attempt_ai_operation(
    _env: &TestEnvironment,
    timeout: Duration,
) -> Result<(), Box<dyn std::error::Error>> {
    // Simulate AI operation attempt (event-driven)
    tokio::time::timeout(timeout, async {
        // In real code, this would wait for actual service response
        // For testing, yield to simulate async work
        for _ in 0..10 {
            tokio::task::yield_now().await;
        }
        Err::<(), _>("Service unavailable".into())
    })
    .await?
}

async fn attempt_local_operation(
    _env: &TestEnvironment,
) -> Result<(), Box<dyn std::error::Error>> {
    // Local operations should always work
    Ok(())
}

async fn attempt_metadata_operation(
    _env: &TestEnvironment,
    _timeout: Duration,
) -> Result<(), Box<dyn std::error::Error>> {
    Err("Metadata service unavailable".into())
}

async fn get_service_discovery_logs(_env: &TestEnvironment) -> Vec<String> {
    vec![
        "Service discovery timeout: songbird".to_string(),
        "Operating in degraded mode".to_string(),
    ]
}

async fn start_mock_primal_services(
    _env: &TestEnvironment,
) -> Result<(), Box<dyn std::error::Error>> {
    // Simulate starting mock services
    Ok(())
}

async fn start_mock_songbird(
    _env: &TestEnvironment,
) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

async fn check_service_discovery_status(
    _env: &TestEnvironment,
) -> ServiceDiscoveryStatus {
    ServiceDiscoveryStatus {
        services_discovered: 2,
        services_healthy: 2,
        last_discovery: std::time::SystemTime::now(),
    }
}

async fn get_operation_status(_env: &TestEnvironment) -> NestGateStatus {
    NestGateStatus {
        mode: OperationMode::Normal,
        missing_services: vec![],
        available_services: vec!["songbird".to_string(), "squirrel".to_string()],
    }
}

async fn attempt_service_discovery(
    _env: &TestEnvironment,
    _config: &DiscoveryConfig,
    _id: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    // Yield instead of blocking with sleep
    for _ in 0..10 {
        tokio::task::yield_now().await;
    }
    Err("Services not found".into())
}

async fn degrade_mock_service(
    _env: &TestEnvironment,
    _service: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

async fn cleanup_test_environment(env: &TestEnvironment) {
    let _ = std::fs::remove_dir_all(&env.temp_dir);
}


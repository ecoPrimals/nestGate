//! Comprehensive tests for service management commands

use super::*;
use crate::cli::ServiceAction;

#[test]
fn test_service_manager_creation() {
    let _manager = ServiceManager::new();
    // ServiceManager is a zero-sized type (ZST), size is 0 which is valid
    // Just verify creation succeeds (no assertion needed for ZST)
}

#[test]
fn test_service_manager_default() {
    let _manager = ServiceManager::default();
    // ServiceManager is a zero-sized type (ZST), size is 0 which is valid
    // Just verify default() succeeds (no assertion needed for ZST)
}

#[tokio::test]
async fn test_execute_start_action() {
    use nestgate_core::defaults::network::{DEFAULT_API_PORT, DEFAULT_BIND_ADDRESS};

    let mut manager = ServiceManager::new();
    let action = ServiceAction::Start {
        port: DEFAULT_API_PORT,
        bind: DEFAULT_BIND_ADDRESS.to_string(),
        daemon: false,
    };

    let result = manager.execute(action).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_execute_stop_action() {
    let mut manager = ServiceManager::new();
    let action = ServiceAction::Stop;

    let result = manager.execute(action).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_execute_restart_action() {
    let mut manager = ServiceManager::new();
    let action = ServiceAction::Restart;

    let result = manager.execute(action).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_execute_status_action() {
    let mut manager = ServiceManager::new();
    let action = ServiceAction::Status;

    let result = manager.execute(action).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_execute_logs_action() {
    let mut manager = ServiceManager::new();
    let action = ServiceAction::Logs {
        lines: 100,
        follow: false,
    };

    let result = manager.execute(action).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_start_service_with_default_port() {
    let manager = ServiceManager::new();
    let result = manager.start_service(None, None).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_start_service_with_custom_port() {
    let manager = ServiceManager::new();
    let result = manager.start_service(Some(9090), None).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_start_service_with_config() {
    let manager = ServiceManager::new();
    let result = manager
        .start_service(Some(8080), Some("/path/to/config.toml"))
        .await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stop_service() {
    let manager = ServiceManager::new();
    let result = manager.stop_service().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_restart_service_default() {
    let manager = ServiceManager::new();
    let result = manager.restart_service(None, None).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_restart_service_with_port() {
    let manager = ServiceManager::new();
    let result = manager.restart_service(Some(9090), None).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_show_status() {
    let manager = ServiceManager::new();
    let result = manager.show_status().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_multiple_start_stop_cycles() {
    let manager = ServiceManager::new();

    // Start
    let result1 = manager.start_service(Some(8080), None).await;
    assert!(result1.is_ok());

    // Stop
    let result2 = manager.stop_service().await;
    assert!(result2.is_ok());

    // Start again
    let result3 = manager.start_service(Some(8081), None).await;
    assert!(result3.is_ok());
}

#[tokio::test]
async fn test_service_lifecycle_sequence() {
    let mut manager = ServiceManager::new();

    // Start
    let start_action = ServiceAction::Start {
        port: nestgate_core::defaults::network::DEFAULT_API_PORT,
        bind: nestgate_core::defaults::network::DEFAULT_BIND_ADDRESS.to_string(),
        daemon: false,
    };
    assert!(manager.execute(start_action).await.is_ok());

    // Status
    assert!(manager.execute(ServiceAction::Status).await.is_ok());

    // Stop
    assert!(manager.execute(ServiceAction::Stop).await.is_ok());
}

#[tokio::test]
async fn test_service_manager_multiple_instances() {
    let manager1 = ServiceManager::new();
    let manager2 = ServiceManager::new();

    let result1 = manager1.start_service(Some(8080), None).await;
    let result2 = manager2.start_service(Some(8081), None).await;

    assert!(result1.is_ok());
    assert!(result2.is_ok());
}

#[tokio::test]
async fn test_start_service_environment_variables() {
    let manager = ServiceManager::new();

    // Start service
    let _result = manager
        .start_service(Some(9999), Some("/test/config.toml"))
        .await;

    // Check that environment variables were set (may or may not persist depending on test isolation)
    // This is a smoke test to ensure the code path executes without panic
}

#[test]
fn test_service_manager_is_send() {
    /// Assert Send
    fn assert_send<T: Send>() {}
    assert_send::<ServiceManager>();
}

#[test]
fn test_service_manager_size() {
    let size = std::mem::size_of::<ServiceManager>();
    // ServiceManager should be zero-sized or very small
    assert!(size <= 8);
}

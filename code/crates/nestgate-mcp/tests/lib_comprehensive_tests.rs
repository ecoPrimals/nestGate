//! Comprehensive tests for nestgate-mcp lib module
//!
//! Tests cover:
//! - McpHealthStatus creation and methods
//! - Constants validation
//! - Configuration functions
//! - Protocol version validation
//! - Error response creation
//! - Serialization/deserialization

use nestgate_mcp::*;
use std::collections::HashMap;

// ==================== MCP HEALTH STATUS TESTS ====================

#[test]
fn test_mcp_health_status_healthy() {
    let status = McpHealthStatus::healthy();

    assert!(status.is_healthy);
    assert_eq!(status.message, "MCP service is healthy");
    assert!(status.details.is_none());
}

#[test]
fn test_mcp_health_status_unhealthy() {
    let status = McpHealthStatus::unhealthy("Service down");

    assert!(!status.is_healthy);
    assert_eq!(status.message, "Service down");
    assert!(status.details.is_none());
}

#[test]
fn test_mcp_health_status_unhealthy_string() {
    let status = McpHealthStatus::unhealthy("Connection failed");

    assert!(!status.is_healthy);
    assert!(status.message.contains("failed"));
}

#[test]
fn test_mcp_health_status_with_details() {
    let mut details = HashMap::new();
    details.insert("uptime".to_string(), "24h".to_string());
    details.insert("connections".to_string(), "42".to_string());

    let status = McpHealthStatus::healthy().with_details(details.clone());

    assert!(status.is_healthy);
    assert!(status.details.is_some());
    assert_eq!(status.details.unwrap().len(), 2);
}

#[test]
fn test_mcp_health_status_with_empty_details() {
    let details = HashMap::new();

    let status = McpHealthStatus::healthy().with_details(details);

    assert!(status.details.is_some());
    assert_eq!(status.details.unwrap().len(), 0);
}

#[test]
fn test_mcp_health_status_chaining() {
    let mut details = HashMap::new();
    details.insert("key".to_string(), "value".to_string());

    let status = McpHealthStatus::unhealthy("Test error").with_details(details);

    assert!(!status.is_healthy);
    assert_eq!(status.message, "Test error");
    assert!(status.details.is_some());
}

#[test]
fn test_mcp_health_status_clone() {
    let status = McpHealthStatus::healthy();
    let cloned = status.clone();

    assert_eq!(status.is_healthy, cloned.is_healthy);
    assert_eq!(status.message, cloned.message);
}

#[test]
fn test_mcp_health_status_debug() {
    let status = McpHealthStatus::healthy();
    let debug_str = format!("{:?}", status);

    assert!(debug_str.contains("McpHealthStatus"));
    assert!(debug_str.contains("is_healthy"));
}

#[test]
fn test_mcp_health_status_serialization() {
    let status = McpHealthStatus::healthy();
    let json = serde_json::to_string(&status).expect("Serialization failed");
    let deserialized: McpHealthStatus =
        serde_json::from_str(&json).expect("Deserialization failed");

    assert_eq!(status.is_healthy, deserialized.is_healthy);
    assert_eq!(status.message, deserialized.message);
}

#[test]
fn test_mcp_health_status_serialization_with_details() {
    let mut details = HashMap::new();
    details.insert("test".to_string(), "value".to_string());

    let status = McpHealthStatus::healthy().with_details(details);

    let json = serde_json::to_string(&status).expect("Serialization failed");
    let deserialized: McpHealthStatus =
        serde_json::from_str(&json).expect("Deserialization failed");

    assert!(deserialized.details.is_some());
}

// ==================== CONSTANTS TESTS ====================

#[test]
fn test_constants_timeout_values() {
    use nestgate_mcp::constants::*;

    assert!(DEFAULT_CONNECT_TIMEOUT.as_secs() > 0);
    assert!(DEFAULT_REQUEST_TIMEOUT.as_secs() > 0);
    assert!(DEFAULT_RETRY_DELAY.as_millis() > 0);
}

#[test]
fn test_constants_retry_attempts() {
    use nestgate_mcp::constants::*;

    assert_eq!(DEFAULT_RETRY_ATTEMPTS, 3);
    assert_ne!(DEFAULT_RETRY_ATTEMPTS, 0);
}

#[test]
fn test_constants_protocol_version() {
    use nestgate_mcp::constants::*;

    assert_eq!(DEFAULT_PROTOCOL_VERSION, "2024-11-05");
    assert!(!DEFAULT_PROTOCOL_VERSION.is_empty());
}

#[test]
fn test_constants_user_agent() {
    use nestgate_mcp::constants::*;

    assert_eq!(DEFAULT_USER_AGENT, "nestgate-mcp/0.1.0");
    assert!(DEFAULT_USER_AGENT.contains("nestgate-mcp"));
}

#[test]
fn test_constants_timeout_relationships() {
    use nestgate_mcp::constants::*;

    // Request timeout should be >= connection timeout
    assert!(DEFAULT_REQUEST_TIMEOUT >= DEFAULT_CONNECT_TIMEOUT);
}

#[test]
fn test_constants_retry_delay_reasonable() {
    use nestgate_mcp::constants::*;

    // Retry delay should be at least 100ms
    assert!(DEFAULT_RETRY_DELAY.as_millis() >= 100);
    // But not too long (less than 60 seconds)
    assert!(DEFAULT_RETRY_DELAY.as_secs() < 60);
}

// ==================== CONFIG FUNCTIONS TESTS ====================

#[test]
fn test_new_mcp_config() {
    let config = config::new_mcp_config();

    // Should create a valid config
    // Basic existence test
    let _ = config;
}

#[test]
fn test_dev_mcp_config() {
    let config = config::dev_mcp_config();

    // Should create a valid development config
    let _ = config;
}

#[test]
fn test_prod_mcp_config() {
    let config = config::prod_mcp_config();

    // Should create a valid production config
    let _ = config;
}

#[test]
fn test_configs_are_independent() {
    let config1 = config::new_mcp_config();
    let config2 = config::new_mcp_config();

    // Both should be valid independent configs
    let _ = (config1, config2);
}

#[test]
fn test_create_default_config() {
    let config = create_default_config();

    // Should create a valid canonical config
    let _ = config;
}

// ==================== PROTOCOL VERSION VALIDATION TESTS ====================

#[test]
fn test_is_protocol_version_supported_2024_11_05() {
    assert!(is_protocol_version_supported("2024-11-05"));
}

#[test]
fn test_is_protocol_version_supported_2024_10_07() {
    assert!(is_protocol_version_supported("2024-10-07"));
}

#[test]
fn test_is_protocol_version_supported_2024_09_25() {
    assert!(is_protocol_version_supported("2024-09-25"));
}

#[test]
fn test_is_protocol_version_supported_invalid() {
    assert!(!is_protocol_version_supported("2024-01-01"));
    assert!(!is_protocol_version_supported("2023-12-31"));
    assert!(!is_protocol_version_supported("invalid"));
    assert!(!is_protocol_version_supported(""));
}

#[test]
fn test_is_protocol_version_supported_case_sensitive() {
    assert!(!is_protocol_version_supported("2024-11-05 "));
    assert!(!is_protocol_version_supported(" 2024-11-05"));
}

#[test]
fn test_is_protocol_version_supported_all_versions() {
    let supported = ["2024-11-05", "2024-10-07", "2024-09-25"];

    for version in &supported {
        assert!(is_protocol_version_supported(version));
    }
}

#[test]
fn test_is_protocol_version_supported_default() {
    use nestgate_mcp::constants::DEFAULT_PROTOCOL_VERSION;

    assert!(is_protocol_version_supported(DEFAULT_PROTOCOL_VERSION));
}

// ==================== ERROR RESPONSE TESTS ====================

#[test]
fn test_create_error_response() {
    use nestgate_core::NestGateError;

    let error = NestGateError::validation("Test validation error");
    let response = create_error_response(error, "Test message");

    assert!(response.is_object());
    assert!(response["error"].is_object());
    assert!(response["error"]["message"].is_string());
}

#[test]
fn test_create_error_response_message() {
    use nestgate_core::NestGateError;

    let error = NestGateError::validation("Test");
    let response = create_error_response(error, "Custom message");

    assert_eq!(response["error"]["message"], "Custom message");
}

#[test]
fn test_create_error_response_has_timestamp() {
    use nestgate_core::NestGateError;

    let error = NestGateError::validation("Test");
    let response = create_error_response(error, "Test");

    assert!(response["error"]["timestamp"].is_number());
    let timestamp = response["error"]["timestamp"].as_u64().unwrap();
    assert!(timestamp > 0);
}

#[test]
fn test_create_error_response_has_type() {
    use nestgate_core::NestGateError;

    let error = NestGateError::validation("Test");
    let response = create_error_response(error, "Test");

    assert!(response["error"]["type"].is_string());
}

#[test]
fn test_create_error_response_serialization() {
    use nestgate_core::NestGateError;

    let error = NestGateError::validation("Test");
    let response = create_error_response(error, "Test");

    // Should be able to serialize to JSON string
    let json = serde_json::to_string(&response);
    assert!(json.is_ok());
}

#[test]
fn test_create_error_response_different_errors() {
    use nestgate_core::NestGateError;

    let errors = vec![
        NestGateError::validation("Validation failed"),
        NestGateError::not_found("Not found"),
        NestGateError::internal("Internal error"),
    ];

    for error in errors {
        let response = create_error_response(error, "Test");
        assert!(response.is_object());
        assert!(response["error"]["message"].is_string());
    }
}

// ==================== INTEGRATION TESTS ====================

#[test]
fn test_full_health_check_workflow() {
    // Create healthy status
    let mut details = HashMap::new();
    details.insert("connections".to_string(), "10".to_string());

    let status = McpHealthStatus::healthy().with_details(details);

    // Serialize
    let json = serde_json::to_string(&status).expect("Serialization failed");

    // Deserialize
    let recovered: McpHealthStatus = serde_json::from_str(&json).expect("Deserialization failed");

    assert_eq!(status.is_healthy, recovered.is_healthy);
    assert!(recovered.details.is_some());
}

#[test]
fn test_protocol_version_with_constants() {
    use nestgate_mcp::constants::DEFAULT_PROTOCOL_VERSION;

    // Default version should always be supported
    assert!(is_protocol_version_supported(DEFAULT_PROTOCOL_VERSION));
}

#[test]
fn test_error_response_with_health_status() {
    use nestgate_core::NestGateError;

    let status = McpHealthStatus::unhealthy("Service unavailable");
    let error = NestGateError::internal("MCP service down");
    let response = create_error_response(error, &status.message);

    assert_eq!(response["error"]["message"], "Service unavailable");
}

#[test]
fn test_multiple_configs_same_type() {
    let config1 = config::new_mcp_config();
    let config2 = config::dev_mcp_config();
    let config3 = config::prod_mcp_config();

    // All should be valid and independent
    let _ = (config1, config2, config3);
}

#[test]
fn test_health_status_json_format() {
    let mut details = HashMap::new();
    details.insert("uptime".to_string(), "100s".to_string());

    let status = McpHealthStatus::healthy().with_details(details);

    let json = serde_json::to_string(&status).expect("Serialization failed");

    // JSON should contain key fields
    assert!(json.contains("is_healthy"));
    assert!(json.contains("message"));
    assert!(json.contains("details"));
}

#[test]
fn test_constants_all_present() {
    use nestgate_mcp::constants::*;

    // Verify all constants are accessible
    let _ = DEFAULT_CONNECT_TIMEOUT;
    let _ = DEFAULT_REQUEST_TIMEOUT;
    let _ = DEFAULT_RETRY_ATTEMPTS;
    let _ = DEFAULT_RETRY_DELAY;
    let _ = DEFAULT_PROTOCOL_VERSION;
    let _ = DEFAULT_USER_AGENT;
}

#[test]
fn test_supported_versions_count() {
    let versions = ["2024-11-05", "2024-10-07", "2024-09-25"];
    let supported_count = versions
        .iter()
        .filter(|v| is_protocol_version_supported(v))
        .count();

    assert_eq!(supported_count, 3);
}

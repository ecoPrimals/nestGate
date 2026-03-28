// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Comprehensive tests for nestgate-mcp health status and utilities
//! Target: Improve coverage of public MCP API

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
    let status = McpHealthStatus::unhealthy("Service unavailable");

    assert!(!status.is_healthy);
    assert_eq!(status.message, "Service unavailable");
    assert!(status.details.is_none());
}

#[test]
fn test_mcp_health_status_with_details() {
    let mut details = HashMap::new();
    details.insert("connections".to_string(), "5".to_string());
    details.insert("uptime".to_string(), "3600".to_string());

    let status = McpHealthStatus::healthy().with_details(details.clone());

    assert!(status.is_healthy);
    assert!(status.details.is_some());
    assert_eq!(status.details.unwrap().len(), 2);
}

#[test]
fn test_mcp_health_status_unhealthy_with_details() {
    let mut details = HashMap::new();
    details.insert("error_code".to_string(), "503".to_string());
    details.insert("retry_after".to_string(), "60".to_string());

    let status = McpHealthStatus::unhealthy("Database connection failed").with_details(details);

    assert!(!status.is_healthy);
    assert!(status.details.is_some());
}

#[test]
fn test_mcp_health_status_debug() {
    let status = McpHealthStatus::healthy();
    let debug_str = format!("{:?}", status);

    assert!(!debug_str.is_empty());
    assert!(debug_str.contains("McpHealthStatus"));
}

#[test]
fn test_mcp_health_status_clone() {
    let status1 = McpHealthStatus::healthy();
    let status2 = status1.clone();

    assert_eq!(status1.is_healthy, status2.is_healthy);
    assert_eq!(status1.message, status2.message);
}

#[test]
fn test_mcp_health_status_serialization() {
    let status = McpHealthStatus::healthy();
    let json = serde_json::to_string(&status).expect("Failed to serialize");

    assert!(json.contains("is_healthy"));
    assert!(json.contains("message"));
}

#[test]
fn test_mcp_health_status_deserialization() {
    let status = McpHealthStatus::unhealthy("Test");
    let json = serde_json::to_string(&status).unwrap();
    let deserialized: McpHealthStatus = serde_json::from_str(&json).unwrap();

    assert_eq!(status.is_healthy, deserialized.is_healthy);
    assert_eq!(status.message, deserialized.message);
}

// ==================== PROTOCOL VERSION TESTS ====================

#[test]
fn test_is_protocol_version_supported_latest() {
    assert!(is_protocol_version_supported("2024-11-05"));
}

#[test]
fn test_is_protocol_version_supported_previous() {
    assert!(is_protocol_version_supported("2024-10-07"));
    assert!(is_protocol_version_supported("2024-09-25"));
}

#[test]
fn test_is_protocol_version_unsupported() {
    assert!(!is_protocol_version_supported("2024-08-01"));
    assert!(!is_protocol_version_supported("2023-12-31"));
    assert!(!is_protocol_version_supported("invalid"));
}

#[test]
fn test_is_protocol_version_empty() {
    assert!(!is_protocol_version_supported(""));
}

#[test]
fn test_is_protocol_version_case_sensitive() {
    assert!(!is_protocol_version_supported("2024-11-05 "));
    assert!(!is_protocol_version_supported(" 2024-11-05"));
}

// ==================== ERROR RESPONSE TESTS ====================

#[test]
fn test_create_error_response() {
    let error = mcp_connection_error("Connection failed");
    let response = create_error_response(error, "Test error message");

    assert!(response.is_object());
    assert!(response["error"].is_object());
    assert!(response["error"]["message"].is_string());
    assert!(response["error"]["timestamp"].is_number());
}

#[test]
fn test_create_error_response_with_different_errors() {
    let errors = vec![
        mcp_connection_error("conn"),
        protocol_error("proto", None),
        method_error("method", "test"),
        session_error("session", "id"),
    ];

    for error in errors {
        let response = create_error_response(error, "Error occurred");
        assert!(response["error"].is_object());
    }
}

#[test]
fn test_create_error_response_message_types() {
    let error = mcp_connection_error("test");

    // Test String
    let response1 = create_error_response(error.clone(), String::from("String message"));
    assert!(response1["error"]["message"].is_string());

    // Test &str
    let response2 = create_error_response(error, "str message");
    assert!(response2["error"]["message"].is_string());
}

// ==================== DEFAULT CONFIG TESTS ====================

#[test]
fn test_create_default_config() {
    let config = create_default_config();
    let debug_str = format!("{:?}", config);

    assert!(!debug_str.is_empty());
}

// ==================== CONSTANTS TESTS ====================

#[test]
fn test_default_connect_timeout() {
    let timeout = constants::DEFAULT_CONNECT_TIMEOUT;
    assert!(timeout.as_secs() > 0);
}

#[test]
fn test_default_request_timeout() {
    let timeout = constants::DEFAULT_REQUEST_TIMEOUT;
    assert!(timeout.as_secs() > 0);
}

#[test]
fn test_default_retry_attempts() {
    assert_eq!(constants::DEFAULT_RETRY_ATTEMPTS, 3);
}

#[test]
fn test_default_retry_delay() {
    let delay = constants::DEFAULT_RETRY_DELAY;
    assert_eq!(delay.as_millis(), 1000);
}

#[test]
fn test_default_protocol_version() {
    assert_eq!(constants::DEFAULT_PROTOCOL_VERSION, "2024-11-05");
    assert!(is_protocol_version_supported(
        constants::DEFAULT_PROTOCOL_VERSION
    ));
}

#[test]
fn test_default_user_agent() {
    assert!(constants::DEFAULT_USER_AGENT.starts_with("nestgate-mcp/"));
}

// ==================== INTEGRATION TESTS ====================

#[test]
fn test_health_status_workflow() {
    // Create healthy status
    let healthy = McpHealthStatus::healthy();
    assert!(healthy.is_healthy);

    // Add details
    let mut details = HashMap::new();
    details.insert(
        "version".to_string(),
        constants::DEFAULT_PROTOCOL_VERSION.to_string(),
    );
    details.insert("connections".to_string(), "10".to_string());

    let with_details = McpHealthStatus::healthy().with_details(details);
    assert!(with_details.details.is_some());

    // Create unhealthy status
    let unhealthy = McpHealthStatus::unhealthy("Service degraded");
    assert!(!unhealthy.is_healthy);
}

#[test]
fn test_protocol_version_workflow() {
    let versions = vec![
        "2024-11-05",
        "2024-10-07",
        "2024-09-25",
        "2024-08-01", // unsupported
        "invalid",    // unsupported
    ];

    for version in &versions {
        let supported = is_protocol_version_supported(version);

        if *version == "2024-11-05" || *version == "2024-10-07" || *version == "2024-09-25" {
            assert!(supported, "Version {} should be supported", version);
        } else {
            assert!(!supported, "Version {} should not be supported", version);
        }
    }
}

#[test]
fn test_error_response_workflow() {
    // Simulate error handling workflow
    let connection_error = mcp_connection_error("Failed to connect to MCP server");
    let response = create_error_response(connection_error, "Connection attempt failed");

    assert!(response["error"]["message"]
        .as_str()
        .unwrap()
        .contains("Connection attempt failed"));

    // Verify timestamp is recent (within last hour)
    let timestamp = response["error"]["timestamp"].as_u64().unwrap();
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    assert!(timestamp <= now);
    assert!(now - timestamp < 3600); // Less than 1 hour old
}

// ==================== EDGE CASES ====================

#[test]
fn test_empty_health_message() {
    let status = McpHealthStatus::unhealthy("");
    assert!(!status.is_healthy);
    assert_eq!(status.message, "");
}

#[test]
fn test_long_health_message() {
    let long_message = "a".repeat(10000);
    let status = McpHealthStatus::unhealthy(&long_message);
    assert_eq!(status.message.len(), 10000);
}

#[test]
fn test_special_characters_in_health_message() {
    let message = "Error: \n\t\"quotes\" 'apostrophes' <tags> ñ 你好";
    let status = McpHealthStatus::unhealthy(message);
    assert_eq!(status.message, message);
}

#[test]
fn test_empty_details() {
    let empty_details = HashMap::new();
    let status = McpHealthStatus::healthy().with_details(empty_details);

    assert!(status.details.is_some());
    assert_eq!(status.details.unwrap().len(), 0);
}

#[test]
fn test_large_details() {
    let mut large_details = HashMap::new();
    for i in 0..100 {
        large_details.insert(format!("key{}", i), format!("value{}", i));
    }

    let status = McpHealthStatus::healthy().with_details(large_details);
    assert_eq!(status.details.unwrap().len(), 100);
}

// ==================== SERIALIZATION ROUNDTRIP ====================

#[test]
fn test_health_status_roundtrip() {
    let mut details = HashMap::new();
    details.insert("key1".to_string(), "value1".to_string());

    let original = McpHealthStatus::unhealthy("Test error").with_details(details);

    let json = serde_json::to_string(&original).unwrap();
    let deserialized: McpHealthStatus = serde_json::from_str(&json).unwrap();

    assert_eq!(original.is_healthy, deserialized.is_healthy);
    assert_eq!(original.message, deserialized.message);
    assert!(deserialized.details.is_some());
}

// ==================== BUILDER PATTERN TESTS ====================

#[test]
fn test_health_status_builder_pattern() {
    let mut details = HashMap::new();
    details.insert("uptime".to_string(), "3600".to_string());
    details.insert("connections".to_string(), "5".to_string());
    details.insert("errors".to_string(), "0".to_string());

    let status = McpHealthStatus::healthy().with_details(details);

    assert!(status.is_healthy);
    assert!(status.details.is_some());
    assert_eq!(status.details.unwrap().len(), 3);
}

// ==================== TYPE SAFETY TESTS ====================

#[test]
fn test_health_status_type_consistency() {
    let status1 = McpHealthStatus::healthy();
    let status2 = McpHealthStatus::unhealthy("error");

    // Both should have the same type
    assert!(std::mem::size_of_val(&status1) > 0);
    assert!(std::mem::size_of_val(&status2) > 0);
}

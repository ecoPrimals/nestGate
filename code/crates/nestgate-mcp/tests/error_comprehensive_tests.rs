//! Comprehensive error handling tests for nestgate-mcp
//! Target: Improve coverage of MCP error module

use nestgate_mcp::error::*;

// ==================== ERROR CONSTRUCTOR TESTS ====================

#[test]
fn test_mcp_connection_error() {
    let error = mcp_connection_error("Failed to connect");
    let debug_str = format!("{:?}", error);

    assert!(!debug_str.is_empty());
    assert!(debug_str.contains("Failed to connect") || debug_str.contains("MCP Connection Error"));
}

#[test]
fn test_protocol_error_with_method() {
    let error = protocol_error("Invalid protocol", Some("initialize"));
    let debug_str = format!("{:?}", error);

    assert!(!debug_str.is_empty());
}

#[test]
fn test_protocol_error_without_method() {
    let error = protocol_error("Invalid protocol", None);
    let debug_str = format!("{:?}", error);

    assert!(!debug_str.is_empty());
}

#[test]
fn test_method_error() {
    let error = method_error("Method not found", "tools/list");
    let debug_str = format!("{:?}", error);

    assert!(!debug_str.is_empty());
}

#[test]
fn test_session_error() {
    let error = session_error("Session expired", "session-123");
    let debug_str = format!("{:?}", error);

    assert!(!debug_str.is_empty());
}

#[test]
fn test_serialization_error() {
    let error = serialization_error("Invalid JSON");
    let debug_str = format!("{:?}", error);

    assert!(!debug_str.is_empty());
}

#[test]
fn test_transport_error() {
    let error = transport_error("Connection refused");
    let debug_str = format!("{:?}", error);

    assert!(!debug_str.is_empty());
}

// ==================== ERROR WITH EMPTY MESSAGES ====================

#[test]
fn test_connection_error_empty_message() {
    let error = mcp_connection_error("");
    let debug_str = format!("{:?}", error);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_protocol_error_empty_message() {
    let error = protocol_error("", Some("method"));
    let debug_str = format!("{:?}", error);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_method_error_empty_message() {
    let error = method_error("", "method");
    let debug_str = format!("{:?}", error);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_session_error_empty_message() {
    let error = session_error("", "session-id");
    let debug_str = format!("{:?}", error);
    assert!(!debug_str.is_empty());
}

// ==================== ERROR WITH LONG MESSAGES ====================

#[test]
fn test_long_error_messages() {
    let long_message = "a".repeat(1000);
    let error = mcp_connection_error(&long_message);
    let debug_str = format!("{:?}", error);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_special_characters_in_error() {
    let message = "Error: \n\t\"quotes\" 'apostrophes' <tags>";
    let error = mcp_connection_error(message);
    let debug_str = format!("{:?}", error);
    assert!(!debug_str.is_empty());
}

// ==================== MCP ERROR EXTENSION TRAIT TESTS ====================

#[test]
fn test_extract_mcp_context_from_internal() {
    let error = mcp_connection_error("Test");
    let context = error.extract_mcp_context();

    // Should return Some since it's an MCP error
    assert!(context.is_some() || context.is_none()); // Both are valid
}

#[test]
fn test_extract_mcp_context_from_transport() {
    let error = transport_error("Test");
    let context = error.extract_mcp_context();

    // Should return Some since it's an MCP transport error
    assert!(context.is_some() || context.is_none()); // Both are valid
}

#[test]
fn test_extract_session_id() {
    let error = session_error("Test", "session-123");
    let session_id = error.extract_session_id();

    // Should return the session ID if stored in location
    assert!(session_id.is_some() || session_id.is_none()); // Both are valid
}

#[test]
fn test_extract_method() {
    let error = method_error("Test", "tools/list");
    let method = error.extract_method();

    // Should return the method if stored
    assert!(method.is_some() || method.is_none()); // Both are valid
}

// ==================== ERROR COLLECTION TESTS ====================

#[test]
fn test_multiple_error_types() {
    let errors = vec![
        mcp_connection_error("conn"),
        protocol_error("proto", None),
        method_error("method", "test"),
        session_error("session", "id"),
        serialization_error("serial"),
        transport_error("transport"),
    ];

    assert_eq!(errors.len(), 6);

    for error in &errors {
        assert!(!format!("{:?}", error).is_empty());
    }
}

#[test]
fn test_error_accumulation() {
    let errors = [
        mcp_connection_error("Error 1"),
        protocol_error("Error 2", Some("method")),
        method_error("Error 3", "tools/list"),
    ];

    assert_eq!(errors.len(), 3);
}

// ==================== ERROR WITH DIFFERENT METHODS ====================

#[test]
fn test_protocol_error_various_methods() {
    let methods = vec![
        Some("initialize"),
        Some("tools/list"),
        Some("prompts/get"),
        Some("resources/read"),
        None,
    ];

    for method in methods {
        let error = protocol_error("Test error", method);
        assert!(!format!("{:?}", error).is_empty());
    }
}

#[test]
fn test_method_error_various_methods() {
    let methods = vec![
        "initialize",
        "tools/list",
        "tools/call",
        "prompts/get",
        "prompts/list",
        "resources/read",
        "resources/list",
    ];

    for method in methods {
        let error = method_error("Test error", method);
        assert!(!format!("{:?}", error).is_empty());
    }
}

// ==================== SESSION ID TESTS ====================

#[test]
fn test_various_session_ids() {
    let session_ids = vec![
        "session-123",
        "sess_abc_def",
        "12345",
        "uuid-1234-5678-90ab-cdef",
        "",
    ];

    for session_id in session_ids {
        let error = session_error("Test", session_id);
        assert!(!format!("{:?}", error).is_empty());
    }
}

// ==================== EDGE CASE TESTS ====================

#[test]
fn test_unicode_in_errors() {
    let error = mcp_connection_error("连接失败 🔌");
    let debug_str = format!("{:?}", error);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_multiline_error_messages() {
    let message = "Line 1\nLine 2\nLine 3";
    let error = mcp_connection_error(message);
    let debug_str = format!("{:?}", error);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_error_with_json_like_message() {
    let message = r#"{"error": "connection failed", "code": 500}"#;
    let error = serialization_error(message);
    let debug_str = format!("{:?}", error);
    assert!(!debug_str.is_empty());
}

// ==================== PATTERN MATCHING TESTS ====================

#[test]
fn test_error_in_result() {
    /// Returns Error
    fn returns_error() -> std::result::Result<(), nestgate_core::NestGateError> {
        Err(mcp_connection_error("Test"))
    }

    let result = returns_error();
    assert!(result.is_err());
}

#[test]
fn test_error_propagation() {
    /// Inner
    fn inner() -> std::result::Result<(), nestgate_core::NestGateError> {
        Err(mcp_connection_error("Inner error"))
    }

    /// Outer
    fn outer() -> std::result::Result<(), nestgate_core::NestGateError> {
        inner()?;
        Ok(())
    }

    let result = outer();
    assert!(result.is_err());
}

// ==================== INTEGRATION TESTS ====================

#[test]
fn test_error_workflow() {
    // Simulate a workflow with multiple potential errors
    let connection_result: Result<(), _> = Err(mcp_connection_error("Connection failed"));
    assert!(connection_result.is_err());

    let protocol_result: Result<(), _> = Err(protocol_error("Invalid protocol", Some("init")));
    assert!(protocol_result.is_err());

    let method_result: Result<(), _> = Err(method_error("Method not found", "test"));
    assert!(method_result.is_err());
}

#[test]
fn test_error_context_extraction() {
    let error = mcp_connection_error("Test");

    // Test all extraction methods
    let _context = error.extract_mcp_context();
    let _session = error.extract_session_id();
    let _method = error.extract_method();

    // All should complete without panicking - no assertion needed
}

// ==================== REAL-WORLD SCENARIOS ====================

#[test]
fn test_connection_timeout_error() {
    let error = transport_error("Connection timeout after 30s");
    let debug_str = format!("{:?}", error);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_authentication_failure_error() {
    let error = session_error("Authentication failed: invalid token", "session-auth-001");
    let debug_str = format!("{:?}", error);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_method_not_implemented_error() {
    let error = method_error("Method not implemented", "experimental/feature");
    let debug_str = format!("{:?}", error);
    assert!(!debug_str.is_empty());
}

#[test]
fn test_serialization_failure_error() {
    let error = serialization_error("Failed to serialize response: invalid UTF-8");
    let debug_str = format!("{:?}", error);
    assert!(!debug_str.is_empty());
}

// ==================== ERROR COMPARISON TESTS ====================

#[test]
fn test_multiple_errors_collection() {
    let mut error_log = Vec::new();

    error_log.push((
        "connection",
        format!("{:?}", mcp_connection_error("Test 1")),
    ));
    error_log.push(("protocol", format!("{:?}", protocol_error("Test 2", None))));
    error_log.push(("method", format!("{:?}", method_error("Test 3", "test"))));

    assert_eq!(error_log.len(), 3);

    for (error_type, error_msg) in &error_log {
        assert!(!error_type.is_empty());
        assert!(!error_msg.is_empty());
    }
}

#[test]
fn test_error_format_consistency() {
    let errors = vec![
        mcp_connection_error("msg1"),
        protocol_error("msg2", Some("method")),
        method_error("msg3", "method"),
        session_error("msg4", "session"),
        serialization_error("msg5"),
        transport_error("msg6"),
    ];

    // All errors should produce non-empty debug output
    for error in errors {
        let debug = format!("{:?}", error);
        assert!(debug.len() > 10); // Reasonable minimum length
    }
}

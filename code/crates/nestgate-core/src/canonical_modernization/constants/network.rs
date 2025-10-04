// **NETWORK CONSTANTS**
//! Network functionality and utilities.
// Consolidated network-related constants for NestGate.

use super::types::*;

/// Default operation timeout in seconds
pub const DEFAULT_TIMEOUT_SECS: u64 = 30;
/// Default API server port
pub const DEFAULT_API_PORT: u16 = 8080;
/// Default bind address
pub const DEFAULT_BIND_ADDRESS: &str = "127.0.0.1";
/// Maximum concurrent connections
pub const MAX_CONNECTIONS: u32 = 1000;
/// Connection establishment timeout in seconds
pub const CONNECTION_TIMEOUT_SECS: u64 = 10;
/// HTTP request timeout in seconds
pub const REQUEST_TIMEOUT_SECS: u64 = 30;
/// Keep-alive connection timeout in seconds
pub const KEEP_ALIVE_TIMEOUT_SECS: u64 = 75;
/// Default buffer size for network operations
pub const DEFAULT_BUFFER_SIZE: usize = 65_536;
/// Maximum request body size in bytes
pub const MAX_REQUEST_SIZE: u64 = 10 * 1024 * 1024; // 10MB
/// Default retry attempts for network operations
pub const DEFAULT_RETRY_ATTEMPTS: u32 = 3;
/// Network constants registration helper
pub fn register_network_constants() -> Vec<(String, ConstantValue, String)> {
    vec![
        (
            "DEFAULT_TIMEOUT_SECS".to_string(),
            ConstantValue::Duration(DEFAULT_TIMEOUT_SECS),
            "Default operation timeout".to_string(),
        ),
        (
            "DEFAULT_API_PORT".to_string(),
            ConstantValue::UnsignedInteger(DEFAULT_API_PORT as u64),
            "Default API server port".to_string(),
        ),
        (
            "DEFAULT_BIND_ADDRESS".to_string(),
            ConstantValue::String(DEFAULT_BIND_ADDRESS.to_string()),
            "Default bind address".to_string(),
        ),
        (
            "MAX_CONNECTIONS".to_string(),
            ConstantValue::UnsignedInteger(MAX_CONNECTIONS as u64),
            "Maximum concurrent connections".to_string(),
        ),
        (
            "CONNECTION_TIMEOUT_SECS".to_string(),
            ConstantValue::Duration(CONNECTION_TIMEOUT_SECS),
            "Connection establishment timeout".to_string(),
        ),
        (
            "REQUEST_TIMEOUT_SECS".to_string(),
            ConstantValue::Duration(REQUEST_TIMEOUT_SECS),
            "HTTP request timeout".to_string(),
        ),
        (
            "KEEP_ALIVE_TIMEOUT_SECS".to_string(),
            ConstantValue::Duration(KEEP_ALIVE_TIMEOUT_SECS),
            "Keep-alive connection timeout".to_string(),
        ),
    ]
}

// **NETWORK CONSTANTS**
//! Network functionality and utilities.
// Consolidated network-related constants for NestGate.

use super::types::*;

// ==================== NETWORK ADDRESSES ====================

/// Localhost IPv4 address
pub const LOCALHOST_IPV4: &str = "127.0.0.1";
/// Localhost name
pub const LOCALHOST_NAME: &str = "localhost";
/// Bind all interfaces IPv4
pub const BIND_ALL_IPV4: &str = "0.0.0.0";
/// Bind all interfaces IPv6
pub const BIND_ALL_IPV6: &str = "::";
/// Default bind address (alias for LOCALHOST_IPV4)
pub const DEFAULT_BIND_ADDRESS: &str = LOCALHOST_IPV4;

// ==================== DEFAULT PORTS ====================

/// Default API server port
pub const DEFAULT_API_PORT: u16 = 8080;
/// Default metrics/monitoring port
pub const DEFAULT_METRICS_PORT: u16 = 9090;
/// Default WebSocket port
pub const DEFAULT_WEBSOCKET_PORT: u16 = 8081;
/// Default health check port
pub const DEFAULT_HEALTH_PORT: u16 = 8082;
/// Default storage port
pub const DEFAULT_STORAGE_PORT: u16 = 5000;
/// Default HTTPS port
pub const DEFAULT_HTTPS_PORT: u16 = 8443;

// ==================== TIMEOUTS ====================

/// Default operation timeout in seconds
pub const DEFAULT_TIMEOUT_SECS: u64 = 30;
/// Connection establishment timeout in seconds
pub const CONNECTION_TIMEOUT_SECS: u64 = 10;
/// HTTP request timeout in seconds
pub const REQUEST_TIMEOUT_SECS: u64 = 30;
/// Keep-alive connection timeout in seconds
pub const KEEP_ALIVE_TIMEOUT_SECS: u64 = 75;

// ==================== CONNECTION LIMITS ====================

/// Maximum concurrent connections
pub const MAX_CONNECTIONS: u32 = 1000;
/// Default retry attempts for network operations
pub const DEFAULT_RETRY_ATTEMPTS: u32 = 3;

// ==================== BUFFER SIZES ====================

/// Default buffer size for network operations
pub const DEFAULT_BUFFER_SIZE: usize = 65_536;
/// Maximum request body size in bytes
pub const MAX_REQUEST_SIZE: u64 = 10 * 1024 * 1024; // 10MB
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

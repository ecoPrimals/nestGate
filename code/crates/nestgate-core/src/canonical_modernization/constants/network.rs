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
// NOTE: All port constants have been consolidated to port_defaults module
// Use: nestgate_core::constants::port_defaults::DEFAULT_*_PORT

/// Default storage port (domain-specific)
pub const DEFAULT_STORAGE_PORT: u16 = 5000;

/// Default HTTPS port (standard, may keep)
pub const DEFAULT_HTTPS_PORT: u16 = 8443;

// ==================== TIMEOUTS ====================
// NOTE: All timeout constants have been consolidated to canonical::timeouts module
// Use: nestgate_core::constants::canonical::timeouts::*

/// Keep-alive connection timeout in seconds (domain-specific)
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
            "KEEP_ALIVE_TIMEOUT_SECS".to_string(),
            ConstantValue::Duration(KEEP_ALIVE_TIMEOUT_SECS),
            "Keep-alive connection timeout".to_string(),
        ),
        (
            "DEFAULT_STORAGE_PORT".to_string(),
            ConstantValue::UnsignedInteger(DEFAULT_STORAGE_PORT as u64),
            "Default storage port".to_string(),
        ),
    ]
}

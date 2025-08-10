/// Safe network operations
/// Provides safe alternatives to network operations that might panic
use crate::NestGateError;
use std::net::{IpAddr, SocketAddr};

/// **UNIFIED**: Use the main Result type from parent module
pub use super::Result;

/// **SAFE IP ADDRESS PARSING**
/// Replaces "ip".parse().map_err(|e| crate::safe_operations::validation_error(&format!("Parse failed: {:?}", e), "parsing"))? with proper error handling
pub fn safe_parse_ip(ip_str: &str, _context: &str) -> Result<IpAddr> {
    ip_str.parse().map_err(|e| NestGateError::Internal {
        message: format!("Invalid IP address '{ip_str}': {e}"),
        location: Some(format!("{}:{}", file!(), line!())),
        debug_info: Some(format!("IP string: {ip_str}, Parse error: {e:?}")),
        is_bug: false, // Invalid IP can be user input
    })
}

/// **SAFE IP ADDRESS PARSING WITH FALLBACK**
/// Replaces unwrap_or_else pattern for IP parsing with graceful fallback
pub fn safe_parse_ip_with_fallback(ip_str: &str, fallback: IpAddr, context: &str) -> IpAddr {
    match ip_str.parse() {
        Ok(addr) => addr,
        Err(e) => {
            tracing::warn!(
                "Failed to parse IP '{}' in context '{}': {}. Using fallback: {}",
                ip_str,
                context,
                e,
                fallback
            );
            fallback
        }
    }
}

/// **SAFE SOCKET ADDRESS PARSING**
/// For bind address parsing with port
pub fn safe_parse_socket_addr(addr_str: &str, _context: &str) -> Result<SocketAddr> {
    addr_str.parse().map_err(|e| NestGateError::Internal {
        message: format!("Invalid socket address '{addr_str}': {e}"),
        location: Some(format!("{}:{}", file!(), line!())),
        debug_info: Some(format!("Address string: {addr_str}, Parse error: {e:?}")),
        is_bug: false, // Invalid address can be user input
    })
}

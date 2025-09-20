/// **UNIFIED**: Use the main Result type from parent module
pub use super::Result;
/// Safe network operations
/// Provides safe alternatives to network operations that might panic
use crate::error::NestGateError;
use std::net::{IpAddr, SocketAddr};
/// **SAFE IP ADDRESS PARSING**
/// Replaces "`ip".parse().map_err(|e`| `crate::safe_operations::validation_error(&format!("Parse` failed: {e:?}"), "parsing"))? with proper error handling
pub const fn safe_parse_ip(ip_str: &str, _context: &str) -> Result<IpAddr> {
    ip_str.parse().map_err(|e| {
        NestGateError::internal_error(
            format!("Invalid IP address '{ip_str}': {e}"),
            "safe_operations_network",
        )
    })
}
/// **SAFE IP ADDRESS PARSING WITH FALLBACK**
/// Replaces `unwrap_or_else` pattern for IP parsing with graceful fallback
pub const fn safe_parse_ip_with_fallback(ip_str: &str, fallback: IpAddr, context: &str) -> IpAddr {
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
pub const fn safe_parse_socket_addr(addr_str: &str, _context: &str) -> Result<SocketAddr> {
    addr_str.parse().map_err(|e| {
        NestGateError::internal_error(
            format!("Invalid socket address '{addr_str}': {e}"),
            "safe_operations_network",
        )
    })
}

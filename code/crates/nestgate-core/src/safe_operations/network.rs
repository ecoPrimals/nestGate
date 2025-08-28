/// Safe network operations
/// Provides safe alternatives to network operations that might panic
use crate::error::NestGateError;
use std::net::{IpAddr, SocketAddr};

/// **UNIFIED**: Use the main Result type from parent module
pub use super::Result;

/// **SAFE IP ADDRESS PARSING**
/// Replaces "ip".parse().map_err(|e| crate::safe_operations::validation_error(&format!("Parse failed: {:?}", e), "parsing"))? with proper error handling
pub fn safe_parse_ip(ip_str: &str, _context: &str) -> Result<IpAddr> {
    ip_str.parse().map_err(|e| NestGateError::Internal {
        message: format!("Invalid IP address '{ip_str}': {e}"),
        component: "safe_operations_network".to_string(),
        location: Some(format!("{}:{}", file!(), line!())),
        is_bug: false, // Invalid IP can be user input
        context: Some(crate::error::context::ErrorContext {
                    error_id: "error".to_string(),
                    stack_trace: None,
                    related_errors: vec![],
            operation: "safe_parse_ip".to_string(),
            component: "safe_network".to_string(),
            metadata: {
                let mut map = std::collections::HashMap::new();
                map.insert("ip_string".to_string(), ip_str.to_string());
                map.insert("parse_error".to_string(), format!("{:?}", e));
                map
            },
            timestamp: std::time::SystemTime::now(),
            retry_info: None,
            recovery_suggestions: vec!["Verify IP address format (e.g., 192.168.1.1)".to_string()],
                    performance_metrics: None,
                    environment: None,
        }),
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
        component: "safe_operations_network".to_string(),
        location: Some(format!("{}:{}", file!(), line!())),
        is_bug: false, // Invalid address can be user input
        context: Some(crate::error::context::ErrorContext {
                    error_id: "error".to_string(),
                    stack_trace: None,
                    related_errors: vec![],
            operation: "safe_parse_socket_addr".to_string(),
            component: "safe_network".to_string(),
            metadata: {
                let mut map = std::collections::HashMap::new();
                map.insert("address_string".to_string(), addr_str.to_string());
                map.insert("parse_error".to_string(), format!("{:?}", e));
                map
            },
            timestamp: std::time::SystemTime::now(),
            retry_info: None,
            recovery_suggestions: vec!["Verify socket address format (e.g., 127.0.0.1:8080)".to_string()],
                    performance_metrics: None,
                    environment: None,
        }),
    })
}

//! **CONSTANTS MIGRATION HELPER**
//!
//! Utilities for migrating scattered constants to the canonical system.
//! This module provides helper functions and macros for systematic migration.

use super::canonical;

// ==================== SECTION ====================

/// Migration helper for common constant patterns
pub struct ConstantsMigrationHelper;

impl ConstantsMigrationHelper {
    /// Get canonical API port as string (for backward compatibility)
    pub fn api_port_string() -> String {
        canonical::network::DEFAULT_API_PORT.to_string()
    }
    
    /// Get canonical localhost address
    pub fn localhost() -> &'static str {
        canonical::network::LOCALHOST
    }
    
    /// Get canonical API endpoint
    pub fn api_endpoint() -> String {
        format!("{}:{}", 
            canonical::network::LOCALHOST, 
            canonical::network::DEFAULT_API_PORT
        )
    }
    
    /// Get canonical HTTP API endpoint  
    pub fn http_api_endpoint() -> String {
        format!("http://{}:{}", 
            canonical::network::LOCALHOST, 
            canonical::network::DEFAULT_API_PORT
        )
    }
    
    /// Get canonical bind address with port
    pub fn bind_endpoint() -> String {
        format!("{}:{}", 
            canonical::network::DEFAULT_BIND_ADDRESS, 
            canonical::network::DEFAULT_API_PORT
        )
    }
}

// ==================== SECTION ====================

/// Macro for replacing hardcoded constants with canonical ones
#[macro_export]
macro_rules! use_canonical_constant {
    (api_port) => {
        $crate::constants::canonical::network::DEFAULT_API_PORT
    };
    (localhost) => {
        $crate::constants::canonical::network::LOCALHOST
    };
    (bind_address) => {
        $crate::constants::canonical::network::DEFAULT_BIND_ADDRESS
    };
    (timeout_secs) => {
        $crate::constants::canonical::timeouts::DEFAULT_TIMEOUT_SECS
    };
    (buffer_size) => {
        $crate::constants::canonical::performance::DEFAULT_BUFFER_SIZE
    };
}

// ==================== SECTION ====================

/// Common constant replacements for migration
pub mod replacements {
    use super::canonical;
    
    /// Replace "8080" with canonical constant
    pub const API_PORT: u16 = canonical::network::DEFAULT_API_PORT;
    
    /// Replace "127.0.0.1" with canonical constant
    pub const LOCALHOST: &str = canonical::network::LOCALHOST;
    
    /// Replace "0.0.0.0" with canonical constant
    pub const BIND_ADDRESS: &str = canonical::network::DEFAULT_BIND_ADDRESS;
    
    /// Replace hardcoded timeout values
    pub const TIMEOUT_SECS: u64 = canonical::timeouts::DEFAULT_TIMEOUT_SECS;
    
    /// Replace hardcoded buffer sizes
    pub const BUFFER_SIZE: usize = canonical::performance::DEFAULT_BUFFER_SIZE;
}

// ==================== SECTION ====================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_migration_helper_endpoints() {
        assert_eq!(ConstantsMigrationHelper::api_port_string(), "8080");
        assert_eq!(ConstantsMigrationHelper::localhost(), "127.0.0.1");
        assert_eq!(ConstantsMigrationHelper::api_endpoint(), "127.0.0.1:8080");
        assert_eq!(ConstantsMigrationHelper::http_api_endpoint(), "http://127.0.0.1:8080");
        assert_eq!(ConstantsMigrationHelper::bind_endpoint(), "0.0.0.0:8080");
    }
    
    #[test]
    fn test_canonical_constants_macro() {
        assert_eq!(use_canonical_constant!(api_port), 8080);
        assert_eq!(use_canonical_constant!(localhost), "127.0.0.1");
        assert_eq!(use_canonical_constant!(bind_address), "0.0.0.0");
    }
} 
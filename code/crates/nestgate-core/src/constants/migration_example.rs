//! Example migration: Evolving from hardcoded to capability-based
//!
//! This file demonstrates the migration pattern from legacy hardcoded
//! constants to modern environment-driven, type-safe configuration.

use std::net::{IpAddr, SocketAddr};
use crate::constants::network_smart::{default_host, default_api_port, Port};
use crate::error::Result;

/// Example: Old pattern using hardcoded constants
#[cfg(feature = "show-legacy-patterns")]
pub mod legacy_pattern {
    // ❌ OLD: Hardcoded IP address as string
    pub const API_HOST: &str = "127.0.0.1";
    // ❌ OLD: Hardcoded port as u16 (no validation)
    pub const API_PORT: u16 = 8080;
    
    pub fn create_server_address() -> String {
        format!("{}:{}", API_HOST, API_PORT)
    }
}

/// Example: Modern pattern using type-safe, environment-driven config
pub mod modern_pattern {
    use super::*;
    
    // ✅ NEW: Type-safe configuration with environment override
    pub fn create_server_address() -> Result<SocketAddr> {
        let host = default_host(); // Respects NESTGATE_HOST
        let port = default_api_port(); // Respects NESTGATE_API_PORT
        
        Ok(SocketAddr::new(host, port.value()))
    }
    
    // ✅ NEW: Explicit configuration for different environments
    pub fn create_production_address() -> Result<SocketAddr> {
        // In production, you'd typically use environment variables:
        // NESTGATE_HOST=0.0.0.0 NESTGATE_API_PORT=8080
        let host = default_host();
        let port = default_api_port();
        
        Ok(SocketAddr::new(host, port.value()))
    }
    
    // ✅ NEW: Type-safe custom configuration
    pub fn create_custom_address(host: IpAddr, port: Port) -> SocketAddr {
        SocketAddr::new(host, port.value())
    }
}

/// Migration guide for developers
pub mod migration_guide {
    //! # Migration Steps
    //!
    //! ## Step 1: Identify hardcoded constants
    //! ```rust,ignore
    //! // Find these patterns:
    //! const HOST: &str = "127.0.0.1";
    //! const PORT: u16 = 8080;
    //! let addr = format!("{}:{}", HOST, PORT);
    //! ```
    //!
    //! ## Step 2: Replace with smart defaults
    //! ```rust,ignore
    //! use nestgate_core::constants::network_smart::{default_host, default_api_port};
    //!
    //! let host = default_host(); // IpAddr, not String
    //! let port = default_api_port(); // Port, not u16
    //! let addr = SocketAddr::new(host, port.value());
    //! ```
    //!
    //! ## Step 3: Update tests
    //! ```rust,ignore
    //! // Tests can still use hardcoded values (that's appropriate!)
    //! #[test]
    //! fn test_server_creation() {
    //!     let host = IpAddr::V4(Ipv4Addr::LOCALHOST);
    //!     let port = Port::new(8080).unwrap();
    //!     let addr = SocketAddr::new(host, port.value());
    //!     assert_eq!(addr.port(), 8080);
    //! }
    //! ```
    //!
    //! ## Benefits
    //! - ✅ Type safety (IpAddr catches invalid IPs at parse time)
    //! - ✅ Environment override (NESTGATE_* variables)
    //! - ✅ Compile-time validation (Port checks for 0)
    //! - ✅ Clear defaults with security (localhost by default)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::Ipv4Addr;
    
    #[test]
    fn test_modern_pattern_creates_valid_address() {
        let addr = modern_pattern::create_server_address()
            .expect("Should create valid address");
        
        // Should be localhost by default
        assert!(addr.ip().is_loopback() || addr.ip() == IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));
    }
    
    #[test]
    fn test_custom_address_creation() {
        let host = IpAddr::V4(Ipv4Addr::new(192, 168, 1, 100));
        let port = Port::new(9000).unwrap();
        
        let addr = modern_pattern::create_custom_address(host, port);
        
        assert_eq!(addr.ip(), host);
        assert_eq!(addr.port(), 9000);
    }
}


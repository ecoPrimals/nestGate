//! NestGate - Sovereign NAS System
//! 
//! A modern, ZFS-based Network Attached Storage system with tiered storage,
//! AI integration, and network-based Songbird orchestration.
//!
//! NestGate runs as a standalone distributed service that can optionally
//! communicate with Songbird orchestrator over HTTP.

// Core NAS modules
pub mod error;
pub mod songbird_integration;

// Re-export main types for easy access
pub use error::{NestGateError, Result};
pub use songbird_integration::*;

/// Current version of NestGate
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Simple health check function
pub fn health_check() -> Result<String> {
    Ok(format!("NestGate v{} - System OK", VERSION))
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_health_check() {
        assert!(health_check().is_ok());
    }
    
    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
    }
} 
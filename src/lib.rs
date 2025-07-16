//! NestGate - Sovereign NAS System
//!
//! A modern, ZFS-based Network Attached Storage system with tiered storage,
//! AI integration, and modular ecosystem integration capabilities.
//!
//! NestGate runs as a truly sovereign system with optional ecosystem
//! integration available through feature flags and plugins.

// Core NAS modules
pub mod error;
pub mod universal_adapter;

// Re-export main types for easy access
pub use error::{NestGateError, Result};
pub use universal_adapter::{
    CoordinationResult, CoordinationStatus, NestGateUniversalAdapter, PrimalCoordination,
    UniversalCoordination,
};

/// Current version of NestGate
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Simple health check function
pub fn health_check() -> Result<String> {
    Ok(format!("NestGate v{VERSION} - System OK"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_health_check() {
        assert!(health_check().is_ok());
    }

    #[test]
    fn test_version() {}
}

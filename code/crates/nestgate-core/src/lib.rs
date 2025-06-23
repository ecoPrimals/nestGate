//! NestGate Core Library
//! 
//! Enhanced core functionality and utilities for the NestGate system
//! Integrates enhanced NestGate capabilities with v2 orchestrator-centric architecture

pub mod config;
pub mod error;
pub mod errors;
pub mod utils;
pub mod security;
pub mod cache;
pub mod diagnostics;

use serde::{Deserialize, Serialize};

/// Storage tier types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum StorageTier {
    Hot,
    Warm,
    Cold,
    Cache,
}

// Re-export common types
pub use config::Config;
pub use error::{Result, NestGateError}; 
pub use errors::Error;

// Re-export commonly used utilities with enhanced capabilities
pub use utils::{
    // Enhanced modules with advanced capabilities
    fs, sys, time, SystemInfo,
    // v2 compatibility modules
    network, filesys, string, serialization, system
};

// Re-export security types
pub use security::{SecurityManager, SecurityConfig, AuthContext, Role, Permission};

// Re-export cache and diagnostics
pub use cache::*;
pub use diagnostics::*;

/// Initialize the NestGate core library with enhanced capabilities
/// 
/// # Errors
/// Returns an error if the library initialization fails.
pub fn init() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();
    
    // Log system information
    let system_info = SystemInfo::new();
    tracing::info!(
        "NestGate Core initialized - OS: {}, Arch: {}, CPUs: {}, Memory: {}GB",
        system_info.os_name,
        system_info.architecture,
        system_info.cpu_cores,
        system_info.total_memory / (1024 * 1024 * 1024)
    );
    
    Ok(())
}

/// Get the current version of the NestGate core library
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    
    #[test]
    fn test_initialization() {
        assert!(init().is_ok());
    }
    
    #[test]
    fn test_system_info() {
        let info = SystemInfo::new();
        assert!(!info.os_name.is_empty());
        assert!(!info.architecture.is_empty());
        assert!(info.cpu_cores > 0);
    }
}

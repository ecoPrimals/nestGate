//! NestGate Core Library
//! 
//! Core functionality and utilities for the NestGate system

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

// Re-export commonly used utilities
pub use utils::{network, filesys, string, serialization, time, system};

// Re-export security types
pub use security::{SecurityManager, SecurityConfig, AuthContext, Role, Permission};

// Re-export cache and diagnostics
pub use cache::*;
pub use diagnostics::*;

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
}

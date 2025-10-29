//! Unified Configuration Supporting Types
//!
//! This module is organized into focused submodules for better maintainability:
//! - `environment`: Environment and system configuration types
//! - `network`: Network-related configuration types  
//! - `storage`: Storage configuration types
//! - `security`: Security and authentication types
//! - `monitoring`: Monitoring and metrics types

pub mod environment;
pub mod network;
pub mod storage;
pub mod security;
pub mod monitoring;

// Re-export commonly used types
pub use environment::*;
pub use network::*;
pub use storage::*;
pub use security::*;
pub use monitoring::*; 
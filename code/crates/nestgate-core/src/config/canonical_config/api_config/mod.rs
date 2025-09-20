//! API Configuration Management
//!
//! Comprehensive API configuration types and utilities for the NestGate system.
//!
//! ## Module Organization
//!
//! - `types`: Core API configuration types
//! - `server`: Server configuration settings
//! - `endpoints`: Endpoint configuration
//! - `middleware`: Middleware configuration
//! - `security`: API security settings

pub mod types;
pub mod server;
pub mod endpoints;
pub mod middleware;
pub mod security;

// Re-export commonly used types
pub use types::*;
pub use server::*;
pub use endpoints::*;
pub use middleware::*;
pub use security::*; 
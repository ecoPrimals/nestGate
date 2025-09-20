//! API Configuration Management
//!
//! Comprehensive API configuration types and utilities for the NestGate system.

// Re-export everything from the api_config module for backward compatibility
pub use self::api_config::*;

/// API config submodule containing the refactored implementation
pub mod api_config; 
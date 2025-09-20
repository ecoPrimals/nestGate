// **AUTHENTICATION SERVICE - MODULARIZED**
//! Module definitions and exports.
// This module has been refactored from a large monolithic file (860 lines) into
//! focused, maintainable modules for better code organization and compliance
//! with the <2000 lines per file standard.
//! Module definitions and exports.
// **MODULAR STRUCTURE**:
//! - `types`: Core authentication types and data structures
//! - `service`: Main authentication service implementation
//! - `config`: Authentication configuration management
//! - `session`: Session management and lifecycle
//! - `oauth`: OAuth2 provider integration
//! - `mfa`: Multi-factor authentication
//! - `password`: Password policies and validation
//! - `tokens`: JWT token management
//! - `stats`: Authentication statistics and metrics

// ==================== MODULAR ORGANIZATION ====================

// Core authentication types and data structures
pub mod types;
// Main authentication service implementation
pub mod service;
// Authentication configuration management
pub mod config;
// Session management and lifecycle
pub mod session;
// OAuth2 provider integration
pub mod oauth;
// Multi-factor authentication
pub mod mfa;
// Password policies and validation
pub mod password;
// JWT token management
pub mod tokens;
// Authentication statistics and metrics
pub mod stats;
// ==================== RE-EXPORTS FOR COMPATIBILITY ====================

pub use types::*;
pub use service::*;
pub use config::*;
pub use session::*;
pub use oauth::*;
pub use mfa::*;
pub use password::*;
pub use tokens::*;
pub use stats::*; 
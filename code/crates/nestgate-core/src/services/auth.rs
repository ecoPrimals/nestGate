// **AUTHENTICATION SERVICE - MODULARIZED**
//! Auth functionality and utilities.
// This file has been refactored from a large monolithic implementation (860 lines)
//! into a clean modular structure for better maintainability and compliance with
//! the <2000 lines per file standard.
//! Auth functionality and utilities.
// **MIGRATION**: All functionality has been moved to focused modules:
//! - `auth/types` - Core authentication types and data structures
//! - `auth/service` - Main authentication service implementation
//! - `auth/config` - Authentication configuration management
//! - `auth/session` - Session management and lifecycle
//! - `auth/oauth` - OAuth2 provider integration
//! - `auth/mfa` - Multi-factor authentication
//! - `auth/password` - Password policies and validation
//! - `auth/tokens` - JWT token management
//! - `auth/stats` - Authentication statistics and metrics

// Re-export the modular authentication service
pub use self::auth::*;

/// Modular authentication service implementation
pub mod auth;

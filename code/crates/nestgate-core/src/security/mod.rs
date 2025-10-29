//! # NestGate Security Module
//! 🛡️ SOVEREIGNTY COMPLIANCE: All security operations use capability-based delegation
//! - Storage access authentication (delegated to security capabilities via universal adapter)
//! - Certificate management and validation
//! - Access control and authorization
//!   All complex authentication logic is delegated to security capabilities through the universal adapter.
pub mod auth;
// Removed auth_errors module - using unified NestGateError
pub mod auth_token;
pub mod auth_types;
pub mod production_hardening;
pub mod universal_auth_adapter;
#[cfg(test)]
// pub mod security_provider_tests; // Removed - had unresolvable imports
#[cfg(test)]
pub mod tests;

// Re-export commonly used types for storage access
// Simplified auth exports - using available types from auth module
pub use auth_types::{AccessLevel, AuthContext, Permission, Role};
// Legacy AuthError removed - use canonical SecurityError from crate::error
pub use auth_token::AuthToken;
pub use universal_auth_adapter::{
    StorageAccessRequest, StorageAccessResponse, UniversalAuthAdapter,
};

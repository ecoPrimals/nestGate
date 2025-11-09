//! # NestGate Security Module (Storage-Focused)
//! 🛡️ SOVEREIGNTY COMPLIANCE: All security operations use capability-based delegation
//! 
//! **NESTGATE STORAGE SECURITY SCOPE:**
//! - Storage access authentication (delegated to security capabilities via universal adapter)
//! - Certificate management for secure storage transport
//! - Access control and authorization for storage operations
//! 
//! **DELEGATED TO SECURITY PRIMAL (e.g., BearDog):**
//! - Rate limiting (removed)
//! - Intrusion detection (removed)
//! - Security hardening (removed)
//! - Input validation (removed)
//! - General authentication systems (removed)
//! 
//! All complex security logic is delegated to security capabilities through the universal adapter.
pub mod auth;
// Removed auth_errors module - using unified NestGateError
pub mod auth_token;
pub mod auth_types;
// REMOVED: production_hardening - delegated to security primal
// REMOVED: rate_limiter - delegated to security primal
// REMOVED: input_validation - delegated to security primal
// REMOVED: hardening modules - delegated to security primal
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

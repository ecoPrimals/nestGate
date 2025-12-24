//! # NestGate Security Module (Storage-Focused)
//! 🛡️ SOVEREIGNTY COMPLIANCE: All security operations use capability-based delegation
//! 
//! **NESTGATE STORAGE SECURITY SCOPE:**
//! - Storage access authentication (delegated to security capabilities via universal adapter)
//! - Certificate management for secure storage transport
//! - Access control and authorization for storage operations
//! - Pluggable authentication providers (JWT, BearDog, future systems)
//! 
//! **DELEGATED TO SECURITY PRIMAL (e.g., BearDog):**
//! - Rate limiting (removed)
//! - Intrusion detection (removed)
//! - Security hardening (removed)
//! - Input validation (removed)
//! - Cryptographic operations (delegated to BearDog)
//! 
//! All complex security logic is delegated to security capabilities through the universal adapter.
//!
//! ## Pluggable Authentication
//! 
//! NestGate supports multiple authentication modes:
//! - **BearDog** (default): Decentralized crypto auth for primal-to-primal
//! - **JWT** (legacy): Shared secret tokens for NAS and external clients
//! - **Auto**: Try BearDog first, fallback to JWT
//!
//! Configure via environment: `NESTGATE_AUTH_MODE=beardog|jwt|auto|none`
pub mod auth;
// Removed auth_errors module - using unified NestGateError
pub mod auth_provider;
pub mod auth_token;
pub mod auth_types;
pub mod jwt_validation;
pub mod permissions;
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
pub use jwt_validation::{validate_jwt_secret, validate_jwt_secret_or_exit, JwtSecretError};
pub use permissions::PermissionManager;
pub use universal_auth_adapter::{
    StorageAccessRequest, StorageAccessResponse, UniversalAuthAdapter,
};

// Export pluggable auth providers
pub use auth_provider::{
    AuthMode, AuthProvider, AuthRequest, AuthResponse, AuthRouter, BearDogAuthProvider,
    JwtAuthProvider, ProviderStatus, create_default_router, create_router_from_env,
};

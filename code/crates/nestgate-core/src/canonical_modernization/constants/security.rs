// **SECURITY CONSTANTS**
//! Security functionality and utilities.
// Security-related constants (delegated to Security primal).
// NOTE: Timeout constants have been consolidated to canonical::timeouts module

/// Maximum authentication attempts (domain-specific)
pub const MAX_AUTH_ATTEMPTS: u32 = 3;

/// Default token expiry in seconds
/// Matches canonical::security::TOKEN_EXPIRATION_S but kept for security module cohesion
pub const DEFAULT_TOKEN_EXPIRY_SECS: u64 = 3600; // 1 hour

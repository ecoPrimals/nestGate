// **SECURITY CONSTANTS**
//! Security functionality and utilities.
// Security-related constants (delegated to Security primal).

/// Default security timeout (local fallback only)
#[deprecated(since = "0.2.0", note = "Use nestgate_core::constants::canonical::timeouts::DEFAULT_TIMEOUT_SECS")]
pub const SECURITY_TIMEOUT_SECS: u64 = 30;

/// Maximum authentication attempts (domain-specific, keep)
pub const MAX_AUTH_ATTEMPTS: u32 = 3;

/// Default token expiry in seconds
/// Matches canonical::security::TOKEN_EXPIRATION_S but kept for security module cohesion
pub const DEFAULT_TOKEN_EXPIRY_SECS: u64 = 3600; // 1 hour

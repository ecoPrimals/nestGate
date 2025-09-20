// **SECURITY CONSTANTS**
//! Security functionality and utilities.
// Security-related constants (delegated to Security primal).

/// Default security timeout (local fallback only)
pub const SECURITY_TIMEOUT_SECS: u64 = 30;
/// Maximum authentication attempts
pub const MAX_AUTH_ATTEMPTS: u32 = 3;
/// Default token expiry in seconds
pub const DEFAULT_TOKEN_EXPIRY_SECS: u64 = 3600; // 1 hour

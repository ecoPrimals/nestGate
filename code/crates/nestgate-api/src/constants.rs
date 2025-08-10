//! API Constants
//!
//! Constants used throughout the NestGate API server

/// Default API port
pub const DEFAULT_API_PORT: u16 = 8080;

/// API version
pub const API_VERSION: &str = "v1";

/// Default request timeout in seconds
pub const DEFAULT_REQUEST_TIMEOUT_SECS: u64 = 30;

/// Maximum request body size (10MB)
pub const MAX_REQUEST_BODY_SIZE: usize = 10 * 1024 * 1024;

/// Default page size for paginated responses
pub const DEFAULT_PAGE_SIZE: usize = 20;

/// Maximum page size for paginated responses
pub const MAX_PAGE_SIZE: usize = 100;

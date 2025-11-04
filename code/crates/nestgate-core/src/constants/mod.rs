//! **NESTGATE CONSTANTS**
//!
//! Centralized constants to reduce hardcoding throughout the codebase.

pub mod canonical_defaults;
pub mod magic_numbers_replacement;
pub mod network;
pub mod network_defaults;
pub mod port_defaults;
pub mod shared;
pub mod system;
pub mod testing;

/// Centralized constants to eliminate hardcoding (addresses, ports, limits)
pub mod hardcoding;

// Re-export commonly used constants for backwards compatibility
// Note: Some constants like DEFAULT_TIMEOUT_MS are defined in multiple modules.
// Use explicit imports from shared:: or network:: to avoid ambiguity.
pub use network::{
    DEFAULT_API_PORT, DEFAULT_BIND_ADDRESS, DEFAULT_HEALTH_PORT, DEFAULT_METRICS_PORT, LOCALHOST,
};
pub use shared::{DEFAULT_BUFFER_SIZE, DEFAULT_MAX_CONNECTIONS, DEFAULT_TIMEOUT_MS};
pub use testing::*;

// New modules available for explicit import to replace hardcoded values:
// - network_defaults:: - IPv4/IPv6 defaults with environment variable support
// - port_defaults:: - Service ports with environment variable support

//! **NESTGATE CONSTANTS**
//!
//! Centralized constants to reduce hardcoding throughout the codebase.

pub mod canonical_defaults;
pub mod canonical;  // Canonical constants - single source of truth
// magic_numbers_replacement removed - was migration artifact with zero usages
pub mod network;
pub mod network_defaults;
pub mod network_hardcoded; // NEW: Centralized network constants (Nov 6, 2025)
pub mod port_defaults;
pub mod shared;
pub mod system;
pub mod testing;

/// Centralized constants to eliminate hardcoding (addresses, ports, limits)
pub mod hardcoding;

// Re-export commonly used constants for backwards compatibility
// Port constants - use port_defaults as single source of truth
pub use port_defaults::{
    DEFAULT_API_PORT, DEFAULT_ADMIN_PORT, DEFAULT_METRICS_PORT, DEFAULT_HEALTH_PORT,
    DEFAULT_GRAFANA_PORT, DEFAULT_POSTGRES_PORT, DEFAULT_REDIS_PORT,
    get_api_port, get_metrics_port, get_health_port,
};

// Network constants (non-port)
pub use network::{
    DEFAULT_BIND_ADDRESS, LOCALHOST,
    NETWORK_BUFFER_SIZE,  // Explicit network buffer size (8192 bytes)
    DEFAULT_TIMEOUT_MS,   // Re-exported from canonical::timeouts (30,000 ms)
};
pub use shared::DEFAULT_MAX_CONNECTIONS;

// Timeout and retry constants - use canonical as single source of truth:
pub use self::canonical::timeouts::{DEFAULT_RETRY_ATTEMPTS, DEFAULT_RETRY_DELAY_MS, DEFAULT_TIMEOUT_SECS};

// For buffer sizes, use domain-specific constants from canonical_defaults:
// - network::NETWORK_BUFFER_SIZE (8192 bytes) for network I/O
// - canonical_defaults::performance::DEFAULT_BUFFER_SIZE (4096 bytes) for disk/general I/O
pub use testing::*;

// Single source of truth modules:
// - canonical:: - All consolidated constants (PREFERRED)
// - port_defaults:: - Service ports with environment variable support
// - timeouts:: - Timeout values with environment variable support
// - network_defaults:: - IPv4/IPv6 defaults with environment variable support

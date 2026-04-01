// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Shared Constants Module
//!
//! This module contains constants that are used across multiple modules
//! to eliminate duplication and provide a single source of truth.

/// Module version for compatibility tracking across the codebase
pub const MODULE_VERSION: &str = "0.6.0";

/// Default maximum number of concurrent connections
pub const DEFAULT_MAX_CONNECTIONS: usize = 1000;

// NOTE: Following constants moved to canonical.rs for consolidation:
// - DEFAULT_TIMEOUT_MS → canonical::timeouts::DEFAULT_TIMEOUT_MS (30,000 ms)
// - DEFAULT_RETRY_ATTEMPTS → canonical::timeouts::DEFAULT_RETRY_ATTEMPTS (3)
// - DEFAULT_BACKOFF_MS → canonical::timeouts::DEFAULT_RETRY_DELAY_MS (1,000 ms)
// - DEFAULT_BUFFER_SIZE → See network::NETWORK_BUFFER_SIZE or canonical_defaults::sizes::DEFAULT_BUFFER_SIZE

/// Default health check interval in seconds
pub const DEFAULT_HEALTH_CHECK_INTERVAL_SECS: u64 = 30;

/// Default log rotation size in megabytes
pub const DEFAULT_LOG_ROTATION_MB: usize = 100;

/// Default cache TTL in seconds
pub const DEFAULT_CACHE_TTL_SECS: u64 = 3600;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shared_constants() {
        assert_eq!(MODULE_VERSION, "0.6.0");
        assert_eq!(DEFAULT_MAX_CONNECTIONS, 1000);
        // Migrated constants (now in canonical.rs):
        // - DEFAULT_TIMEOUT_MS → canonical::timeouts::DEFAULT_TIMEOUT_MS
        // - DEFAULT_RETRY_ATTEMPTS → canonical::timeouts::DEFAULT_RETRY_ATTEMPTS
        // - DEFAULT_BACKOFF_MS → canonical::timeouts::DEFAULT_RETRY_DELAY_MS
        // - DEFAULT_BUFFER_SIZE → network::NETWORK_BUFFER_SIZE or canonical_defaults::sizes::DEFAULT_BUFFER_SIZE
    }
}

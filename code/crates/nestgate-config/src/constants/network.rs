// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! **NETWORK CONSTANTS**
//!
//! Network-related constants and default values.

// Re-export shared network constants
pub use super::shared::DEFAULT_MAX_CONNECTIONS;

// Re-export timeout from canonical (single source of truth)
pub use crate::constants::canonical::timeouts::DEFAULT_TIMEOUT_MS;

// Use canonical domain-specific buffer size for network operations
pub use super::canonical_defaults::performance::NETWORK_BUFFER_SIZE;

// Backward compatibility: DEFAULT_BUFFER_SIZE for network code should use NETWORK_BUFFER_SIZE
/// Default network buffer size for I/O operations
///
/// **Size**: 8KB (8192 bytes) - optimized for network I/O
///
/// **Trade-offs**:
/// - Smaller buffers: Lower memory, more system calls
/// - Larger buffers: Higher memory, fewer system calls
/// - 8KB balances these for typical network operations
///
/// **Performance**: Reduces system call overhead while fitting comfortably in CPU cache.
///
/// **Evolution**: Future versions will use adaptive buffer sizing based on:
/// - Connection bandwidth detection
/// - Available system memory
/// - Traffic patterns (bulk transfer vs request/response)
pub const DEFAULT_BUFFER_SIZE: usize = super::canonical_defaults::performance::NETWORK_BUFFER_SIZE;

// ==================== PORT CONSTANTS ====================
// All port constants now come from port_defaults.rs (single source of truth)
// Re-exported here for backward compatibility
pub use super::port_defaults::{
    DEFAULT_API_PORT, DEFAULT_GRAFANA_PORT, DEFAULT_HEALTH_PORT, DEFAULT_METRICS_PORT,
    DEFAULT_POSTGRES_PORT as DEFAULT_DB_PORT, DEFAULT_REDIS_PORT,
};

/// Default localhost address
pub const LOCALHOST: &str = "127.0.0.1";

/// Default bind address
pub const DEFAULT_BIND_ADDRESS: &str = "0.0.0.0";

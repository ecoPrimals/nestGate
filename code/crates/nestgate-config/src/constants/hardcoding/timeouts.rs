// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Timeout constants for network and operation timing (milliseconds).
//!
//! **Evolution Path**: These will be replaced by capability-based adaptive timeouts that:
//! - Learn from actual operation latencies
//! - Adapt to network conditions
//! - Scale based on system load
//! - Use service-specific SLAs discovered at runtime

/// Default connection timeout (5 seconds)
pub const CONNECT_MS: u64 = 5_000;

/// Default request timeout (30 seconds)
pub const REQUEST_MS: u64 = 30_000;

/// Default long operation timeout (5 minutes)
pub const LONG_OPERATION_MS: u64 = 300_000;

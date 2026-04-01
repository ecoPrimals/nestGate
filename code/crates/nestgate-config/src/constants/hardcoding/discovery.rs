// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Default service discovery timeouts and port scan ranges.

/// Default service discovery timeout (milliseconds)
pub const TIMEOUT_MS: u64 = 5000;

/// Default retry attempts for service discovery
pub const RETRY_ATTEMPTS: u32 = 3;

/// Default port range start for capability scanning
pub const SCAN_PORT_START: u16 = 3000;

/// Default port range end for capability scanning
pub const SCAN_PORT_END: u16 = 3999;

/// Get discovery timeout from environment or default
#[must_use]
pub fn get_timeout_ms() -> u64 {
    crate::constants::hardcoding::runtime_defaults::RuntimeDefaults::discovery_timeout_ms()
}

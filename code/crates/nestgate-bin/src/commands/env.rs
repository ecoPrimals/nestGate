// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Environment-variable fallbacks for bind address and API port (`UniBin` compliance).

/// Read port from environment with fallback chain (`UniBin` compliance)
/// Priority: `NESTGATE_API_PORT` → `NESTGATE_HTTP_PORT` → `NESTGATE_PORT` → default
pub fn port_from_env_or_default() -> u16 {
    std::env::var("NESTGATE_API_PORT")
        .or_else(|_| std::env::var("NESTGATE_HTTP_PORT"))
        .or_else(|_| std::env::var("NESTGATE_PORT"))
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(nestgate_core::constants::DEFAULT_API_PORT)
}

/// Read bind address from environment with fallback (`UniBin` compliance)
/// Priority: `NESTGATE_BIND` → `NESTGATE_BIND_ADDRESS` → `NESTGATE_HOST` → default
pub fn bind_from_env_or_default() -> String {
    std::env::var("NESTGATE_BIND")
        .or_else(|_| std::env::var("NESTGATE_BIND_ADDRESS"))
        .or_else(|_| std::env::var("NESTGATE_HOST"))
        .ok()
        .unwrap_or_else(|| nestgate_core::constants::DEFAULT_BIND_ADDRESS.to_string())
}

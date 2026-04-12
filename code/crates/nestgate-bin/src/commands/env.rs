// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Environment-variable fallbacks for bind address and API port (`UniBin` compliance).
//!
//! All helpers accept `&(impl EnvSource + ?Sized)` for concurrent-safe testing via `MapEnv`.
//! Production callers pass `&ProcessEnv`.

use nestgate_types::EnvSource;

/// Read port from environment with fallback chain (`UniBin` compliance).
/// Priority: `NESTGATE_API_PORT` → `NESTGATE_HTTP_PORT` → `NESTGATE_PORT` → default
pub fn port_from_env_source(env: &(impl EnvSource + ?Sized)) -> u16 {
    env.get("NESTGATE_API_PORT")
        .or_else(|| env.get("NESTGATE_HTTP_PORT"))
        .or_else(|| env.get("NESTGATE_PORT"))
        .and_then(|s| s.parse().ok())
        .unwrap_or(nestgate_core::constants::DEFAULT_API_PORT)
}

/// Read bind address from environment with fallback (`UniBin` compliance).
/// Priority: `NESTGATE_BIND` → `NESTGATE_BIND_ADDRESS` → `NESTGATE_HOST` → default
pub fn bind_from_env_source(env: &(impl EnvSource + ?Sized)) -> String {
    env.get("NESTGATE_BIND")
        .or_else(|| env.get("NESTGATE_BIND_ADDRESS"))
        .or_else(|| env.get("NESTGATE_HOST"))
        .unwrap_or_else(|| nestgate_core::constants::DEFAULT_BIND_ADDRESS.to_string())
}

/// Process-env delegates for production use.
pub fn port_from_env_or_default() -> u16 {
    port_from_env_source(&nestgate_types::ProcessEnv)
}

/// Process-env delegates for production use.
pub fn bind_from_env_or_default() -> String {
    bind_from_env_source(&nestgate_types::ProcessEnv)
}

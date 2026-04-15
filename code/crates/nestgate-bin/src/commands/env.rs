// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Environment-variable fallbacks for bind address and API port (`UniBin` compliance).
//!
//! All helpers accept `&(impl EnvSource + ?Sized)` for concurrent-safe testing via `MapEnv`.
//! Production callers pass `&ProcessEnv`.
//!
//! TCP JSON-RPC (socket-only mode) also honors `NESTGATE_JSONRPC_TCP` (truthy values) to listen on
//! the default API port from `nestgate-config` when no `--port` / `--listen` / explicit port env is set.

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

/// TCP JSON-RPC port **only when** one of the `UniBin` port variables is explicitly set.
///
/// Returns [`None`] if none are set, or if the first present value fails to parse as `u16`
/// (no implicit TCP activation from invalid config).
pub fn env_port_if_set_source(env: &(impl EnvSource + ?Sized)) -> Option<u16> {
    if let Some(s) = env.get("NESTGATE_API_PORT") {
        return s.parse().ok();
    }
    if let Some(s) = env.get("NESTGATE_HTTP_PORT") {
        return s.parse().ok();
    }
    if let Some(s) = env.get("NESTGATE_PORT") {
        return s.parse().ok();
    }
    None
}

/// When set to a truthy value (`1`, `true`, `yes`, `on`), request TCP JSON-RPC on
/// [`nestgate_core::constants::DEFAULT_API_PORT`] if no `--port` / `--listen` / explicit port env applies.
pub fn tcp_jsonrpc_default_port_requested_source(env: &(impl EnvSource + ?Sized)) -> bool {
    env.get("NESTGATE_JSONRPC_TCP")
        .as_deref()
        .is_some_and(jsonrpc_tcp_truthy)
}

fn jsonrpc_tcp_truthy(s: &str) -> bool {
    matches!(
        s.trim().to_ascii_lowercase().as_str(),
        "1" | "true" | "yes" | "on"
    )
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

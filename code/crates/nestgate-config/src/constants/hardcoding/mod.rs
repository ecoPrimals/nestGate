// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! ⚠️ **DEPRECATED**: This module is being phased out in favor of capability-based configuration
//!
//! # Primal model: self-knowledge vs peers
//!
//! Primal code carries **self-knowledge** only: this process’s identity, capabilities, and own
//! listen endpoints. **Other primals** (orchestrator, storage peers, etc.) are **not** baked in at
//! compile time; their host/port (or URL) must come from **capability discovery at runtime**
//! (service registry, mDNS, mesh, etc.). The numeric port constants in [`runtime_fallback_ports`]
//! exist solely as
//! **fallback defaults** for bootstrap, tests, and legacy paths—prefer `RuntimeDefaults` (env then
//! fallback) or your discovery layer in production.
//!
//! # Migration Path
//!
//! Instead of using hardcoded constants, use `CapabilityConfig` for runtime discovery:
//!
//! ```rust,ignore
//! # use nestgate_core::capability_config::CapabilityConfig;
//! # use anyhow::Result;
//! # fn example() -> Result<()> {
//! // ❌ OLD: Hardcoded
//! // const API_PORT: u16 = 8080;
//!
//! // ✅ NEW: Capability-based
//! let config = CapabilityConfig::from_env()?;
//! let api_endpoint = config.get_endpoint("api")?;
//! # Ok(())
//! # }
//! ```
//!
//! Prefer `runtime_defaults` (env-first) and [`runtime_fallback_ports`] for numeric fallbacks.
//!
//! ## Environment variables (central audit)
//!
//! | Variable | Purpose |
//! |----------|---------|
//! | `NESTGATE_BIND_ADDRESS`, `NESTGATE_API_PORT`, `NESTGATE_METRICS_PORT`, `NESTGATE_HEALTH_PORT` | Core listen ports |
//! | `NESTGATE_ORCHESTRATOR_URL` | Full orchestrator base URL when not using discovery (read via env; prefer capability discovery in production) |
//! | `NESTGATE_ORCHESTRATOR_ADDR` | Orchestrator peer when discovery is empty (resolve via capability discovery in production) |
//! | `NESTGATE_WEBSOCKET_PORT`, `NESTGATE_RPC_PORT`, `NESTGATE_MQ_PORT`, `NESTGATE_ORCHESTRATION_PORT` | Service ports (see getters below) |
//! | `NESTGATE_DISCOVERY_TIMEOUT_MS` | Discovery timeout ([`crate::constants::hardcoding::discovery::get_timeout_ms`]) |
//!
//! **[`runtime_fallback_ports`] symbols → env overrides (document every fallback; wire via config/discovery in production):**
//!
//! | Symbol | `NESTGATE_*` override |
//! |----------|------------------------|
//! | `HTTP` | `NESTGATE_HTTP_PORT` (also influences orchestrator fallback with `NESTGATE_ORCHESTRATOR_ADDR` / `NESTGATE_ORCHESTRATOR_URL`) |
//! | `HTTPS` | `NESTGATE_HTTPS_PORT` |
//! | `API`, `API_ALT` | `NESTGATE_API_PORT` (see `RuntimeDefaults::api_port`); alt: `NESTGATE_API_ALT_PORT` |
//! | `METRICS` | `NESTGATE_METRICS_PORT` |
//! | `HEALTH` | `NESTGATE_HEALTH_PORT` |
//! | `GRPC` | `NESTGATE_RPC_PORT` (see `RuntimeDefaults::grpc_port`) |
//! | `WEBSOCKET` | `NESTGATE_WEBSOCKET_PORT` |
//! | `ADMIN` | `NESTGATE_ADMIN_PORT` |
//! | `STORAGE` | `NESTGATE_STORAGE_PORT` |
//! | `ORCHESTRATION` | `NESTGATE_ORCHESTRATION_PORT` |
//! | `COMPUTE` | `NESTGATE_COMPUTE_PORT` |
//! | `EXTENDED_SERVICES` | `NESTGATE_EXTENDED_SERVICES_PORT` |
//! | `DISCOVERY_SERVICE` | `NESTGATE_DISCOVERY_SERVICE_PORT` |
//! | `ORCHESTRATOR_DEFAULT` | `NESTGATE_ORCHESTRATOR_PORT` |
//! | `POSTGRES` | `NESTGATE_POSTGRES_PORT` |
//! | `REDIS` | `NESTGATE_REDIS_PORT` |
//! | `MONGODB` | `NESTGATE_MONGODB_PORT` |
//! | `MYSQL` | `NESTGATE_MYSQL_PORT` |
//! | `discovery::SCAN_PORT_START` / `SCAN_PORT_END` | `NESTGATE_DISCOVERY_SCAN_PORT_START` / `NESTGATE_DISCOVERY_SCAN_PORT_END` |
//! | `timeouts::*`, `limits::*` | `NESTGATE_CONNECT_TIMEOUT_MS`, `NESTGATE_REQUEST_TIMEOUT_MS`, `NESTGATE_LONG_OPERATION_TIMEOUT_MS`, `NESTGATE_BUFFER_SIZE`, `NESTGATE_MAX_CONNECTIONS` (when wired) |
//!
//! Timeouts and limits in [`crate::constants::hardcoding::timeouts`] and [`crate::constants::hardcoding::limits`] remain compile-time defaults; override via
//! capability config or future env wiring where those domains expose runtime tuning.

pub mod addresses;
pub mod discovery;
pub mod limits;
pub mod runtime_defaults;
pub mod runtime_fallback_ports;
pub mod timeouts;

pub use runtime_defaults::{
    RuntimeDefaults, get_api_port, get_bind_address, get_grpc_port, get_health_port,
    get_message_queue_port, get_metrics_port, get_orchestration_service_port, get_websocket_port,
    get_zfs_bind_port,
};

#[cfg(test)]
mod tests;

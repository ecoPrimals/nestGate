// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Data domain semantic methods (live feeds, NOT storage)
//!
//! `NestGate` routes data requests to external capability providers. It does not
//! implement data fetching directly. The `data.*` namespace is distinct from
//! `storage.*` (persistent key-value) and `session.*` (game session snapshots).
//!
//! Per wateringHole capability-based discovery, callers must discover a primal
//! that advertises the `"data"` capability and delegate there via IPC. These
//! handlers return structured JSON-RPC errors directing callers to discovery,
//! mirroring the crypto delegation pattern (`crypto.rs`).

use super::SemanticRouter;
use nestgate_types::error::{NestGateError, Result};
use serde_json::Value;

fn data_delegation_error(operation: &str) -> NestGateError {
    NestGateError::not_implemented(format!(
        "data.{operation}: NestGate delegates data operations to data capability providers. \
         Discover them via `discovery.query` with capability=\"data\" or \
         `NESTGATE_CAPABILITY_DATA` environment variable."
    ))
}

/// Route `data.ncbi_search` → data capability provider (delegation)
pub(super) fn data_ncbi_search(_router: &SemanticRouter, _params: Value) -> Result<Value> {
    tracing::debug!("data.ncbi_search: delegating to data capability provider");
    Err(data_delegation_error("ncbi_search"))
}

/// Route `data.ncbi_fetch` → data capability provider (delegation)
pub(super) fn data_ncbi_fetch(_router: &SemanticRouter, _params: Value) -> Result<Value> {
    tracing::debug!("data.ncbi_fetch: delegating to data capability provider");
    Err(data_delegation_error("ncbi_fetch"))
}

/// Route `data.noaa_ghcnd` → data capability provider (delegation)
pub(super) fn data_noaa_ghcnd(_router: &SemanticRouter, _params: Value) -> Result<Value> {
    tracing::debug!("data.noaa_ghcnd: delegating to data capability provider");
    Err(data_delegation_error("noaa_ghcnd"))
}

/// Route `data.iris_stations` → data capability provider (delegation)
pub(super) fn data_iris_stations(_router: &SemanticRouter, _params: Value) -> Result<Value> {
    tracing::debug!("data.iris_stations: delegating to data capability provider");
    Err(data_delegation_error("iris_stations"))
}

/// Route `data.iris_events` → data capability provider (delegation)
pub(super) fn data_iris_events(_router: &SemanticRouter, _params: Value) -> Result<Value> {
    tracing::debug!("data.iris_events: delegating to data capability provider");
    Err(data_delegation_error("iris_events"))
}

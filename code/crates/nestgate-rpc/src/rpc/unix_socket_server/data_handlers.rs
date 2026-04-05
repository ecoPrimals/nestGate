// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Data domain JSON-RPC handlers (live feeds, NOT storage).
//!
//! `NestGate` is a storage primal and does not implement data fetching directly.
//! These handlers return structured delegation errors directing callers to
//! discover a primal that advertises the `"data"` capability via
//! `discovery.query` or `NESTGATE_CAPABILITY_DATA`. This mirrors the crypto
//! delegation pattern in `semantic_router/crypto.rs`.
//!
//! `data.*` is intentionally excluded from `capabilities.list` — `NestGate`
//! routes these method names for ecosystem convenience but does not provide
//! the underlying data service.

use nestgate_types::error::{NestGateError, Result};
use serde_json::Value;

fn data_delegation_error(operation: &str) -> NestGateError {
    NestGateError::not_implemented(format!(
        "data.{operation}: NestGate delegates data operations to data capability providers. \
         Discover them via `discovery.query` with capability=\"data\" or \
         `NESTGATE_CAPABILITY_DATA` environment variable."
    ))
}

/// `data.ncbi_search` — delegated to data capability provider.
pub(super) fn data_ncbi_search(_params: Option<&Value>) -> Result<Value> {
    tracing::debug!("data.ncbi_search: delegating to data capability provider");
    Err(data_delegation_error("ncbi_search"))
}

/// `data.ncbi_fetch` — delegated to data capability provider.
pub(super) fn data_ncbi_fetch(_params: Option<&Value>) -> Result<Value> {
    tracing::debug!("data.ncbi_fetch: delegating to data capability provider");
    Err(data_delegation_error("ncbi_fetch"))
}

/// `data.noaa_ghcnd` — delegated to data capability provider.
pub(super) fn data_noaa_ghcnd(_params: Option<&Value>) -> Result<Value> {
    tracing::debug!("data.noaa_ghcnd: delegating to data capability provider");
    Err(data_delegation_error("noaa_ghcnd"))
}

/// `data.iris_stations` — delegated to data capability provider.
pub(super) fn data_iris_stations(_params: Option<&Value>) -> Result<Value> {
    tracing::debug!("data.iris_stations: delegating to data capability provider");
    Err(data_delegation_error("iris_stations"))
}

/// `data.iris_events` — delegated to data capability provider.
pub(super) fn data_iris_events(_params: Option<&Value>) -> Result<Value> {
    tracing::debug!("data.iris_events: delegating to data capability provider");
    Err(data_delegation_error("iris_events"))
}

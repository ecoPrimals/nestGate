// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Data domain semantic methods (live feeds, NOT storage)
//!
//! These methods route to external live data providers. The `data.*` namespace
//! is distinct from `storage.*` (persistent key-value) and `session.*`
//! (game session snapshots).
//!
//! External data sources (`NCBI`, `NOAA`, `IRIS`) require HTTP access
//! to public APIs. NestGate provides the routing surface; the actual data
//! fetching is delegated to a configured data provider or returns a
//! structured "source not configured" response.

use super::SemanticRouter;
use nestgate_types::error::{NestGateError, Result};
use serde_json::Value;

fn data_source_not_configured(source: &str, method: &str) -> NestGateError {
    NestGateError::not_implemented(format!(
        "data.{method}: {source} data source is not configured. \
         Configure external data providers via `NESTGATE_EXTERNAL_*` \
         environment variables or capability-based discovery."
    ))
}

/// Route `data.ncbi_search` → NCBI database search
pub(super) fn data_ncbi_search(_router: &SemanticRouter, _params: Value) -> Result<Value> {
    tracing::debug!("data.ncbi_search: external data source");
    Err(data_source_not_configured("NCBI", "ncbi_search"))
}

/// Route `data.ncbi_fetch` → NCBI record fetch
pub(super) fn data_ncbi_fetch(_router: &SemanticRouter, _params: Value) -> Result<Value> {
    tracing::debug!("data.ncbi_fetch: external data source");
    Err(data_source_not_configured("NCBI", "ncbi_fetch"))
}

/// Route `data.noaa_ghcnd` → NOAA weather station data
pub(super) fn data_noaa_ghcnd(_router: &SemanticRouter, _params: Value) -> Result<Value> {
    tracing::debug!("data.noaa_ghcnd: external data source");
    Err(data_source_not_configured("NOAA GHCND", "noaa_ghcnd"))
}

/// Route `data.iris_stations` → IRIS seismic station listing
pub(super) fn data_iris_stations(_router: &SemanticRouter, _params: Value) -> Result<Value> {
    tracing::debug!("data.iris_stations: external data source");
    Err(data_source_not_configured("IRIS", "iris_stations"))
}

/// Route `data.iris_events` → IRIS seismic event listing
pub(super) fn data_iris_events(_router: &SemanticRouter, _params: Value) -> Result<Value> {
    tracing::debug!("data.iris_events: external data source");
    Err(data_source_not_configured("IRIS", "iris_events"))
}

// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Data Provider JSON-RPC Handlers
//!
//! **Integration:** Live providers are implemented in `nestgate-core` `data_sources`.

use nestgate_types::error::{NestGateError, Result};
use serde_json::Value;

/// data.ncbi_search — not implemented in this crate.
pub(super) async fn data_ncbi_search(_params: &Option<Value>) -> Result<Value> {
    tracing::debug!("feature pending: data.ncbi_search (nestgate-core data_sources)");
    Err(NestGateError::not_implemented(
        "wire cross-crate dep: nestgate-core data_sources",
    ))
}

/// data.ncbi_fetch — not implemented in this crate.
pub(super) async fn data_ncbi_fetch(_params: &Option<Value>) -> Result<Value> {
    tracing::debug!("feature pending: data.ncbi_fetch (nestgate-core data_sources)");
    Err(NestGateError::not_implemented(
        "wire cross-crate dep: nestgate-core data_sources",
    ))
}

/// data.noaa_ghcnd — not implemented in this crate.
pub(super) async fn data_noaa_ghcnd(_params: &Option<Value>) -> Result<Value> {
    tracing::debug!("feature pending: data.noaa_ghcnd (nestgate-core data_sources)");
    Err(NestGateError::not_implemented(
        "wire cross-crate dep: nestgate-core data_sources",
    ))
}

/// data.iris_stations — not implemented in this crate.
pub(super) async fn data_iris_stations(_params: &Option<Value>) -> Result<Value> {
    tracing::debug!("feature pending: data.iris_stations (nestgate-core data_sources)");
    Err(NestGateError::not_implemented(
        "wire cross-crate dep: nestgate-core data_sources",
    ))
}

/// data.iris_events — not implemented in this crate.
pub(super) async fn data_iris_events(_params: &Option<Value>) -> Result<Value> {
    tracing::debug!("feature pending: data.iris_events (nestgate-core data_sources)");
    Err(NestGateError::not_implemented(
        "wire cross-crate dep: nestgate-core data_sources",
    ))
}

// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Data Provider JSON-RPC Handlers
//!
//! TODO: wire to nestgate-core `data_sources` live providers.

use nestgate_types::error::{NestGateError, Result};
use serde_json::Value;

/// data.ncbi_search — stub.
pub(super) async fn data_ncbi_search(_params: &Option<Value>) -> Result<Value> {
    Err(NestGateError::not_implemented(
        "wire cross-crate dep: nestgate-core data_sources",
    ))
}

/// data.ncbi_fetch — stub.
pub(super) async fn data_ncbi_fetch(_params: &Option<Value>) -> Result<Value> {
    Err(NestGateError::not_implemented(
        "wire cross-crate dep: nestgate-core data_sources",
    ))
}

/// data.noaa_ghcnd — stub.
pub(super) async fn data_noaa_ghcnd(_params: &Option<Value>) -> Result<Value> {
    Err(NestGateError::not_implemented(
        "wire cross-crate dep: nestgate-core data_sources",
    ))
}

/// data.iris_stations — stub.
pub(super) async fn data_iris_stations(_params: &Option<Value>) -> Result<Value> {
    Err(NestGateError::not_implemented(
        "wire cross-crate dep: nestgate-core data_sources",
    ))
}

/// data.iris_events — stub.
pub(super) async fn data_iris_events(_params: &Option<Value>) -> Result<Value> {
    Err(NestGateError::not_implemented(
        "wire cross-crate dep: nestgate-core data_sources",
    ))
}

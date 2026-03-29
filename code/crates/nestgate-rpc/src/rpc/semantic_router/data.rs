// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Data domain semantic methods
//!
//! **Integration:** Live data providers live under `nestgate-core` `data_sources`.

use super::SemanticRouter;
use nestgate_types::error::{NestGateError, Result};
use serde_json::Value;

pub(super) async fn data_ncbi_search(_router: &SemanticRouter, _params: Value) -> Result<Value> {
    tracing::debug!("feature pending: data.ncbi_search (nestgate-core data_sources)");
    Err(NestGateError::not_implemented(
        "wire cross-crate dep: nestgate-core data_sources",
    ))
}

pub(super) async fn data_ncbi_fetch(_router: &SemanticRouter, _params: Value) -> Result<Value> {
    tracing::debug!("feature pending: data.ncbi_fetch (nestgate-core data_sources)");
    Err(NestGateError::not_implemented(
        "wire cross-crate dep: nestgate-core data_sources",
    ))
}

pub(super) async fn data_noaa_ghcnd(_router: &SemanticRouter, _params: Value) -> Result<Value> {
    tracing::debug!("feature pending: data.noaa_ghcnd (nestgate-core data_sources)");
    Err(NestGateError::not_implemented(
        "wire cross-crate dep: nestgate-core data_sources",
    ))
}

pub(super) async fn data_iris_stations(_router: &SemanticRouter, _params: Value) -> Result<Value> {
    tracing::debug!("feature pending: data.iris_stations (nestgate-core data_sources)");
    Err(NestGateError::not_implemented(
        "wire cross-crate dep: nestgate-core data_sources",
    ))
}

pub(super) async fn data_iris_events(_router: &SemanticRouter, _params: Value) -> Result<Value> {
    tracing::debug!("feature pending: data.iris_events (nestgate-core data_sources)");
    Err(NestGateError::not_implemented(
        "wire cross-crate dep: nestgate-core data_sources",
    ))
}

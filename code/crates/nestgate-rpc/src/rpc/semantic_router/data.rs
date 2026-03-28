// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Data domain semantic methods
//!
//! TODO: wire to nestgate-core `data_sources`.

use super::SemanticRouter;
use nestgate_types::error::{NestGateError, Result};
use serde_json::Value;

pub(super) async fn data_ncbi_search(_router: &SemanticRouter, _params: Value) -> Result<Value> {
    Err(NestGateError::not_implemented(
        "wire cross-crate dep: nestgate-core data_sources",
    ))
}

pub(super) async fn data_ncbi_fetch(_router: &SemanticRouter, _params: Value) -> Result<Value> {
    Err(NestGateError::not_implemented(
        "wire cross-crate dep: nestgate-core data_sources",
    ))
}

pub(super) async fn data_noaa_ghcnd(_router: &SemanticRouter, _params: Value) -> Result<Value> {
    Err(NestGateError::not_implemented(
        "wire cross-crate dep: nestgate-core data_sources",
    ))
}

pub(super) async fn data_iris_stations(_router: &SemanticRouter, _params: Value) -> Result<Value> {
    Err(NestGateError::not_implemented(
        "wire cross-crate dep: nestgate-core data_sources",
    ))
}

pub(super) async fn data_iris_events(_router: &SemanticRouter, _params: Value) -> Result<Value> {
    Err(NestGateError::not_implemented(
        "wire cross-crate dep: nestgate-core data_sources",
    ))
}

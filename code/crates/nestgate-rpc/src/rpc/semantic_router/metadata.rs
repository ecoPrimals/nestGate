// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Metadata domain semantic methods
//!
//! TODO: wire to nestgate-core `service_metadata`.

use super::SemanticRouter;
use nestgate_types::error::{NestGateError, Result};
use serde_json::Value;

pub(super) async fn metadata_store(_router: &SemanticRouter, _params: Value) -> Result<Value> {
    Err(NestGateError::not_implemented(
        "wire cross-crate dep: nestgate-core service_metadata",
    ))
}

pub(super) async fn metadata_retrieve(_router: &SemanticRouter, _params: Value) -> Result<Value> {
    Err(NestGateError::not_implemented(
        "wire cross-crate dep: nestgate-core service_metadata",
    ))
}

pub(super) async fn metadata_search(_router: &SemanticRouter, _params: Value) -> Result<Value> {
    Err(NestGateError::not_implemented(
        "wire cross-crate dep: nestgate-core service_metadata",
    ))
}

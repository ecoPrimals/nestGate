// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Discovery domain semantic methods
//!
//! TODO: wire to nestgate-core `service_metadata` / nestgate-discovery.

use super::SemanticRouter;
use nestgate_types::error::{NestGateError, Result};
use serde_json::{json, Value};

pub(super) async fn discovery_announce(_router: &SemanticRouter, _params: Value) -> Result<Value> {
    Err(NestGateError::not_implemented(
        "wire cross-crate dep: nestgate-discovery / service_metadata",
    ))
}

pub(super) async fn discovery_query(_router: &SemanticRouter, _params: Value) -> Result<Value> {
    Err(NestGateError::not_implemented(
        "wire cross-crate dep: nestgate-discovery / service_metadata",
    ))
}

pub(super) async fn discovery_list(_router: &SemanticRouter, _params: Value) -> Result<Value> {
    Err(NestGateError::not_implemented(
        "wire cross-crate dep: nestgate-discovery / service_metadata",
    ))
}

pub(super) async fn discovery_capabilities(
    _router: &SemanticRouter,
    _params: Value,
) -> Result<Value> {
    Ok(json!({
        "capabilities": ["storage", "discovery", "metadata", "health"]
    }))
}

// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Discovery domain semantic methods
//!
//! **Integration:** Full discovery responses come from `nestgate-core` `service_metadata` and
//! `nestgate-discovery` when those are callable from this router.

use super::SemanticRouter;
use nestgate_types::error::{NestGateError, Result};
use serde_json::{Value, json};

pub(super) fn discovery_announce(_router: &SemanticRouter, _params: Value) -> Result<Value> {
    tracing::debug!("feature pending: discovery.announce (nestgate-discovery / service_metadata)");
    Err(NestGateError::not_implemented(
        "wire cross-crate dep: nestgate-discovery / service_metadata",
    ))
}

pub(super) fn discovery_query(_router: &SemanticRouter, _params: Value) -> Result<Value> {
    tracing::debug!("feature pending: discovery.query (nestgate-discovery / service_metadata)");
    Err(NestGateError::not_implemented(
        "wire cross-crate dep: nestgate-discovery / service_metadata",
    ))
}

pub(super) fn discovery_list(_router: &SemanticRouter, _params: Value) -> Result<Value> {
    tracing::debug!("feature pending: discovery.list (nestgate-discovery / service_metadata)");
    Err(NestGateError::not_implemented(
        "wire cross-crate dep: nestgate-discovery / service_metadata",
    ))
}

#[expect(
    clippy::unnecessary_wraps,
    reason = "JSON-RPC semantic handlers use Result for uniform router dispatch"
)]
pub(super) fn discovery_capabilities(_router: &SemanticRouter, _params: Value) -> Result<Value> {
    Ok(json!({
        "capabilities": ["storage", "discovery", "metadata", "health"]
    }))
}

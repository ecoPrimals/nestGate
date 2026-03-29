// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Metadata domain semantic methods
//!
//! **Integration:** Metadata persistence routes to `nestgate-core` `service_metadata` when linked.

use super::SemanticRouter;
use nestgate_types::error::{NestGateError, Result};
use serde_json::Value;

pub(super) fn metadata_store(_router: &SemanticRouter, _params: Value) -> Result<Value> {
    tracing::debug!("feature pending: metadata.store (nestgate-core service_metadata)");
    Err(NestGateError::not_implemented(
        "wire cross-crate dep: nestgate-core service_metadata",
    ))
}

pub(super) fn metadata_retrieve(_router: &SemanticRouter, _params: Value) -> Result<Value> {
    tracing::debug!("feature pending: metadata.retrieve (nestgate-core service_metadata)");
    Err(NestGateError::not_implemented(
        "wire cross-crate dep: nestgate-core service_metadata",
    ))
}

pub(super) fn metadata_search(_router: &SemanticRouter, _params: Value) -> Result<Value> {
    tracing::debug!("feature pending: metadata.search (nestgate-core service_metadata)");
    Err(NestGateError::not_implemented(
        "wire cross-crate dep: nestgate-core service_metadata",
    ))
}

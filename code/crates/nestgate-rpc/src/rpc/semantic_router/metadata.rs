// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rpc::NestGateRpcClient;
    use crate::rpc::semantic_router::SemanticRouter;
    use serde_json::json;
    use std::sync::Arc;

    fn router() -> SemanticRouter {
        let client = NestGateRpcClient::new("tarpc://127.0.0.1:65534").expect("client");
        SemanticRouter::new(Arc::new(client))
    }

    #[test]
    fn metadata_store_retrieve_search_not_implemented() {
        let r = router();
        for f in [metadata_store, metadata_retrieve, metadata_search] {
            let e = f(&r, json!({})).expect_err("not implemented");
            assert!(e.to_string().contains("service_metadata"));
        }
    }
}

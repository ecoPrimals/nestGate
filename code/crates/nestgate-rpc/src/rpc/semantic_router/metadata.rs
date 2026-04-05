// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Metadata domain semantic methods
//!
//! **Integration:** Metadata operations are delegated to the injected
//! [`MetadataBackend`](crate::rpc::metadata_backend::MetadataBackend) implementation. At daemon startup, `nestgate-core`'s
//! `ServiceMetadataStore` is wired as the backend; standalone / test mode
//! uses the in-memory default.

use super::SemanticRouter;
use crate::rpc::metadata_backend::ServiceRecord;
use nestgate_types::error::{NestGateError, Result};
use serde_json::{Value, json};
use std::collections::HashMap;

/// Route `metadata.store` → store service metadata via backend.
pub(super) async fn metadata_store(router: &SemanticRouter, params: Value) -> Result<Value> {
    let name = params["name"]
        .as_str()
        .ok_or_else(|| NestGateError::invalid_input_with_field("name", "string required"))?;

    let capabilities: Vec<String> = params["capabilities"]
        .as_array()
        .map(|arr| {
            arr.iter()
                .filter_map(serde_json::Value::as_str)
                .map(String::from)
                .collect()
        })
        .unwrap_or_default();

    let endpoint = params["endpoint"].as_str().map(String::from);

    let metadata: HashMap<String, String> = params["metadata"]
        .as_object()
        .map(|obj| {
            obj.iter()
                .filter_map(|(k, v)| v.as_str().map(|s| (k.clone(), s.to_string())))
                .collect()
        })
        .unwrap_or_default();

    let record = ServiceRecord {
        name: name.to_string(),
        capabilities,
        endpoint,
        metadata,
    };

    router.metadata.store_service(record).await?;

    Ok(json!({
        "status": "stored",
        "name": name,
    }))
}

/// Route `metadata.retrieve` → get service metadata by name via backend.
pub(super) async fn metadata_retrieve(router: &SemanticRouter, params: Value) -> Result<Value> {
    let name = params["name"]
        .as_str()
        .ok_or_else(|| NestGateError::invalid_input_with_field("name", "string required"))?;

    let record = router.metadata.get_service(name).await?;

    serde_json::to_value(&record).map_err(|e| {
        NestGateError::internal_error(
            format!("Failed to serialize service record: {e}"),
            "semantic_router",
        )
    })
}

/// Route `metadata.search` → search services by capability via backend.
pub(super) async fn metadata_search(router: &SemanticRouter, params: Value) -> Result<Value> {
    let capability = params["capability"]
        .as_str()
        .ok_or_else(|| NestGateError::invalid_input_with_field("capability", "string required"))?;

    let results = router.metadata.find_by_capability(capability).await?;

    Ok(json!({
        "capability": capability,
        "services": results,
        "count": results.len(),
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rpc::NestGateRpcClient;
    use crate::rpc::metadata_backend::InMemoryMetadataBackend;
    use crate::rpc::semantic_router::SemanticRouter;
    use serde_json::json;
    use std::sync::Arc;

    fn router() -> SemanticRouter {
        let client = NestGateRpcClient::new("tarpc://127.0.0.1:65534").expect("client");
        SemanticRouter::with_metadata_backend(
            Arc::new(client),
            Arc::new(InMemoryMetadataBackend::new()),
        )
    }

    #[tokio::test]
    async fn metadata_store_and_retrieve() {
        let r = router();
        let store_result = metadata_store(
            &r,
            json!({
                "name": "test-service",
                "capabilities": ["storage", "compute"],
                "endpoint": "http://localhost:9090"
            }),
        )
        .await
        .expect("store");
        assert_eq!(store_result["status"], "stored");

        let retrieved = metadata_retrieve(&r, json!({"name": "test-service"}))
            .await
            .expect("retrieve");
        assert_eq!(retrieved["name"], "test-service");
        assert_eq!(retrieved["capabilities"].as_array().expect("arr").len(), 2);
    }

    #[tokio::test]
    async fn metadata_search_by_capability() {
        let r = router();
        metadata_store(
            &r,
            json!({
                "name": "svc-a",
                "capabilities": ["storage"]
            }),
        )
        .await
        .expect("store");

        let results = metadata_search(&r, json!({"capability": "storage"}))
            .await
            .expect("search");
        assert_eq!(results["count"], 1);

        let empty = metadata_search(&r, json!({"capability": "quantum"}))
            .await
            .expect("search");
        assert_eq!(empty["count"], 0);
    }

    #[tokio::test]
    async fn metadata_retrieve_not_found() {
        let r = router();
        let e = metadata_retrieve(&r, json!({"name": "nonexistent"}))
            .await
            .expect_err("not found");
        assert!(!e.to_string().is_empty());
    }

    #[tokio::test]
    async fn metadata_store_missing_name() {
        let r = router();
        let e = metadata_store(&r, json!({"capabilities": ["x"]}))
            .await
            .expect_err("missing name");
        assert!(!e.to_string().is_empty());
    }
}

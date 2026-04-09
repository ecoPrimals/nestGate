// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Session persistence semantic methods (`session.*`)
//!
//! Durable session state is stored via [`crate::rpc::NestGateRpcClient`] (storage backend);
//! session index entries for listing use [`crate::rpc::metadata_backend::MetadataBackend`].

use super::SemanticRouter;
use crate::rpc::metadata_backend::ServiceRecord;
use crate::rpc::tarpc_types::OperationResult;
use bytes::Bytes;
use chrono::Utc;
use nestgate_types::error::{NestGateError, Result};
use serde_json::{Value, json};
use std::collections::HashMap;

/// True when `delete_object` failed because the blob was already absent (local storage, tarpc
/// `ObjectNotFound`, or tarpc internal error wrapping `File not found` from the backend).
fn is_delete_object_absent(e: &NestGateError) -> bool {
    let s = e.to_string();
    s.contains("Object not found:") || (s.contains("File not found:") && s.contains("object "))
}

/// Dataset name for session blobs (keys are `session/{id}`).
const SESSION_DATASET: &str = "session";
/// Prefix for session index records in [`MetadataBackend`] (`session/{id}`).
const SESSION_INDEX_PREFIX: &str = "session/";
/// Capability marker on session index records.
const SESSION_INDEX_CAPABILITY: &str = "session.index";

fn session_storage_key(id: &str) -> String {
    format!("{SESSION_INDEX_PREFIX}{id}")
}

fn session_index_name(id: &str) -> String {
    session_storage_key(id)
}

/// Route `session.save` — persist session JSON under `session/{id}` and update metadata index.
pub(super) async fn session_save(router: &SemanticRouter, params: Value) -> Result<Value> {
    let id = params["id"]
        .as_str()
        .ok_or_else(|| NestGateError::invalid_input_with_field("id", "string required"))?;
    if id.is_empty() {
        return Err(NestGateError::invalid_input_with_field(
            "id",
            "non-empty string required",
        ));
    }

    let data = params
        .get("data")
        .filter(|v| !v.is_null())
        .cloned()
        .unwrap_or_else(|| json!({}));

    let key = session_storage_key(id);
    let now = Utc::now().to_rfc3339();

    let (created_at, prev_data) = router
        .client
        .retrieve_object(SESSION_DATASET, &key)
        .await
        .map_or_else(
            |_| (now.clone(), None),
            |bytes| {
                serde_json::from_slice::<Value>(&bytes).map_or_else(
                    |_| (now.clone(), None),
                    |doc| {
                        (
                            doc["created_at"].as_str().unwrap_or(&now).to_string(),
                            Some(doc["data"].clone()),
                        )
                    },
                )
            },
        );

    let merged_data = if let Some(pd) = prev_data {
        if data.is_object() && pd.is_object() {
            let mut base = pd.as_object().cloned().unwrap_or_default();
            if let Some(obj) = data.as_object() {
                for (k, v) in obj {
                    base.insert(k.clone(), v.clone());
                }
            }
            Value::Object(base)
        } else {
            data
        }
    } else {
        data
    };

    let doc = json!({
        "id": id,
        "data": merged_data,
        "created_at": created_at,
        "updated_at": now,
    });

    let payload = serde_json::to_vec(&doc).map_err(|e| {
        NestGateError::invalid_input_with_field("data", format!("cannot serialize session: {e}"))
    })?;

    router
        .client
        .store_object(SESSION_DATASET, &key, Bytes::from(payload), None)
        .await?;

    let record = ServiceRecord {
        name: session_index_name(id),
        capabilities: vec![SESSION_INDEX_CAPABILITY.to_string()],
        endpoint: None,
        metadata: HashMap::from([
            (
                "created_at".to_string(),
                doc["created_at"].as_str().unwrap_or("").to_string(),
            ),
            (
                "updated_at".to_string(),
                doc["updated_at"].as_str().unwrap_or("").to_string(),
            ),
        ]),
    };

    if let Err(e) = router.metadata.store_service(record).await {
        let _ = router.client.delete_object(SESSION_DATASET, &key).await;
        return Err(e);
    }

    Ok(json!({
        "status": "saved",
        "id": id,
        "created_at": doc["created_at"],
        "updated_at": doc["updated_at"],
    }))
}

/// Route `session.load` — load session JSON from storage.
pub(super) async fn session_load(router: &SemanticRouter, params: Value) -> Result<Value> {
    let id = params["id"]
        .as_str()
        .ok_or_else(|| NestGateError::invalid_input_with_field("id", "string required"))?;
    if id.is_empty() {
        return Err(NestGateError::invalid_input_with_field(
            "id",
            "non-empty string required",
        ));
    }

    let key = session_storage_key(id);
    let bytes = router.client.retrieve_object(SESSION_DATASET, &key).await?;

    let doc: Value = serde_json::from_slice(&bytes).map_err(|e| {
        NestGateError::internal_error(
            format!("session payload is not valid JSON: {e}"),
            "semantic_router.session",
        )
    })?;

    Ok(doc)
}

/// Route `session.list` — list session index entries from metadata (`session/` prefix).
pub(super) async fn session_list(router: &SemanticRouter, params: Value) -> Result<Value> {
    let prefix = params["prefix"].as_str().unwrap_or(SESSION_INDEX_PREFIX);

    let records = router.metadata.list_services_by_name_prefix(prefix).await?;

    let sessions: Vec<Value> = records
        .into_iter()
        .map(|r| {
            let sid = r
                .name
                .strip_prefix(SESSION_INDEX_PREFIX)
                .unwrap_or(&r.name)
                .to_string();
            json!({
                "id": sid,
                "name": r.name,
                "capabilities": r.capabilities,
                "metadata": r.metadata,
            })
        })
        .collect();

    let count = sessions.len();
    Ok(json!({
        "prefix": prefix,
        "sessions": sessions,
        "count": count,
    }))
}

/// Route `session.delete` — remove session blob and metadata index entry.
pub(super) async fn session_delete(router: &SemanticRouter, params: Value) -> Result<Value> {
    let id = params["id"]
        .as_str()
        .ok_or_else(|| NestGateError::invalid_input_with_field("id", "string required"))?;
    if id.is_empty() {
        return Err(NestGateError::invalid_input_with_field(
            "id",
            "non-empty string required",
        ));
    }

    let key = session_storage_key(id);
    let name = session_index_name(id);

    let del = match router.client.delete_object(SESSION_DATASET, &key).await {
        Ok(r) => r,
        Err(e) if is_delete_object_absent(&e) => OperationResult {
            success: true,
            message: format!("session object {key} was already absent"),
            metadata: HashMap::new(),
        },
        Err(e) => return Err(e),
    };

    let _ = router.metadata.delete_service(&name).await;

    Ok(json!({
        "success": del.success,
        "message": del.message,
        "id": id,
    }))
}

#[cfg(test)]
mod tests {
    use super::super::tests::spawn_local_tarpc_server;
    use super::*;
    use crate::rpc::NestGateRpcClient;
    use crate::rpc::metadata_backend::InMemoryMetadataBackend;
    use crate::rpc::semantic_router::SemanticRouter;
    use serde_json::json;
    use std::sync::Arc;

    fn router_offline() -> SemanticRouter {
        let client = NestGateRpcClient::new("tarpc://127.0.0.1:65534").expect("client");
        SemanticRouter::with_metadata_backend(
            Arc::new(client),
            Arc::new(InMemoryMetadataBackend::new()),
        )
    }

    #[tokio::test]
    async fn session_save_missing_id_errors() {
        let r = router_offline();
        let e = session_save(&r, json!({"data": {}}))
            .await
            .expect_err("missing id");
        assert!(!e.to_string().is_empty());
    }

    #[tokio::test]
    async fn session_save_empty_id_errors() {
        let r = router_offline();
        let e = session_save(&r, json!({"id": "", "data": {}}))
            .await
            .expect_err("empty id");
        assert!(!e.to_string().is_empty());
    }

    #[tokio::test]
    async fn session_load_missing_id_errors() {
        let r = router_offline();
        let e = session_load(&r, json!({})).await.expect_err("missing id");
        assert!(!e.to_string().is_empty());
    }

    #[tokio::test]
    async fn session_load_empty_id_errors() {
        let r = router_offline();
        let e = session_load(&r, json!({"id": ""}))
            .await
            .expect_err("empty id");
        assert!(!e.to_string().is_empty());
    }

    #[tokio::test]
    async fn session_delete_missing_id_errors() {
        let r = router_offline();
        let e = session_delete(&r, json!({})).await.expect_err("missing id");
        assert!(!e.to_string().is_empty());
    }

    #[tokio::test]
    async fn session_delete_empty_id_errors() {
        let r = router_offline();
        let e = session_delete(&r, json!({"id": ""}))
            .await
            .expect_err("empty id");
        assert!(!e.to_string().is_empty());
    }

    #[tokio::test]
    async fn session_list_default_prefix_ok_without_server() {
        let r = router_offline();
        let v = session_list(&r, json!({})).await.expect("list");
        assert_eq!(v["count"], 0);
        assert_eq!(v["prefix"], SESSION_INDEX_PREFIX);
    }

    #[tokio::test]
    async fn session_roundtrip_with_tarpc_server() {
        let (addr, server_handle) = spawn_local_tarpc_server().await;
        let endpoint = format!("tarpc://{}", addr);
        let client = Arc::new(NestGateRpcClient::new(&endpoint).expect("client"));
        let router =
            SemanticRouter::with_metadata_backend(client, Arc::new(InMemoryMetadataBackend::new()));

        router
            .call_method(
                "storage.dataset.create",
                json!({"name": SESSION_DATASET, "description": "sessions"}),
            )
            .await
            .expect("create session dataset");

        let sid = "test-session-1";
        let saved = router
            .call_method(
                "session.save",
                json!({
                    "id": sid,
                    "data": {"role": "user", "n": 1}
                }),
            )
            .await
            .expect("session.save");
        assert_eq!(saved["status"], "saved");
        assert_eq!(saved["id"], sid);

        let loaded = router
            .call_method("session.load", json!({"id": sid}))
            .await
            .expect("session.load");
        assert_eq!(loaded["id"], sid);
        assert_eq!(loaded["data"]["role"], "user");
        assert!(loaded["created_at"].as_str().is_some());
        assert!(loaded["updated_at"].as_str().is_some());

        let listed = router
            .call_method("session.list", json!({}))
            .await
            .expect("session.list");
        assert_eq!(listed["count"], 1);
        let arr = listed["sessions"].as_array().expect("sessions");
        assert_eq!(arr[0]["id"], sid);

        let deleted = router
            .call_method("session.delete", json!({"id": sid}))
            .await
            .expect("session.delete");
        assert_eq!(deleted["id"], sid);

        let listed2 = router
            .call_method("session.list", json!({}))
            .await
            .expect("session.list after delete");
        assert_eq!(listed2["count"], 0);

        server_handle.abort();
    }

    #[tokio::test]
    async fn session_save_twice_preserves_created_at() {
        let (addr, server_handle) = spawn_local_tarpc_server().await;
        let endpoint = format!("tarpc://{}", addr);
        let client = Arc::new(NestGateRpcClient::new(&endpoint).expect("client"));
        let router =
            SemanticRouter::with_metadata_backend(client, Arc::new(InMemoryMetadataBackend::new()));

        router
            .call_method(
                "storage.dataset.create",
                json!({"name": SESSION_DATASET, "description": "sessions"}),
            )
            .await
            .expect("create session dataset");

        let sid = "created-at-stable";
        let first = router
            .call_method("session.save", json!({"id": sid, "data": {"k": 1}}))
            .await
            .expect("session.save 1");
        let created_first = first["created_at"]
            .as_str()
            .expect("created_at")
            .to_string();

        tokio::time::sleep(std::time::Duration::from_millis(30)).await;

        let second = router
            .call_method("session.save", json!({"id": sid, "data": {"k": 2}}))
            .await
            .expect("session.save 2");
        assert_eq!(
            second["created_at"].as_str().expect("created_at 2"),
            created_first.as_str(),
            "created_at must not change on update"
        );
        assert_ne!(
            second["updated_at"].as_str(),
            second["created_at"].as_str(),
            "updated_at should differ from created_at after second save"
        );

        let loaded = router
            .call_method("session.load", json!({"id": sid}))
            .await
            .expect("load");
        assert_eq!(
            loaded["created_at"].as_str().expect("loaded created"),
            created_first.as_str()
        );

        server_handle.abort();
    }

    #[tokio::test]
    async fn session_list_respects_prefix_filter() {
        let (addr, server_handle) = spawn_local_tarpc_server().await;
        let endpoint = format!("tarpc://{}", addr);
        let client = Arc::new(NestGateRpcClient::new(&endpoint).expect("client"));
        let router =
            SemanticRouter::with_metadata_backend(client, Arc::new(InMemoryMetadataBackend::new()));

        router
            .call_method(
                "storage.dataset.create",
                json!({"name": SESSION_DATASET, "description": "sessions"}),
            )
            .await
            .expect("create session dataset");

        for id in ["alpha-one", "alpha-two", "beta"] {
            router
                .call_method("session.save", json!({"id": id, "data": {}}))
                .await
                .expect("save");
        }

        let filtered = router
            .call_method(
                "session.list",
                json!({"prefix": format!("{SESSION_INDEX_PREFIX}alpha")}),
            )
            .await
            .expect("list filtered");
        assert_eq!(filtered["count"], 2);
        let ids: Vec<_> = filtered["sessions"]
            .as_array()
            .expect("sessions")
            .iter()
            .map(|v| v["id"].as_str().expect("id"))
            .collect();
        assert!(ids.contains(&"alpha-one"));
        assert!(ids.contains(&"alpha-two"));

        server_handle.abort();
    }

    #[tokio::test]
    async fn session_delete_missing_is_idempotent() {
        let (addr, server_handle) = spawn_local_tarpc_server().await;
        let endpoint = format!("tarpc://{}", addr);
        let client = Arc::new(NestGateRpcClient::new(&endpoint).expect("client"));
        let router =
            SemanticRouter::with_metadata_backend(client, Arc::new(InMemoryMetadataBackend::new()));

        router
            .call_method(
                "storage.dataset.create",
                json!({"name": SESSION_DATASET, "description": "sessions"}),
            )
            .await
            .expect("create session dataset");

        let gone = session_delete(&router, json!({"id": "never-created-session-id"}))
            .await
            .expect("idempotent delete");
        assert!(gone["success"].as_bool().unwrap());

        server_handle.abort();
    }
}

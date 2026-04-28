// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Session JSON-RPC Handlers
//!
//! Convenience methods `session.save` and `session.load` for game session persistence.
//! Sessions are stored under the `_sessions` namespace within a family's dataset,
//! backed by the same durable filesystem as `storage.*`.

use nestgate_config::config::storage_paths::get_storage_base_path;
use nestgate_types::error::{NestGateError, Result};
use serde_json::{Value, json};
use std::path::PathBuf;
use tracing::debug;

use super::StorageState;
use super::storage_handlers::resolve_family_id;

/// Build the filesystem path for a session key.
///
/// Sessions live at `{base}/datasets/{family}/_sessions/{session_id}`.
fn session_path(family_id: &str, session_id: &str) -> PathBuf {
    get_storage_base_path()
        .join("datasets")
        .join(family_id)
        .join("_sessions")
        .join(session_id)
}

/// session.save - Persist a game session snapshot
pub(super) async fn session_save(params: Option<&Value>, state: &StorageState) -> Result<Value> {
    let params = params
        .ok_or_else(|| NestGateError::invalid_input_with_field("params", "params required"))?;

    let session_id = params["session_id"].as_str().ok_or_else(|| {
        NestGateError::invalid_input_with_field("session_id", "session_id (string) required")
    })?;

    let data = params
        .get("data")
        .filter(|v| !v.is_null())
        .or_else(|| params.get("state").filter(|v| !v.is_null()))
        .ok_or_else(|| {
            NestGateError::invalid_input_with_field("data", "data or state (json) required")
        })?;

    let family_id = resolve_family_id(params, state)?;

    let bytes = serde_json::to_vec_pretty(data)
        .map_err(|e| NestGateError::io_error(format!("Failed to serialize session: {e}")))?;

    debug!(
        "session.save: family_id='{}', session_id='{}', size={} bytes",
        family_id,
        session_id,
        bytes.len()
    );

    let path = session_path(family_id, session_id);
    if let Some(parent) = path.parent() {
        tokio::fs::create_dir_all(parent).await.map_err(|e| {
            NestGateError::io_error(format!("Failed to create session directory: {e}"))
        })?;
    }
    tokio::fs::write(&path, &bytes).await.map_err(|e| {
        NestGateError::io_error(format!("Failed to write session {session_id}: {e}"))
    })?;

    Ok(json!({
        "status": "saved",
        "session_id": session_id,
        "family_id": family_id,
        "size": bytes.len()
    }))
}

/// session.load - Load a previously saved game session
pub(super) async fn session_load(params: Option<&Value>, state: &StorageState) -> Result<Value> {
    let params = params
        .ok_or_else(|| NestGateError::invalid_input_with_field("params", "params required"))?;

    let session_id = params["session_id"].as_str().ok_or_else(|| {
        NestGateError::invalid_input_with_field("session_id", "session_id (string) required")
    })?;

    let family_id = resolve_family_id(params, state)?;

    debug!(
        "session.load: family_id='{}', session_id='{}'",
        family_id, session_id
    );

    let path = session_path(family_id, session_id);
    if !path.exists() {
        return Ok(json!({
            "data": null,
            "session_id": session_id,
            "found": false
        }));
    }

    let bytes = tokio::fs::read(&path).await.map_err(|e| {
        NestGateError::io_error(format!("Failed to read session {session_id}: {e}"))
    })?;
    let data: Value = serde_json::from_slice(&bytes)
        .unwrap_or_else(|_| Value::String(String::from_utf8_lossy(&bytes).into_owned()));

    Ok(json!({
        "data": data,
        "session_id": session_id,
        "family_id": family_id,
        "found": true,
        "size": bytes.len()
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rpc::audit_storage::AuditStorage;
    use crate::rpc::template_storage::TemplateStorage;

    async fn mock_state(family_id: Option<&str>) -> StorageState {
        StorageState {
            templates: TemplateStorage::new(),
            audits: AuditStorage::new(),
            family_id: family_id.map(String::from),
            storage_initialized: true,
            encryption: None,
        }
    }

    #[tokio::test]
    async fn session_save_and_load_round_trip() {
        let state = mock_state(Some("test-session")).await;
        let family_id = format!("test-session-{}", uuid::Uuid::new_v4());

        let save_params = json!({
            "family_id": &family_id,
            "session_id": "save1",
            "data": {"level": 5, "score": 9001, "checkpoint": "boss_room"}
        });
        let save_result = session_save(Some(&save_params), &state).await;
        assert!(save_result.is_ok(), "save failed: {save_result:?}");
        assert_eq!(save_result.unwrap()["status"], "saved");

        let load_params = json!({"family_id": &family_id, "session_id": "save1"});
        let load_result = session_load(Some(&load_params), &state).await;
        assert!(load_result.is_ok(), "load failed: {load_result:?}");
        let loaded = load_result.unwrap();
        assert_eq!(loaded["found"], true);
        assert_eq!(loaded["data"]["level"], 5);
        assert_eq!(loaded["data"]["score"], 9001);

        let _ =
            tokio::fs::remove_dir_all(get_storage_base_path().join("datasets").join(&family_id))
                .await;
    }

    #[tokio::test]
    async fn session_load_missing_returns_null() {
        let state = mock_state(Some("test-session")).await;
        let family_id = format!("test-session-miss-{}", uuid::Uuid::new_v4());

        let load_params = json!({"family_id": &family_id, "session_id": "nonexistent"});
        let result = session_load(Some(&load_params), &state).await.unwrap();
        assert_eq!(result["found"], false);
        assert!(result["data"].is_null());
    }

    #[tokio::test]
    async fn session_save_requires_session_id() {
        let state = mock_state(Some("test")).await;
        let params = json!({"family_id": "f", "data": {}});
        assert!(session_save(Some(&params), &state).await.is_err());
    }

    #[tokio::test]
    async fn session_save_requires_data() {
        let state = mock_state(Some("test")).await;
        let params = json!({"family_id": "f", "session_id": "s"});
        assert!(session_save(Some(&params), &state).await.is_err());
    }

    #[tokio::test]
    async fn session_save_accepts_state_field() {
        let state = mock_state(Some("test")).await;
        let family_id = format!("test-session-state-{}", uuid::Uuid::new_v4());

        let params = json!({
            "family_id": &family_id,
            "session_id": "s1",
            "state": {"hp": 100}
        });
        let result = session_save(Some(&params), &state).await;
        assert!(result.is_ok());

        let load_params = json!({"family_id": &family_id, "session_id": "s1"});
        let loaded = session_load(Some(&load_params), &state).await.unwrap();
        assert_eq!(loaded["data"]["hp"], 100);

        let _ =
            tokio::fs::remove_dir_all(get_storage_base_path().join("datasets").join(&family_id))
                .await;
    }
}

// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Ionic Bond Ledger JSON-RPC Handlers
//!
//! `bonding.ledger.store`, `bonding.ledger.retrieve`, `bonding.ledger.list` provide
//! durable persistence for ionic bond contracts on behalf of the security capability
//! provider. Records are scoped under `{base}/datasets/{family}/__bonding/`.

use nestgate_config::config::storage_paths::get_storage_base_path;
use nestgate_types::error::{NestGateError, Result};
use serde_json::{Value, json};
use std::path::PathBuf;
use tracing::debug;

use super::StorageState;
use super::storage_handlers::{ensure_parent_dirs, resolve_family_id};

/// Build the filesystem path for a bonding ledger record.
///
/// Bond records live under `{base}/datasets/{family}/__bonding/{contract_id}/{record_type}.json`.
fn bonding_record_path(family_id: &str, contract_id: &str, record_type: &str) -> PathBuf {
    get_storage_base_path()
        .join("datasets")
        .join(family_id)
        .join("__bonding")
        .join(contract_id)
        .join(format!("{record_type}.json"))
}

/// `bonding.ledger.store` — persist an ionic bond record.
///
/// Stores a bond record (proposal, active contract, or provenance seal) under a
/// dedicated `__bonding` namespace. The security capability provider
/// uses this to durably persist bond lifecycle events.
pub(super) async fn bonding_ledger_store(
    params: Option<&Value>,
    state: &StorageState,
) -> Result<Value> {
    let params = params
        .ok_or_else(|| NestGateError::invalid_input_with_field("params", "params required"))?;

    let contract_id = params["contract_id"].as_str().ok_or_else(|| {
        NestGateError::invalid_input_with_field("contract_id", "contract_id (string) required")
    })?;
    let record_type = params["record_type"].as_str().unwrap_or("contract");
    let family_id = resolve_family_id(params, state)?;

    let data = params.get("data").ok_or_else(|| {
        NestGateError::invalid_input_with_field("data", "data (json object) required")
    })?;

    debug!(
        "bonding.ledger.store: family='{}', contract='{}', type='{}'",
        family_id, contract_id, record_type
    );

    let record_path = bonding_record_path(family_id, contract_id, record_type);
    ensure_parent_dirs(&record_path).await?;

    let bytes = serde_json::to_vec_pretty(data)
        .map_err(|e| NestGateError::io_error(format!("Failed to serialize bond record: {e}")))?;

    tokio::fs::write(&record_path, &bytes).await.map_err(|e| {
        NestGateError::io_error(format!(
            "Failed to write bond record {family_id}/{contract_id}/{record_type}: {e}"
        ))
    })?;

    Ok(json!({
        "status": "stored",
        "contract_id": contract_id,
        "record_type": record_type,
        "family_id": family_id,
        "size": bytes.len()
    }))
}

/// `bonding.ledger.retrieve` — retrieve an ionic bond record by contract ID.
pub(super) async fn bonding_ledger_retrieve(
    params: Option<&Value>,
    state: &StorageState,
) -> Result<Value> {
    let params = params
        .ok_or_else(|| NestGateError::invalid_input_with_field("params", "params required"))?;

    let contract_id = params["contract_id"].as_str().ok_or_else(|| {
        NestGateError::invalid_input_with_field("contract_id", "contract_id (string) required")
    })?;
    let record_type = params["record_type"].as_str().unwrap_or("contract");
    let family_id = resolve_family_id(params, state)?;

    let record_path = bonding_record_path(family_id, contract_id, record_type);
    if !record_path.exists() {
        return Ok(json!({
            "data": null,
            "contract_id": contract_id,
            "record_type": record_type,
            "family_id": family_id
        }));
    }

    let bytes = tokio::fs::read(&record_path).await.map_err(|e| {
        NestGateError::io_error(format!(
            "Failed to read bond record {family_id}/{contract_id}/{record_type}: {e}"
        ))
    })?;

    let data: Value = serde_json::from_slice(&bytes)
        .map_err(|e| NestGateError::io_error(format!("Corrupt bond record: {e}")))?;

    Ok(json!({
        "data": data,
        "contract_id": contract_id,
        "record_type": record_type,
        "family_id": family_id
    }))
}

/// `bonding.ledger.list` — list all bond contract IDs for a family.
///
/// Returns `{contracts: [{contract_id, record_types: [...]}]}`.
pub(super) async fn bonding_ledger_list(
    params: Option<&Value>,
    state: &StorageState,
) -> Result<Value> {
    let params = params
        .ok_or_else(|| NestGateError::invalid_input_with_field("params", "params required"))?;

    let family_id = resolve_family_id(params, state)?;
    let bonding_dir = get_storage_base_path()
        .join("datasets")
        .join(family_id)
        .join("__bonding");

    let mut contracts = Vec::new();

    if bonding_dir.exists() {
        let mut entries = tokio::fs::read_dir(&bonding_dir)
            .await
            .map_err(|e| NestGateError::io_error(format!("Failed to read bonding dir: {e}")))?;

        while let Ok(Some(entry)) = entries.next_entry().await {
            if entry.path().is_dir() {
                let contract_id = entry.file_name().to_string_lossy().into_owned();
                let mut record_types = Vec::new();

                if let Ok(mut sub_entries) = tokio::fs::read_dir(entry.path()).await {
                    while let Ok(Some(sub)) = sub_entries.next_entry().await {
                        let name = sub.file_name().to_string_lossy().into_owned();
                        if let Some(rt) = name.strip_suffix(".json") {
                            record_types.push(rt.to_owned());
                        }
                    }
                }

                contracts.push(json!({
                    "contract_id": contract_id,
                    "record_types": record_types
                }));
            }
        }
    }

    Ok(json!({
        "contracts": contracts,
        "count": contracts.len(),
        "family_id": family_id
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    async fn mock_state(family_id: Option<&str>) -> StorageState {
        StorageState {
            templates: crate::rpc::template_storage::TemplateStorage::new(),
            audits: crate::rpc::audit_storage::AuditStorage::new(),
            family_id: family_id.map(String::from),
            storage_initialized: true,
        }
    }

    #[tokio::test]
    async fn bonding_ledger_store_requires_contract_id() {
        let state = mock_state(Some("test")).await;
        let params = Some(json!({"family_id": "test", "data": {}}));
        assert!(bonding_ledger_store(params.as_ref(), &state).await.is_err());
    }

    #[tokio::test]
    async fn bonding_ledger_store_requires_data() {
        let state = mock_state(Some("test")).await;
        let params = Some(json!({"family_id": "test", "contract_id": "c1"}));
        assert!(bonding_ledger_store(params.as_ref(), &state).await.is_err());
    }

    #[tokio::test]
    async fn bonding_ledger_retrieve_missing_returns_null() {
        let state = mock_state(Some("test")).await;
        let params = Some(json!({"family_id": "test", "contract_id": "no-such-bond"}));
        let result = bonding_ledger_retrieve(params.as_ref(), &state)
            .await
            .unwrap();
        assert!(result["data"].is_null());
    }

    #[tokio::test]
    async fn bonding_ledger_round_trip() {
        let state = mock_state(Some("test-bond")).await;
        let family_id = format!("test-bond-{}", uuid::Uuid::new_v4());
        let contract_id = "bond-001";

        let proposal = json!({
            "proposer_family": &family_id,
            "acceptor_family": "other-family",
            "capabilities_offered": ["storage.retrieve"],
            "ttl_secs": 3600,
            "trust_model": "Contractual"
        });

        let store_p = json!({
            "family_id": &family_id,
            "contract_id": contract_id,
            "record_type": "proposal",
            "data": proposal
        });
        let store_result = bonding_ledger_store(Some(&store_p), &state).await;
        assert!(store_result.is_ok(), "bond store: {store_result:?}");
        assert_eq!(store_result.unwrap()["status"], "stored");

        let retrieve_p = json!({
            "family_id": &family_id,
            "contract_id": contract_id,
            "record_type": "proposal"
        });
        let retrieve_result = bonding_ledger_retrieve(Some(&retrieve_p), &state).await;
        assert!(retrieve_result.is_ok());
        let data = &retrieve_result.unwrap()["data"];
        assert_eq!(data["proposer_family"], family_id.as_str());
        assert_eq!(data["trust_model"], "Contractual");

        let list_p = json!({"family_id": &family_id});
        let list_result = bonding_ledger_list(Some(&list_p), &state).await;
        assert!(list_result.is_ok());
        let contracts = list_result.unwrap();
        assert_eq!(contracts["count"], 1);
        let items = contracts["contracts"].as_array().unwrap();
        assert_eq!(items[0]["contract_id"], contract_id);
        assert!(
            items[0]["record_types"]
                .as_array()
                .unwrap()
                .contains(&json!("proposal"))
        );

        let _ =
            tokio::fs::remove_dir_all(get_storage_base_path().join("datasets").join(&family_id))
                .await;
    }

    #[tokio::test]
    async fn bonding_ledger_multiple_record_types() {
        let state = mock_state(Some("test-bond-multi")).await;
        let family_id = format!("test-bond-multi-{}", uuid::Uuid::new_v4());
        let contract_id = "bond-multi";

        let proposal = json!({"family_id": &family_id, "contract_id": contract_id, "record_type": "proposal", "data": {"phase": "propose"}});
        assert!(bonding_ledger_store(Some(&proposal), &state).await.is_ok());

        let active = json!({"family_id": &family_id, "contract_id": contract_id, "record_type": "active", "data": {"phase": "active", "accepted_at": "2026-04-11"}});
        assert!(bonding_ledger_store(Some(&active), &state).await.is_ok());

        let seal = json!({"family_id": &family_id, "contract_id": contract_id, "record_type": "seal", "data": {"phase": "sealed", "merkle_root": "abc123"}});
        assert!(bonding_ledger_store(Some(&seal), &state).await.is_ok());

        let list_p = json!({"family_id": &family_id});
        let list_result = bonding_ledger_list(Some(&list_p), &state).await.unwrap();
        let items = list_result["contracts"].as_array().unwrap();
        assert_eq!(items.len(), 1);
        let record_types = items[0]["record_types"].as_array().unwrap();
        assert_eq!(record_types.len(), 3);

        let _ =
            tokio::fs::remove_dir_all(get_storage_base_path().join("datasets").join(&family_id))
                .await;
    }

    #[tokio::test]
    async fn bonding_ledger_list_empty_family() {
        let state = mock_state(Some("test-bond-empty")).await;
        let family_id = format!("test-bond-empty-{}", uuid::Uuid::new_v4());
        let list_p = json!({"family_id": &family_id});
        let result = bonding_ledger_list(Some(&list_p), &state).await.unwrap();
        assert_eq!(result["count"], 0);
        assert_eq!(result["contracts"].as_array().unwrap().len(), 0);
    }
}

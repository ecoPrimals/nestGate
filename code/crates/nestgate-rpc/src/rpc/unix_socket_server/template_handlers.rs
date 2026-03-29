// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Template JSON-RPC Handlers
//!
//! Extracted from `unix_socket_server` for domain-based refactoring.
//! Handles: templates.store, templates.retrieve, templates.list, `templates.community_top`

use nestgate_types::error::{NestGateError, Result};
use serde_json::{Value, json};
use tracing::debug;

use super::StorageState;

/// templates.store - Store graph template
pub(super) async fn templates_store(params: Option<&Value>, state: &StorageState) -> Result<Value> {
    let params = params
        .ok_or_else(|| NestGateError::invalid_input_with_field("params", "params required"))?;

    let name = params["name"]
        .as_str()
        .ok_or_else(|| NestGateError::invalid_input_with_field("name", "name (string) required"))?
        .to_string();
    let description = params["description"]
        .as_str()
        .ok_or_else(|| {
            NestGateError::invalid_input_with_field("description", "description (string) required")
        })?
        .to_string();
    let graph_data = params["graph_data"].clone();
    let user_id = params["user_id"]
        .as_str()
        .ok_or_else(|| {
            NestGateError::invalid_input_with_field("user_id", "user_id (string) required")
        })?
        .to_string();
    let family_id = params["family_id"]
        .as_str()
        .ok_or_else(|| {
            NestGateError::invalid_input_with_field("family_id", "family_id (string) required")
        })?
        .to_string();

    // Parse metadata
    let metadata = if let Some(meta_value) = params.get("metadata") {
        serde_json::from_value(meta_value.clone()).map_err(|e| {
            NestGateError::invalid_input_with_field(
                "metadata",
                format!("Invalid metadata format: {e}"),
            )
        })?
    } else {
        crate::rpc::template_storage::TemplateMetadata::default()
    };

    let (template_id, version) = state
        .templates
        .store_template(name, description, graph_data, user_id, family_id, metadata)
        .await?;

    debug!("Stored template '{}' (version {})", template_id, version);

    Ok(json!({
        "template_id": template_id,
        "version": version,
        "created_at": chrono::Utc::now().to_rfc3339(),
        "success": true
    }))
}

/// templates.retrieve - Retrieve graph template by ID
pub(super) async fn templates_retrieve(
    params: Option<&Value>,
    state: &StorageState,
) -> Result<Value> {
    let params = params
        .ok_or_else(|| NestGateError::invalid_input_with_field("params", "params required"))?;

    let template_id = params["template_id"].as_str().ok_or_else(|| {
        NestGateError::invalid_input_with_field("template_id", "template_id (string) required")
    })?;
    let family_id = params["family_id"].as_str().ok_or_else(|| {
        NestGateError::invalid_input_with_field("family_id", "family_id (string) required")
    })?;

    let template = state
        .templates
        .retrieve_template(template_id, family_id)
        .await?;

    debug!(
        "Retrieved template '{}' for family '{}'",
        template_id, family_id
    );

    serde_json::to_value(template)
        .map_err(|e| NestGateError::api(format!("Failed to serialize template: {e}")))
}

/// templates.list - List templates with filtering
pub(super) async fn templates_list(params: Option<&Value>, state: &StorageState) -> Result<Value> {
    let params = params
        .ok_or_else(|| NestGateError::invalid_input_with_field("params", "params required"))?;

    let family_id = params["family_id"].as_str().ok_or_else(|| {
        NestGateError::invalid_input_with_field("family_id", "family_id (string) required")
    })?;

    // Optional filters
    let user_id = params.get("user_id").and_then(|v| v.as_str());
    let niche_type = params.get("niche_type").and_then(|v| v.as_str());
    let is_community = params
        .get("is_community")
        .and_then(serde_json::Value::as_bool);

    let tags: Option<Vec<String>> = params.get("tags").and_then(|v| {
        v.as_array().map(|arr| {
            arr.iter()
                .filter_map(|t| t.as_str().map(String::from))
                .collect()
        })
    });

    let templates = state
        .templates
        .list_templates(
            family_id,
            user_id,
            tags.as_deref(),
            niche_type,
            is_community,
        )
        .await?;

    debug!(
        "Listed {} templates for family '{}' with filters",
        templates.len(),
        family_id
    );

    Ok(json!({
        "templates": templates,
        "total": templates.len()
    }))
}

/// `templates.community_top` - Get top community templates
pub(super) async fn templates_community_top(
    params: Option<&Value>,
    state: &StorageState,
) -> Result<Value> {
    let params = params
        .ok_or_else(|| NestGateError::invalid_input_with_field("params", "params required"))?;

    let niche_type = params.get("niche_type").and_then(|v| v.as_str());
    let limit = params
        .get("limit")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(10) as usize;
    let min_usage = params
        .get("min_usage")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);

    let top_templates = state
        .templates
        .get_community_top(niche_type, limit, min_usage)
        .await?;

    let result: Vec<Value> = top_templates
        .into_iter()
        .map(|(template, score)| {
            json!({
                "id": template.id,
                "name": template.name,
                "description": template.description,
                "score": score,
                "usage_count": template.metadata.usage_count,
                "success_rate": template.metadata.success_rate,
                "community_rating": template.metadata.community_rating,
                "rating_count": template.metadata.rating_count,
                "metadata": {
                    "tags": template.metadata.tags,
                    "niche_type": template.metadata.niche_type
                }
            })
        })
        .collect();

    debug!(
        "Retrieved {} top community templates (niche: {:?})",
        result.len(),
        niche_type
    );

    Ok(json!({
        "templates": result
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rpc::template_storage::TemplateMetadata;
    use serde_json::json;

    fn test_state() -> StorageState {
        StorageState {
            templates: crate::rpc::template_storage::TemplateStorage::new(),
            audits: crate::rpc::audit_storage::AuditStorage::new(),
            family_id: None,
            storage_initialized: true,
        }
    }

    #[tokio::test]
    async fn templates_store_missing_params_errors() {
        let state = test_state();
        let r = templates_store(None, &state).await;
        assert!(r.is_err());
    }

    #[tokio::test]
    async fn templates_store_roundtrip() {
        let state = test_state();
        let params = json!({
            "name": "tpl1",
            "description": "d",
            "graph_data": {"nodes": []},
            "user_id": "u1",
            "family_id": "fam1",
            "metadata": {"tags": ["a"], "niche_type": "web", "is_community": false}
        });
        let out = templates_store(Some(&params), &state).await.unwrap();
        assert_eq!(out["success"], true);
        let tid = out["template_id"].as_str().unwrap();

        let retrieve_p = json!({"template_id": tid, "family_id": "fam1"});
        let got = templates_retrieve(Some(&retrieve_p), &state).await.unwrap();
        assert_eq!(got["name"], "tpl1");

        let list_p = json!({"family_id": "fam1"});
        let listed = templates_list(Some(&list_p), &state).await.unwrap();
        assert_eq!(listed["total"], 1);
    }

    #[tokio::test]
    async fn templates_community_top_empty() {
        let state = test_state();
        let top_p = json!({"limit": 5});
        let out = templates_community_top(Some(&top_p), &state).await.unwrap();
        assert_eq!(out["templates"].as_array().unwrap().len(), 0);
    }

    #[tokio::test]
    async fn templates_store_default_metadata() {
        let state = test_state();
        let params = json!({
            "name": "n",
            "description": "d",
            "graph_data": {},
            "user_id": "u",
            "family_id": "f"
        });
        let out = templates_store(Some(&params), &state).await.unwrap();
        let tid = out["template_id"].as_str().unwrap();
        let retrieve_p = json!({"template_id": tid, "family_id": "f"});
        let got = templates_retrieve(Some(&retrieve_p), &state).await.unwrap();
        let meta: TemplateMetadata = serde_json::from_value(got["metadata"].clone()).unwrap();
        assert!(meta.tags.is_empty());
    }
}

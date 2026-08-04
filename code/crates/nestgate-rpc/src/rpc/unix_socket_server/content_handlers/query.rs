// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! `content.query` — filter CAS objects by sidecar metadata fields.
//!
//! Scans `_content/{prefix}/` directories and reads `.meta.json` sidecars,
//! returning objects whose metadata matches the requested filters.

use nestgate_config::config::storage_paths::get_storage_base_path;
use nestgate_types::error::{NestGateError, Result};
use serde_json::{Value, json};
use tracing::debug;

use super::super::storage_paths::resolve_family_id;
use super::super::StorageState;
use super::{SIDECAR_PROVENANCE_KEYS, merge_sidecar_fields};

/// `content.query` — filter content-addressed objects by sidecar metadata.
///
/// ## Parameters
///
/// | field         | type   | required | description                          |
/// |---------------|--------|----------|--------------------------------------|
/// | `content_type`| string | no       | Exact match on stored content type   |
/// | `source`      | string | no       | Exact match on provenance source     |
/// | `pipeline`    | string | no       | Exact match on provenance pipeline   |
/// | `stored_by`   | string | no       | Exact match on stored-by identity    |
/// | `parent_hash` | string | no       | Exact match on parent hash           |
/// | `limit`       | u64    | no       | Max results (default 100)            |
/// | `offset`      | u64    | no       | Skip N matching results              |
/// | `family_id`   | string | no       | Family namespace                     |
///
/// At least one filter field must be provided. Returns matching objects with
/// their full sidecar metadata.
pub async fn content_query(params: Option<&Value>, state: &StorageState) -> Result<Value> {
    let empty = json!({});
    let params = params.unwrap_or(&empty);
    let family_id = resolve_family_id(params, state)?;

    let filters = extract_filters(params)?;
    if filters.is_empty() {
        return Err(NestGateError::invalid_input_with_field(
            "filter",
            "at least one filter field required (content_type, source, pipeline, stored_by, parent_hash)",
        ));
    }

    let limit = usize::try_from(params["limit"].as_u64().unwrap_or(100).min(10_000))
        .unwrap_or(100);
    let offset = usize::try_from(params["offset"].as_u64().unwrap_or(0)).unwrap_or(0);

    let content_dir = get_storage_base_path()
        .join("datasets")
        .join(family_id)
        .join("_content");

    let mut matches: Vec<Value> = Vec::new();
    let mut scanned: usize = 0;
    let mut skipped: usize = 0;

    if !content_dir.exists() {
        return Ok(json!({
            "results": [],
            "count": 0,
            "scanned": 0,
            "family_id": family_id,
        }));
    }

    let mut prefix_dirs = tokio::fs::read_dir(&content_dir).await.map_err(|e| {
        NestGateError::io_error(format!("Failed to list content for {family_id}: {e}"))
    })?;

    'outer: while let Ok(Some(prefix_entry)) = prefix_dirs.next_entry().await {
        if !prefix_entry
            .file_type()
            .await
            .map(|ft| ft.is_dir())
            .unwrap_or(false)
        {
            continue;
        }

        let mut entries = tokio::fs::read_dir(prefix_entry.path())
            .await
            .map_err(|e| {
                NestGateError::io_error(format!("Failed to read content prefix dir: {e}"))
            })?;

        while let Ok(Some(entry)) = entries.next_entry().await {
            let name = entry.file_name();
            let name_str = name.to_string_lossy();
            if name_str.ends_with(".meta.json") || name_str.starts_with('.') {
                continue;
            }

            let meta_path = entry.path().with_extension("meta.json");
            let blob_name = format!("{name_str}.meta.json");
            let meta_path_alt = entry.path().parent().map(|p| p.join(&blob_name));

            let sidecar_path = if meta_path.exists() {
                meta_path
            } else if let Some(ref alt) = meta_path_alt
                && alt.exists()
            {
                alt.clone()
            } else {
                continue;
            };

            scanned += 1;

            let sidecar_bytes = match tokio::fs::read(&sidecar_path).await {
                Ok(b) => b,
                Err(e) => {
                    debug!(hash = %name_str, error = %e, "skipping unreadable sidecar");
                    continue;
                }
            };

            let sidecar: Value = match serde_json::from_slice(&sidecar_bytes) {
                Ok(v) => v,
                Err(e) => {
                    debug!(hash = %name_str, error = %e, "skipping malformed sidecar");
                    continue;
                }
            };

            if !matches_filters(&sidecar, &filters) {
                continue;
            }

            if skipped < offset {
                skipped += 1;
                continue;
            }

            let size = entry.metadata().await.map(|m| m.len()).unwrap_or(0);
            let mut result = json!({
                "hash": &*name_str,
                "size": size,
            });
            merge_sidecar_fields(&mut result, &sidecar);

            matches.push(result);
            if matches.len() >= limit {
                break 'outer;
            }
        }
    }

    let count = matches.len();
    Ok(json!({
        "results": matches,
        "count": count,
        "scanned": scanned,
        "family_id": family_id,
    }))
}

/// Extract filter key-value pairs from query parameters.
fn extract_filters(params: &Value) -> Result<Vec<(&'static str, String)>> {
    let filterable: &[&str] = &["content_type", "source", "pipeline", "stored_by", "parent_hash"];
    let mut filters = Vec::new();
    for key in filterable {
        if let Some(val) = params[*key].as_str() {
            let static_key = SIDECAR_PROVENANCE_KEYS
                .iter()
                .find(|k| **k == *key)
                .copied()
                .unwrap_or(key);
            filters.push((static_key, val.to_string()));
        }
    }
    Ok(filters)
}

/// Check whether a sidecar matches all filter predicates.
fn matches_filters(sidecar: &Value, filters: &[(&str, String)]) -> bool {
    filters.iter().all(|(key, expected)| {
        sidecar
            .get(*key)
            .and_then(Value::as_str)
            .is_some_and(|v| v == expected)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_filters_picks_known_keys() {
        let params = json!({"content_type": "text/plain", "source": "ci", "unknown": "x"});
        let filters = extract_filters(&params).unwrap();
        assert_eq!(filters.len(), 2);
        assert!(filters.iter().any(|(k, v)| *k == "content_type" && v == "text/plain"));
        assert!(filters.iter().any(|(k, v)| *k == "source" && v == "ci"));
    }

    #[test]
    fn extract_filters_empty_when_no_keys() {
        let params = json!({"family_id": "test"});
        let filters = extract_filters(&params).unwrap();
        assert!(filters.is_empty());
    }

    #[test]
    fn matches_filters_all_must_match() {
        let sidecar = json!({"content_type": "text/plain", "source": "ci", "pipeline": "build"});
        let filters = vec![("content_type", "text/plain".into()), ("source", "ci".into())];
        assert!(matches_filters(&sidecar, &filters));

        let filters_mismatch = vec![("content_type", "text/plain".into()), ("source", "manual".into())];
        assert!(!matches_filters(&sidecar, &filters_mismatch));
    }

    #[test]
    fn matches_filters_missing_key_fails() {
        let sidecar = json!({"content_type": "text/plain"});
        let filters = vec![("source", "ci".into())];
        assert!(!matches_filters(&sidecar, &filters));
    }

    #[test]
    fn matches_filters_empty_always_matches() {
        let sidecar = json!({"content_type": "text/plain"});
        let filters: Vec<(&str, String)> = vec![];
        assert!(matches_filters(&sidecar, &filters));
    }
}

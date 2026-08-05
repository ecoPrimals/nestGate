// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! `dataset.convergence` — report CAS provenance state per dataset path.
//!
//! For a given filesystem path, walks all regular files, BLAKE3 hashes each,
//! and checks whether the hash exists in the CAS store. Reports a convergence
//! classification matching the westGate sweep taxonomy:
//!
//! | State | Meaning |
//! |-------|---------|
//! | `CONVERGED` | 100% of files have matching CAS objects |
//! | `PARTIAL` | Some files CAS'd, some not |
//! | `PRIMORDIAL` | No files have CAS objects |
//! | `EMPTY` | Directory contains no regular files |
//! | `MISSING` | Directory does not exist |
//!
//! Designed as the trust gate for spring data consumption — springs can
//! call `dataset.convergence` to verify data integrity before processing.

use nestgate_types::error::{NestGateError, Result};
use serde_json::{Value, json};
use std::io::Read;
use std::path::{Path, PathBuf};
use tracing::debug;

use super::StorageState;
use super::storage_paths::{content_key_path, resolve_family_id};

/// `dataset.convergence` — report provenance state per dataset path.
///
/// ## Parameters
///
/// | Field | Type | Required | Description |
/// |-------|------|----------|-------------|
/// | `path` | string | yes | Absolute filesystem path of the dataset |
/// | `family_id` | string | no | CAS family (default from server state) |
/// | `include_details` | bool | no | Include per-file details (default: false) |
/// | `sample_limit` | u64 | no | Max files to scan (default: unlimited) |
///
/// ## Returns
///
/// ```json
/// {
///   "state": "PARTIAL",
///   "path": "/mnt/nestgate/datasets/gps",
///   "total_files": 1500,
///   "total_bytes": 52428800,
///   "cas_files": 1200,
///   "cas_bytes": 41943040,
///   "missing_files": 300,
///   "convergence_pct": 80.0,
///   "family_id": "my-family",
///   "scanned_at": "2026-08-05T14:00:00Z"
/// }
/// ```
pub async fn dataset_convergence(params: Option<&Value>, state: &StorageState) -> Result<Value> {
    let params = params
        .ok_or_else(|| NestGateError::invalid_input_with_field("params", "params required"))?;

    let path_str = params["path"]
        .as_str()
        .ok_or_else(|| {
            NestGateError::invalid_input_with_field(
                "path",
                "path (absolute filesystem path) required",
            )
        })?
        .to_owned();

    let dir_path = PathBuf::from(&path_str);
    if !dir_path.is_absolute() {
        return Err(NestGateError::invalid_input_with_field(
            "path",
            "must be an absolute path",
        ));
    }

    let family_id = resolve_family_id(params, state)?.to_owned();
    let include_details = params["include_details"].as_bool().unwrap_or(false);
    let sample_limit = params["sample_limit"]
        .as_u64()
        .map(|n| usize::try_from(n).unwrap_or(usize::MAX));

    tokio::task::spawn_blocking(move || {
        do_convergence_scan(&dir_path, &family_id, include_details, sample_limit)
    })
    .await
    .map_err(|e| NestGateError::io_error(format!("Convergence scan panicked: {e}")))?
}

/// Convergence state classification.
const fn classify_state(total: usize, cas_count: usize) -> &'static str {
    if total == 0 {
        "EMPTY"
    } else if cas_count == total {
        "CONVERGED"
    } else if cas_count == 0 {
        "PRIMORDIAL"
    } else {
        "PARTIAL"
    }
}

fn do_convergence_scan(
    root: &Path,
    family_id: &str,
    include_details: bool,
    sample_limit: Option<usize>,
) -> Result<Value> {
    if !root.exists() {
        return Ok(json!({
            "state": "MISSING",
            "path": root.display().to_string(),
            "total_files": 0,
            "total_bytes": 0,
            "cas_files": 0,
            "cas_bytes": 0,
            "missing_files": 0,
            "convergence_pct": 0.0,
            "family_id": family_id,
            "scanned_at": chrono::Utc::now().to_rfc3339(),
        }));
    }

    if !root.is_dir() {
        return Err(NestGateError::invalid_input_with_field(
            "path",
            format!("not a directory: {}", root.display()),
        ));
    }

    debug!(
        "dataset.convergence: scanning {} (family={family_id})",
        root.display()
    );

    let mut total_files: usize = 0;
    let mut total_bytes: u64 = 0;
    let mut cas_files: usize = 0;
    let mut cas_bytes: u64 = 0;
    let mut converged: Vec<Value> = Vec::new();
    let mut unconverged: Vec<Value> = Vec::new();

    walk_for_convergence(
        root,
        root,
        family_id,
        include_details,
        sample_limit,
        &mut total_files,
        &mut total_bytes,
        &mut cas_files,
        &mut cas_bytes,
        &mut converged,
        &mut unconverged,
    );

    let missing_files = total_files.saturating_sub(cas_files);
    let convergence_pct = if total_files == 0 {
        0.0
    } else {
        #[expect(
            clippy::cast_precision_loss,
            reason = "file counts will never exceed f64 mantissa precision in practice"
        )]
        let pct = (cas_files as f64 / total_files as f64) * 100.0;
        (pct * 100.0).round() / 100.0
    };
    let state = classify_state(total_files, cas_files);

    debug!(
        "dataset.convergence: {state} — {cas_files}/{total_files} files \
         ({convergence_pct:.2}%), {cas_bytes}/{total_bytes} bytes"
    );

    let mut result = json!({
        "state": state,
        "path": root.display().to_string(),
        "total_files": total_files,
        "total_bytes": total_bytes,
        "cas_files": cas_files,
        "cas_bytes": cas_bytes,
        "missing_files": missing_files,
        "convergence_pct": convergence_pct,
        "family_id": family_id,
        "scanned_at": chrono::Utc::now().to_rfc3339(),
    });

    if include_details {
        result["converged_files"] = json!(converged);
        result["unconverged_files"] = json!(unconverged);
    }

    Ok(result)
}

#[expect(clippy::too_many_arguments, reason = "accumulator threading for recursive walk")]
fn walk_for_convergence(
    root: &Path,
    current: &Path,
    family_id: &str,
    include_details: bool,
    sample_limit: Option<usize>,
    total_files: &mut usize,
    total_bytes: &mut u64,
    cas_files: &mut usize,
    cas_bytes: &mut u64,
    converged: &mut Vec<Value>,
    unconverged: &mut Vec<Value>,
) {
    let Ok(entries) = std::fs::read_dir(current) else {
        return;
    };

    for entry_result in entries {
        if let Some(limit) = sample_limit
            && *total_files >= limit
        {
            return;
        }

        let Ok(entry) = entry_result else {
            continue;
        };
        let path = entry.path();

        let Ok(ft) = entry.file_type() else {
            continue;
        };

        if ft.is_dir() {
            walk_for_convergence(
                root,
                &path,
                family_id,
                include_details,
                sample_limit,
                total_files,
                total_bytes,
                cas_files,
                cas_bytes,
                converged,
                unconverged,
            );
            continue;
        }

        if !ft.is_file() {
            continue;
        }

        let size = entry.metadata().map(|m| m.len()).unwrap_or(0);
        *total_files += 1;
        *total_bytes += size;

        let relative = path
            .strip_prefix(root)
            .map_or_else(
                |_| path.display().to_string(),
                |r| r.to_string_lossy().replace('\\', "/"),
            );

        match hash_and_check_cas(&path, family_id) {
            Some(hash) => {
                *cas_files += 1;
                *cas_bytes += size;
                if include_details {
                    converged.push(json!({
                        "path": relative,
                        "hash": hash,
                        "size": size,
                    }));
                }
            }
            None => {
                if include_details {
                    unconverged.push(json!({
                        "path": relative,
                        "size": size,
                    }));
                }
            }
        }
    }
}

/// Hash a file and check CAS presence. Returns `Some(hash)` if in CAS.
fn hash_and_check_cas(path: &Path, family_id: &str) -> Option<String> {
    let mut file = std::fs::File::open(path).ok()?;
    let mut hasher = blake3::Hasher::new();
    let mut buf = vec![0u8; 64 * 1024];
    loop {
        let n = file.read(&mut buf).ok()?;
        if n == 0 {
            break;
        }
        hasher.update(&buf[..n]);
    }
    let hash_hex = hasher.finalize().to_hex().to_string();
    let cas_path = content_key_path(family_id, &hash_hex);
    if cas_path.exists() {
        Some(hash_hex)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classify_state_empty() {
        assert_eq!(classify_state(0, 0), "EMPTY");
    }

    #[test]
    fn classify_state_converged() {
        assert_eq!(classify_state(10, 10), "CONVERGED");
    }

    #[test]
    fn classify_state_primordial() {
        assert_eq!(classify_state(10, 0), "PRIMORDIAL");
    }

    #[test]
    fn classify_state_partial() {
        assert_eq!(classify_state(10, 5), "PARTIAL");
    }

    #[tokio::test]
    async fn convergence_rejects_relative_path() {
        let state = StorageState::new().unwrap();
        let params = json!({"path": "relative/path", "family_id": "test"});
        let result = dataset_convergence(Some(&params), &state).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn convergence_missing_directory() {
        let state = StorageState::new().unwrap();
        let params = json!({
            "path": "/tmp/nestgate_convergence_nonexistent_test_dir_99999",
            "family_id": "test"
        });
        let result = dataset_convergence(Some(&params), &state).await.unwrap();
        assert_eq!(result["state"], "MISSING");
        assert_eq!(result["total_files"], 0);
    }

    #[tokio::test]
    async fn convergence_empty_directory() {
        let dir = tempfile::tempdir().unwrap();
        let state = StorageState::new().unwrap();
        let params = json!({
            "path": dir.path().display().to_string(),
            "family_id": "test-conv"
        });
        let result = dataset_convergence(Some(&params), &state).await.unwrap();
        assert_eq!(result["state"], "EMPTY");
        assert_eq!(result["total_files"], 0);
        assert_eq!(result["convergence_pct"], 0.0);
    }

    #[tokio::test]
    async fn convergence_primordial_directory() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("file.txt"), b"test data").unwrap();

        let state = StorageState::new().unwrap();
        let params = json!({
            "path": dir.path().display().to_string(),
            "family_id": "test-conv-primordial"
        });
        let result = dataset_convergence(Some(&params), &state).await.unwrap();
        assert_eq!(result["state"], "PRIMORDIAL");
        assert_eq!(result["total_files"], 1);
        assert_eq!(result["cas_files"], 0);
        assert_eq!(result["missing_files"], 1);
    }

    #[tokio::test]
    async fn convergence_with_details() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("a.txt"), b"alpha").unwrap();
        std::fs::write(dir.path().join("b.txt"), b"beta").unwrap();

        let state = StorageState::new().unwrap();
        let params = json!({
            "path": dir.path().display().to_string(),
            "family_id": "test-conv-details",
            "include_details": true
        });
        let result = dataset_convergence(Some(&params), &state).await.unwrap();
        assert_eq!(result["state"], "PRIMORDIAL");
        assert!(result["unconverged_files"].as_array().unwrap().len() == 2);
        assert!(result["converged_files"].as_array().unwrap().is_empty());
    }

    #[tokio::test]
    async fn convergence_sample_limit() {
        let dir = tempfile::tempdir().unwrap();
        for i in 0..10 {
            std::fs::write(dir.path().join(format!("f{i}.txt")), format!("data{i}")).unwrap();
        }

        let state = StorageState::new().unwrap();
        let params = json!({
            "path": dir.path().display().to_string(),
            "family_id": "test-conv-limit",
            "sample_limit": 3
        });
        let result = dataset_convergence(Some(&params), &state).await.unwrap();
        assert!(result["total_files"].as_u64().unwrap() <= 3);
    }
}

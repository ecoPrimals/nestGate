// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! `content.ingest` — bulk directory scan → BLAKE3 hash → CAS store.
//!
//! Recursively walks a local directory, hashes every regular file with
//! BLAKE3, stores to CAS with automatic dedup, and returns a manifest of
//! `{relative_path: blake3_hex}` pairs plus aggregate statistics.
//!
//! Designed to eliminate `revalidate_data.py` and other multi-step
//! directory-to-CAS Python glue scripts.
//!
//! # Parameters
//!
//! | Field | Type | Required | Description |
//! |-------|------|----------|-------------|
//! | `directory` | string | yes | Absolute path to directory to ingest |
//! | `family_id` | string | no | CAS family (default from server state) |
//! | `source` | string | no | Provenance source field |
//! | `pipeline` | string | no | Provenance pipeline (default: `content.ingest`) |
//! | `stored_by` | string | no | Provenance `stored_by` (default: `nestgate`) |
//! | `follow_symlinks` | bool | no | Follow symlinks during walk (default: false) |
//! | `collection` | string | no | If set, publish manifest as this collection |
//!
//! # Returns
//!
//! ```json
//! {
//!   "manifest": {"relative/path": "<blake3_hex>", ...},
//!   "count": 42,
//!   "bytes_total": 123456,
//!   "bytes_stored": 100000,
//!   "deduplicated": 5,
//!   "errors": [],
//!   "family_id": "my-family",
//!   "directory": "/path/to/dir",
//!   "ingested_at": "2026-08-05T12:00:00Z"
//! }
//! ```

use nestgate_types::error::{NestGateError, Result};
use serde_json::{Value, json};
use std::collections::BTreeMap;
use std::io::Read;
use std::path::{Path, PathBuf};
use tracing::{debug, warn};

use super::super::StorageState;
use super::super::storage_paths::{content_key_path, resolve_family_id};

/// Maximum file size for inline ingestion (256 MiB).
///
/// Files larger than this are skipped with a warning in `errors` rather
/// than consuming unbounded memory. Callers should use
/// `content.store_stream` for larger individual files.
const INLINE_MAX_FILE_SIZE: u64 = 256 * 1024 * 1024;

/// `content.ingest` — scan directory, hash all files, bulk store to CAS.
pub async fn content_ingest(params: Option<&Value>, state: &StorageState) -> Result<Value> {
    let params = params
        .ok_or_else(|| NestGateError::invalid_input_with_field("params", "params required"))?;

    let directory = params["directory"]
        .as_str()
        .ok_or_else(|| {
            NestGateError::invalid_input_with_field(
                "directory",
                "directory (absolute path string) required",
            )
        })?
        .to_owned();

    let dir_path = PathBuf::from(&directory);
    if !dir_path.is_absolute() {
        return Err(NestGateError::invalid_input_with_field(
            "directory",
            "must be an absolute path",
        ));
    }

    let family_id = resolve_family_id(params, state)?.to_owned();
    let source = params["source"]
        .as_str()
        .unwrap_or(&directory)
        .to_owned();
    let pipeline = params["pipeline"]
        .as_str()
        .unwrap_or("content.ingest")
        .to_owned();
    let stored_by = params["stored_by"]
        .as_str()
        .unwrap_or("nestgate")
        .to_owned();
    let follow_symlinks = params["follow_symlinks"].as_bool().unwrap_or(false);
    let collection = params["collection"].as_str().map(String::from);

    let result = tokio::task::spawn_blocking(move || {
        do_ingest(
            &dir_path,
            &family_id,
            &source,
            &pipeline,
            &stored_by,
            follow_symlinks,
        )
    })
    .await
    .map_err(|e| NestGateError::io_error(format!("Ingest task panicked: {e}")))?;

    let mut result = result?;

    if let Some(ref coll) = collection {
        let manifest_params = json!({
            "collection": coll,
            "manifest": result["manifest"],
            "family_id": result["family_id"],
        });
        match super::content_publish(Some(&manifest_params), state).await {
            Ok(pub_result) => {
                result["collection"] = json!(coll);
                result["collection_published"] = json!(true);
                result["collection_entry_count"] = pub_result
                    .get("entry_count")
                    .cloned()
                    .unwrap_or(json!(null));
            }
            Err(e) => {
                result["collection"] = json!(coll);
                result["collection_published"] = json!(false);
                result["collection_error"] = json!(e.to_string());
            }
        }
    }

    Ok(result)
}

/// Blocking directory walk → hash → CAS store.
fn do_ingest(
    root: &Path,
    family_id: &str,
    source: &str,
    pipeline: &str,
    stored_by: &str,
    follow_symlinks: bool,
) -> Result<Value> {
    if !root.exists() {
        return Err(NestGateError::invalid_input_with_field(
            "directory",
            format!("directory does not exist: {}", root.display()),
        ));
    }
    if !root.is_dir() {
        return Err(NestGateError::invalid_input_with_field(
            "directory",
            format!("path is not a directory: {}", root.display()),
        ));
    }

    debug!(
        "content.ingest: scanning {} (family={family_id})",
        root.display()
    );

    let mut manifest: BTreeMap<String, String> = BTreeMap::new();
    let mut errors: Vec<Value> = Vec::new();
    let mut bytes_total: u64 = 0;
    let mut bytes_stored: u64 = 0;
    let mut dedup_count: u64 = 0;

    walk_directory(
        root,
        root,
        follow_symlinks,
        family_id,
        source,
        pipeline,
        stored_by,
        &mut manifest,
        &mut errors,
        &mut bytes_total,
        &mut bytes_stored,
        &mut dedup_count,
    )?;

    let count = manifest.len();

    debug!(
        "content.ingest: {count} files, {bytes_total} bytes total, \
         {bytes_stored} bytes stored, {dedup_count} deduplicated"
    );

    Ok(json!({
        "manifest": manifest,
        "count": count,
        "bytes_total": bytes_total,
        "bytes_stored": bytes_stored,
        "deduplicated": dedup_count,
        "errors": errors,
        "family_id": family_id,
        "directory": root.display().to_string(),
        "ingested_at": chrono::Utc::now().to_rfc3339(),
    }))
}

/// Recursively walk a directory and ingest each regular file.
#[expect(clippy::too_many_arguments, reason = "accumulator threading for recursive walk")]
fn walk_directory(
    root: &Path,
    current: &Path,
    follow_symlinks: bool,
    family_id: &str,
    source: &str,
    pipeline: &str,
    stored_by: &str,
    manifest: &mut BTreeMap<String, String>,
    errors: &mut Vec<Value>,
    bytes_total: &mut u64,
    bytes_stored: &mut u64,
    dedup_count: &mut u64,
) -> Result<()> {
    let entries = std::fs::read_dir(current).map_err(|e| {
        NestGateError::io_error(format!(
            "content.ingest: cannot read directory {}: {e}",
            current.display()
        ))
    })?;

    for entry_result in entries {
        let entry = match entry_result {
            Ok(e) => e,
            Err(e) => {
                errors.push(json!({
                    "path": current.display().to_string(),
                    "error": format!("readdir entry error: {e}"),
                }));
                continue;
            }
        };

        let path = entry.path();

        let file_type = if follow_symlinks {
            match std::fs::metadata(&path) {
                Ok(m) => m.file_type(),
                Err(e) => {
                    errors.push(json!({
                        "path": path.display().to_string(),
                        "error": format!("stat error: {e}"),
                    }));
                    continue;
                }
            }
        } else {
            match entry.file_type() {
                Ok(ft) => ft,
                Err(e) => {
                    errors.push(json!({
                        "path": path.display().to_string(),
                        "error": format!("file_type error: {e}"),
                    }));
                    continue;
                }
            }
        };

        if file_type.is_dir() {
            walk_directory(
                root,
                &path,
                follow_symlinks,
                family_id,
                source,
                pipeline,
                stored_by,
                manifest,
                errors,
                bytes_total,
                bytes_stored,
                dedup_count,
            )?;
            continue;
        }

        if file_type.is_symlink() && !follow_symlinks {
            continue;
        }

        if !file_type.is_file() {
            continue;
        }

        let relative = if let Ok(rel) = path.strip_prefix(root) {
            rel.to_string_lossy().replace('\\', "/")
        } else {
            errors.push(json!({
                "path": path.display().to_string(),
                "error": "failed to compute relative path",
            }));
            continue;
        };

        match ingest_file(&path, family_id, source, pipeline, stored_by) {
            Ok(FileResult {
                hash,
                size,
                was_dedup,
            }) => {
                *bytes_total += size;
                if was_dedup {
                    *dedup_count += 1;
                } else {
                    *bytes_stored += size;
                }
                manifest.insert(relative, hash);
            }
            Err(e) => {
                errors.push(json!({
                    "path": path.display().to_string(),
                    "relative": relative,
                    "error": e.to_string(),
                }));
            }
        }
    }

    Ok(())
}

struct FileResult {
    hash: String,
    size: u64,
    was_dedup: bool,
}

fn ingest_file(
    path: &Path,
    family_id: &str,
    source: &str,
    pipeline: &str,
    stored_by: &str,
) -> Result<FileResult> {
    let meta = std::fs::metadata(path).map_err(|e| {
        NestGateError::io_error(format!(
            "content.ingest: stat failed for {}: {e}",
            path.display()
        ))
    })?;

    let size = meta.len();

    if size > INLINE_MAX_FILE_SIZE {
        return Err(NestGateError::io_error(format!(
            "file exceeds {} MiB limit (use content.store_stream): {}",
            INLINE_MAX_FILE_SIZE / (1024 * 1024),
            path.display()
        )));
    }

    let mut file = std::fs::File::open(path).map_err(|e| {
        NestGateError::io_error(format!(
            "content.ingest: open failed for {}: {e}",
            path.display()
        ))
    })?;

    let mut hasher = blake3::Hasher::new();
    let mut buf = vec![0u8; 64 * 1024];
    loop {
        let n = file.read(&mut buf).map_err(|e| {
            NestGateError::io_error(format!(
                "content.ingest: read error for {}: {e}",
                path.display()
            ))
        })?;
        if n == 0 {
            break;
        }
        hasher.update(&buf[..n]);
    }

    let hash_hex = hasher.finalize().to_hex().to_string();
    let cas_path = content_key_path(family_id, &hash_hex);

    if cas_path.exists() {
        debug!(
            "content.ingest: dedup hit {} → {hash_hex}",
            path.display()
        );
        return Ok(FileResult {
            hash: hash_hex,
            size,
            was_dedup: true,
        });
    }

    if let Some(parent) = cas_path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| {
            NestGateError::io_error(format!("content.ingest: mkdir failed: {e}"))
        })?;
    }

    std::fs::copy(path, &cas_path).map_err(|e| {
        NestGateError::io_error(format!(
            "content.ingest: copy to CAS failed for {}: {e}",
            path.display()
        ))
    })?;

    let sidecar = json!({
        "hash": hash_hex,
        "size": size,
        "content_type": guess_content_type(path),
        "stored_at": chrono::Utc::now().to_rfc3339(),
        "source": source,
        "pipeline": pipeline,
        "stored_by": stored_by,
    });
    let meta_path = cas_path.with_file_name(format!("{hash_hex}.meta.json"));
    if let Err(e) = std::fs::write(
        &meta_path,
        serde_json::to_vec_pretty(&sidecar).unwrap_or_default(),
    ) {
        warn!(
            "content.ingest: sidecar write failed for {hash_hex}: {e}"
        );
    }

    Ok(FileResult {
        hash: hash_hex,
        size,
        was_dedup: false,
    })
}

/// Best-effort MIME type from file extension.
fn guess_content_type(path: &Path) -> &'static str {
    let ext = path
        .extension()
        .and_then(std::ffi::OsStr::to_str)
        .unwrap_or("");
    match ext {
        "json" => "application/json",
        "csv" => "text/csv",
        "tsv" => "text/tab-separated-values",
        "txt" | "log" | "md" => "text/plain",
        "html" | "htm" => "text/html",
        "xml" => "application/xml",
        "yaml" | "yml" => "application/x-yaml",
        "toml" => "application/toml",
        "gz" | "gzip" => "application/gzip",
        "bz2" => "application/x-bzip2",
        "xz" => "application/x-xz",
        "zst" | "zstd" => "application/zstd",
        "tar" => "application/x-tar",
        "zip" => "application/zip",
        "pdf" => "application/pdf",
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "svg" => "image/svg+xml",
        "wasm" => "application/wasm",
        "js" => "application/javascript",
        "css" => "text/css",
        "parquet" => "application/x-parquet",
        "arrow" | "feather" => "application/vnd.apache.arrow.file",
        "h5" | "hdf5" => "application/x-hdf5",
        "npy" => "application/x-numpy",
        "pdb" => "chemical/x-pdb",
        "fasta" | "fa" => "application/x-fasta",
        "fastq" | "fq" => "application/x-fastq",
        "bed" => "application/x-bed",
        "vcf" => "text/x-vcard",
        "bam" => "application/x-bam",
        _ => "application/octet-stream",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn guess_content_type_json() {
        assert_eq!(guess_content_type(Path::new("data.json")), "application/json");
    }

    #[test]
    fn guess_content_type_unknown() {
        assert_eq!(
            guess_content_type(Path::new("file.xyz")),
            "application/octet-stream"
        );
    }

    #[test]
    fn guess_content_type_no_extension() {
        assert_eq!(
            guess_content_type(Path::new("README")),
            "application/octet-stream"
        );
    }

    #[test]
    fn guess_content_type_science_formats() {
        assert_eq!(guess_content_type(Path::new("protein.pdb")), "chemical/x-pdb");
        assert_eq!(guess_content_type(Path::new("reads.fastq")), "application/x-fastq");
        assert_eq!(guess_content_type(Path::new("data.parquet")), "application/x-parquet");
    }

    #[tokio::test]
    async fn ingest_rejects_relative_path() {
        let state = StorageState::new().unwrap();
        let params = json!({"directory": "relative/path", "family_id": "test"});
        let result = content_ingest(Some(&params), &state).await;
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("absolute"), "error should mention absolute: {err}");
    }

    #[tokio::test]
    async fn ingest_rejects_missing_directory() {
        let state = StorageState::new().unwrap();
        let params = json!({"directory": "/tmp/nestgate_ingest_nonexistent_test_dir_12345", "family_id": "test"});
        let result = content_ingest(Some(&params), &state).await;
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("does not exist"), "error should mention not existing: {err}");
    }

    #[tokio::test]
    async fn ingest_empty_directory() {
        let dir = tempfile::tempdir().unwrap();
        let state = StorageState::new().unwrap();
        let params = json!({
            "directory": dir.path().display().to_string(),
            "family_id": "test-ingest"
        });
        let result = content_ingest(Some(&params), &state).await.unwrap();
        assert_eq!(result["count"], 0);
        assert_eq!(result["bytes_total"], 0);
        assert_eq!(result["deduplicated"], 0);
        assert!(result["manifest"].as_object().unwrap().is_empty());
    }

    #[tokio::test]
    async fn ingest_files_produces_manifest() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("hello.txt"), b"hello world").unwrap();
        std::fs::create_dir_all(dir.path().join("sub")).unwrap();
        std::fs::write(dir.path().join("sub/data.json"), b"{}").unwrap();

        let state = StorageState::new().unwrap();
        let params = json!({
            "directory": dir.path().display().to_string(),
            "family_id": "test-ingest"
        });
        let result = content_ingest(Some(&params), &state).await.unwrap();
        assert_eq!(result["count"], 2);
        let manifest = result["manifest"].as_object().unwrap();
        assert!(manifest.contains_key("hello.txt"));
        assert!(manifest.contains_key("sub/data.json"));
        assert_eq!(result["errors"].as_array().unwrap().len(), 0);
    }

    #[tokio::test]
    async fn ingest_deduplicates() {
        let dir = tempfile::tempdir().unwrap();
        let unique = format!("ingest-dedup-test-{}", uuid::Uuid::new_v4());
        let content = unique.as_bytes();
        std::fs::write(dir.path().join("a.txt"), content).unwrap();
        std::fs::write(dir.path().join("b.txt"), content).unwrap();

        let state = StorageState::new().unwrap();
        let params = json!({
            "directory": dir.path().display().to_string(),
            "family_id": "test-ingest-dedup"
        });
        let result = content_ingest(Some(&params), &state).await.unwrap();
        assert_eq!(result["count"], 2);
        assert_eq!(result["deduplicated"], 1);

        let manifest = result["manifest"].as_object().unwrap();
        let hash_a = manifest["a.txt"].as_str().unwrap();
        let hash_b = manifest["b.txt"].as_str().unwrap();
        assert_eq!(hash_a, hash_b);
    }
}

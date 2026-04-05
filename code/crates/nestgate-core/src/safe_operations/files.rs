// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Safe file operations utilities.
//!
//! Provides safe alternatives to file operations that might panic or fail.

use crate::{NestGateError, Result};
use std::path::Path;
use tokio::fs;

/// Safe file read operation
pub async fn safe_read_to_string<P: AsRef<Path>>(path: P) -> Result<String> {
    let path_ref = path.as_ref();
    fs::read_to_string(path_ref).await.map_err(|e| {
        NestGateError::io_error(format!(
            "Failed to read file '{}': {}",
            path_ref.display(),
            e
        ))
    })
}

/// Safe file write operation
pub async fn safe_write<P: AsRef<Path>>(path: P, contents: &str) -> Result<()> {
    let path_ref = path.as_ref();
    fs::write(path_ref, contents).await.map_err(|e| {
        NestGateError::io_error(format!(
            "Failed to write file '{}': {}",
            path_ref.display(),
            e
        ))
    })
}

/// Safe directory creation
pub async fn safe_create_dir_all<P: AsRef<Path>>(path: P) -> Result<()> {
    let path_ref = path.as_ref();
    fs::create_dir_all(path_ref).await.map_err(|e| {
        NestGateError::io_error(format!(
            "Failed to create directory '{}': {}",
            path_ref.display(),
            e
        ))
    })
}

/// Safe temporary directory creation
pub fn safe_create_temp_dir(_context: &str) -> Result<tempfile::TempDir> {
    tempfile::TempDir::new()
        .map_err(|e| NestGateError::io_error(format!("Failed to create temporary directory: {e}")))
}

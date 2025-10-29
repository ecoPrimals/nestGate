// Metadata handling and utility functions for filesystem operations

use super::types::FileMetadata;
use crate::Result;
use std::borrow::Cow;
use std::collections::HashMap;
use std::os::unix::fs::PermissionsExt;
use tokio::fs;

#[cfg(unix)]
use std::os::unix::fs::MetadataExt;

/// Create file metadata from filesystem metadata
    let metadata = fs::metadata(path)
        .await
        .map_err(|_e| crate::error::NestGateError::storage_error(

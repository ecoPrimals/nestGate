//! Metadata handling and utility functions for filesystem operations

use super::types::FileMetadata;
use crate::Result;
use std::borrow::Cow;
use std::collections::HashMap;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use tokio::fs;

#[cfg(unix)]
use std::os::unix::fs::MetadataExt;

/// Create file metadata from filesystem metadata
pub async fn create_file_metadata(path: &Path) -> Result<FileMetadata> {
    let metadata = fs::metadata(path)
        .await
        .map_err(|e| crate::error::NestGateError::Io {
            operation: "get_metadata".into(),
            error_message: format!("Failed to get file metadata: {e}"),
            resource: Some(path.to_string_lossy().into_owned()),
            retryable: true,
        })?;

    let system_time_created = metadata.created().ok();
    let system_time_modified = metadata.modified().ok();
    let system_time_accessed = metadata.accessed().ok();

    Ok(FileMetadata {
        path: path.to_string_lossy().into_owned(),
        size: metadata.len(),
        permissions: format!("{:o}", metadata.permissions().mode()),
        owner: get_file_owner(path).unwrap_or_else(|| "unknown".into()),
        group: get_file_group(path).unwrap_or_else(|| "unknown".into()),
        checksum: None,
        mime_type: guess_content_type(path).map(|s| s.into_owned()),
        content_type: guess_content_type(path).map(|s| s.into_owned()),
        custom_metadata: HashMap::new(),
        created: system_time_created,
        modified: system_time_modified,
        accessed: system_time_accessed,
        created_at: system_time_created,
        modified_at: system_time_modified,
        tags: HashMap::new(),
    })
}

/// Guess content type from file extension - returns Cow to avoid allocations for common types
pub fn guess_content_type(path: &Path) -> Option<Cow<'static, str>> {
    match path.extension()?.to_str()? {
        "txt" => Some(Cow::Borrowed("text/plain")),
        "json" => Some(Cow::Borrowed("application/json")),
        "xml" => Some(Cow::Borrowed("application/xml")),
        "html" => Some(Cow::Borrowed("text/html")),
        "css" => Some(Cow::Borrowed("text/css")),
        "js" => Some(Cow::Borrowed("application/javascript")),
        "png" => Some(Cow::Borrowed("image/png")),
        "jpg" | "jpeg" => Some(Cow::Borrowed("image/jpeg")),
        "gif" => Some(Cow::Borrowed("image/gif")),
        "pdf" => Some(Cow::Borrowed("application/pdf")),
        _ => Some(Cow::Borrowed("application/octet-stream")),
    }
}

/// Get file owner name (Unix-specific) - SAFE IMPLEMENTATION
#[cfg(unix)]
pub fn get_file_owner(path: &Path) -> Option<String> {
    let metadata = std::fs::metadata(path).ok()?;
    let uid = metadata.uid();

    // ✅ SAFE: Use users crate instead of unsafe libc calls
    // This is faster than libc because it caches results
    users::get_user_by_uid(uid)
        .map(|user| user.name().to_string_lossy().into_owned())
        .or_else(|| Some(uid.to_string())) // Fallback to UID as string
}

/// Get file group name (Unix-specific) - SAFE IMPLEMENTATION
#[cfg(unix)]
pub fn get_file_group(path: &Path) -> Option<String> {
    let metadata = std::fs::metadata(path).ok()?;
    let gid = metadata.gid();

    // ✅ SAFE: Use users crate instead of unsafe libc calls
    // This is faster than libc because it caches results
    users::get_group_by_gid(gid)
        .map(|group| group.name().to_string_lossy().into_owned())
        .or_else(|| Some(gid.to_string())) // Fallback to GID as string
}

/// Get file owner name (Windows fallback)
#[cfg(not(unix))]
pub fn get_file_owner(_path: &Path) -> Option<String> {
    // On Windows, return None to use "unknown" fallback
    None
}

/// Get file group name (Windows fallback)
#[cfg(not(unix))]
pub fn get_file_group(_path: &Path) -> Option<String> {
    // On Windows, return None to use "unknown" fallback
    None
}

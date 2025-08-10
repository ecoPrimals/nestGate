//! File and directory operations for the filesystem storage backend

use crate::Result;
use std::path::Path;
use tokio::fs;
use tokio::io::AsyncWriteExt;

/// Perform atomic write operation
pub async fn atomic_write(path: &Path, content: &[u8], use_atomic: bool) -> Result<()> {
    if use_atomic {
        // Write to temporary file first
        let temp_path = path.with_extension("tmp");

        let mut file =
            fs::File::create(&temp_path)
                .await
                .map_err(|e| crate::error::NestGateError::Io {
                    operation: "create_temp_file".to_string(),
                    error_message: format!("Failed to create temporary file: {e}"),
                    resource: Some(temp_path.to_string_lossy().to_string()),
                    retryable: true,
                })?;

        file.write_all(content)
            .await
            .map_err(|e| crate::error::NestGateError::Io {
                operation: "write_temp_file".to_string(),
                error_message: format!("Failed to write to temporary file: {e}"),
                resource: Some(temp_path.to_string_lossy().to_string()),
                retryable: true,
            })?;

        file.sync_all()
            .await
            .map_err(|e| crate::error::NestGateError::Io {
                operation: "sync_temp_file".to_string(),
                error_message: format!("Failed to sync temporary file: {e}"),
                resource: Some(temp_path.to_string_lossy().to_string()),
                retryable: true,
            })?;

        // Atomically rename temporary file to target
        fs::rename(&temp_path, path)
            .await
            .map_err(|e| crate::error::NestGateError::Io {
                operation: "atomic_rename".to_string(),
                error_message: format!("Failed to rename temporary file: {e}"),
                resource: Some(path.to_string_lossy().to_string()),
                retryable: true,
            })?;
    } else {
        // Direct write
        let mut file =
            fs::File::create(path)
                .await
                .map_err(|e| crate::error::NestGateError::Io {
                    operation: "create_file".to_string(),
                    error_message: format!("Failed to create file: {e}"),
                    resource: Some(path.to_string_lossy().to_string()),
                    retryable: true,
                })?;

        file.write_all(content)
            .await
            .map_err(|e| crate::error::NestGateError::Io {
                operation: "write_file".to_string(),
                error_message: format!("Failed to write file: {e}"),
                resource: Some(path.to_string_lossy().to_string()),
                retryable: true,
            })?;
    }

    Ok(())
}

/// Validate path to prevent directory traversal attacks
pub fn validate_path(path: &str) -> Result<()> {
    let path = Path::new(path);

    // Prevent directory traversal attacks
    if path.is_absolute() || path.to_string_lossy().contains("..") {
        return Err(crate::error::NestGateError::Validation {
            field: "path".to_string(),
            message: "Path contains invalid characters or is absolute".to_string(),
            current_value: Some(path.to_string_lossy().to_string()),
            expected: Some("relative path without '..' components".to_string()),
            user_error: true,
        });
    }

    Ok(())
}

/// Check file size limits
pub fn check_size_limit(size: usize, max_size: u64) -> Result<()> {
    if max_size > 0 && size as u64 > max_size {
        return Err(crate::error::NestGateError::ResourceExhausted {
            resource: "file_size".to_string(),
            current: size as u64,
            limit: max_size,
            retry_after: Some(std::time::Duration::from_secs(5)),
            scaling_suggestion: Some(
                "Increase max file size limit or compress the file".to_string(),
            ),
        });
    }
    Ok(())
}

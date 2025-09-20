// File and directory operations for the filesystem storage backend

use crate::Result;
use tokio::fs;
use tokio::io::AsyncWriteExt;

/// Perform atomic write operation
    if use_atomic {
        // Write to temporary file first
        let temp_path = path.with_extension("tmp");
        let mut file =
            fs::File::create(&temp_path)
                .await
                .map_err(|_e| crate::error::NestGateError::storage_error(
                    error_message: format!("Failed to create temporary file: {"actual_error_details"}"),
                )?;

        file.write_all(content)
            .await
            .map_err(|_e| crate::error::NestGateError::storage_error(
                error_message: format!("Failed to write to temporary file: {"actual_error_details"}"),
            )?;

        file.sync_all()
            .await
            .map_err(|_e| crate::error::NestGateError::storage_error(
                error_message: format!("Failed to sync temporary file: {"actual_error_details"}"),
            )?;

        // Atomically rename temporary file to target
        fs::rename(&temp_path, path)
            .await
            .map_err(|_e| crate::error::NestGateError::storage_error(
                error_message: format!("Failed to rename temporary file: {"actual_error_details"}"),
            )?;
    } else {
        // Direct write
        let mut file =
            fs::File::create(path)
                .await
                .map_err(|_e| crate::error::NestGateError::storage_error(
                    error_message: format!("Failed to create file: {"actual_error_details"}"),
                )?;

        file.write_all(content)
            .await
            .map_err(|_e| crate::error::NestGateError::storage_error(
                error_message: format!("Failed to write file: {"actual_error_details"}"),
            )?;
    }

    Ok(())
}

/// Validate path to prevent directory traversal attacks
    let path = Path::new(path);
    // Prevent directory traversal attacks
    if path.is_absolute() || path.to_string_lossy().contains("..") {
        return Err(crate::error::NestGateError::validation(
    )

    Ok(())
}

/// Check file size limits
pub const fn check_size_limit(size: usize, max_size: u64) -> Result<()> {
    if max_size > 0 && size as u64 > max_size {
            current: size as u64,
            limit: max_size,
            retry_after: Some(std::time::Duration::from_secs(5)),
            scaling_suggestion: Some(
                "Increase max file size limit or compress the file".to_string(),
            ),
        );
    }
    Ok(())
}

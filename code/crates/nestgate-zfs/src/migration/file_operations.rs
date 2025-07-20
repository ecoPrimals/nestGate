//! ZFS Migration File Operations - File copying, verification, and metadata handling
//!
//! Contains the core file operations for migration including high-performance
//! file copying with progress tracking, integrity verification, and metadata preservation.

use std::path::PathBuf;
use std::time::Instant;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tracing::debug;

use nestgate_core::{NestGateError, Result as CoreResult};

use super::types::MigrationJob;

/// Copy file with progress tracking using zero-copy buffer
pub async fn copy_file_with_progress(
    source_path: &PathBuf,
    target_path: &PathBuf,
    job: &mut MigrationJob,
) -> CoreResult<()> {
    let mut source_file = tokio::fs::File::open(source_path)
        .await
        .map_err(|e| NestGateError::Storage(format!("Failed to open source file: {e}")))?;

    let mut target_file = tokio::fs::File::create(target_path)
        .await
        .map_err(|e| NestGateError::Storage(format!("Failed to create target file: {e}")))?;

    // Use zero-copy buffer management for better performance
    let buffer_size = 4 * 1024 * 1024; // 4MB buffer instead of 1MB

    // Use buffer pooling to reduce allocations
    static BUFFER_MANAGER: std::sync::OnceLock<
        std::sync::Arc<std::sync::Mutex<nestgate_core::zero_copy::BufferManager>>,
    > = std::sync::OnceLock::new();
    let buffer_manager = BUFFER_MANAGER.get_or_init(|| {
        std::sync::Arc::new(std::sync::Mutex::new(
            nestgate_core::zero_copy::BufferManager::new(4),
        ))
    });

    let mut buffer = {
        let mut manager = buffer_manager.lock().map_err(|e| {
            NestGateError::Storage(format!("Failed to acquire buffer manager lock: {e}"))
        })?;
        manager.get_buffer(buffer_size)
    };
    let mut total_copied = 0u64;
    let start_time = Instant::now();

    loop {
        let bytes_read = source_file
            .read(&mut buffer)
            .await
            .map_err(|e| NestGateError::Storage(format!("Failed to read source file: {e}")))?;

        if bytes_read == 0 {
            break; // EOF
        }

        target_file
            .write_all(&buffer[..bytes_read])
            .await
            .map_err(|e| NestGateError::Storage(format!("Failed to write target file: {e}")))?;

        total_copied += bytes_read as u64;

        // Update progress
        job.progress = (total_copied as f64 / job.file_size as f64) * 100.0;

        // Calculate transfer rate and ETA
        let elapsed = start_time.elapsed().as_secs_f64();
        if elapsed > 0.0 {
            job.transfer_rate = total_copied as f64 / elapsed;

            if job.transfer_rate > 0.0 {
                let remaining_bytes = job.file_size.saturating_sub(total_copied);
                job.eta_seconds = Some((remaining_bytes as f64 / job.transfer_rate) as u64);
            }
        }

        // Yield less frequently for better performance
        if total_copied % (50 * 1024 * 1024) == 0 {
            // Every 50MB instead of 10MB
            tokio::task::yield_now().await;
        }
    }

    // Ensure all data is written to disk
    target_file
        .sync_all()
        .await
        .map_err(|e| NestGateError::Storage(format!("Failed to sync target file: {e}")))?;

    // Return buffer to pool for reuse (zero-copy optimization)
    {
        let mut manager = buffer_manager.lock().map_err(|e| {
            NestGateError::Storage(format!(
                "Failed to acquire buffer manager lock for return: {e}"
            ))
        })?;
        manager.return_buffer(buffer);
    }

    Ok(())
}

/// Verify file integrity after copy
pub async fn verify_file_integrity(source_path: &PathBuf, target_path: &PathBuf) -> CoreResult<()> {
    // Compare file sizes
    let source_metadata = tokio::fs::metadata(source_path)
        .await
        .map_err(|e| NestGateError::Storage(format!("Failed to get source metadata: {e}")))?;

    let target_metadata = tokio::fs::metadata(target_path)
        .await
        .map_err(|e| NestGateError::Storage(format!("Failed to get target metadata: {e}")))?;

    if source_metadata.len() != target_metadata.len() {
        return Err(NestGateError::Storage(format!(
            "File size mismatch: source {} bytes, target {} bytes",
            source_metadata.len(),
            target_metadata.len()
        )));
    }

    // For small files, do a full content comparison
    if source_metadata.len() < 10 * 1024 * 1024 {
        // 10MB threshold
        let source_content = tokio::fs::read(source_path).await.map_err(|e| {
            NestGateError::Storage(format!("Failed to read source for verification: {e}"))
        })?;

        let target_content = tokio::fs::read(target_path).await.map_err(|e| {
            NestGateError::Storage(format!("Failed to read target for verification: {e}"))
        })?;

        if source_content != target_content {
            return Err(NestGateError::Storage(
                "File content mismatch after copy".to_string(),
            ));
        }
    }

    Ok(())
}

/// Update file metadata and access patterns
pub async fn update_file_metadata(target_path: &PathBuf, job: &MigrationJob) -> CoreResult<()> {
    // Preserve original timestamps if possible
    if let Ok(source_metadata) = tokio::fs::metadata(&job.source_path).await {
        if let Ok(modified_time) = source_metadata.modified() {
            // Note: Setting file times requires platform-specific code
            // For now, we'll just log this operation
            debug!(
                "Would preserve modified time: {:?} for {:?}",
                modified_time, target_path
            );
        }
    }

    // Record migration in metadata (could be extended to use extended attributes)
    debug!(
        "Recording migration metadata for {:?} -> {:?}",
        job.source_path, target_path
    );

    Ok(())
}

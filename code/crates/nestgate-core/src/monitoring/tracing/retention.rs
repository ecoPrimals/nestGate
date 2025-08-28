//! **LOG RETENTION AND CLEANUP**
//!
//! Log retention and cleanup management functionality.
//! Extracted from tracing_setup.rs for file size compliance.

use crate::{NestGateError, Result};
use std::path::PathBuf;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tracing::{debug, error, warn};

use super::config::LogRetentionConfig;

/// Log retention manager
pub struct LogRetentionManager {
    config: LogRetentionConfig,
}

impl LogRetentionManager {
    /// Create new retention manager
    pub fn new(config: LogRetentionConfig) -> Self {
        Self { config }
    }

    /// Clean up old log files
    pub async fn cleanup_logs(&self, log_dir: &PathBuf) -> Result<()> {
        if !log_dir.exists() {
            return Ok(());
        }

        let mut files_to_remove = Vec::new();
        let mut total_size = 0u64;

        // Scan log files
        let mut entries =
            tokio::fs::read_dir(log_dir)
                .await
                .map_err(|e| NestGateError::Internal {
                    message: format!("Failed to read log directory: {e}"),
                    location: Some(file!().to_string()),
                    context: None,
                    is_bug: false,
                })?;

        while let Some(entry) = entries
            .next_entry()
            .await
            .map_err(|e| NestGateError::Internal {
                message: format!("Failed to read directory entry: {e}"),
                location: Some(file!().to_string()),
                context: None,
                is_bug: false,
            })?
        {
            let path = entry.path();
            if !path.is_file() {
                continue;
            }

            let metadata = entry
                .metadata()
                .await
                .map_err(|e| NestGateError::Internal {
                    message: format!("Failed to read file metadata: {e}"),
                    location: Some(file!().to_string()),
                    context: None,
                    is_bug: false,
                })?;

            let modified = metadata
                .modified()
                .map_err(|e| NestGateError::Internal {
                    message: format!("Failed to read file modification time: {e}"),
                    location: Some(file!().to_string()),
                    context: None,
                    is_bug: false,
                })?;

            let age = SystemTime::now()
                .duration_since(modified)
                .unwrap_or_default();

            // Check if file is too old
            if age > self.config.max_age {
                files_to_remove.push(path);
                continue;
            }

            // Check if file is too large
            if metadata.len() > self.config.max_size {
                files_to_remove.push(path);
                continue;
            }

            total_size += metadata.len();
        }

        // Remove old files
        for file_path in files_to_remove {
            if let Err(e) = tokio::fs::remove_file(&file_path).await {
                warn!("Failed to remove old log file {:?}: {}", file_path, e);
            } else {
                debug!("Removed old log file: {:?}", file_path);
            }
        }

        debug!("Log cleanup completed, total size: {} bytes", total_size);
        Ok(())
    }

    /// Start log retention background task
    pub fn start_retention_task(&self, log_dir: PathBuf) -> tokio::task::JoinHandle<()> {
        let manager = self.clone();

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(3600)); // Check every hour

            loop {
                interval.tick().await;

                if let Err(e) = manager.cleanup_logs(&log_dir).await {
                    error!("Failed to cleanup logs: {}", e);
                }
            }
        })
    }
}

impl Clone for LogRetentionManager {
    fn clone(&self) -> Self {
        Self {
            config: self.config.clone(),
        }
    }
} 
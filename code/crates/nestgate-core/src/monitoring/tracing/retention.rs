//! # Log Retention Management
//! Retention functionality and utilities.
// Log file cleanup, rotation, and retention policies

use crate::{NestGateError, Result};
use std::time::{Duration, SystemTime};
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
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
                pub fn cleanup_logs(&self, log_dir: &PathBuf) -> Result<()>  {
        if !log_dir.exists() {
            return Ok(());
        }

        let mut files_to_remove = Vec::new();
        let mut total_size = 0u64;

        // Scan log files
        let mut entries = tokio::fs::read_dir(log_dir)
            .await
            .map_err(|e| NestGateError::internal_error(
                debug_info: None,
            )?;

        while let Some(entry) = entries
            .next_entry()
            .await
            .map_err(|e| NestGateError::internal_error(
                debug_info: None,
            })?
        {
            let path = entry.path();
            if !path.is_file() {
                continue;
            }

            let metadata = entry
                .metadata()
                .await
                .map_err(|e| NestGateError::internal_error(

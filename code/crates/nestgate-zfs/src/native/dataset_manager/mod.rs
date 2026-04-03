// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! ZFS Dataset Manager
//!
//! This module provides production-ready ZFS dataset management operations
//! including creation, deletion, property management, and monitoring.
//!
//! # Overview
//!
//! The dataset manager handles all dataset-level operations:
//! - Creating datasets with custom properties
//! - Listing and querying dataset information
//! - Setting and getting dataset properties
//! - Deleting datasets safely
//! - Managing storage tiers (Hot, Warm, Cold, Archive)
//!
//! # Examples
//!
//! ```rust,ignore
//! use nestgate_zfs::native::dataset_manager::{NativeZfsDatasetManager, DatasetCreateOptions};
//! use std::sync::Arc;
//!
//! let executor = Arc::new(NativeZfsCommandExecutor::new());
//! let manager = NativeZfsDatasetManager::new(executor);
//!
//! // Create dataset with compression
//! let options = DatasetCreateOptions {
//!     compression: Some("lz4".to_string()),
//!     ..Default::default()
//! };
//! manager.create_dataset("mypool/dataset", options).await?;
//! ```
//!
//! # Safety
//!
//! All operations use safe subprocess execution through `NativeZfsCommandExecutor`.
//! No unsafe code is used.

mod create_destroy;
mod mount_properties;
mod query;
mod types;

pub use types::DatasetCreateOptions;

use super::command_executor::NativeZfsCommandExecutor;
use std::sync::Arc;

/// Native ZFS dataset manager
///
/// Provides high-level dataset management operations built on top of
/// the native ZFS command executor.
///
/// # Examples
///
/// ```rust,ignore
/// let executor = Arc::new(NativeZfsCommandExecutor::new());
/// let manager = NativeZfsDatasetManager::new(executor);
///
/// // List all datasets
/// let datasets = manager.list_datasets().await?;
/// ```
pub struct NativeZfsDatasetManager {
    /// Command executor for ZFS operations
    pub(in crate::native::dataset_manager) command_executor: Arc<NativeZfsCommandExecutor>,
}

impl NativeZfsDatasetManager {
    /// Create a new dataset manager
    ///
    /// # Arguments
    ///
    /// * `command_executor` - Shared command executor for ZFS operations
    ///
    /// # Returns
    ///
    /// New dataset manager instance
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use std::sync::Arc;
    ///
    /// let executor = Arc::new(NativeZfsCommandExecutor::new());
    /// let manager = NativeZfsDatasetManager::new(executor);
    /// ```
    #[must_use]
    pub const fn new(command_executor: Arc<NativeZfsCommandExecutor>) -> Self {
        Self { command_executor }
    }
}

// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! **S3-COMPATIBLE CLIENT**
//!
//! Vendor-agnostic S3-compatible client abstraction.

use super::config::ConfigSource;

/// S3-compatible object storage client (vendor-agnostic)
///
/// **DESIGN**: Abstracts S3-compatible API without hardcoding any vendor.
/// Works with ANY service that implements S3-compatible protocol.
#[allow(dead_code)] // Fields used for future telemetry/debugging features
pub struct ObjectStorageClient {
    /// Discovered endpoint (runtime configuration)
    pub(super) endpoint: String,
    /// Region (if applicable)
    pub(super) region: String,
    /// Configuration source
    pub(super) config_source: ConfigSource,
    /// Optional path-style requests (for MinIO, legacy S3)
    pub(super) path_style: bool,
}

impl ObjectStorageClient {
    /// Create new client from configuration
    #[must_use]
    pub fn new(
        endpoint: String,
        region: String,
        config_source: ConfigSource,
        path_style: bool,
    ) -> Self {
        Self {
            endpoint,
            region,
            config_source,
            path_style,
        }
    }

    /// Get endpoint
    #[must_use]
    pub fn endpoint(&self) -> &str {
        &self.endpoint
    }

    /// Get region
    #[must_use]
    pub fn region(&self) -> &str {
        &self.region
    }

    /// Check if path-style requests are enabled
    #[must_use]
    pub fn is_path_style(&self) -> bool {
        self.path_style
    }
}

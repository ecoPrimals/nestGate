// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! **STORAGE PROVIDER DETECTION**
//!
//! Provider enum and endpoint-based detection.

use serde::{Deserialize, Serialize};

/// Detected storage provider
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StorageProvider {
    /// AWS S3
    AwsS3,
    /// MinIO
    MinIO,
    /// Wasabi
    Wasabi,
    /// DigitalOcean Spaces
    DigitalOceanSpaces,
    /// Linode Object Storage
    LinodeObjectStorage,
    /// Backblaze B2
    BackblazeB2,
    /// Unknown S3-compatible provider
    Unknown,
}

impl StorageProvider {
    /// Detect provider from endpoint
    #[must_use]
    pub fn detect_from_endpoint(endpoint: &str) -> Self {
        if endpoint.contains("amazonaws.com") || endpoint.contains("s3.") {
            Self::AwsS3
        } else if endpoint.contains("minio")
            || endpoint.contains("min.io")
            || endpoint.contains("localhost:9000")
        {
            Self::MinIO
        } else if endpoint.contains("wasabi") {
            Self::Wasabi
        } else if endpoint.contains("digitalocean") {
            Self::DigitalOceanSpaces
        } else if endpoint.contains("linode") {
            Self::LinodeObjectStorage
        } else if endpoint.contains("backblaze") || endpoint.contains("b2") {
            Self::BackblazeB2
        } else {
            Self::Unknown
        }
    }
}

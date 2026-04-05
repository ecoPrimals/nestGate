// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! **STORAGE PROVIDER DETECTION**
//!
//! Provider enum and endpoint-based detection.

use serde::{Deserialize, Serialize};

/// Detected storage provider
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StorageProvider {
    /// AWS S3
    AwsS3,
    /// `MinIO`
    MinIO,
    /// Wasabi
    Wasabi,
    /// `DigitalOcean` Spaces
    DigitalOceanSpaces,
    /// Linode Object Storage
    LinodeObjectStorage,
    /// Backblaze B2
    BackblazeB2,
    /// Unknown S3-compatible provider
    Unknown,
}

impl StorageProvider {
    /// Detect provider from endpoint.
    ///
    /// Treats `localhost:9000` as a MinIO-oriented hint: that host/port matches the well-known
    /// default MinIO API port in local development (not a universal S3 rule).
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn storage_provider_detect_from_endpoint_branches() {
        assert!(matches!(
            StorageProvider::detect_from_endpoint("https://bucket.s3.amazonaws.com"),
            StorageProvider::AwsS3
        ));
        assert!(matches!(
            StorageProvider::detect_from_endpoint("http://localhost:9000"),
            StorageProvider::MinIO
        ));
        assert!(matches!(
            StorageProvider::detect_from_endpoint("https://example.wasabisys.com"),
            StorageProvider::Wasabi
        ));
        assert!(matches!(
            StorageProvider::detect_from_endpoint("https://nyc3.digitalocean.com"),
            StorageProvider::DigitalOceanSpaces
        ));
        assert!(matches!(
            StorageProvider::detect_from_endpoint("https://us-east-1.linodeobjects.com"),
            StorageProvider::LinodeObjectStorage
        ));
        assert!(matches!(
            StorageProvider::detect_from_endpoint("https://f004.backblazeb2.com"),
            StorageProvider::BackblazeB2
        ));
        assert!(matches!(
            StorageProvider::detect_from_endpoint("https://example.com"),
            StorageProvider::Unknown
        ));
    }

    #[test]
    fn storage_provider_serde_roundtrip() {
        let roundtrip = |p: StorageProvider| {
            let json = serde_json::to_string(&p).unwrap();
            let back: StorageProvider = serde_json::from_str(&json).unwrap();
            assert!(std::mem::discriminant(&p) == std::mem::discriminant(&back));
        };
        roundtrip(StorageProvider::AwsS3);
        roundtrip(StorageProvider::MinIO);
        roundtrip(StorageProvider::Wasabi);
        roundtrip(StorageProvider::DigitalOceanSpaces);
        roundtrip(StorageProvider::LinodeObjectStorage);
        roundtrip(StorageProvider::BackblazeB2);
        roundtrip(StorageProvider::Unknown);
    }
}

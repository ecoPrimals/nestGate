// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! # Universal Storage Adapter
//!
//! Adapts discovered protocols to a common storage interface.

use super::operations::ObjectAddressing;
use super::protocol::DiscoveredProtocol;
use nestgate_types::error::{NestGateError, Result};
use std::sync::Arc;

/// Universal storage adapter
///
/// Works with ANY storage protocol discovered at runtime.
#[derive(Clone)]
pub struct UniversalStorageAdapter {
    /// Storage endpoint
    endpoint: String,

    /// Discovered protocol
    protocol: Arc<DiscoveredProtocol>,
}

impl UniversalStorageAdapter {
    /// Create a new universal storage adapter
    pub fn new(endpoint: impl Into<String>, protocol: DiscoveredProtocol) -> Self {
        Self {
            endpoint: endpoint.into(),
            protocol: Arc::new(protocol),
        }
    }

    /// Get the endpoint
    #[must_use]
    pub fn endpoint(&self) -> &str {
        &self.endpoint
    }

    /// Get the protocol
    #[must_use]
    pub fn protocol(&self) -> &DiscoveredProtocol {
        &self.protocol
    }

    /// Read data from storage
    ///
    /// Adapts to the discovered protocol automatically.
    ///
    /// # Errors
    ///
    /// Returns an error if the transport is unsupported or if filesystem I/O fails.
    pub async fn read(&self, key: &str) -> Result<Vec<u8>> {
        // Implementation will adapt based on protocol.operation_pattern
        // For now, this is a placeholder that demonstrates the concept

        match &self.protocol.transport {
            super::TransportProtocol::Http { .. } => Ok(self.http_read(key)),
            super::TransportProtocol::UnixSocket { .. } => {
                // Local filesystem read
                self.fs_read(key).await
            }
            _ => Err(NestGateError::not_implemented(
                "Transport not yet implemented",
            )),
        }
    }

    /// Write data to storage
    ///
    /// # Errors
    ///
    /// Returns an error if the transport is unsupported or if filesystem I/O fails.
    pub async fn write(&self, key: &str, data: &[u8]) -> Result<()> {
        match &self.protocol.transport {
            super::TransportProtocol::Http { .. } => {
                self.http_write(key, data);
                Ok(())
            }
            super::TransportProtocol::UnixSocket { .. } => {
                // Local filesystem write
                self.fs_write(key, data).await
            }
            _ => Err(NestGateError::not_implemented(
                "Transport not yet implemented",
            )),
        }
    }

    /// Delete data from storage
    ///
    /// # Errors
    ///
    /// Returns an error if the transport is unsupported or if filesystem I/O fails.
    pub async fn delete(&self, key: &str) -> Result<()> {
        match &self.protocol.transport {
            super::TransportProtocol::Http { .. } => {
                self.http_delete(key);
                Ok(())
            }
            super::TransportProtocol::UnixSocket { .. } => {
                // Local filesystem delete
                self.fs_delete(key).await
            }
            _ => Err(NestGateError::not_implemented(
                "Transport not yet implemented",
            )),
        }
    }

    /// List keys/objects
    ///
    /// # Errors
    ///
    /// Returns an error if the transport is unsupported or if filesystem I/O fails.
    pub async fn list(&self, prefix: &str) -> Result<Vec<String>> {
        match &self.protocol.transport {
            super::TransportProtocol::Http { .. } => Ok(self.http_list(prefix)),
            super::TransportProtocol::UnixSocket { .. } => {
                // Local filesystem list
                self.fs_list(prefix).await
            }
            _ => Err(NestGateError::not_implemented(
                "Transport not yet implemented",
            )),
        }
    }

    // ==================== HTTP Operations ====================

    fn http_read(&self, key: &str) -> Vec<u8> {
        // Build URL based on addressing pattern
        let _url = self.build_url(key);

        // Build authenticated request
        // let request = self.build_authenticated_request(&url, "GET");

        // Send request and get response
        // For now, placeholder
        format!("HTTP read from {} key {key}", self.endpoint).into_bytes()
    }

    fn http_write(&self, key: &str, _data: &[u8]) {
        let _url = self.build_url(key);
        // Build authenticated PUT/POST request
        // Send data
    }

    fn http_delete(&self, key: &str) {
        let _url = self.build_url(key);
        // Build authenticated DELETE request
        // Send request
    }

    fn http_list(&self, prefix: &str) -> Vec<String> {
        let _url = self.build_url(prefix);
        // Build authenticated GET request
        // Parse response for list of keys
        vec![]
    }

    fn build_url(&self, key: &str) -> String {
        match &self.protocol.operation_pattern {
            super::StorageOperationPattern::ObjectStore { addressing, .. } => {
                match addressing {
                    ObjectAddressing::PathBased => {
                        format!("{}/{}", self.endpoint, key)
                    }
                    ObjectAddressing::SubdomainBased => {
                        // Extract bucket from key and build subdomain URL
                        format!("{}/{}", self.endpoint, key)
                    }
                    ObjectAddressing::QueryBased => {
                        format!("{}?key={}", self.endpoint, key)
                    }
                    ObjectAddressing::HeaderBased { .. } => {
                        // Key will be in headers, just use endpoint
                        self.endpoint.clone()
                    }
                }
            }
            _ => format!("{}/{}", self.endpoint, key),
        }
    }

    // ==================== Filesystem Operations ====================

    async fn fs_read(&self, key: &str) -> Result<Vec<u8>> {
        let path = std::path::Path::new(&self.endpoint).join(key);
        tokio::fs::read(path)
            .await
            .map_err(|e| NestGateError::io_error(e.to_string()))
    }

    async fn fs_write(&self, key: &str, data: &[u8]) -> Result<()> {
        let path = std::path::Path::new(&self.endpoint).join(key);

        // Create parent directories if needed
        if let Some(parent) = path.parent() {
            tokio::fs::create_dir_all(parent)
                .await
                .map_err(|e| NestGateError::io_error(e.to_string()))?;
        }

        tokio::fs::write(path, data)
            .await
            .map_err(|e| NestGateError::io_error(e.to_string()))
    }

    async fn fs_delete(&self, key: &str) -> Result<()> {
        let path = std::path::Path::new(&self.endpoint).join(key);
        tokio::fs::remove_file(path)
            .await
            .map_err(|e| NestGateError::io_error(e.to_string()))
    }

    async fn fs_list(&self, prefix: &str) -> Result<Vec<String>> {
        let base_path = std::path::Path::new(&self.endpoint);
        let search_path = base_path.join(prefix);

        let mut keys = Vec::new();

        if let Ok(mut entries) = tokio::fs::read_dir(search_path).await {
            while let Ok(Some(entry)) = entries.next_entry().await {
                if let Ok(name) = entry.file_name().into_string() {
                    keys.push(format!("{prefix}/{name}"));
                }
            }
        }

        Ok(keys)
    }
}

impl std::fmt::Debug for UniversalStorageAdapter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UniversalStorageAdapter")
            .field("endpoint", &self.endpoint)
            .field("protocol", &self.protocol.description())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::universal_storage::universal::transport::FramingProtocol;
    use crate::universal_storage::universal::{
        AuthenticationPattern, HttpVersion, ObjectAddressing, ObjectOrganization,
        StorageOperationPattern, TransportProtocol,
    };
    use anyhow::Result;

    fn http_object_adapter(
        addressing: ObjectAddressing,
        endpoint: &str,
    ) -> UniversalStorageAdapter {
        let protocol = DiscoveredProtocol::new(
            TransportProtocol::Http {
                version: HttpVersion::Http1_1,
                tls: None,
            },
            StorageOperationPattern::ObjectStore {
                addressing,
                organization: ObjectOrganization::Hierarchical { separator: '/' },
            },
            AuthenticationPattern::None,
        );
        UniversalStorageAdapter::new(endpoint, protocol)
    }

    #[test]
    fn adapter_creation_and_endpoint() -> Result<()> {
        let adapter = http_object_adapter(
            ObjectAddressing::PathBased,
            "http://storage.example.com/bucket",
        );
        assert_eq!(adapter.endpoint(), "http://storage.example.com/bucket");
        Ok(())
    }

    #[test]
    fn build_url_path_query_and_header_based() -> Result<()> {
        let base = "http://storage.example.com/bucket";
        assert_eq!(
            http_object_adapter(ObjectAddressing::PathBased, base).build_url("k"),
            "http://storage.example.com/bucket/k"
        );
        assert_eq!(
            http_object_adapter(ObjectAddressing::QueryBased, base).build_url("mykey"),
            "http://storage.example.com/bucket?key=mykey"
        );
        assert_eq!(
            http_object_adapter(
                ObjectAddressing::HeaderBased {
                    location_headers: vec!["X-Loc".to_string()],
                },
                base
            )
            .build_url("ignored"),
            base.to_string()
        );
        Ok(())
    }

    #[tokio::test]
    async fn http_read_returns_placeholder_bytes() -> Result<()> {
        let adapter = http_object_adapter(ObjectAddressing::PathBased, "http://localhost:9000");
        let data = adapter.read("obj").await?;
        assert!(data.starts_with(b"HTTP read from http://localhost:9000"));
        Ok(())
    }

    #[tokio::test]
    async fn unsupported_tcp_transport_returns_not_implemented() -> Result<()> {
        let protocol = DiscoveredProtocol::new(
            TransportProtocol::Tcp {
                framing: FramingProtocol::LengthPrefixed { length_bytes: 4 },
            },
            StorageOperationPattern::ObjectStore {
                addressing: ObjectAddressing::PathBased,
                organization: ObjectOrganization::Flat,
            },
            AuthenticationPattern::None,
        );
        let adapter = UniversalStorageAdapter::new("127.0.0.1:9999", protocol);
        let err = adapter.read("k").await.expect_err("tcp unsupported");
        assert!(
            err.to_string().to_lowercase().contains("not implemented")
                || err.to_string().contains("not yet implemented")
        );
        Ok(())
    }

    #[tokio::test]
    async fn filesystem_round_trip_via_tempdir() -> Result<()> {
        let dir = tempfile::tempdir()?;
        let root = dir
            .path()
            .to_str()
            .ok_or_else(|| anyhow::anyhow!("temp path utf-8"))?;
        let protocol = DiscoveredProtocol::new(
            TransportProtocol::UnixSocket {
                path: root.to_string(),
            },
            StorageOperationPattern::FileSystem {
                path_separator: '/',
                case_sensitive: true,
            },
            AuthenticationPattern::None,
        );
        let adapter = UniversalStorageAdapter::new(root, protocol);
        adapter.write("nestgate_test.txt", b"payload").await?;
        let data = adapter.read("nestgate_test.txt").await?;
        assert_eq!(data, b"payload");
        adapter.delete("nestgate_test.txt").await?;
        Ok(())
    }
}

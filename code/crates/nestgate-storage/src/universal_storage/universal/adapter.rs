// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! # Universal Storage Adapter
//!
//! Adapts discovered protocols to a common storage interface.

use super::protocol::DiscoveredProtocol;
use nestgate_types::error::{NestGateError, Result};
use std::sync::Arc;
use std::time::Duration;

const HTTP_IO_TIMEOUT: Duration = Duration::from_secs(30);

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
            super::TransportProtocol::Http { .. } => self.http_read(key).await,
            super::TransportProtocol::UnixSocket { .. } => self.fs_read(key).await,
            _ => Err(NestGateError::not_implemented(
                "Transport not supported by universal adapter (use HTTP with http:// endpoint, or Unix socket mapped to a filesystem root)",
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
            super::TransportProtocol::Http { .. } => self.http_write(key, data).await,
            super::TransportProtocol::UnixSocket { .. } => self.fs_write(key, data).await,
            _ => Err(NestGateError::not_implemented(
                "Transport not supported by universal adapter (use HTTP with http:// endpoint, or Unix socket mapped to a filesystem root)",
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
            super::TransportProtocol::Http { .. } => self.http_delete(key).await,
            super::TransportProtocol::UnixSocket { .. } => self.fs_delete(key).await,
            _ => Err(NestGateError::not_implemented(
                "Transport not supported by universal adapter (use HTTP with http:// endpoint, or Unix socket mapped to a filesystem root)",
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
            super::TransportProtocol::Http { .. } => self.http_list(prefix).await,
            super::TransportProtocol::UnixSocket { .. } => self.fs_list(prefix).await,
            _ => Err(NestGateError::not_implemented(
                "Transport not supported by universal adapter (use HTTP with http:// endpoint, or Unix socket mapped to a filesystem root)",
            )),
        }
    }

    // ==================== HTTP Operations ====================
    // Plain HTTP/1.1 object GET/PUT/DELETE via `http_minimal` (no TLS).

    async fn http_read(&self, key: &str) -> Result<Vec<u8>> {
        let url = super::http_minimal::object_url(&self.endpoint, key)?;
        super::http_minimal::http_get_object(&url, HTTP_IO_TIMEOUT).await
    }

    async fn http_write(&self, key: &str, data: &[u8]) -> Result<()> {
        let url = super::http_minimal::object_url(&self.endpoint, key)?;
        super::http_minimal::http_put_object(&url, data, HTTP_IO_TIMEOUT).await
    }

    async fn http_delete(&self, key: &str) -> Result<()> {
        let url = super::http_minimal::object_url(&self.endpoint, key)?;
        super::http_minimal::http_delete_object(&url, HTTP_IO_TIMEOUT).await
    }

    async fn http_list(&self, prefix: &str) -> Result<Vec<String>> {
        let url = super::http_minimal::list_url(&self.endpoint, prefix)?;
        super::http_minimal::http_list_prefix(&url, HTTP_IO_TIMEOUT).await
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

    #[tokio::test]
    async fn http_write_uses_minimal_http_client() -> Result<()> {
        // Port 1 is typically closed — expect a connection-level error from the minimal client.
        let adapter = http_object_adapter(ObjectAddressing::PathBased, "http://127.0.0.1:1");
        let err = adapter
            .write("k", b"data")
            .await
            .expect_err("no listener on :1");
        let msg = err.to_string().to_lowercase();
        assert!(
            msg.contains("connect") || msg.contains("network") || msg.contains("refused"),
            "unexpected error: {err}"
        );
        Ok(())
    }

    #[tokio::test]
    async fn http_read_uses_minimal_http_client() -> Result<()> {
        let adapter = http_object_adapter(ObjectAddressing::PathBased, "http://127.0.0.1:1");
        let err = adapter.read("obj").await.expect_err("no listener on :1");
        let msg = err.to_string().to_lowercase();
        assert!(
            msg.contains("connect") || msg.contains("network") || msg.contains("refused"),
            "unexpected error: {err}"
        );
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
        let msg = err.to_string().to_lowercase();
        assert!(
            msg.contains("not supported") || msg.contains("not implemented"),
            "unexpected error: {err}"
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

// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! **BEARDOG SECURITY INTEGRATION**
//!
//! BearDog client for hardware-backed security and authentication.

use nestgate_core::capability_discovery::CapabilityDiscovery;
use nestgate_core::error::{NestGateError, Result};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use tokio::fs;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::UnixStream;
use tracing::{info, warn};

/// **BEARDOG CLIENT**
///
/// Client for communicating with BearDog security provider via Unix sockets.
///
/// ## BearDog Integration
///
/// BearDog provides:
/// - Hardware-backed encryption/decryption
/// - Sovereign identity management
/// - Token generation and validation
/// - Certificate management
///
/// ## Runtime Discovery
///
/// The client discovers BearDog via:
/// 1. `NESTGATE_SECURITY_PROVIDER` environment variable
/// 2. Socket scanning: `/tmp/beardog-{family}-*.sock`
/// 3. Fallback: `/tmp/beardog-default-default.sock`
pub struct BearDogClient {
    socket_path: PathBuf,
    connected: bool,
}

impl BearDogClient {
    /// Create new BearDog client
    ///
    /// # Errors
    ///
    /// Returns error if socket path is invalid
    pub fn new(socket_path: impl AsRef<Path>) -> Result<Self> {
        let socket_path = socket_path.as_ref().to_path_buf();

        if socket_path.as_os_str().is_empty() {
            return Err(NestGateError::api_error(
                "BearDog socket path cannot be empty",
            ));
        }

        Ok(Self {
            socket_path,
            connected: false,
        })
    }

    /// Discover security provider via capability-based discovery or runtime socket scanning.
    ///
    /// **Discovery order**:
    /// 1. `NESTGATE_SECURITY_PROVIDER` environment variable
    /// 2. Capability discovery (IPC gateway + "security" capability)
    /// 3. Socket scan fallback: `/tmp/beardog-{family}-*.sock`, etc.
    ///
    /// # Errors
    ///
    /// Returns error if security provider cannot be discovered
    pub async fn discover(family_id: &str) -> Result<Self> {
        // 1. Environment variable first
        if let Ok(socket_path) = std::env::var("NESTGATE_SECURITY_PROVIDER") {
            info!(
                "Found security provider via NESTGATE_SECURITY_PROVIDER: {}",
                socket_path
            );
            return Self::new(socket_path);
        }

        // 2. Capability discovery (IPC gateway + "security" capability)
        if let Ok(songbird) = CapabilityDiscovery::discover_songbird_ipc().await {
            let mut discovery = CapabilityDiscovery::new(songbird);
            if let Ok(endpoint) = discovery.find("security").await {
                let ep = endpoint.endpoint;
                // Endpoint may be Unix socket path or URL; use as path if it looks like one
                if ep.starts_with('/') && std::path::Path::new(&ep).exists() {
                    info!("Found security provider via capability discovery: {}", ep);
                    return Self::new(ep);
                }
            }
        }

        // 3. Scan for security provider sockets via env or XDG runtime
        let security_slug =
            std::env::var("NESTGATE_SECURITY_SLUG").unwrap_or_else(|_| "beardog".to_string());

        let socket_dirs = Self::candidate_socket_dirs();
        for dir in &socket_dirs {
            let patterns = vec![
                format!("{dir}/{security_slug}-{family_id}-*.sock"),
                format!("{dir}/{security_slug}-{family_id}.sock"),
                format!("{dir}/{security_slug}-default-default.sock"),
            ];
            for pattern in patterns {
                if let Ok(socket) = Self::try_socket(&pattern).await {
                    info!("Discovered security provider at: {}", pattern);
                    return Self::new(socket);
                }
            }
        }

        warn!("Security provider not found - security features disabled");
        Err(NestGateError::network_error("Security provider not found"))
    }

    /// Candidate directories where primal sockets may live, ordered by preference:
    /// 1. XDG_RUNTIME_DIR (recommended, per-user, tmpfs)
    /// 2. /run/user/{uid} (standard XDG fallback)
    /// 3. /tmp (least secure, universal fallback)
    fn candidate_socket_dirs() -> Vec<String> {
        let mut dirs = Vec::with_capacity(3);
        if let Ok(xdg) = std::env::var("XDG_RUNTIME_DIR") {
            dirs.push(xdg);
            // XDG_RUNTIME_DIR is typically /run/user/{uid} — no need to duplicate
        } else if let Ok(uid) = std::env::var("UID").or_else(|_| std::env::var("EUID")) {
            dirs.push(format!("/run/user/{uid}"));
        }
        dirs.push("/tmp".to_string());
        dirs
    }

    async fn try_socket(pattern: &str) -> Result<PathBuf> {
        if pattern.contains('*') {
            let path = Path::new(pattern);
            let parent = path.parent().unwrap_or_else(|| Path::new("."));
            let file_pattern = path.file_name().and_then(|n| n.to_str()).ok_or_else(|| {
                NestGateError::network_error("Invalid socket glob: missing file name")
            })?;
            let star = file_pattern.find('*').ok_or_else(|| {
                NestGateError::network_error("Invalid socket glob: expected '*' in file name")
            })?;
            let prefix = &file_pattern[..star];
            let suffix = &file_pattern[star + 1..];

            let mut entries = fs::read_dir(parent).await.map_err(|e| {
                NestGateError::network_error(&format!("Failed to read {}: {e}", parent.display()))
            })?;
            loop {
                let entry = match entries.next_entry().await {
                    Ok(Some(e)) => e,
                    Ok(None) => break,
                    Err(e) => {
                        return Err(NestGateError::network_error(&format!(
                            "Failed to read directory entry: {e}"
                        )));
                    }
                };
                let candidate = entry.path();
                let name = entry.file_name();
                let name = name.to_string_lossy();
                if name.starts_with(prefix)
                    && name.ends_with(suffix)
                    && UnixStream::connect(&candidate).await.is_ok()
                {
                    return Ok(candidate);
                }
            }
            return Err(NestGateError::network_error(
                "No matching BearDog socket for glob pattern",
            ));
        }

        let path = PathBuf::from(pattern);
        if path.exists() {
            // Try to connect
            UnixStream::connect(&path)
                .await
                .map_err(|e| NestGateError::network_error(&format!("Failed to connect: {e}")))?;
            Ok(path)
        } else {
            Err(NestGateError::network_error("Socket does not exist"))
        }
    }

    /// Connect to BearDog
    ///
    /// # Errors
    ///
    /// Returns error if connection fails
    pub async fn connect(&mut self) -> Result<()> {
        if !self.socket_path.exists() {
            return Err(NestGateError::network_error(&format!(
                "BearDog socket not found: {}",
                self.socket_path.display()
            )));
        }

        // Test connection
        let _stream = UnixStream::connect(&self.socket_path).await.map_err(|e| {
            NestGateError::network_error(&format!("Failed to connect to BearDog: {e}"))
        })?;

        self.connected = true;
        info!("✅ Connected to BearDog: {}", self.socket_path.display());
        Ok(())
    }

    /// Encrypt data using BearDog
    ///
    /// # Errors
    ///
    /// Returns error if encryption fails
    pub async fn encrypt(&self, plaintext: &[u8]) -> Result<Vec<u8>> {
        if !self.connected {
            return Err(NestGateError::security_error("Not connected to BearDog"));
        }

        let request = BearDogRequest {
            method: "encrypt".to_string(),
            data: plaintext.to_vec(),
        };

        let response = self.send_request(&request).await?;
        Ok(response.data)
    }

    /// Decrypt data using BearDog
    ///
    /// # Errors
    ///
    /// Returns error if decryption fails
    pub async fn decrypt(&self, ciphertext: &[u8]) -> Result<Vec<u8>> {
        if !self.connected {
            return Err(NestGateError::security_error("Not connected to BearDog"));
        }

        let request = BearDogRequest {
            method: "decrypt".to_string(),
            data: ciphertext.to_vec(),
        };

        let response = self.send_request(&request).await?;
        Ok(response.data)
    }

    /// Generate authentication token
    ///
    /// # Errors
    ///
    /// Returns error if token generation fails
    pub async fn generate_token(&self, identity: &str) -> Result<String> {
        if !self.connected {
            return Err(NestGateError::security_error("Not connected to BearDog"));
        }

        let request = BearDogRequest {
            method: "generate_token".to_string(),
            data: identity.as_bytes().to_vec(),
        };

        let response = self.send_request(&request).await?;
        String::from_utf8(response.data)
            .map_err(|e| NestGateError::security_error(&format!("Invalid token: {e}")))
    }

    /// Validate authentication token
    ///
    /// # Errors
    ///
    /// Returns error if validation fails
    pub async fn validate_token(&self, token: &str) -> Result<bool> {
        if !self.connected {
            return Err(NestGateError::security_error("Not connected to BearDog"));
        }

        let request = BearDogRequest {
            method: "validate_token".to_string(),
            data: token.as_bytes().to_vec(),
        };

        let response = self.send_request(&request).await?;
        Ok(!response.data.is_empty() && response.data[0] == 1)
    }

    async fn send_request(&self, request: &BearDogRequest) -> Result<BearDogResponse> {
        let mut stream = UnixStream::connect(&self.socket_path)
            .await
            .map_err(|e| NestGateError::network_error(&format!("Failed to connect: {e}")))?;

        // Serialize and send request
        let request_json = serde_json::to_vec(request)
            .map_err(|e| NestGateError::api_error(&format!("Failed to serialize request: {e}")))?;

        stream
            .write_all(&request_json)
            .await
            .map_err(|e| NestGateError::network_error(&format!("Failed to send request: {e}")))?;

        // Read response
        let mut buffer = vec![0u8; 65536];
        let n = stream
            .read(&mut buffer)
            .await
            .map_err(|e| NestGateError::network_error(&format!("Failed to read response: {e}")))?;

        // Deserialize response
        let response: BearDogResponse = serde_json::from_slice(&buffer[..n]).map_err(|e| {
            NestGateError::api_error(&format!("Failed to deserialize response: {e}"))
        })?;

        if !response.success {
            return Err(NestGateError::security_error(&format!(
                "BearDog error: {}",
                response
                    .error
                    .unwrap_or_else(|| "Unknown error".to_string())
            )));
        }

        Ok(response)
    }

    /// Check if connected to BearDog
    #[must_use]
    pub const fn is_connected(&self) -> bool {
        self.connected
    }

    /// Get socket path
    #[must_use]
    pub fn socket_path(&self) -> &Path {
        &self.socket_path
    }
}

// ============================================================================
// BearDog Protocol Types
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
struct BearDogRequest {
    method: String,
    data: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
struct BearDogResponse {
    success: bool,
    data: Vec<u8>,
    #[serde(default)]
    error: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_client_creation() {
        let temp_dir = TempDir::new().unwrap();
        let socket_path = temp_dir.path().join("beardog.sock");
        let client = BearDogClient::new(&socket_path);
        assert!(client.is_ok());
    }

    #[test]
    fn test_client_empty_path() {
        let result = BearDogClient::new("");
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_discover_fallback() {
        // Should fail gracefully when BearDog is not available
        let result = BearDogClient::discover("test").await;
        // In test environment, BearDog won't be available
        assert!(result.is_err());
    }
}

// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! **CAPABILITY-BASED SECURITY PROVIDER CLIENT**
//!
//! Client for discovering and communicating with whichever primal provides
//! the "security" capability at runtime (hardware-backed crypto, identity,
//! tokens, certificates). `NestGate` has no compile-time knowledge of *which*
//! primal fills this role.

use nestgate_core::capability_discovery::CapabilityDiscovery;
use nestgate_core::error::{NestGateError, Result};
use nestgate_types::{EnvSource, ProcessEnv};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use tokio::fs;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::UnixStream;
use tracing::{info, warn};

/// Client for whichever primal provides the "security" capability.
///
/// Capabilities offered by the discovered provider:
/// - Hardware-backed encryption/decryption
/// - Sovereign identity management
/// - Token generation and validation
/// - Certificate management
///
/// ## Runtime Discovery
///
/// 1. `NESTGATE_SECURITY_PROVIDER` environment variable (explicit path)
/// 2. Capability discovery via IPC gateway ("security" capability)
/// 3. Socket scan: `{NESTGATE_SECURITY_SLUG}-{family}-*.sock` (slug defaults
///    to `"security"` — override via env to match your provider's convention)
pub struct SecurityProviderClient {
    socket_path: PathBuf,
    connected: bool,
}

impl SecurityProviderClient {
    /// Create a new security provider client for the given socket path.
    ///
    /// # Errors
    ///
    /// Returns error if the socket path is empty.
    pub fn new(socket_path: impl AsRef<Path>) -> Result<Self> {
        let socket_path = socket_path.as_ref().to_path_buf();

        if socket_path.as_os_str().is_empty() {
            return Err(NestGateError::api_error(
                "Security provider socket path cannot be empty",
            ));
        }

        Ok(Self {
            socket_path,
            connected: false,
        })
    }

    /// Discover the security provider via capability-based discovery or
    /// runtime socket scanning.
    ///
    /// **Discovery order**:
    /// 1. `NESTGATE_SECURITY_PROVIDER` environment variable
    /// 2. Capability discovery (IPC gateway + "security" capability)
    /// 3. Socket scan: `{NESTGATE_SECURITY_SLUG}-{family}-*.sock`
    ///
    /// # Errors
    ///
    /// Returns error if no security provider can be discovered.
    pub async fn discover(family_id: &str) -> Result<Self> {
        Self::discover_from_env_source(&ProcessEnv, family_id).await
    }

    /// Like [`Self::discover`], but reads `NESTGATE_SECURITY_*` / `XDG_RUNTIME_DIR` from `env`.
    pub async fn discover_from_env_source(
        env: &(impl EnvSource + ?Sized),
        family_id: &str,
    ) -> Result<Self> {
        if let Some(socket_path) = env.get("NESTGATE_SECURITY_PROVIDER") {
            info!(
                "Found security provider via NESTGATE_SECURITY_PROVIDER: {}",
                socket_path
            );
            return Self::new(socket_path);
        }

        if let Ok(ipc) = CapabilityDiscovery::discover_orchestration_ipc().await {
            let mut discovery = CapabilityDiscovery::new(ipc);
            if let Ok(endpoint) = discovery.find("security").await {
                let ep = endpoint.endpoint;
                if ep.starts_with('/') && std::path::Path::new(&ep).exists() {
                    info!("Found security provider via capability discovery: {}", ep);
                    return Self::new(ep);
                }
            }
        }

        let security_slug = env
            .get("NESTGATE_SECURITY_SLUG")
            .unwrap_or_else(|| "security".to_string());

        let socket_dirs = Self::candidate_socket_dirs_from_env(env);
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
    /// 1. `XDG_RUNTIME_DIR` (recommended, per-user, tmpfs)
    /// 2. /run/user/{uid} (standard XDG fallback)
    /// 3. /tmp (least secure, universal fallback)
    fn candidate_socket_dirs_from_env(env: &(impl EnvSource + ?Sized)) -> Vec<String> {
        let mut dirs = Vec::with_capacity(3);
        if let Some(xdg) = env.get("XDG_RUNTIME_DIR") {
            dirs.push(xdg);
            // XDG_RUNTIME_DIR is typically /run/user/{uid} — no need to duplicate
        } else if let Some(uid) = env.get("UID").or_else(|| env.get("EUID")) {
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
                NestGateError::network_error(format!("Failed to read {}: {e}", parent.display()))
            })?;
            loop {
                let entry = match entries.next_entry().await {
                    Ok(Some(e)) => e,
                    Ok(None) => break,
                    Err(e) => {
                        return Err(NestGateError::network_error(format!(
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
                "No matching security provider socket for glob pattern",
            ));
        }

        let path = PathBuf::from(pattern);
        if path.exists() {
            // Try to connect
            UnixStream::connect(&path)
                .await
                .map_err(|e| NestGateError::network_error(format!("Failed to connect: {e}")))?;
            Ok(path)
        } else {
            Err(NestGateError::network_error("Socket does not exist"))
        }
    }

    /// Connect to the security provider.
    ///
    /// # Errors
    ///
    /// Returns error if the socket does not exist or connection fails.
    pub async fn connect(&mut self) -> Result<()> {
        if !self.socket_path.exists() {
            return Err(NestGateError::network_error(format!(
                "Security provider socket not found: {}",
                self.socket_path.display()
            )));
        }

        let _stream = UnixStream::connect(&self.socket_path).await.map_err(|e| {
            NestGateError::network_error(format!("Failed to connect to security provider: {e}"))
        })?;

        self.connected = true;
        info!(
            "Connected to security provider: {}",
            self.socket_path.display()
        );
        Ok(())
    }

    /// Encrypt data via the security provider.
    ///
    /// # Errors
    ///
    /// Returns error if not connected or encryption fails.
    pub async fn encrypt(&self, plaintext: &[u8]) -> Result<Vec<u8>> {
        if !self.connected {
            return Err(NestGateError::security_error(
                "Not connected to security provider",
            ));
        }

        let request = SecurityProviderRequest {
            method: "encrypt".to_string(),
            data: plaintext.to_vec(),
        };

        let response = self.send_request(&request).await?;
        Ok(response.data)
    }

    /// Decrypt data via the security provider.
    ///
    /// # Errors
    ///
    /// Returns error if not connected or decryption fails.
    pub async fn decrypt(&self, ciphertext: &[u8]) -> Result<Vec<u8>> {
        if !self.connected {
            return Err(NestGateError::security_error(
                "Not connected to security provider",
            ));
        }

        let request = SecurityProviderRequest {
            method: "decrypt".to_string(),
            data: ciphertext.to_vec(),
        };

        let response = self.send_request(&request).await?;
        Ok(response.data)
    }

    /// Generate an authentication token.
    ///
    /// # Errors
    ///
    /// Returns error if not connected or token generation fails.
    pub async fn generate_token(&self, identity: &str) -> Result<String> {
        if !self.connected {
            return Err(NestGateError::security_error(
                "Not connected to security provider",
            ));
        }

        let request = SecurityProviderRequest {
            method: "generate_token".to_string(),
            data: identity.as_bytes().to_vec(),
        };

        let response = self.send_request(&request).await?;
        String::from_utf8(response.data)
            .map_err(|e| NestGateError::security_error(format!("Invalid token: {e}")))
    }

    /// Validate an authentication token.
    ///
    /// # Errors
    ///
    /// Returns error if not connected or validation fails.
    pub async fn validate_token(&self, token: &str) -> Result<bool> {
        if !self.connected {
            return Err(NestGateError::security_error(
                "Not connected to security provider",
            ));
        }

        let request = SecurityProviderRequest {
            method: "validate_token".to_string(),
            data: token.as_bytes().to_vec(),
        };

        let response = self.send_request(&request).await?;
        Ok(!response.data.is_empty() && response.data[0] == 1)
    }

    async fn send_request(
        &self,
        request: &SecurityProviderRequest,
    ) -> Result<SecurityProviderResponse> {
        let mut stream = UnixStream::connect(&self.socket_path)
            .await
            .map_err(|e| NestGateError::network_error(format!("Failed to connect: {e}")))?;

        // Serialize and send request
        let request_json = serde_json::to_vec(request)
            .map_err(|e| NestGateError::api_error(format!("Failed to serialize request: {e}")))?;

        stream
            .write_all(&request_json)
            .await
            .map_err(|e| NestGateError::network_error(format!("Failed to send request: {e}")))?;

        let mut buffer = vec![0u8; 65536];
        let n = stream
            .read(&mut buffer)
            .await
            .map_err(|e| NestGateError::network_error(format!("Failed to read response: {e}")))?;

        let response: SecurityProviderResponse =
            serde_json::from_slice(&buffer[..n]).map_err(|e| {
                NestGateError::api_error(format!("Failed to deserialize response: {e}"))
            })?;

        if !response.success {
            return Err(NestGateError::security_error(format!(
                "Security provider error: {}",
                response
                    .error
                    .unwrap_or_else(|| "Unknown error".to_string())
            )));
        }

        Ok(response)
    }

    /// Check if connected to the security provider.
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
// Security Provider Protocol Types
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
struct SecurityProviderRequest {
    method: String,
    data: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
struct SecurityProviderResponse {
    success: bool,
    data: Vec<u8>,
    #[serde(default)]
    error: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use nestgate_types::MapEnv;
    use serde_json::json;
    use std::sync::Arc;
    use std::time::Duration;
    use tempfile::TempDir;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::UnixListener;

    #[test]
    fn security_protocol_types_serde_roundtrip() {
        let req = json!({
            "method": "encrypt",
            "data": [1u8, 2, 3]
        });
        let back: serde_json::Value =
            serde_json::from_str(&serde_json::to_string(&req).unwrap()).unwrap();
        assert_eq!(back["method"], "encrypt");

        let resp = json!({
            "success": true,
            "data": [9u8],
            "error": null
        });
        let s = serde_json::to_string(&resp).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&s).unwrap();
        assert!(parsed["success"].as_bool().unwrap());
    }

    #[test]
    fn test_client_creation() {
        let temp_dir = TempDir::new().unwrap();
        let socket_path = temp_dir.path().join("security.sock");
        let client = SecurityProviderClient::new(&socket_path);
        assert!(client.is_ok());
        let c = client.unwrap();
        assert_eq!(c.socket_path(), socket_path);
        assert!(!c.is_connected());
    }

    #[test]
    fn test_client_empty_path() {
        let result = SecurityProviderClient::new("");
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_discover_fallback() {
        let result = SecurityProviderClient::discover("test").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn discover_via_env_returns_client_without_socket_file() {
        let env = MapEnv::from([(
            "NESTGATE_SECURITY_PROVIDER",
            "/tmp/nestgate-test-security-not-created.sock",
        )]);
        let c = SecurityProviderClient::discover_from_env_source(&env, "fam")
            .await
            .expect("env path");
        assert_eq!(
            c.socket_path(),
            std::path::Path::new("/tmp/nestgate-test-security-not-created.sock")
        );
    }

    #[tokio::test]
    async fn connect_errors_when_socket_missing() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("nope.sock");
        let mut c = SecurityProviderClient::new(&path).unwrap();
        assert!(c.connect().await.is_err());
        assert!(!c.is_connected());
    }

    #[tokio::test]
    async fn encrypt_decrypt_errors_when_not_connected() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("x.sock");
        let c = SecurityProviderClient::new(&path).unwrap();
        assert!(c.encrypt(b"hi").await.is_err());
        assert!(c.decrypt(b"hi").await.is_err());
        assert!(c.generate_token("id").await.is_err());
        assert!(c.validate_token("t").await.is_err());
    }

    /// Minimal security-provider-compatible mock responder for testing.
    async fn run_mock_security_provider(
        path: std::path::PathBuf,
        on_success: bool,
        token_bytes: Vec<u8>,
        idle: Duration,
    ) {
        run_mock_security_provider_signaled(path, on_success, token_bytes, idle, None).await;
    }

    async fn run_mock_security_provider_signaled(
        path: std::path::PathBuf,
        on_success: bool,
        token_bytes: Vec<u8>,
        idle: Duration,
        ready: Option<Arc<tokio::sync::Notify>>,
    ) {
        let _ = tokio::fs::remove_file(&path).await;
        let listener = UnixListener::bind(&path).expect("bind mock security provider");
        if let Some(n) = ready {
            n.notify_one();
        }
        loop {
            let accept_fut = listener.accept();
            let Ok(accept_result) = tokio::time::timeout(idle, accept_fut).await else {
                break;
            };
            let Ok((mut stream, _)) = accept_result else {
                break;
            };
            let mut buf = vec![0u8; 65536];
            let Ok(n) = stream.read(&mut buf).await else {
                continue;
            };
            if n == 0 {
                continue;
            }
            let v: serde_json::Value = match serde_json::from_slice(&buf[..n]) {
                Ok(v) => v,
                Err(_) => continue,
            };
            let method = v["method"].as_str().unwrap_or("");
            let data = v["data"]
                .as_array()
                .map(|a| {
                    a.iter()
                        .filter_map(|x| x.as_u64().map(|n| n as u8))
                        .collect::<Vec<u8>>()
                })
                .unwrap_or_default();

            let (success, out_data, err) = if on_success {
                let out = match method {
                    "encrypt" | "decrypt" => data,
                    "generate_token" => b"tok".to_vec(),
                    "validate_token" => token_bytes.clone(),
                    _ => vec![],
                };
                (true, out, None)
            } else {
                (false, vec![], Some("mock failure".to_string()))
            };

            let resp = json!({
                "success": success,
                "data": out_data,
                "error": err
            });
            let bytes = serde_json::to_vec(&resp).unwrap();
            let _ = stream.write_all(&bytes).await;
        }
    }

    /// Spawn the mock server and wait for it to be ready (listener bound).
    async fn spawn_mock_server(
        path: &std::path::Path,
        on_success: bool,
        token_bytes: Vec<u8>,
        idle: Duration,
    ) -> tokio::task::JoinHandle<()> {
        let ready = Arc::new(tokio::sync::Notify::new());
        let path_owned = path.to_path_buf();
        let r = Arc::clone(&ready);
        let handle = tokio::spawn(run_mock_security_provider_signaled(
            path_owned,
            on_success,
            token_bytes,
            idle,
            Some(r),
        ));
        ready.notified().await;
        handle
    }

    #[tokio::test]
    async fn connect_encrypt_roundtrip_with_mock_server() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("sec.sock");
        let server = spawn_mock_server(&path, true, vec![], Duration::from_millis(400)).await;

        let mut c = SecurityProviderClient::new(&path).unwrap();
        c.connect().await.expect("connect");
        assert!(c.is_connected());
        let out = c.encrypt(b"plain").await.expect("encrypt");
        assert_eq!(out, b"plain");

        let _ = server.await;
    }

    #[tokio::test]
    async fn send_request_returns_security_error_when_success_false() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("sec2.sock");
        let server = spawn_mock_server(&path, false, vec![], Duration::from_millis(400)).await;

        let mut c = SecurityProviderClient::new(&path).unwrap();
        c.connect().await.unwrap();
        let err = c
            .encrypt(b"x")
            .await
            .expect_err("expect security provider err");
        assert!(
            err.to_string().contains("mock failure")
                || err.to_string().contains("Security provider")
        );

        let _ = server.await;
    }

    #[tokio::test]
    async fn validate_token_interprets_first_byte() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("sec3.sock");
        let server = spawn_mock_server(&path, true, vec![1], Duration::from_millis(400)).await;

        let mut c = SecurityProviderClient::new(&path).unwrap();
        c.connect().await.unwrap();
        assert!(c.validate_token("any").await.unwrap());

        let _ = server.await;
    }

    #[tokio::test]
    async fn discover_finds_socket_via_glob_under_tmp() {
        let dir = TempDir::new().unwrap();
        let sock_name = "security-globfam-zz.sock";
        let path = dir.path().join(sock_name);
        let _ = tokio::fs::remove_file(&path).await;
        let _listener = UnixListener::bind(&path).expect("bind for discover");
        let path_clone = path.clone();

        let env = MapEnv::from([
            ("XDG_RUNTIME_DIR", dir.path().to_str().expect("utf8 path")),
            ("NESTGATE_SECURITY_SLUG", "security"),
        ]);
        let c = SecurityProviderClient::discover_from_env_source(&env, "globfam")
            .await
            .expect("glob discover");
        assert_eq!(c.socket_path(), path_clone);
    }
}

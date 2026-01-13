//! # 🎵 Songbird Auto-Registration
//!
//! **Automatic Service Registration with Songbird Orchestrator**
//!
//! Implements automatic service registration and health reporting
//! with the Songbird orchestration primal at startup.
//!
//! ## Philosophy
//! - **Self-Knowledge**: Register own capabilities only
//! - **Runtime Discovery**: Discover Songbird via $SONGBIRD_FAMILY_ID
//! - **Zero Hardcoding**: All configuration from environment
//! - **Graceful Fallback**: Continues without Songbird if unavailable
//! - **Modern Async**: Native async/await patterns
//!
//! ## Environment Variables
//! - `NESTGATE_FAMILY_ID` (required): Own family identifier
//! - `SONGBIRD_FAMILY_ID` (optional): Songbird's family identifier for discovery
//! - `NESTGATE_DISABLE_SONGBIRD` (optional): Disable auto-registration
//!
//! ## Usage
//! ```no_run
//! use nestgate_core::rpc::songbird_registration::SongbirdRegistration;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let family_id = std::env::var("NESTGATE_FAMILY_ID")?;
//! let registration = SongbirdRegistration::new(&family_id).await?;
//! registration.register().await?;
//! # Ok(())
//! # }
//! ```

use crate::error::{NestGateError, Result};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::time::sleep;
use tracing::{debug, error, info, warn};

/// Service registration data for Songbird
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceRegistrationData {
    /// Service identifier (NestGate family ID)
    pub service_id: String,
    /// Service type (always "storage" for NestGate)
    pub service_type: String,
    /// Primal name (always "nestgate")
    pub primal_name: String,
    /// Service capabilities
    pub capabilities: Vec<String>,
    /// Service tags for discovery
    pub tags: Vec<String>,
    /// Unix socket path for IPC
    pub socket_path: String,
    /// Service version
    pub version: String,
    /// Registration timestamp (ISO 8601)
    pub registration_time: String,
}

/// Health status report for Songbird
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthReport {
    /// Service identifier
    pub service_id: String,
    /// Health status: "healthy", "degraded", "unhealthy"
    pub status: String,
    /// Timestamp (ISO 8601)
    pub timestamp: String,
    /// Optional health details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
}

/// Songbird auto-registration manager
pub struct SongbirdRegistration {
    /// NestGate family ID
    family_id: String,
    /// Songbird socket path (if discovered)
    songbird_socket: Option<String>,
    /// Registration enabled flag
    enabled: bool,
}

impl SongbirdRegistration {
    /// Create new Songbird registration manager
    ///
    /// # Self-Knowledge Principle
    /// - Uses own family_id for registration
    /// - Discovers Songbird socket path at runtime
    /// - No hardcoded Songbird assumptions
    ///
    /// # Arguments
    /// - `family_id`: NestGate's family identifier from $NESTGATE_FAMILY_ID
    ///
    /// # Errors
    /// - Returns error if configuration is invalid
    pub async fn new(family_id: &str) -> Result<Self> {
        // Check if registration is disabled
        let enabled = std::env::var("NESTGATE_DISABLE_SONGBIRD")
            .map(|v| v != "1" && v.to_lowercase() != "true")
            .unwrap_or(true);

        if !enabled {
            info!("🎵 Songbird auto-registration disabled by environment");
            return Ok(Self {
                family_id: family_id.to_string(),
                songbird_socket: None,
                enabled: false,
            });
        }

        // Discover Songbird socket path via environment
        let songbird_socket = Self::discover_songbird_socket().await;

        if songbird_socket.is_none() {
            warn!("🎵 Songbird socket not discovered - continuing without orchestration");
            info!("   Set $SONGBIRD_FAMILY_ID to enable auto-registration");
        }

        Ok(Self {
            family_id: family_id.to_string(),
            songbird_socket,
            enabled,
        })
    }

    /// Discover Songbird socket path
    ///
    /// # Self-Knowledge Principle
    /// - Uses $SONGBIRD_FAMILY_ID for discovery (not hardcoded)
    /// - Falls back to common patterns only as last resort
    /// - Logs all discovery attempts for transparency
    async fn discover_songbird_socket() -> Option<String> {
        // Primary: Use $SONGBIRD_FAMILY_ID for discovery
        if let Ok(songbird_family_id) = std::env::var("SONGBIRD_FAMILY_ID") {
            let uid = crate::platform::get_current_uid();
            let socket_path = format!("/run/user/{}/songbird-{}.sock", uid, songbird_family_id);

            debug!("Checking Songbird socket: {}", socket_path);

            if std::path::Path::new(&socket_path).exists() {
                info!("✅ Discovered Songbird socket: {}", socket_path);
                return Some(socket_path);
            } else {
                warn!(
                    "🎵 Songbird socket not found at expected path: {}",
                    socket_path
                );
            }
        }

        // No fallback to hardcoded paths - pure discovery
        debug!("Songbird socket discovery failed - no $SONGBIRD_FAMILY_ID set");
        None
    }

    /// Register with Songbird orchestrator
    ///
    /// Sends service registration data to Songbird via Unix socket.
    /// Returns Ok(()) whether registration succeeds or not (graceful fallback).
    ///
    /// # Errors
    /// - Never returns error (logs failures and continues)
    pub async fn register(&self) -> Result<()> {
        if !self.enabled {
            debug!("Songbird registration disabled");
            return Ok(());
        }

        let songbird_socket = match &self.songbird_socket {
            Some(socket) => socket,
            None => {
                debug!("No Songbird socket available - skipping registration");
                return Ok(());
            }
        };

        info!("🎵 Registering with Songbird orchestrator...");

        // Build registration data
        let uid = crate::platform::get_current_uid();
        let socket_path = format!("/run/user/{}/nestgate-{}.sock", uid, self.family_id);

        let registration = ServiceRegistrationData {
            service_id: self.family_id.clone(),
            service_type: "storage".to_string(),
            primal_name: "nestgate".to_string(),
            capabilities: vec![
                "storage".to_string(),
                "persistence".to_string(),
                "key-value".to_string(),
                "blob-storage".to_string(),
                "json-rpc".to_string(),
                "unix-socket".to_string(),
            ],
            tags: vec![
                "storage".to_string(),
                "nestgate".to_string(),
                "primal".to_string(),
            ],
            socket_path,
            version: env!("CARGO_PKG_VERSION").to_string(),
            registration_time: chrono::Utc::now().to_rfc3339(),
        };

        // Attempt registration via JSON-RPC over Unix socket
        match Self::send_registration(songbird_socket, &registration).await {
            Ok(()) => {
                info!("✅ Successfully registered with Songbird");
                info!("   Family ID: {}", self.family_id);
                info!("   Capabilities: {}", registration.capabilities.len());
            }
            Err(e) => {
                warn!("⚠️  Failed to register with Songbird: {}", e);
                warn!("   Continuing without orchestration (graceful fallback)");
            }
        }

        Ok(())
    }

    /// Send registration data to Songbird
    async fn send_registration(
        socket_path: &str,
        registration: &ServiceRegistrationData,
    ) -> Result<()> {
        use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
        use tokio::net::UnixStream;

        // Connect to Songbird
        let stream = UnixStream::connect(socket_path).await.map_err(|e| {
            NestGateError::configuration_error(
                "songbird_connect",
                &format!("Failed to connect to Songbird socket: {}", e),
            )
        })?;

        let (reader, mut writer) = stream.into_split();
        let mut reader = BufReader::new(reader);

        // Build JSON-RPC request
        let request = serde_json::json!({
            "jsonrpc": "2.0",
            "method": "register_service",
            "params": registration,
            "id": 1
        });

        let request_json = serde_json::to_string(&request)
            .map_err(|e| NestGateError::api(format!("Failed to serialize request: {}", e)))?;

        // Send request
        writer
            .write_all(request_json.as_bytes())
            .await
            .map_err(|e| NestGateError::io_error(format!("Failed to send request: {}", e)))?;
        writer
            .write_all(b"\n")
            .await
            .map_err(|e| NestGateError::io_error(format!("Failed to send newline: {}", e)))?;

        // Read response
        let mut response_line = String::new();
        reader
            .read_line(&mut response_line)
            .await
            .map_err(|e| NestGateError::io_error(format!("Failed to read response: {}", e)))?;

        // Parse response
        let response: serde_json::Value = serde_json::from_str(&response_line)
            .map_err(|e| NestGateError::api(format!("Failed to parse response: {}", e)))?;

        // Check for errors
        if let Some(error) = response.get("error") {
            return Err(NestGateError::api(format!(
                "Songbird registration failed: {}",
                error
            )));
        }

        debug!("Received registration response: {:?}", response);
        Ok(())
    }

    /// Send periodic health report to Songbird
    ///
    /// Should be called periodically (e.g., every 30 seconds) to maintain
    /// service registration and report health status.
    ///
    /// # Errors
    /// - Never returns error (logs failures and continues)
    pub async fn report_health(&self) -> Result<()> {
        if !self.enabled {
            return Ok(());
        }

        let songbird_socket = match &self.songbird_socket {
            Some(socket) => socket,
            None => return Ok(()),
        };

        let health = HealthReport {
            service_id: self.family_id.clone(),
            status: "healthy".to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            details: None,
        };

        match Self::send_health_report(songbird_socket, &health).await {
            Ok(()) => {
                debug!("Health report sent to Songbird");
            }
            Err(e) => {
                warn!("Failed to send health report: {}", e);
            }
        }

        Ok(())
    }

    /// Send health report to Songbird
    async fn send_health_report(socket_path: &str, health: &HealthReport) -> Result<()> {
        use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
        use tokio::net::UnixStream;

        let stream = UnixStream::connect(socket_path).await.map_err(|e| {
            NestGateError::configuration_error(
                "songbird_connect",
                &format!("Failed to connect to Songbird: {}", e),
            )
        })?;

        let (reader, mut writer) = stream.into_split();
        let mut reader = BufReader::new(reader);

        let request = serde_json::json!({
            "jsonrpc": "2.0",
            "method": "report_health",
            "params": health,
            "id": 1
        });

        let request_json = serde_json::to_string(&request)
            .map_err(|e| NestGateError::api(format!("Failed to serialize request: {}", e)))?;

        writer
            .write_all(request_json.as_bytes())
            .await
            .map_err(|e| NestGateError::io_error(format!("Failed to send request: {}", e)))?;
        writer
            .write_all(b"\n")
            .await
            .map_err(|e| NestGateError::io_error(format!("Failed to send newline: {}", e)))?;

        let mut response_line = String::new();
        reader
            .read_line(&mut response_line)
            .await
            .map_err(|e| NestGateError::io_error(format!("Failed to read response: {}", e)))?;

        Ok(())
    }

    /// Start periodic health reporting
    ///
    /// Spawns a background task that sends health reports every 30 seconds.
    /// The task runs indefinitely until the program exits.
    pub fn start_health_reporting(&self) {
        if !self.enabled {
            return;
        }

        let registration = self.clone();
        tokio::spawn(async move {
            loop {
                sleep(Duration::from_secs(30)).await;
                if let Err(e) = registration.report_health().await {
                    error!("Health reporting error: {}", e);
                }
            }
        });

        info!("🎵 Started periodic health reporting (30s interval)");
    }
}

impl Clone for SongbirdRegistration {
    fn clone(&self) -> Self {
        Self {
            family_id: self.family_id.clone(),
            songbird_socket: self.songbird_socket.clone(),
            enabled: self.enabled,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_registration_disabled() {
        std::env::set_var("NESTGATE_DISABLE_SONGBIRD", "true");
        let registration = SongbirdRegistration::new("test_family").await.unwrap();
        assert!(!registration.enabled);
        std::env::remove_var("NESTGATE_DISABLE_SONGBIRD");
    }

    #[tokio::test]
    async fn test_registration_data_serialization() {
        let data = ServiceRegistrationData {
            service_id: "test-123".to_string(),
            service_type: "storage".to_string(),
            primal_name: "nestgate".to_string(),
            capabilities: vec!["storage".to_string()],
            tags: vec!["test".to_string()],
            socket_path: "/tmp/test.sock".to_string(),
            version: "1.0.0".to_string(),
            registration_time: "2026-01-10T00:00:00Z".to_string(),
        };

        let json = serde_json::to_string(&data).unwrap();
        assert!(json.contains("test-123"));
        assert!(json.contains("storage"));
    }

    #[tokio::test]
    async fn test_health_report_serialization() {
        let health = HealthReport {
            service_id: "test-456".to_string(),
            status: "healthy".to_string(),
            timestamp: "2026-01-10T00:00:00Z".to_string(),
            details: None,
        };

        let json = serde_json::to_string(&health).unwrap();
        assert!(json.contains("test-456"));
        assert!(json.contains("healthy"));
    }

    #[tokio::test]
    async fn test_graceful_fallback_no_songbird() {
        // Without SONGBIRD_FAMILY_ID, should succeed but not register
        std::env::remove_var("SONGBIRD_FAMILY_ID");
        let registration = SongbirdRegistration::new("test_family").await.unwrap();
        assert!(registration.songbird_socket.is_none());

        // Register should succeed (graceful fallback)
        let result = registration.register().await;
        assert!(result.is_ok());
    }
}

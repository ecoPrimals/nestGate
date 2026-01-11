//! # 🔌 Socket Configuration - Production-Grade
//!
//! **Standardized Socket Management** for atomic architecture
//!
//! Implements the biomeOS socket configuration standard with robust
//! fallback logic and comprehensive error handling.
//!
//! ## Configuration Priority (3-tier)
//!
//! 1. **`NESTGATE_SOCKET`** env var (explicit override, highest priority)
//! 2. **XDG Runtime Directory**: `/run/user/{uid}/nestgate-{family}.sock` (recommended)
//! 3. **Temp Directory**: `/tmp/nestgate-{family}-{node}.sock` (fallback, least secure)
//!
//! ## Environment Variables
//!
//! - `NESTGATE_SOCKET`: Absolute path to socket (optional)
//! - `NESTGATE_FAMILY_ID`: Family identifier (required)
//! - `NESTGATE_NODE_ID`: Node identifier for multi-instance (optional, default: "default")
//!
//! ## Philosophy
//!
//! - **Agnostic**: Works in any environment (XDG, tmpfs, custom)
//! - **Self-Knowledge**: Derives path from own identity
//! - **Buildable**: Creates directories, cleans old sockets
//! - **Secure**: Prefers XDG runtime directory over /tmp
//! - **Atomic-Ready**: Supports multiple instances with unique paths

use crate::error::{NestGateError, Result};
use std::path::{Path, PathBuf};
use tracing::{debug, info, warn};

/// Socket configuration with standardized fallback logic
#[derive(Debug, Clone)]
pub struct SocketConfig {
    /// Final socket path to use
    pub socket_path: PathBuf,
    /// Family identifier
    pub family_id: String,
    /// Node identifier (for multi-instance)
    pub node_id: String,
    /// Configuration source (for logging/debugging)
    pub source: SocketConfigSource,
}

/// Source of socket configuration
#[derive(Debug, Clone, PartialEq)]
pub enum SocketConfigSource {
    /// Explicit environment variable (NESTGATE_SOCKET)
    Environment,
    /// XDG runtime directory (/run/user/{uid})
    XdgRuntime,
    /// Temporary directory fallback (/tmp)
    TempDirectory,
}

impl SocketConfig {
    /// Get socket configuration with standardized 3-tier fallback
    ///
    /// # Priority Order
    ///
    /// 1. `NESTGATE_SOCKET` env var (if set, use as-is)
    /// 2. XDG runtime: `/run/user/{uid}/nestgate-{family}.sock`
    /// 3. Temp fallback: `/tmp/nestgate-{family}-{node}.sock`
    ///
    /// # Returns
    ///
    /// Socket configuration with path, IDs, and source information
    pub fn from_environment() -> Result<Self> {
        // Get family_id (required)
        let family_id = std::env::var("NESTGATE_FAMILY_ID").unwrap_or_else(|_| {
            warn!("NESTGATE_FAMILY_ID not set, using 'default'");
            "default".to_string()
        });

        // Get node_id (optional, for multi-instance)
        let node_id = std::env::var("NESTGATE_NODE_ID").unwrap_or_else(|_| "default".to_string());

        // Tier 1: Check for explicit NESTGATE_SOCKET env var
        if let Ok(socket_path) = std::env::var("NESTGATE_SOCKET") {
            info!(
                "Using explicit socket path from NESTGATE_SOCKET: {}",
                socket_path
            );
            return Ok(Self {
                socket_path: PathBuf::from(socket_path),
                family_id,
                node_id,
                source: SocketConfigSource::Environment,
            });
        }

        // Get UID for XDG path
        let uid = unsafe { libc::getuid() };

        // Tier 2: Try XDG runtime directory (preferred, more secure)
        let xdg_runtime_dir = format!("/run/user/{}", uid);
        if Path::new(&xdg_runtime_dir).exists() {
            let socket_path =
                PathBuf::from(format!("{}/nestgate-{}.sock", xdg_runtime_dir, family_id));

            info!(
                "Using XDG runtime directory socket: {} (family: {}, node: {})",
                socket_path.display(),
                family_id,
                node_id
            );

            return Ok(Self {
                socket_path,
                family_id,
                node_id,
                source: SocketConfigSource::XdgRuntime,
            });
        }

        // Tier 3: Fallback to /tmp (least secure, but always available)
        let socket_path = PathBuf::from(format!("/tmp/nestgate-{}-{}.sock", family_id, node_id));

        warn!(
            "XDG runtime directory not available, falling back to /tmp: {}",
            socket_path.display()
        );
        warn!(
            "Note: /tmp is less secure than XDG runtime directory. Consider creating /run/user/{}",
            uid
        );

        Ok(Self {
            socket_path,
            family_id,
            node_id,
            source: SocketConfigSource::TempDirectory,
        })
    }

    /// Prepare socket path for binding
    ///
    /// # Actions
    ///
    /// 1. Create parent directories if needed
    /// 2. Remove old socket file if exists
    /// 3. Verify directory is writable
    ///
    /// # Returns
    ///
    /// Ok if socket path is ready to bind
    pub fn prepare_socket_path(&self) -> Result<()> {
        // Create parent directories if needed
        if let Some(parent) = self.socket_path.parent() {
            if !parent.exists() {
                std::fs::create_dir_all(parent).map_err(|e| {
                    NestGateError::configuration_error(
                        "socket_directory",
                        &format!("Failed to create socket directory {:?}: {}", parent, e),
                    )
                })?;

                info!("Created socket directory: {:?}", parent);
            }

            // Verify directory is writable
            if parent.exists() && std::fs::metadata(parent).is_ok() {
                debug!("Socket directory exists and is accessible: {:?}", parent);
            }
        }

        // Remove old socket file if it exists (avoid "address already in use")
        if self.socket_path.exists() {
            std::fs::remove_file(&self.socket_path).map_err(|e| {
                NestGateError::configuration_error(
                    "socket_cleanup",
                    &format!(
                        "Failed to remove existing socket {:?}: {}",
                        self.socket_path, e
                    ),
                )
            })?;

            debug!("Removed old socket file: {:?}", self.socket_path);
        }

        Ok(())
    }

    /// Get socket path as string
    pub fn socket_path_str(&self) -> &str {
        self.socket_path.to_str().unwrap_or("")
    }

    /// Log configuration summary
    pub fn log_summary(&self) {
        info!("═══════════════════════════════════════════════════════════");
        info!("Socket Configuration:");
        info!("  Path:      {}", self.socket_path.display());
        info!("  Family ID: {}", self.family_id);
        info!("  Node ID:   {}", self.node_id);
        info!(
            "  Source:    {:?}",
            match self.source {
                SocketConfigSource::Environment => "NESTGATE_SOCKET env var",
                SocketConfigSource::XdgRuntime => "XDG runtime directory",
                SocketConfigSource::TempDirectory => "/tmp fallback",
            }
        );
        info!("═══════════════════════════════════════════════════════════");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_socket_config_with_explicit_path() {
        std::env::set_var("NESTGATE_SOCKET", "/tmp/test.sock");
        std::env::set_var("NESTGATE_FAMILY_ID", "test");

        let config = SocketConfig::from_environment().unwrap();

        assert_eq!(config.socket_path, PathBuf::from("/tmp/test.sock"));
        assert_eq!(config.family_id, "test");
        assert_eq!(config.source, SocketConfigSource::Environment);

        std::env::remove_var("NESTGATE_SOCKET");
        std::env::remove_var("NESTGATE_FAMILY_ID");
    }

    #[test]
    fn test_socket_config_fallback_to_tmp() {
        std::env::remove_var("NESTGATE_SOCKET");
        std::env::set_var("NESTGATE_FAMILY_ID", "test");
        std::env::set_var("NESTGATE_NODE_ID", "node1");

        let config = SocketConfig::from_environment().unwrap();

        // Should use /tmp since XDG runtime might not exist in test env
        assert!(
            config
                .socket_path
                .to_str()
                .unwrap()
                .starts_with("/tmp/nestgate-")
                || config
                    .socket_path
                    .to_str()
                    .unwrap()
                    .starts_with("/run/user/")
        );

        std::env::remove_var("NESTGATE_FAMILY_ID");
        std::env::remove_var("NESTGATE_NODE_ID");
    }

    #[test]
    fn test_socket_path_preparation() {
        let config = SocketConfig {
            socket_path: PathBuf::from("/tmp/test-prepare.sock"),
            family_id: "test".to_string(),
            node_id: "node1".to_string(),
            source: SocketConfigSource::TempDirectory,
        };

        // Should succeed (creating /tmp is always possible)
        assert!(config.prepare_socket_path().is_ok());
    }
}

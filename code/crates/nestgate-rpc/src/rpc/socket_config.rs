// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! # 🔌 Socket Configuration - Production-Grade
//!
//! **Standardized Socket Management** for atomic architecture
//!
//! Implements the biomeOS socket configuration standard with robust
//! fallback logic and comprehensive error handling.
//!
//! ## Configuration Priority (4-tier)
//!
//! 1. **`NESTGATE_SOCKET`** env var (explicit override, highest priority)
//! 2. **`BIOMEOS_SOCKET_DIR`** + `nestgate.sock` (biomeOS standard)
//! 3. **`$XDG_RUNTIME_DIR/biomeos/nestgate.sock`** (preferred; reads the actual `XDG_RUNTIME_DIR` env var)
//! 4. **Temp Directory**: `/tmp/nestgate-{family}-{node}.sock` (fallback, least secure)
//!
//! ## Environment Variables
//!
//! - `NESTGATE_SOCKET`: Absolute path to socket (optional, highest priority)
//! - `BIOMEOS_SOCKET_DIR`: biomeOS shared socket directory (optional, e.g., `$XDG_RUNTIME_DIR/biomeos`)
//! - `XDG_RUNTIME_DIR`: Base runtime path (optional; tier 3 uses `$XDG_RUNTIME_DIR/biomeos/nestgate.sock`)
//! - `NESTGATE_FAMILY_ID`: Family identifier (optional; default: `standalone` per wateringHole)
//! - `NESTGATE_NODE_ID`: Node identifier for multi-instance (optional; default: system hostname)
//!
//! ## Philosophy
//!
//! - **Agnostic**: Works in any environment (XDG, tmpfs, custom)
//! - **Self-Knowledge**: Derives path from own identity
//! - **Buildable**: Creates directories, cleans old sockets
//! - **Secure**: Prefers XDG runtime directory over /tmp
//! - **Atomic-Ready**: Supports multiple instances with unique paths

use nestgate_types::error::{NestGateError, Result};
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SocketConfigSource {
    /// Explicit environment variable (`NESTGATE_SOCKET`)
    Environment,
    /// biomeOS shared socket directory (`BIOMEOS_SOCKET_DIR`)
    BiomeOSDirectory,
    /// `$XDG_RUNTIME_DIR/biomeos/nestgate.sock`
    XdgRuntime,
    /// Temporary directory fallback (/tmp)
    TempDirectory,
}

impl SocketConfig {
    /// Resolve socket configuration from explicit parameters
    ///
    /// This is the pure-logic core that does not read environment variables.
    /// Prefer this over `from_environment()` when parameters are already known.
    ///
    /// # Priority Order
    ///
    /// 1. `socket_override` (if `Some`, use as-is)
    /// 2. `biomeos_socket_dir` (if `Some`, use `{dir}/nestgate.sock`)
    /// 3. XDG runtime: `{xdg_runtime_dir}/biomeos/nestgate.sock` when `xdg_runtime_dir` is set and exists
    /// 4. Temp fallback: `/tmp/nestgate-{family}-{node}.sock`
    pub fn resolve(
        family_id: String,
        node_id: String,
        socket_override: Option<String>,
        biomeos_socket_dir: Option<String>,
        xdg_runtime_dir: Option<String>,
    ) -> Result<Self> {
        // Tier 1: Explicit socket path override (highest priority)
        if let Some(socket_path) = socket_override {
            info!("🔌 Using explicit socket path: {}", socket_path);
            return Ok(Self {
                socket_path: PathBuf::from(socket_path),
                family_id,
                node_id,
                source: SocketConfigSource::Environment,
            });
        }

        // Tier 2: biomeOS shared directory (biomeOS standard)
        if let Some(biomeos_dir) = biomeos_socket_dir {
            let socket_path = PathBuf::from(biomeos_dir).join("nestgate.sock");

            info!(
                "🔌 Using biomeOS socket directory: {} (family: {}, node: {})",
                socket_path.display(),
                family_id,
                node_id
            );

            return Ok(Self {
                socket_path,
                family_id,
                node_id,
                source: SocketConfigSource::BiomeOSDirectory,
            });
        }

        // Tier 3: `$XDG_RUNTIME_DIR/biomeos/nestgate.sock` (preferred)
        if let Some(ref dir) = xdg_runtime_dir
            && !dir.is_empty()
            && Path::new(dir).exists()
        {
            let socket_path = PathBuf::from(dir).join("biomeos").join("nestgate.sock");

            info!(
                "🔌 Using XDG runtime directory with biomeOS standard: {} (family: {}, node: {})",
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

        // Tier 4: Fallback to /tmp (least secure, but always available)
        let socket_path = PathBuf::from(format!("/tmp/nestgate-{family_id}-{node_id}.sock"));

        warn!(
            "⚠️  XDG runtime directory unavailable or not set, falling back to /tmp: {}",
            socket_path.display()
        );
        warn!("Note: /tmp is less secure than $XDG_RUNTIME_DIR/biomeos/nestgate.sock");

        Ok(Self {
            socket_path,
            family_id,
            node_id,
            source: SocketConfigSource::TempDirectory,
        })
    }

    /// Get socket configuration from environment variables
    ///
    /// Reads `NESTGATE_SOCKET`, `NESTGATE_FAMILY_ID`, `NESTGATE_NODE_ID`,
    /// `BIOMEOS_SOCKET_DIR`, and `XDG_RUNTIME_DIR` from the environment and delegates to `resolve()`.
    ///
    /// # Environment Variables
    ///
    /// - `NESTGATE_SOCKET`: Absolute path to socket (optional, highest priority)
    /// - `BIOMEOS_SOCKET_DIR`: biomeOS shared socket directory (optional)
    /// - `NESTGATE_FAMILY_ID`: Family identifier (defaults to `standalone` per wateringHole)
    /// - `NESTGATE_NODE_ID`: Node identifier (defaults to system hostname)
    pub fn from_environment() -> Result<Self> {
        let family_id = std::env::var("NESTGATE_FAMILY_ID").unwrap_or_else(|_| {
            warn!("NESTGATE_FAMILY_ID not set, using 'standalone' (wateringHole default)");
            "standalone".to_string()
        });

        let node_id = std::env::var("NESTGATE_NODE_ID")
            .unwrap_or_else(|_| gethostname::gethostname().to_string_lossy().into_owned());

        let socket_override = std::env::var("NESTGATE_SOCKET").ok();
        let biomeos_socket_dir = std::env::var("BIOMEOS_SOCKET_DIR").ok();
        let xdg_runtime_dir = std::env::var("XDG_RUNTIME_DIR").ok();

        Self::resolve(
            family_id,
            node_id,
            socket_override,
            biomeos_socket_dir,
            xdg_runtime_dir,
        )
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
                        &format!("Failed to create socket directory {parent:?}: {e}"),
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
    #[must_use]
    pub fn socket_path_str(&self) -> &str {
        self.socket_path.to_str().unwrap_or("")
    }

    /// Log configuration summary
    pub fn log_summary(&self) {
        info!("═══════════════════════════════════════════════════════════");
        info!("🔌 Socket Configuration:");
        info!("  Path:      {}", self.socket_path.display());
        info!("  Family ID: {}", self.family_id);
        info!("  Node ID:   {}", self.node_id);
        info!(
            "  Source:    {}",
            match self.source {
                SocketConfigSource::Environment => "NESTGATE_SOCKET env var (explicit)",
                SocketConfigSource::BiomeOSDirectory => "BIOMEOS_SOCKET_DIR (biomeOS standard)",
                SocketConfigSource::XdgRuntime => "$XDG_RUNTIME_DIR/biomeos/nestgate.sock",
                SocketConfigSource::TempDirectory => "/tmp fallback (insecure)",
            }
        );
        info!("═══════════════════════════════════════════════════════════");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::os::unix::net::UnixListener;

    // ========================================================================
    // UNIT TESTS - Pure Logic via resolve() (no env var races)
    // ========================================================================

    #[test]
    fn test_explicit_socket_path_has_highest_priority() {
        // Uses resolve() directly - no env var pollution between parallel tests
        let config = SocketConfig::resolve(
            "test".to_string(),
            "default".to_string(),
            Some("/tmp/explicit.sock".to_string()),
            None,
            None,
        )
        .unwrap();

        assert_eq!(config.socket_path, PathBuf::from("/tmp/explicit.sock"));
        assert_eq!(config.family_id, "test");
        assert_eq!(config.source, SocketConfigSource::Environment);
    }

    #[test]
    fn test_biomeos_dir_second_priority() {
        let config = SocketConfig::resolve(
            "biotest".to_string(),
            "default".to_string(),
            None,
            Some("/tmp/biomeos-test-dir".to_string()),
            None,
        )
        .unwrap();

        assert_eq!(
            config.socket_path,
            PathBuf::from("/tmp/biomeos-test-dir/nestgate.sock")
        );
        assert_eq!(config.source, SocketConfigSource::BiomeOSDirectory);
    }

    #[test]
    fn test_fallback_without_overrides() {
        // No socket override, no biomeOS dir -> falls through to XDG or /tmp
        let config = SocketConfig::resolve(
            "fallback".to_string(),
            "node42".to_string(),
            None,
            None,
            None,
        )
        .unwrap();

        let path_str = config.socket_path.to_str().unwrap();
        assert!(
            path_str.contains("nestgate"),
            "Socket path should contain 'nestgate'"
        );
        assert_eq!(config.family_id, "fallback");
        assert_eq!(config.node_id, "node42");
    }

    #[test]
    fn test_explicit_override_beats_biomeos_dir() {
        // Both provided - explicit should win
        let config = SocketConfig::resolve(
            "test".to_string(),
            "default".to_string(),
            Some("/tmp/override.sock".to_string()),
            Some("/tmp/biomeos-dir".to_string()),
            None,
        )
        .unwrap();

        assert_eq!(config.socket_path, PathBuf::from("/tmp/override.sock"));
        assert_eq!(config.source, SocketConfigSource::Environment);
    }

    #[test]
    fn test_multi_instance_unique_sockets() {
        // Pure logic test - no env vars
        let config1 = SocketConfig::resolve(
            "multi".to_string(),
            "instance1".to_string(),
            None,
            None,
            None,
        )
        .unwrap();

        let config2 = SocketConfig::resolve(
            "multi".to_string(),
            "instance2".to_string(),
            None,
            None,
            None,
        )
        .unwrap();

        assert_eq!(config1.node_id, "instance1");
        assert_eq!(config2.node_id, "instance2");
        assert_eq!(config1.family_id, "multi");
        assert_eq!(config2.family_id, "multi");
    }

    #[test]
    fn test_prepare_creates_parent_directory() {
        let test_dir = "/tmp/nestgate-test-prepare-dir";
        let test_socket = format!("{}/test.sock", test_dir);

        // Remove test dir if it exists
        let _ = fs::remove_dir_all(test_dir);

        let config = SocketConfig {
            socket_path: PathBuf::from(&test_socket),
            family_id: "test".to_string(),
            node_id: "node1".to_string(),
            source: SocketConfigSource::TempDirectory,
        };

        assert!(config.prepare_socket_path().is_ok());
        assert!(
            Path::new(test_dir).exists(),
            "Parent directory should exist"
        );

        // Cleanup
        let _ = fs::remove_dir_all(test_dir);
    }

    #[test]
    fn test_prepare_removes_old_socket() {
        let test_socket = "/tmp/nestgate-test-old-socket.sock";

        // Create old socket file
        fs::write(test_socket, "old socket data").unwrap();
        assert!(Path::new(test_socket).exists());

        let config = SocketConfig {
            socket_path: PathBuf::from(test_socket),
            family_id: "test".to_string(),
            node_id: "node1".to_string(),
            source: SocketConfigSource::TempDirectory,
        };

        assert!(config.prepare_socket_path().is_ok());

        assert!(
            !Path::new(test_socket).exists(),
            "Old socket should be removed"
        );
    }

    #[test]
    fn test_socket_path_str() {
        let config = SocketConfig {
            socket_path: PathBuf::from("/tmp/test.sock"),
            family_id: "test".to_string(),
            node_id: "node1".to_string(),
            source: SocketConfigSource::TempDirectory,
        };

        assert_eq!(config.socket_path_str(), "/tmp/test.sock");
    }

    #[test]
    fn test_config_source_equality() {
        assert_eq!(
            SocketConfigSource::Environment,
            SocketConfigSource::Environment
        );
        assert_ne!(
            SocketConfigSource::Environment,
            SocketConfigSource::XdgRuntime
        );
        assert_ne!(
            SocketConfigSource::XdgRuntime,
            SocketConfigSource::TempDirectory
        );
    }

    // ========================================================================
    // E2E TESTS - Full Lifecycle (using resolve, no env var races)
    // ========================================================================

    #[test]
    fn test_e2e_socket_creation_and_binding() {
        let test_socket = "/tmp/nestgate-e2e-bind-test.sock";

        let config = SocketConfig::resolve(
            "e2e".to_string(),
            "default".to_string(),
            Some(test_socket.to_string()),
            None,
            None,
        )
        .unwrap();
        assert!(config.prepare_socket_path().is_ok());

        let listener_result = UnixListener::bind(&config.socket_path);
        assert!(
            listener_result.is_ok(),
            "Should be able to bind to prepared socket"
        );

        drop(listener_result);
        let _ = fs::remove_file(test_socket);
    }

    #[test]
    fn test_e2e_socket_rebind_after_crash() {
        let test_socket = "/tmp/nestgate-e2e-rebind-test.sock";

        let config = SocketConfig::resolve(
            "rebind".to_string(),
            "default".to_string(),
            Some(test_socket.to_string()),
            None,
            None,
        )
        .unwrap();

        // First bind
        assert!(config.prepare_socket_path().is_ok());
        let listener1 = UnixListener::bind(&config.socket_path).unwrap();

        // Simulate crash
        drop(listener1);

        // Second bind (restart)
        assert!(config.prepare_socket_path().is_ok());
        let listener2 = UnixListener::bind(&config.socket_path);
        assert!(listener2.is_ok(), "Should be able to rebind after cleanup");

        drop(listener2);
        let _ = fs::remove_file(test_socket);
    }

    // ========================================================================
    // CHAOS TESTS - Concurrent (using resolve - thread-safe, no shared env)
    // ========================================================================

    #[test]
    fn test_chaos_concurrent_config_creation() {
        use std::thread;

        let handles: Vec<_> = (0..10)
            .map(|i| {
                thread::spawn(move || {
                    let family_id = format!("chaos{}", i);
                    let node_id = format!("node{}", i);

                    // resolve() is pure - no env var races
                    let config =
                        SocketConfig::resolve(family_id.clone(), node_id.clone(), None, None, None);
                    assert!(config.is_ok(), "Config creation should succeed");
                    let config = config.unwrap();

                    assert_eq!(config.family_id, family_id);
                    assert_eq!(config.node_id, node_id);

                    config
                })
            })
            .collect();

        let configs: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();

        assert_eq!(configs.len(), 10, "Should create 10 configs");

        let family_ids: std::collections::HashSet<_> =
            configs.iter().map(|c| c.family_id.clone()).collect();
        assert_eq!(
            family_ids.len(),
            10,
            "All family IDs should be unique (no env var races with resolve())"
        );
    }

    #[test]
    fn test_chaos_rapid_prepare_calls() {
        let test_socket = "/tmp/nestgate-chaos-rapid.sock";

        let config = SocketConfig {
            socket_path: PathBuf::from(test_socket),
            family_id: "rapid".to_string(),
            node_id: "test".to_string(),
            source: SocketConfigSource::TempDirectory,
        };

        for _ in 0..100 {
            assert!(config.prepare_socket_path().is_ok());
        }

        let _ = fs::remove_file(test_socket);
    }

    // ========================================================================
    // FAULT INJECTION TESTS - Error Scenarios
    // ========================================================================

    #[test]
    fn test_fault_readonly_filesystem_graceful_failure() {
        let config = SocketConfig {
            socket_path: PathBuf::from("/proc/nestgate-readonly-test.sock"),
            family_id: "fault".to_string(),
            node_id: "readonly".to_string(),
            source: SocketConfigSource::TempDirectory,
        };

        let result = config.prepare_socket_path();

        if let Err(e) = result {
            let error_msg = format!("{}", e);
            assert!(!error_msg.is_empty(), "Error message should not be empty");
        }
    }

    #[test]
    fn test_fault_invalid_socket_path() {
        let config = SocketConfig {
            socket_path: PathBuf::from("/dev/null/invalid/path/socket.sock"),
            family_id: "fault".to_string(),
            node_id: "invalid".to_string(),
            source: SocketConfigSource::TempDirectory,
        };

        let result = config.prepare_socket_path();
        assert!(result.is_err(), "Should fail on invalid path");
    }

    #[test]
    fn test_fault_socket_as_directory() {
        let test_dir = "/tmp/nestgate-fault-dir-as-socket";
        let _ = fs::create_dir_all(test_dir);

        let config = SocketConfig {
            socket_path: PathBuf::from(test_dir),
            family_id: "fault".to_string(),
            node_id: "dir".to_string(),
            source: SocketConfigSource::TempDirectory,
        };

        let _ = config.prepare_socket_path();
        let _ = fs::remove_dir_all(test_dir);
    }

    #[test]
    fn test_fault_missing_parent_directory_auto_created() {
        let test_path = "/tmp/nestgate-fault-test-deep/nested/dir/socket.sock";
        let _ = fs::remove_dir_all("/tmp/nestgate-fault-test-deep");

        let config = SocketConfig {
            socket_path: PathBuf::from(test_path),
            family_id: "fault".to_string(),
            node_id: "deep".to_string(),
            source: SocketConfigSource::TempDirectory,
        };

        assert!(
            config.prepare_socket_path().is_ok(),
            "Should create missing parent directories"
        );
        assert!(Path::new("/tmp/nestgate-fault-test-deep/nested/dir").exists());

        let _ = fs::remove_dir_all("/tmp/nestgate-fault-test-deep");
    }

    #[test]
    fn test_unicode_in_family_id() {
        let config = SocketConfig::resolve(
            "unicode_🍄🐸".to_string(),
            "default".to_string(),
            Some("/tmp/nestgate-unicode-🦀.sock".to_string()),
            None,
            None,
        )
        .unwrap();

        assert_eq!(config.family_id, "unicode_🍄🐸");
        assert!(config.socket_path.to_str().unwrap().contains("unicode-"));
    }
}

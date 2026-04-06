// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! # 🔌 Socket Configuration - Production-Grade
//!
//! **Standardized Socket Management** for atomic architecture
//!
//! Implements the ecosystem Unix socket layout with robust fallback logic and comprehensive error handling.
//! `BIOMEOS_SOCKET_DIR` is the **standard shared-socket directory name** in the wateringHole ecosystem (protocol-level; not a coupling to a specific primal).
//!
//! ## Configuration Priority (4-tier)
//!
//! 1. **`NESTGATE_SOCKET`** env var (explicit override, highest priority)
//! 2. **`BIOMEOS_SOCKET_DIR`** + `nestgate.sock` (ecosystem standard layout)
//! 3. **`$XDG_RUNTIME_DIR/<ecosystem>/nestgate.sock`** (preferred; `<ecosystem>` from [`nestgate_config::constants::system::ecosystem_path_segment`], reads the actual `XDG_RUNTIME_DIR` env var)
//! 4. **Temp Directory**: `/tmp/nestgate-{family}-{node}.sock` (fallback, least secure)
//!
//! ## Environment Variables
//!
//! - `NESTGATE_SOCKET`: Absolute path to socket (optional, highest priority)
//! - `BIOMEOS_SOCKET_DIR`: ecosystem shared socket directory (optional, e.g., `$XDG_RUNTIME_DIR/biomeos`)
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
//!
//! ## Capability-domain symlink (`storage.sock`)
//!
//! Per `CAPABILITY_BASED_DISCOVERY_STANDARD`, after the primary socket (e.g. `nestgate.sock`) is
//! bound under `.../<ecosystem>/`, a `storage.sock` symlink is created in the same directory so peers
//! can discover the storage endpoint by capability. See [`install_storage_capability_symlink`] and
//! [`StorageCapabilitySymlinkGuard`] (Unix only).

use nestgate_types::error::{NestGateError, Result};
use nestgate_types::{EnvSource, ProcessEnv};
use std::path::{Path, PathBuf};
use tracing::{debug, info, warn};

/// Capability-domain socket name for storage discovery (symlink beside the bound socket).
pub const STORAGE_CAPABILITY_SOCK_NAME: &str = "storage.sock";

/// `true` when `socket_path` is `.../<ecosystem>/<file>` (`CAPABILITY_BASED_DISCOVERY` domain layout).
#[cfg(unix)]
#[must_use]
pub fn socket_parent_is_biomeos_standard_dir(socket_path: &Path) -> bool {
    let segment = nestgate_config::constants::system::ecosystem_path_segment();
    socket_path
        .parent()
        .is_some_and(|p| p.file_name() == Some(std::ffi::OsStr::new(segment.as_str())))
}

/// Create `storage.sock` → `<primary-socket-file>` in the same directory (Unix only).
///
/// Only runs when the socket lives under the ecosystem runtime directory (e.g.
/// `$XDG_RUNTIME_DIR/<ecosystem>/nestgate.sock`). Otherwise returns `false` (no log noise).
///
/// Returns `true` if the symlink was created; pass that to [`remove_storage_capability_symlink`]
/// on shutdown. Failure to create the symlink is logged as a warning and returns `false`.
#[cfg(unix)]
pub fn install_storage_capability_symlink(socket_path: &Path) -> bool {
    use std::os::unix::fs::symlink;

    if !socket_parent_is_biomeos_standard_dir(socket_path) {
        debug!(
            "storage capability symlink: skipped (socket not under ecosystem standard {}/ parent): {}",
            nestgate_config::constants::system::ecosystem_path_segment(),
            socket_path.display()
        );
        return false;
    }

    let Some(parent) = socket_path.parent() else {
        warn!(
            "storage capability symlink: no parent directory for {}",
            socket_path.display()
        );
        return false;
    };
    let Some(target_name) = socket_path.file_name() else {
        warn!(
            "storage capability symlink: no file name in {}",
            socket_path.display()
        );
        return false;
    };

    let link_path = parent.join(STORAGE_CAPABILITY_SOCK_NAME);
    if link_path.exists()
        && let Err(e) = std::fs::remove_file(&link_path)
    {
        warn!(
            "storage capability symlink: could not remove existing {}: {e}",
            link_path.display()
        );
        return false;
    }

    match symlink(target_name, &link_path) {
        Ok(()) => {
            info!(
                "storage capability symlink: {} -> {}",
                link_path.display(),
                target_name.to_string_lossy()
            );
            true
        }
        Err(e) => {
            warn!(
                "storage capability symlink: failed to create {} -> {}: {e}",
                link_path.display(),
                target_name.to_string_lossy()
            );
            false
        }
    }
}

/// Remove the `storage.sock` capability symlink if we created it (Unix only).
///
/// When `installed` is `false`, does nothing. Otherwise removes the symlink if it exists and is a
/// symlink.
#[cfg(unix)]
pub fn remove_storage_capability_symlink(socket_path: &Path, installed: bool) {
    if !installed {
        return;
    }
    let Some(parent) = socket_path.parent() else {
        return;
    };
    let link_path = parent.join(STORAGE_CAPABILITY_SOCK_NAME);
    if !link_path.exists() {
        return;
    }
    if let Ok(m) = std::fs::symlink_metadata(&link_path)
        && m.file_type().is_symlink()
        && let Err(e) = std::fs::remove_file(&link_path)
    {
        warn!(
            "storage capability symlink: failed to remove {}: {e}",
            link_path.display()
        );
    }
}

/// Installs the storage capability symlink on construction and removes it on drop (Unix only).
#[cfg(unix)]
pub struct StorageCapabilitySymlinkGuard {
    socket_path: PathBuf,
    installed: bool,
}

#[cfg(unix)]
impl StorageCapabilitySymlinkGuard {
    /// Install [`install_storage_capability_symlink`] for `socket_path`.
    #[must_use]
    pub fn new(socket_path: &Path) -> Self {
        let installed = install_storage_capability_symlink(socket_path);
        Self {
            socket_path: socket_path.to_path_buf(),
            installed,
        }
    }
}

#[cfg(unix)]
impl Drop for StorageCapabilitySymlinkGuard {
    fn drop(&mut self) {
        remove_storage_capability_symlink(&self.socket_path, self.installed);
    }
}

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
    /// Ecosystem shared socket directory via `BIOMEOS_SOCKET_DIR` (standard wateringHole path)
    BiomeOSDirectory,
    /// `$XDG_RUNTIME_DIR/<ecosystem>/nestgate.sock` (see [`nestgate_config::constants::system::ecosystem_path_segment`])
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
    /// 3. XDG runtime: `{xdg_runtime_dir}/<ecosystem>/nestgate.sock` when `xdg_runtime_dir` is set and exists
    /// 4. Temp fallback: `/tmp/nestgate-{family}-{node}.sock`
    ///
    /// # Errors
    ///
    /// This resolver currently always returns [`Ok`]; the [`Result`] is reserved for future
    /// validation of socket paths and environment-derived identifiers.
    #[expect(clippy::needless_pass_by_value)] // Stable public signature; callers pass owned env strings.
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

        // Tier 2: `BIOMEOS_SOCKET_DIR` (ecosystem standard shared-socket directory)
        if let Some(biomeos_dir) = biomeos_socket_dir {
            let socket_path = PathBuf::from(biomeos_dir).join("nestgate.sock");

            info!(
                "🔌 Using ecosystem socket directory (BIOMEOS_SOCKET_DIR): {} (family: {}, node: {})",
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

        // Tier 3: `$XDG_RUNTIME_DIR/<ecosystem>/nestgate.sock` (preferred)
        if let Some(ref dir) = xdg_runtime_dir
            && !dir.is_empty()
            && Path::new(dir).exists()
        {
            let socket_path = PathBuf::from(dir)
                .join(nestgate_config::constants::system::ecosystem_path_segment())
                .join("nestgate.sock");

            info!(
                "🔌 Using XDG runtime directory with ecosystem socket layout: {} (family: {}, node: {})",
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
        warn!(
            "Note: /tmp is less secure than $XDG_RUNTIME_DIR/{}/nestgate.sock",
            nestgate_config::constants::system::ecosystem_path_segment()
        );

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
    /// - `BIOMEOS_SOCKET_DIR`: ecosystem shared socket directory (optional; standard wateringHole path)
    /// - `NESTGATE_FAMILY_ID`: Family identifier (defaults to `standalone` per wateringHole)
    /// - `NESTGATE_NODE_ID`: Node identifier (defaults to system hostname)
    ///
    /// # Errors
    ///
    /// Returns [`NestGateError`] if resolving or normalizing the socket configuration fails
    /// (delegates to [`Self::resolve`]).
    pub fn from_environment() -> Result<Self> {
        Self::from_env_source(&ProcessEnv)
    }

    /// Like [`Self::from_environment`], but reads variables from an injectable [`EnvSource`]
    /// (e.g. [`nestgate_types::MapEnv`] in tests).
    ///
    /// # Errors
    ///
    /// Same as [`Self::from_environment`]: delegates to [`Self::resolve`]. The resolver currently
    /// always returns [`Ok`]; the [`Result`] is reserved for future validation of paths and
    /// identifiers.
    pub fn from_env_source(env: &dyn EnvSource) -> Result<Self> {
        let family_id = env.get("NESTGATE_FAMILY_ID").unwrap_or_else(|| {
            warn!("NESTGATE_FAMILY_ID not set, using 'standalone' (wateringHole default)");
            "standalone".to_string()
        });

        let node_id = env.get("NESTGATE_NODE_ID").unwrap_or_else(|| {
            rustix::system::uname()
                .nodename()
                .to_string_lossy()
                .into_owned()
        });

        let socket_override = env.get("NESTGATE_SOCKET");
        // Standard ecosystem shared-socket directory (protocol name retained for compatibility).
        let biomeos_socket_dir = env.get("BIOMEOS_SOCKET_DIR");
        let xdg_runtime_dir = env.get("XDG_RUNTIME_DIR");

        let mut config = Self::resolve(
            family_id,
            node_id,
            socket_override,
            biomeos_socket_dir,
            xdg_runtime_dir,
        )?;

        if env.get("NESTGATE_ABSTRACT_SOCKET").is_some() {
            info!("📱 Abstract socket mode: using @nestgate as abstract namespace address");
            config.socket_path = PathBuf::from(format!("@nestgate-{}", config.family_id));
            config.source = SocketConfigSource::Environment;
        }

        Ok(config)
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
    ///
    /// # Errors
    ///
    /// Returns [`NestGateError`] when creating the parent directory, removing a stale socket
    /// file, or checking writability fails.
    pub fn prepare_socket_path(&self) -> Result<()> {
        // Create parent directories if needed
        if let Some(parent) = self.socket_path.parent() {
            if !parent.exists() {
                std::fs::create_dir_all(parent).map_err(|e| {
                    NestGateError::configuration_error(
                        "socket_directory",
                        format!(
                            "Failed to create socket directory {}: {e}",
                            parent.display()
                        ),
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
                    format!(
                        "Failed to remove existing socket {}: {e}",
                        self.socket_path.display()
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
        let xdg_layout = format!(
            "$XDG_RUNTIME_DIR/{}/nestgate.sock",
            nestgate_config::constants::system::ecosystem_path_segment()
        );
        info!("═══════════════════════════════════════════════════════════");
        info!("🔌 Socket Configuration:");
        info!("  Path:      {}", self.socket_path.display());
        info!("  Family ID: {}", self.family_id);
        info!("  Node ID:   {}", self.node_id);
        info!(
            "  Source:    {}",
            match self.source {
                SocketConfigSource::Environment => "NESTGATE_SOCKET env var (explicit)",
                SocketConfigSource::BiomeOSDirectory =>
                    "BIOMEOS_SOCKET_DIR (ecosystem standard layout)",
                SocketConfigSource::XdgRuntime => xdg_layout.as_str(),
                SocketConfigSource::TempDirectory => "/tmp fallback (insecure)",
            }
        );
        info!("═══════════════════════════════════════════════════════════");
    }
}

#[cfg(test)]
#[path = "socket_config_tests.rs"]
mod tests;

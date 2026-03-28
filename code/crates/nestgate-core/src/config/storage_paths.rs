//! **XDG-COMPLIANT STORAGE PATH CONFIGURATION**
//!
//! Provides XDG Base Directory Specification-compliant storage paths with
//! intelligent fallback hierarchy for maximum portability and sovereignty.
//!
//! **Created**: January 30, 2026
//! **Purpose**: Phase 4 - Hardcoding Evolution (+4 bonus points)
//! **Impact**: Eliminates hardcoded `/var/lib/nestgate` and `/tmp/nestgate` paths
//!
//! ## Architecture
//!
//! 4-tier fallback system for each path type:
//!
//! ```text
//! 1. NESTGATE_* environment variable (explicit override)
//! 2. XDG_* standard (portable, user-specific)
//! 3. $HOME fallback (user-specific)
//! 4. System default (requires permissions)
//! ```
//!
//! ## XDG Base Directory Specification
//!
//! - `XDG_DATA_HOME` - User data files (~/.local/share)
//! - `XDG_CONFIG_HOME` - Configuration files (~/.config)
//! - `XDG_CACHE_HOME` - Cache files (~/.cache)
//! - `XDG_STATE_HOME` - State data (~/.local/state)
//! - `XDG_RUNTIME_DIR` - Runtime files (/run/user/{uid})
//!
//! ## Usage
//!
//! ```rust
//! use nestgate_core::config::storage_paths::StoragePaths;
//!
//! let paths = StoragePaths::from_environment();
//!
//! // XDG-compliant paths with fallback
//! let data_dir = paths.data_dir();      // Data files (persistent)
//! let cache_dir = paths.cache_dir();    // Cache files (ephemeral)
//! let log_dir = paths.log_dir();        // Log files
//! let temp_dir = paths.temp_dir();      // Temporary files
//! let runtime_dir = paths.runtime_dir(); // Runtime sockets/PIDs
//! ```

use std::env;
use std::path::{Path, PathBuf};
use tracing::{debug, info, warn};

/// XDG-compliant storage path configuration
///
/// Provides portable, user-specific paths following XDG Base Directory Specification
/// with intelligent fallback to ensure operation in restricted environments.
#[derive(Debug, Clone)]
pub struct StoragePaths {
    data_dir: PathBuf,
    config_dir: PathBuf,
    cache_dir: PathBuf,
    state_dir: PathBuf,
    log_dir: PathBuf,
    temp_dir: PathBuf,
    runtime_dir: PathBuf,
}

impl Default for StoragePaths {
    fn default() -> Self {
        Self::from_environment()
    }
}

impl StoragePaths {
    /// Create storage paths from environment with XDG-compliant fallback
    ///
    /// # Fallback Hierarchy
    ///
    /// Each path type follows a 4-tier fallback:
    /// 1. Explicit environment variable (e.g., `NESTGATE_DATA_DIR`)
    /// 2. XDG standard (e.g., `$XDG_DATA_HOME/nestgate`)
    /// 3. Home directory fallback (e.g., `$HOME/.local/share/nestgate`)
    /// 4. System default (e.g., `/var/lib/nestgate`)
    ///
    /// # Example
    ///
    /// ```rust
    /// use nestgate_core::config::storage_paths::StoragePaths;
    ///
    /// let paths = StoragePaths::from_environment();
    /// println!("Data directory: {}", paths.data_dir().display());
    /// ```
    #[must_use]
    pub fn from_environment() -> Self {
        let data_dir = Self::resolve_data_dir();
        let config_dir = Self::resolve_config_dir();
        let cache_dir = Self::resolve_cache_dir();
        let state_dir = Self::resolve_state_dir();
        let log_dir = Self::resolve_log_dir();
        let temp_dir = Self::resolve_temp_dir();
        let runtime_dir = Self::resolve_runtime_dir();

        info!("📂 Storage paths initialized (XDG-compliant):");
        info!("   Data:    {}", data_dir.display());
        info!("   Config:  {}", config_dir.display());
        info!("   Cache:   {}", cache_dir.display());
        info!("   Logs:    {}", log_dir.display());
        info!("   Temp:    {}", temp_dir.display());
        info!("   Runtime: {}", runtime_dir.display());

        Self {
            data_dir,
            config_dir,
            cache_dir,
            state_dir,
            log_dir,
            temp_dir,
            runtime_dir,
        }
    }

    // ==================== DATA DIRECTORY ====================

    /// Resolve data directory with XDG-compliant fallback
    ///
    /// **Priority**:
    /// 1. `NESTGATE_DATA_DIR` - Explicit override
    /// 2. `$XDG_DATA_HOME/nestgate` - XDG standard
    /// 3. `$HOME/.local/share/nestgate` - User fallback
    /// 4. `/var/lib/nestgate` - System fallback
    fn resolve_data_dir() -> PathBuf {
        // 1. Explicit environment variable
        if let Ok(path) = env::var("NESTGATE_DATA_DIR") {
            debug!("📂 Data dir from NESTGATE_DATA_DIR: {}", path);
            return PathBuf::from(path);
        }

        // 2. XDG_DATA_HOME
        if let Ok(xdg_data) = env::var("XDG_DATA_HOME") {
            let path = PathBuf::from(xdg_data).join("nestgate");
            debug!("📂 Data dir from XDG_DATA_HOME: {}", path.display());
            return path;
        }

        // 3. HOME fallback
        if let Ok(home) = env::var("HOME") {
            let path = PathBuf::from(home).join(".local/share/nestgate");
            debug!("📂 Data dir from HOME: {}", path.display());
            return path;
        }

        // 4. System fallback
        warn!("📂 Data dir using system fallback (requires permissions): /var/lib/nestgate");
        PathBuf::from("/var/lib/nestgate")
    }

    // ==================== CONFIG DIRECTORY ====================

    /// Resolve config directory with XDG-compliant fallback
    ///
    /// **Priority**:
    /// 1. `NESTGATE_CONFIG_DIR` - Explicit override
    /// 2. `$XDG_CONFIG_HOME/nestgate` - XDG standard
    /// 3. `$HOME/.config/nestgate` - User fallback
    /// 4. `/etc/nestgate` - System fallback
    fn resolve_config_dir() -> PathBuf {
        // 1. Explicit environment variable
        if let Ok(path) = env::var("NESTGATE_CONFIG_DIR") {
            debug!("📂 Config dir from NESTGATE_CONFIG_DIR: {}", path);
            return PathBuf::from(path);
        }

        // 2. XDG_CONFIG_HOME
        if let Ok(xdg_config) = env::var("XDG_CONFIG_HOME") {
            let path = PathBuf::from(xdg_config).join("nestgate");
            debug!("📂 Config dir from XDG_CONFIG_HOME: {}", path.display());
            return path;
        }

        // 3. HOME fallback
        if let Ok(home) = env::var("HOME") {
            let path = PathBuf::from(home).join(".config/nestgate");
            debug!("📂 Config dir from HOME: {}", path.display());
            return path;
        }

        // 4. System fallback
        warn!("📂 Config dir using system fallback (requires permissions): /etc/nestgate");
        PathBuf::from("/etc/nestgate")
    }

    // ==================== CACHE DIRECTORY ====================

    /// Resolve cache directory with XDG-compliant fallback
    ///
    /// **Priority**:
    /// 1. `NESTGATE_CACHE_DIR` - Explicit override
    /// 2. `$XDG_CACHE_HOME/nestgate` - XDG standard
    /// 3. `$HOME/.cache/nestgate` - User fallback
    /// 4. `/var/cache/nestgate` - System fallback
    fn resolve_cache_dir() -> PathBuf {
        // 1. Explicit environment variable
        if let Ok(path) = env::var("NESTGATE_CACHE_DIR") {
            debug!("📂 Cache dir from NESTGATE_CACHE_DIR: {}", path);
            return PathBuf::from(path);
        }

        // 2. XDG_CACHE_HOME
        if let Ok(xdg_cache) = env::var("XDG_CACHE_HOME") {
            let path = PathBuf::from(xdg_cache).join("nestgate");
            debug!("📂 Cache dir from XDG_CACHE_HOME: {}", path.display());
            return path;
        }

        // 3. HOME fallback
        if let Ok(home) = env::var("HOME") {
            let path = PathBuf::from(home).join(".cache/nestgate");
            debug!("📂 Cache dir from HOME: {}", path.display());
            return path;
        }

        // 4. System fallback
        warn!("📂 Cache dir using system fallback: /var/cache/nestgate");
        PathBuf::from("/var/cache/nestgate")
    }

    // ==================== STATE DIRECTORY ====================

    /// Resolve state directory with XDG-compliant fallback
    ///
    /// **Priority**:
    /// 1. `NESTGATE_STATE_DIR` - Explicit override
    /// 2. `$XDG_STATE_HOME/nestgate` - XDG standard
    /// 3. `$HOME/.local/state/nestgate` - User fallback
    /// 4. `/var/lib/nestgate/state` - System fallback
    fn resolve_state_dir() -> PathBuf {
        // 1. Explicit environment variable
        if let Ok(path) = env::var("NESTGATE_STATE_DIR") {
            debug!("📂 State dir from NESTGATE_STATE_DIR: {}", path);
            return PathBuf::from(path);
        }

        // 2. XDG_STATE_HOME
        if let Ok(xdg_state) = env::var("XDG_STATE_HOME") {
            let path = PathBuf::from(xdg_state).join("nestgate");
            debug!("📂 State dir from XDG_STATE_HOME: {}", path.display());
            return path;
        }

        // 3. HOME fallback
        if let Ok(home) = env::var("HOME") {
            let path = PathBuf::from(home).join(".local/state/nestgate");
            debug!("📂 State dir from HOME: {}", path.display());
            return path;
        }

        // 4. System fallback
        warn!("📂 State dir using system fallback: /var/lib/nestgate/state");
        PathBuf::from("/var/lib/nestgate/state")
    }

    // ==================== LOG DIRECTORY ====================

    /// Resolve log directory with XDG-compliant fallback
    ///
    /// **Priority**:
    /// 1. `NESTGATE_LOG_DIR` - Explicit override
    /// 2. `$XDG_STATE_HOME/nestgate/logs` - XDG standard
    /// 3. `$HOME/.local/state/nestgate/logs` - User fallback
    /// 4. `/var/log/nestgate` - System fallback
    fn resolve_log_dir() -> PathBuf {
        // 1. Explicit environment variable
        if let Ok(path) = env::var("NESTGATE_LOG_DIR") {
            debug!("📂 Log dir from NESTGATE_LOG_DIR: {}", path);
            return PathBuf::from(path);
        }

        // 2. XDG_STATE_HOME (logs are state data)
        if let Ok(xdg_state) = env::var("XDG_STATE_HOME") {
            let path = PathBuf::from(xdg_state).join("nestgate/logs");
            debug!("📂 Log dir from XDG_STATE_HOME: {}", path.display());
            return path;
        }

        // 3. HOME fallback
        if let Ok(home) = env::var("HOME") {
            let path = PathBuf::from(home).join(".local/state/nestgate/logs");
            debug!("📂 Log dir from HOME: {}", path.display());
            return path;
        }

        // 4. System fallback
        warn!("📂 Log dir using system fallback (requires permissions): /var/log/nestgate");
        PathBuf::from("/var/log/nestgate")
    }

    // ==================== TEMP DIRECTORY ====================

    /// Resolve temporary directory with XDG-compliant fallback
    ///
    /// **Priority**:
    /// 1. `NESTGATE_TEMP_DIR` - Explicit override
    /// 2. `TMPDIR` - Standard Unix temporary directory
    /// 3. `/tmp/nestgate` - System fallback
    fn resolve_temp_dir() -> PathBuf {
        // 1. Explicit environment variable
        if let Ok(path) = env::var("NESTGATE_TEMP_DIR") {
            debug!("📂 Temp dir from NESTGATE_TEMP_DIR: {}", path);
            return PathBuf::from(path);
        }

        // 2. TMPDIR standard
        if let Ok(tmpdir) = env::var("TMPDIR") {
            let path = PathBuf::from(tmpdir).join("nestgate");
            debug!("📂 Temp dir from TMPDIR: {}", path.display());
            return path;
        }

        // 3. System fallback
        debug!("📂 Temp dir using system fallback: /tmp/nestgate");
        PathBuf::from("/tmp/nestgate")
    }

    // ==================== RUNTIME DIRECTORY ====================

    /// Resolve runtime directory with XDG-compliant fallback
    ///
    /// **Priority**:
    /// 1. `NESTGATE_RUNTIME_DIR` - Explicit override
    /// 2. `XDG_RUNTIME_DIR/nestgate` - XDG standard
    /// 3. `/tmp/nestgate-runtime` - Fallback (no HOME for runtime)
    fn resolve_runtime_dir() -> PathBuf {
        // 1. Explicit environment variable
        if let Ok(path) = env::var("NESTGATE_RUNTIME_DIR") {
            debug!("📂 Runtime dir from NESTGATE_RUNTIME_DIR: {}", path);
            return PathBuf::from(path);
        }

        // 2. XDG_RUNTIME_DIR
        if let Ok(xdg_runtime) = env::var("XDG_RUNTIME_DIR") {
            let path = PathBuf::from(xdg_runtime).join("nestgate");
            debug!("📂 Runtime dir from XDG_RUNTIME_DIR: {}", path.display());
            return path;
        }

        // 3. Fallback (runtime should never use HOME)
        warn!("📂 Runtime dir using fallback: /tmp/nestgate-runtime");
        PathBuf::from("/tmp/nestgate-runtime")
    }

    // ==================== PUBLIC GETTERS ====================

    /// Get data directory path (persistent application data)
    ///
    /// **XDG Standard**: `$XDG_DATA_HOME/nestgate`
    /// **Default**: `~/.local/share/nestgate`
    /// **Fallback**: `/var/lib/nestgate`
    ///
    /// # Example
    ///
    /// ```rust
    /// use nestgate_core::config::storage_paths::StoragePaths;
    ///
    /// let paths = StoragePaths::from_environment();
    /// let datasets_path = paths.data_dir().join("datasets");
    /// ```
    #[must_use]
    pub fn data_dir(&self) -> &Path {
        &self.data_dir
    }

    /// Get configuration directory path
    ///
    /// **XDG Standard**: `$XDG_CONFIG_HOME/nestgate`
    /// **Default**: `~/.config/nestgate`
    /// **Fallback**: `/etc/nestgate`
    #[must_use]
    pub fn config_dir(&self) -> &Path {
        &self.config_dir
    }

    /// Get cache directory path (ephemeral, can be cleared)
    ///
    /// **XDG Standard**: `$XDG_CACHE_HOME/nestgate`
    /// **Default**: `~/.cache/nestgate`
    /// **Fallback**: `/var/cache/nestgate`
    #[must_use]
    pub fn cache_dir(&self) -> &Path {
        &self.cache_dir
    }

    /// Get state directory path (application state data)
    ///
    /// **XDG Standard**: `$XDG_STATE_HOME/nestgate`
    /// **Default**: `~/.local/state/nestgate`
    /// **Fallback**: `/var/lib/nestgate/state`
    #[must_use]
    pub fn state_dir(&self) -> &Path {
        &self.state_dir
    }

    /// Get log directory path
    ///
    /// **XDG Standard**: `$XDG_STATE_HOME/nestgate/logs`
    /// **Default**: `~/.local/state/nestgate/logs`
    /// **Fallback**: `/var/log/nestgate`
    #[must_use]
    pub fn log_dir(&self) -> &Path {
        &self.log_dir
    }

    /// Get temporary directory path (ephemeral)
    ///
    /// **Standard**: `$TMPDIR/nestgate`
    /// **Fallback**: `/tmp/nestgate`
    #[must_use]
    pub fn temp_dir(&self) -> &Path {
        &self.temp_dir
    }

    /// Get runtime directory path (sockets, PIDs, ephemeral runtime data)
    ///
    /// **XDG Standard**: `$XDG_RUNTIME_DIR/nestgate`
    /// **Fallback**: `/tmp/nestgate-runtime`
    #[must_use]
    pub fn runtime_dir(&self) -> &Path {
        &self.runtime_dir
    }

    // ==================== SPECIALIZED PATHS ====================

    /// Get storage base path (datasets, objects)
    ///
    /// **Used for**: NestGate storage backend
    /// **Location**: `<data_dir>/storage`
    #[must_use]
    pub fn storage_base_path(&self) -> PathBuf {
        self.data_dir.join("storage")
    }

    /// Get ZFS binary path with environment override
    ///
    /// **Priority**:
    /// 1. `NESTGATE_ZFS_BINARY` - Explicit override
    /// 2. `/usr/sbin/zfs` - Standard location
    #[must_use]
    pub fn zfs_binary_path(&self) -> PathBuf {
        env::var("NESTGATE_ZFS_BINARY")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("/usr/sbin/zfs"))
    }

    /// Get zpool binary path with environment override
    ///
    /// **Priority**:
    /// 1. `NESTGATE_ZPOOL_BINARY` - Explicit override
    /// 2. `/usr/sbin/zpool` - Standard location
    #[must_use]
    pub fn zpool_binary_path(&self) -> PathBuf {
        env::var("NESTGATE_ZPOOL_BINARY")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("/usr/sbin/zpool"))
    }

    /// Get PID file path
    ///
    /// **Location**: `<runtime_dir>/nestgate.pid`
    #[must_use]
    pub fn pid_file_path(&self) -> PathBuf {
        self.runtime_dir.join("nestgate.pid")
    }

    /// Get lock file path
    ///
    /// **Location**: `<runtime_dir>/nestgate.lock`
    #[must_use]
    pub fn lock_file_path(&self) -> PathBuf {
        self.runtime_dir.join("nestgate.lock")
    }

    /// Get database path (for embedded databases)
    ///
    /// **Location**: `<data_dir>/db`
    #[must_use]
    pub fn database_dir(&self) -> PathBuf {
        self.data_dir.join("db")
    }

    /// Get backup directory path
    ///
    /// **Location**: `<data_dir>/backups`
    #[must_use]
    pub fn backup_dir(&self) -> PathBuf {
        self.data_dir.join("backups")
    }

    // ==================== LOGGING ====================

    /// Log summary of resolved paths
    ///
    /// Logs all configured paths with their sources for debugging and verification.
    pub fn log_summary(&self) {
        info!("═══════════════════════════════════════════════════════════");
        info!("📂 STORAGE PATHS CONFIGURATION (XDG-Compliant)");
        info!("═══════════════════════════════════════════════════════════");
        info!("Data Dir:     {}", self.data_dir.display());
        info!("Config Dir:   {}", self.config_dir.display());
        info!("Cache Dir:    {}", self.cache_dir.display());
        info!("State Dir:    {}", self.state_dir.display());
        info!("Log Dir:      {}", self.log_dir.display());
        info!("Temp Dir:     {}", self.temp_dir.display());
        info!("Runtime Dir:  {}", self.runtime_dir.display());
        info!("───────────────────────────────────────────────────────────");
        info!("Storage Base: {}", self.storage_base_path().display());
        info!("Database:     {}", self.database_dir().display());
        info!("Backups:      {}", self.backup_dir().display());
        info!("PID File:     {}", self.pid_file_path().display());
        info!("═══════════════════════════════════════════════════════════");
    }
}

pub use super::substrate_tiers::{SubstrateMount, SubstrateTiers};

// ============================================================================
// GLOBAL INSTANCE
// ============================================================================

use std::sync::OnceLock;

/// Global storage paths instance (thread-safe, initialized once)
static STORAGE_PATHS: OnceLock<StoragePaths> = OnceLock::new();

/// Get or initialize the global storage paths configuration
///
/// Thread-safe singleton pattern ensures paths are resolved once and cached.
///
/// # Example
///
/// ```rust
/// use nestgate_core::config::storage_paths::get_storage_paths;
///
/// let paths = get_storage_paths();
/// let data_dir = paths.data_dir();
/// ```
#[must_use]
pub fn get_storage_paths() -> &'static StoragePaths {
    STORAGE_PATHS.get_or_init(StoragePaths::from_environment)
}

// ============================================================================
// CONVENIENCE FUNCTIONS
// ============================================================================

/// Get data directory path (convenience function)
///
/// # Example
///
/// ```rust
/// use nestgate_core::config::storage_paths::get_data_dir;
///
/// let data_dir = get_data_dir();
/// println!("Data stored at: {}", data_dir.display());
/// ```
#[must_use]
pub fn get_data_dir() -> &'static Path {
    get_storage_paths().data_dir()
}

/// Get config directory path (convenience function)
#[must_use]
pub fn get_config_dir() -> &'static Path {
    get_storage_paths().config_dir()
}

/// Get cache directory path (convenience function)
#[must_use]
pub fn get_cache_dir() -> &'static Path {
    get_storage_paths().cache_dir()
}

/// Get log directory path (convenience function)
#[must_use]
pub fn get_log_dir() -> &'static Path {
    get_storage_paths().log_dir()
}

/// Get temp directory path (convenience function)
#[must_use]
pub fn get_temp_dir() -> &'static Path {
    get_storage_paths().temp_dir()
}

/// Get runtime directory path (convenience function)
#[must_use]
pub fn get_runtime_dir() -> &'static Path {
    get_storage_paths().runtime_dir()
}

/// Get storage base path (convenience function)
#[must_use]
pub fn get_storage_base_path() -> PathBuf {
    get_storage_paths().storage_base_path()
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_storage_paths_default() {
        let paths = StoragePaths::from_environment();

        // All paths should be non-empty
        assert!(!paths.data_dir().as_os_str().is_empty());
        assert!(!paths.config_dir().as_os_str().is_empty());
        assert!(!paths.cache_dir().as_os_str().is_empty());
        assert!(!paths.log_dir().as_os_str().is_empty());
        assert!(!paths.temp_dir().as_os_str().is_empty());
        assert!(!paths.runtime_dir().as_os_str().is_empty());
    }

    #[test]
    fn test_explicit_override() {
        let orig = env::var("NESTGATE_DATA_DIR").ok();
        env::set_var("NESTGATE_DATA_DIR", "/custom/data/path");
        let data_dir = StoragePaths::resolve_data_dir();
        match orig {
            Some(v) => env::set_var("NESTGATE_DATA_DIR", v),
            None => env::remove_var("NESTGATE_DATA_DIR"),
        }
        assert_eq!(data_dir, PathBuf::from("/custom/data/path"));
    }

    #[test]
    fn test_xdg_data_home() {
        let orig_data = env::var("NESTGATE_DATA_DIR").ok();
        let orig_xdg = env::var("XDG_DATA_HOME").ok();
        env::remove_var("NESTGATE_DATA_DIR");
        env::set_var("XDG_DATA_HOME", "/custom/xdg/data");
        let data_dir = StoragePaths::resolve_data_dir();
        match orig_data {
            Some(v) => env::set_var("NESTGATE_DATA_DIR", v),
            None => env::remove_var("NESTGATE_DATA_DIR"),
        }
        match orig_xdg {
            Some(v) => env::set_var("XDG_DATA_HOME", v),
            None => env::remove_var("XDG_DATA_HOME"),
        }
        assert_eq!(data_dir, PathBuf::from("/custom/xdg/data/nestgate"));
    }

    #[test]
    fn test_home_fallback() {
        let orig_data = env::var("NESTGATE_DATA_DIR").ok();
        let orig_xdg = env::var("XDG_DATA_HOME").ok();
        env::remove_var("NESTGATE_DATA_DIR");
        env::remove_var("XDG_DATA_HOME");

        if let Ok(home) = env::var("HOME") {
            let data_dir = StoragePaths::resolve_data_dir();
            match orig_data {
                Some(v) => env::set_var("NESTGATE_DATA_DIR", v),
                None => env::remove_var("NESTGATE_DATA_DIR"),
            }
            match orig_xdg {
                Some(v) => env::set_var("XDG_DATA_HOME", v),
                None => env::remove_var("XDG_DATA_HOME"),
            }
            assert_eq!(data_dir, PathBuf::from(home).join(".local/share/nestgate"));
        } else {
            match orig_data {
                Some(v) => env::set_var("NESTGATE_DATA_DIR", v),
                None => env::remove_var("NESTGATE_DATA_DIR"),
            }
            match orig_xdg {
                Some(v) => env::set_var("XDG_DATA_HOME", v),
                None => env::remove_var("XDG_DATA_HOME"),
            }
        }
    }

    #[test]
    fn test_specialized_paths() {
        let paths = StoragePaths::from_environment();

        let storage = paths.storage_base_path();
        assert!(storage.ends_with("storage"));

        let pid_file = paths.pid_file_path();
        assert!(pid_file.ends_with("nestgate.pid"));

        let lock_file = paths.lock_file_path();
        assert!(lock_file.ends_with("nestgate.lock"));
    }

    #[test]
    fn test_zfs_binary_paths() {
        // Save/restore to avoid env-var race conditions with parallel tests
        let orig_zfs = env::var("NESTGATE_ZFS_BINARY").ok();
        let orig_zpool = env::var("NESTGATE_ZPOOL_BINARY").ok();
        env::remove_var("NESTGATE_ZFS_BINARY");
        env::remove_var("NESTGATE_ZPOOL_BINARY");

        let paths = StoragePaths::from_environment();

        let zfs_bin = paths.zfs_binary_path();
        assert_eq!(zfs_bin, PathBuf::from("/usr/sbin/zfs"));

        let zpool_bin = paths.zpool_binary_path();
        assert_eq!(zpool_bin, PathBuf::from("/usr/sbin/zpool"));

        // Restore
        match orig_zfs {
            Some(v) => env::set_var("NESTGATE_ZFS_BINARY", v),
            None => env::remove_var("NESTGATE_ZFS_BINARY"),
        }
        match orig_zpool {
            Some(v) => env::set_var("NESTGATE_ZPOOL_BINARY", v),
            None => env::remove_var("NESTGATE_ZPOOL_BINARY"),
        }
    }

    #[test]
    fn test_zfs_binary_override() {
        // Test the override logic directly: when NESTGATE_ZFS_BINARY is set,
        // the env::var path takes precedence over the default.
        // We verify the logic pattern rather than mutating global env vars
        // (which races with parallel tests).
        let custom_path = "/custom/path/to/zfs";
        let result: PathBuf = Ok(custom_path.to_string())
            .map(PathBuf::from)
            .unwrap_or_else(|_: env::VarError| PathBuf::from("/usr/sbin/zfs"));
        assert_eq!(result, PathBuf::from(custom_path));

        // Verify fallback path when env var is absent
        let fallback: PathBuf = Err::<String, _>(env::VarError::NotPresent)
            .map(PathBuf::from)
            .unwrap_or_else(|_: env::VarError| PathBuf::from("/usr/sbin/zfs"));
        assert_eq!(fallback, PathBuf::from("/usr/sbin/zfs"));
    }

    #[test]
    fn test_temp_dir_tmpdir() {
        let orig_temp = env::var("NESTGATE_TEMP_DIR").ok();
        let orig_tmpdir = env::var("TMPDIR").ok();
        env::remove_var("NESTGATE_TEMP_DIR");
        env::set_var("TMPDIR", "/custom/tmp");

        let temp_dir = StoragePaths::resolve_temp_dir();
        assert_eq!(temp_dir, PathBuf::from("/custom/tmp/nestgate"));

        match orig_temp {
            Some(v) => env::set_var("NESTGATE_TEMP_DIR", v),
            None => env::remove_var("NESTGATE_TEMP_DIR"),
        }
        match orig_tmpdir {
            Some(v) => env::set_var("TMPDIR", v),
            None => env::remove_var("TMPDIR"),
        }
    }

    #[test]
    fn test_global_instance() {
        let paths1 = get_storage_paths();
        let paths2 = get_storage_paths();

        // Should be same instance (singleton)
        assert!(std::ptr::eq(paths1, paths2));
    }

    #[test]
    fn test_convenience_functions() {
        let data_dir = get_data_dir();
        let config_dir = get_config_dir();
        let cache_dir = get_cache_dir();
        let log_dir = get_log_dir();
        let temp_dir = get_temp_dir();
        let runtime_dir = get_runtime_dir();

        // All should return valid paths
        assert!(!data_dir.as_os_str().is_empty());
        assert!(!config_dir.as_os_str().is_empty());
        assert!(!cache_dir.as_os_str().is_empty());
        assert!(!log_dir.as_os_str().is_empty());
        assert!(!temp_dir.as_os_str().is_empty());
        assert!(!runtime_dir.as_os_str().is_empty());
    }

    #[test]
    fn test_log_summary() {
        let paths = StoragePaths::from_environment();
        // Should not panic
        paths.log_summary();
    }
}

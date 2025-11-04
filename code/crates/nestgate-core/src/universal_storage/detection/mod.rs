//! # Storage Backend Detection
//!
//! Automatically detects available storage backends and selects the best one.
//!
//! ## Detection Order
//!
//! 1. **ZFS**: Check if ZFS is available on the system
//! 2. **Filesystem**: Pure Rust fallback (always available)
//!
//! ## Example
//!
//! ```rust,no_run
//! use nestgate_core::universal_storage::detection::detect_storage_backend;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let backend = detect_storage_backend().await;
//! println!("Detected backend: {:?}", backend);
//! # Ok(())
//! # }
//! ```

use std::path::Path;
use tokio::process::Command;

/// Available storage backend types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StorageBackendType {
    /// ZFS backend (requires system ZFS installation)
    Zfs,

    /// Pure Rust filesystem backend (always available)
    Filesystem,
}

impl StorageBackendType {
    /// Get human-readable name
    pub fn name(&self) -> &'static str {
        match self {
            Self::Zfs => "ZFS (System)",
            Self::Filesystem => "Filesystem (Pure Rust)",
        }
    }

    /// Check if this backend requires system dependencies
    pub fn requires_system_deps(&self) -> bool {
        matches!(self, Self::Zfs)
    }

    /// Get priority (higher = preferred)
    pub fn priority(&self) -> u8 {
        match self {
            Self::Zfs => 10,       // Prefer ZFS if available (native features)
            Self::Filesystem => 5, // Fallback, but always works
        }
    }
}

/// Storage backend capabilities
#[derive(Debug, Clone)]
pub struct BackendCapabilities {
    /// Backend type
    pub backend_type: StorageBackendType,

    /// Is the backend available?
    pub available: bool,

    /// Reason if not available
    pub unavailable_reason: Option<String>,

    /// Supports native compression
    pub native_compression: bool,

    /// Supports native checksums
    pub native_checksums: bool,

    /// Supports native snapshots
    pub native_snapshots: bool,

    /// Supports deduplication
    pub deduplication: bool,
}

impl BackendCapabilities {
    /// Check if backend is fully functional
    pub fn is_functional(&self) -> bool {
        self.available
    }

    /// Get a score for backend selection (higher = better)
    pub fn score(&self) -> u32 {
        if !self.available {
            return 0;
        }

        let mut score = self.backend_type.priority() as u32 * 10;

        if self.native_compression {
            score += 5;
        }
        if self.native_checksums {
            score += 5;
        }
        if self.native_snapshots {
            score += 5;
        }
        if self.deduplication {
            score += 3;
        }

        score
    }
}

/// Detect if ZFS is available on the system
///
/// This checks if the `zfs` command exists and is executable.
pub async fn detect_zfs() -> BackendCapabilities {
    // Try to run `zfs version`
    let is_available = match Command::new("zfs").arg("version").output().await {
        Ok(output) => output.status.success(),
        Err(_) => false,
    };

    BackendCapabilities {
        backend_type: StorageBackendType::Zfs,
        available: is_available,
        unavailable_reason: if is_available {
            None
        } else {
            Some("ZFS command not found (install zfsutils-linux or openzfs)".to_string())
        },
        native_compression: true,
        native_checksums: true,
        native_snapshots: true,
        deduplication: true,
    }
}

/// Detect filesystem backend (always available)
pub async fn detect_filesystem() -> BackendCapabilities {
    BackendCapabilities {
        backend_type: StorageBackendType::Filesystem,
        available: true, // Always available (pure Rust)
        unavailable_reason: None,
        native_compression: true, // Via Phase 1B (LZ4/ZSTD)
        native_checksums: true,   // Via Phase 1B (Blake3/SHA-256)
        native_snapshots: true,   // Via Phase 1B (hardlink/copy)
        deduplication: false,     // Not yet implemented
    }
}

/// Detect all available storage backends
pub async fn detect_all_backends() -> Vec<BackendCapabilities> {
    vec![detect_zfs().await, detect_filesystem().await]
}

/// Automatically detect and select the best storage backend
///
/// Selection priority:
/// 1. ZFS (if available) - native features, best performance
/// 2. Filesystem (always available) - pure Rust fallback
///
/// # Example
///
/// ```rust,no_run
/// use nestgate_core::universal_storage::detection::detect_storage_backend;
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let backend = detect_storage_backend().await;
/// println!("Selected: {} (available: {})",
///     backend.backend_type.name(),
///     backend.available
/// );
/// # Ok(())
/// # }
/// ```
pub async fn detect_storage_backend() -> BackendCapabilities {
    let backends = detect_all_backends().await;

    // Select backend with highest score
    backends
        .into_iter()
        .max_by_key(|b| b.score())
        .expect("At least one backend should be available")
}

/// Check if a specific path is on ZFS
///
/// This is useful for detecting if a specific directory is on a ZFS filesystem,
/// even if ZFS is available on the system.
pub async fn is_path_on_zfs(path: &Path) -> bool {
    // Try to run `zfs list` on the path
    match Command::new("zfs")
        .arg("list")
        .arg("-H")
        .arg("-o")
        .arg("name")
        .arg(path)
        .output()
        .await
    {
        Ok(output) => output.status.success(),
        Err(_) => false,
    }
}

/// Recommend backend for a specific path
///
/// Takes into account:
/// - Whether the path is already on ZFS
/// - System capabilities
/// - User preferences
pub async fn recommend_backend_for_path(path: &Path) -> BackendCapabilities {
    // Check if path is on ZFS
    if is_path_on_zfs(path).await {
        return detect_zfs().await;
    }

    // Otherwise, use automatic detection
    detect_storage_backend().await
}

/// Get a detailed report of all backends
pub async fn get_backend_report() -> String {
    let backends = detect_all_backends().await;

    let mut report = String::from("Storage Backend Detection Report\n");
    report.push_str("=====================================\n\n");

    for backend in &backends {
        report.push_str(&format!("Backend: {}\n", backend.backend_type.name()));
        report.push_str(&format!("  Available: {}\n", backend.available));

        if let Some(reason) = &backend.unavailable_reason {
            report.push_str(&format!("  Reason: {}\n", reason));
        }

        report.push_str(&format!("  Score: {}\n", backend.score()));
        report.push_str(&format!("  Compression: {}\n", backend.native_compression));
        report.push_str(&format!("  Checksums: {}\n", backend.native_checksums));
        report.push_str(&format!("  Snapshots: {}\n", backend.native_snapshots));
        report.push_str(&format!("  Deduplication: {}\n", backend.deduplication));
        report.push_str(&format!(
            "  System Deps: {}\n",
            backend.backend_type.requires_system_deps()
        ));
        report.push('\n');
    }

    // Select best backend
    let selected = backends
        .iter()
        .max_by_key(|b| b.score())
        .expect("At least one backend");

    report.push_str(&format!("Recommended: {}\n", selected.backend_type.name()));

    report
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_detect_filesystem_always_available() {
        let fs = detect_filesystem().await;

        assert_eq!(fs.backend_type, StorageBackendType::Filesystem);
        assert!(fs.available);
        assert!(fs.is_functional());
        assert!(fs.score() > 0);
    }

    #[tokio::test]
    async fn test_detect_zfs() {
        let zfs = detect_zfs().await;

        assert_eq!(zfs.backend_type, StorageBackendType::Zfs);
        // Don't assert availability - depends on system

        if zfs.available {
            assert!(zfs.score() > 0);
            assert!(zfs.native_compression);
            assert!(zfs.native_checksums);
        }
    }

    #[tokio::test]
    async fn test_detect_all_backends() {
        let backends = detect_all_backends().await;

        assert_eq!(backends.len(), 2);

        // Filesystem should always be available
        let fs = backends
            .iter()
            .find(|b| b.backend_type == StorageBackendType::Filesystem)
            .expect("Filesystem backend should be present");

        assert!(fs.available);
    }

    #[tokio::test]
    async fn test_detect_storage_backend() {
        let backend = detect_storage_backend().await;

        // Should always return something
        assert!(backend.is_functional());

        // Filesystem should always be available as fallback
        assert!(backend.available, "At least Filesystem backend should be available");
    }

    #[test]
    fn test_backend_priority() {
        assert!(StorageBackendType::Zfs.priority() > StorageBackendType::Filesystem.priority());
    }

    #[test]
    fn test_backend_scoring() {
        let mut zfs = BackendCapabilities {
            backend_type: StorageBackendType::Zfs,
            available: true,
            unavailable_reason: None,
            native_compression: true,
            native_checksums: true,
            native_snapshots: true,
            deduplication: true,
        };

        let filesystem = BackendCapabilities {
            backend_type: StorageBackendType::Filesystem,
            available: true,
            unavailable_reason: None,
            native_compression: true,
            native_checksums: true,
            native_snapshots: true,
            deduplication: false,
        };

        // ZFS should score higher due to dedup + priority
        assert!(zfs.score() > filesystem.score());

        // Unavailable backend should score 0
        zfs.available = false;
        assert_eq!(zfs.score(), 0);
    }

    #[tokio::test]
    async fn test_backend_report() {
        let report = get_backend_report().await;

        assert!(report.contains("Storage Backend Detection"));
        assert!(report.contains("Filesystem (Pure Rust)"));
        assert!(report.contains("Recommended:"));
    }
}

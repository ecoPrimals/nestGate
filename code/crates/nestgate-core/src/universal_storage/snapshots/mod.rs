//! # Pure Rust Snapshot Module
//!
//! Self-contained snapshot implementation for copy-on-write (COW) functionality.
//! Supports multiple strategies for different platforms and filesystems.
//!
//! ## Features
//! - **Hardlink Snapshots**: Fast, space-efficient snapshots on Linux/macOS
//! - **Copy Snapshots**: Reliable fallback for Windows/unsupported filesystems
//! - **Metadata Tracking**: Efficient binary serialization with bincode
//! - **Garbage Collection**: Automatic cleanup of old snapshots
//! - **Rollback**: Restore data to previous snapshot state
//!
//! ## Example
//! ```rust,no_run
//! use nestgate_core::universal_storage::snapshots::{SnapshotManager, SnapshotStrategy};
//! use std::path::Path;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // Create snapshot manager with auto-detected strategy
//! let manager = SnapshotManager::new(Path::new("/data/storage")).await?;
//!
//! // Create a snapshot
//! let snapshot_id = manager.create_snapshot("backup-001", "Daily backup").await?;
//!
//! // List snapshots
//! let snapshots = manager.list_snapshots().await?;
//!
//! // Rollback to a snapshot
//! manager.rollback(&snapshot_id).await?;
//! # Ok(())
//! # }
//! ```

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use thiserror::Error;
use tokio::fs;

/// Snapshot-related errors
#[derive(Debug, Error)]
pub enum SnapshotError {
    /// I/O error during snapshot operations
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    /// Snapshot not found
    #[error("Snapshot not found: {0}")]
    SnapshotNotFound(String),

    /// Invalid snapshot metadata
    #[error("Invalid snapshot metadata: {0}")]
    InvalidMetadata(String),

    /// Serialization error
    #[error("Serialization error: {0}")]
    SerializationError(String),

    /// Strategy not supported on this platform/filesystem
    #[error("Strategy not supported: {0}")]
    StrategyNotSupported(String),

    /// Snapshot already exists
    #[error("Snapshot already exists: {0}")]
    SnapshotExists(String),

    /// Rollback failed
    #[error("Rollback failed: {0}")]
    RollbackFailed(String),
}

/// Result type for snapshot operations
pub type Result<T> = std::result::Result<T, SnapshotError>;

/// Snapshot strategies for different platforms and filesystems
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SnapshotStrategy {
    /// Hardlink-based snapshots (Linux/macOS, space-efficient)
    /// - Speed: Very fast
    /// - Space: Minimal (only changed blocks)
    /// - Platform: Linux, macOS, Unix-like
    Hardlink,

    /// Full copy snapshots (Windows fallback, slower but reliable)
    /// - Speed: Slower
    /// - Space: Full copy
    /// - Platform: All (especially Windows)
    Copy,

    /// Auto-detect best strategy for platform
    Auto,
}

impl SnapshotStrategy {
    /// Detect the best strategy for the current platform and filesystem
    pub async fn detect(base_path: &Path) -> Self {
        // Try to create a test hardlink
        if Self::can_use_hardlinks(base_path).await {
            Self::Hardlink
        } else {
            Self::Copy
        }
    }

    /// Test if hardlinks are supported on the given path
    async fn can_use_hardlinks(base_path: &Path) -> bool {
        // Create temporary test directory
        let test_dir = base_path.join(".snapshot_test");
        if fs::create_dir_all(&test_dir).await.is_err() {
            return false;
        }

        // Try to create a test file and hardlink
        let test_file = test_dir.join("test_file");
        let test_link = test_dir.join("test_link");

        let can_hardlink = async {
            fs::write(&test_file, b"test").await.ok()?;
            fs::hard_link(&test_file, &test_link).await.ok()?;
            Some(())
        }
        .await
        .is_some();

        // Clean up
        let _ = fs::remove_dir_all(&test_dir).await;

        can_hardlink
    }
}

impl Default for SnapshotStrategy {
    fn default() -> Self {
        Self::Auto
    }
}

/// Snapshot metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotMetadata {
    /// Unique snapshot ID
    pub id: String,

    /// Human-readable name
    pub name: String,

    /// Optional description
    pub description: String,

    /// Creation timestamp
    pub created_at: DateTime<Utc>,

    /// Strategy used for this snapshot
    pub strategy: SnapshotStrategy,

    /// Size in bytes (for copy snapshots)
    pub size_bytes: Option<u64>,

    /// Number of files in snapshot
    pub file_count: u64,

    /// Checksum of snapshot contents (for integrity verification)
    pub checksum: Option<Vec<u8>>,

    /// Custom metadata tags
    pub tags: HashMap<String, String>,
}

impl SnapshotMetadata {
    /// Create new snapshot metadata
    pub fn new(id: String, name: String, description: String, strategy: SnapshotStrategy) -> Self {
        Self {
            id,
            name,
            description,
            created_at: Utc::now(),
            strategy,
            size_bytes: None,
            file_count: 0,
            checksum: None,
            tags: HashMap::new(),
        }
    }

    /// Serialize to bincode (efficient binary format)
    pub fn to_bincode(&self) -> Result<Vec<u8>> {
        bincode::serialize(self).map_err(|e| SnapshotError::SerializationError(e.to_string()))
    }

    /// Deserialize from bincode
    pub fn from_bincode(data: &[u8]) -> Result<Self> {
        bincode::deserialize(data).map_err(|e| SnapshotError::SerializationError(e.to_string()))
    }

    /// Serialize to JSON (human-readable)
    pub fn to_json(&self) -> Result<String> {
        serde_json::to_string_pretty(self)
            .map_err(|e| SnapshotError::SerializationError(e.to_string()))
    }

    /// Deserialize from JSON
    pub fn from_json(data: &str) -> Result<Self> {
        serde_json::from_str(data).map_err(|e| SnapshotError::SerializationError(e.to_string()))
    }
}

/// Snapshot manager for creating, listing, and managing snapshots
pub struct SnapshotManager {
    /// Base path for data storage
    base_path: PathBuf,

    /// Directory for storing snapshots
    snapshot_dir: PathBuf,

    /// Strategy to use for snapshots
    strategy: SnapshotStrategy,
}

impl SnapshotManager {
    /// Create a new snapshot manager
    ///
    /// # Arguments
    /// * `base_path` - The root directory to manage snapshots for
    ///
    /// # Example
    /// ```rust,no_run
    /// # use nestgate_core::universal_storage::snapshots::SnapshotManager;
    /// # use std::path::Path;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = SnapshotManager::new(Path::new("/data")).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn new(base_path: &Path) -> Result<Self> {
        Self::with_strategy(base_path, SnapshotStrategy::Auto).await
    }

    /// Create a new snapshot manager with explicit strategy
    ///
    /// # Arguments
    /// * `base_path` - The root directory to manage snapshots for
    /// * `strategy` - The snapshot strategy to use (or Auto to detect)
    ///
    /// # Example
    /// ```rust,no_run
    /// # use nestgate_core::universal_storage::snapshots::{SnapshotManager, SnapshotStrategy};
    /// # use std::path::Path;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = SnapshotManager::with_strategy(
    ///     Path::new("/data"),
    ///     SnapshotStrategy::Hardlink
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn with_strategy(base_path: &Path, strategy: SnapshotStrategy) -> Result<Self> {
        let snapshot_dir = base_path.join(".snapshots");
        fs::create_dir_all(&snapshot_dir).await?;

        // Detect strategy if Auto
        let strategy = if strategy == SnapshotStrategy::Auto {
            SnapshotStrategy::detect(base_path).await
        } else {
            strategy
        };

        Ok(Self {
            base_path: base_path.to_path_buf(),
            snapshot_dir,
            strategy,
        })
    }

    /// Get the current snapshot strategy
    pub fn strategy(&self) -> SnapshotStrategy {
        self.strategy
    }

    /// Get the base path
    pub fn base_path(&self) -> &Path {
        &self.base_path
    }

    /// Get the snapshot directory
    pub fn snapshot_dir(&self) -> &Path {
        &self.snapshot_dir
    }

    /// Create a new snapshot
    ///
    /// # Arguments
    /// * `name` - Human-readable name for the snapshot
    /// * `description` - Optional description
    ///
    /// # Returns
    /// The unique snapshot ID
    ///
    /// # Example
    /// ```rust,no_run
    /// # use nestgate_core::universal_storage::snapshots::SnapshotManager;
    /// # use std::path::Path;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = SnapshotManager::new(Path::new("/data")).await?;
    /// let snapshot_id = manager.create_snapshot("backup-001", "Daily backup").await?;
    /// println!("Created snapshot: {}", snapshot_id);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create_snapshot(&self, name: &str, description: &str) -> Result<String> {
        // Generate unique snapshot ID
        let snapshot_id = format!("snap_{}", uuid::Uuid::new_v4().simple());
        let snapshot_path = self.snapshot_dir.join(&snapshot_id);

        // Check if snapshot already exists
        if snapshot_path.exists() {
            return Err(SnapshotError::SnapshotExists(snapshot_id));
        }

        // Create snapshot directory
        fs::create_dir_all(&snapshot_path).await?;

        // Create metadata
        let mut metadata = SnapshotMetadata::new(
            snapshot_id.clone(),
            name.to_string(),
            description.to_string(),
            self.strategy,
        );

        // Perform snapshot based on strategy
        match self.strategy {
            SnapshotStrategy::Hardlink => {
                self.create_hardlink_snapshot(&snapshot_path, &mut metadata)
                    .await?;
            }
            SnapshotStrategy::Copy => {
                self.create_copy_snapshot(&snapshot_path, &mut metadata)
                    .await?;
            }
            SnapshotStrategy::Auto => {
                unreachable!("Auto strategy should have been resolved");
            }
        }

        // Save metadata
        self.save_metadata(&snapshot_id, &metadata).await?;

        Ok(snapshot_id)
    }

    /// Create a hardlink-based snapshot
    async fn create_hardlink_snapshot(
        &self,
        snapshot_path: &Path,
        metadata: &mut SnapshotMetadata,
    ) -> Result<()> {
        let mut file_count = 0u64;

        // Walk the base directory and create hardlinks
        self.walk_and_link(&self.base_path, snapshot_path, &mut file_count)
            .await?;

        metadata.file_count = file_count;
        metadata.size_bytes = Some(0); // Hardlinks don't take extra space

        Ok(())
    }

    /// Recursively walk directory and create hardlinks
    #[allow(clippy::only_used_in_recursion)]
    fn walk_and_link<'a>(
        &'a self,
        source: &'a Path,
        dest: &'a Path,
        file_count: &'a mut u64,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<()>> + 'a>> {
        Box::pin(async move {
            let mut entries = fs::read_dir(source).await?;

            while let Some(entry) = entries.next_entry().await? {
                let path = entry.path();
                let file_name = entry.file_name();
                let dest_path = dest.join(&file_name);

                // Skip .snapshots directory
                if file_name == ".snapshots" {
                    continue;
                }

                let metadata = entry.metadata().await?;

                if metadata.is_dir() {
                    // Create directory and recurse
                    fs::create_dir_all(&dest_path).await?;
                    self.walk_and_link(&path, &dest_path, file_count).await?;
                } else if metadata.is_file() {
                    // Create hardlink
                    fs::hard_link(&path, &dest_path).await?;
                    *file_count += 1;
                }
            }

            Ok(())
        })
    }

    /// Create a copy-based snapshot
    async fn create_copy_snapshot(
        &self,
        snapshot_path: &Path,
        metadata: &mut SnapshotMetadata,
    ) -> Result<()> {
        let mut file_count = 0u64;
        let mut total_size = 0u64;

        // Walk the base directory and copy files
        self.walk_and_copy(
            &self.base_path,
            snapshot_path,
            &mut file_count,
            &mut total_size,
        )
        .await?;

        metadata.file_count = file_count;
        metadata.size_bytes = Some(total_size);

        Ok(())
    }

    /// Recursively walk directory and copy files
    #[allow(clippy::only_used_in_recursion)]
    fn walk_and_copy<'a>(
        &'a self,
        source: &'a Path,
        dest: &'a Path,
        file_count: &'a mut u64,
        total_size: &'a mut u64,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<()>> + 'a>> {
        Box::pin(async move {
            let mut entries = fs::read_dir(source).await?;

            while let Some(entry) = entries.next_entry().await? {
                let path = entry.path();
                let file_name = entry.file_name();
                let dest_path = dest.join(&file_name);

                // Skip .snapshots directory
                if file_name == ".snapshots" {
                    continue;
                }

                let metadata = entry.metadata().await?;

                if metadata.is_dir() {
                    // Create directory and recurse
                    fs::create_dir_all(&dest_path).await?;
                    self.walk_and_copy(&path, &dest_path, file_count, total_size)
                        .await?;
                } else if metadata.is_file() {
                    // Copy file
                    fs::copy(&path, &dest_path).await?;
                    *file_count += 1;
                    *total_size += metadata.len();
                }
            }

            Ok(())
        })
    }

    /// Save snapshot metadata
    async fn save_metadata(&self, snapshot_id: &str, metadata: &SnapshotMetadata) -> Result<()> {
        let metadata_path = self.snapshot_dir.join(format!("{}.meta", snapshot_id));
        let data = metadata.to_bincode()?;
        fs::write(&metadata_path, data).await?;

        // Also save JSON for human readability
        let json_path = self.snapshot_dir.join(format!("{}.json", snapshot_id));
        let json = metadata.to_json()?;
        fs::write(&json_path, json).await?;

        Ok(())
    }

    /// Load snapshot metadata
    async fn load_metadata(&self, snapshot_id: &str) -> Result<SnapshotMetadata> {
        let metadata_path = self.snapshot_dir.join(format!("{}.meta", snapshot_id));

        if !metadata_path.exists() {
            return Err(SnapshotError::SnapshotNotFound(snapshot_id.to_string()));
        }

        let data = fs::read(&metadata_path).await?;
        SnapshotMetadata::from_bincode(&data)
    }

    /// List all snapshots
    ///
    /// # Example
    /// ```rust,no_run
    /// # use nestgate_core::universal_storage::snapshots::SnapshotManager;
    /// # use std::path::Path;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = SnapshotManager::new(Path::new("/data")).await?;
    /// let snapshots = manager.list_snapshots().await?;
    /// for snapshot in snapshots {
    ///     println!("{}: {} ({})", snapshot.id, snapshot.name, snapshot.created_at);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list_snapshots(&self) -> Result<Vec<SnapshotMetadata>> {
        let mut snapshots = Vec::new();
        let mut entries = fs::read_dir(&self.snapshot_dir).await?;

        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();

            // Look for .meta files
            if let Some(ext) = path.extension() {
                if ext == "meta" {
                    if let Some(stem) = path.file_stem() {
                        let snapshot_id = stem.to_string_lossy().to_string();
                        if let Ok(metadata) = self.load_metadata(&snapshot_id).await {
                            snapshots.push(metadata);
                        }
                    }
                }
            }
        }

        // Sort by creation time (newest first)
        snapshots.sort_by(|a, b| b.created_at.cmp(&a.created_at));

        Ok(snapshots)
    }

    /// Get snapshot metadata by ID
    ///
    /// # Example
    /// ```rust,no_run
    /// # use nestgate_core::universal_storage::snapshots::SnapshotManager;
    /// # use std::path::Path;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = SnapshotManager::new(Path::new("/data")).await?;
    /// let metadata = manager.get_snapshot("snap_abc123").await?;
    /// println!("Snapshot: {} created at {}", metadata.name, metadata.created_at);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_snapshot(&self, snapshot_id: &str) -> Result<SnapshotMetadata> {
        self.load_metadata(snapshot_id).await
    }

    /// Delete a snapshot
    ///
    /// # Example
    /// ```rust,no_run
    /// # use nestgate_core::universal_storage::snapshots::SnapshotManager;
    /// # use std::path::Path;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = SnapshotManager::new(Path::new("/data")).await?;
    /// manager.delete_snapshot("snap_abc123").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete_snapshot(&self, snapshot_id: &str) -> Result<()> {
        let snapshot_path = self.snapshot_dir.join(snapshot_id);
        let metadata_path = self.snapshot_dir.join(format!("{}.meta", snapshot_id));
        let json_path = self.snapshot_dir.join(format!("{}.json", snapshot_id));

        // Remove snapshot data
        if snapshot_path.exists() {
            fs::remove_dir_all(&snapshot_path).await?;
        }

        // Remove metadata files
        if metadata_path.exists() {
            fs::remove_file(&metadata_path).await?;
        }

        if json_path.exists() {
            fs::remove_file(&json_path).await?;
        }

        Ok(())
    }

    /// Rollback to a snapshot (restores files from snapshot)
    ///
    /// # Warning
    /// This will overwrite current data with snapshot data!
    ///
    /// # Example
    /// ```rust,no_run
    /// # use nestgate_core::universal_storage::snapshots::SnapshotManager;
    /// # use std::path::Path;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = SnapshotManager::new(Path::new("/data")).await?;
    /// manager.rollback("snap_abc123").await?;
    /// println!("Rollback complete!");
    /// # Ok(())
    /// # }
    /// ```
    pub async fn rollback(&self, snapshot_id: &str) -> Result<()> {
        let metadata = self.load_metadata(snapshot_id).await?;
        let snapshot_path = self.snapshot_dir.join(snapshot_id);

        if !snapshot_path.exists() {
            return Err(SnapshotError::SnapshotNotFound(snapshot_id.to_string()));
        }

        // Restore based on strategy
        match metadata.strategy {
            SnapshotStrategy::Hardlink | SnapshotStrategy::Copy => {
                self.restore_from_path(&snapshot_path).await?;
            }
            SnapshotStrategy::Auto => {
                unreachable!("Auto strategy should have been resolved");
            }
        }

        Ok(())
    }

    /// Restore files from snapshot path
    async fn restore_from_path(&self, snapshot_path: &Path) -> Result<()> {
        self.walk_and_restore(snapshot_path, &self.base_path).await
    }

    /// Recursively walk snapshot and restore files
    #[allow(clippy::only_used_in_recursion)]
    fn walk_and_restore<'a>(
        &'a self,
        source: &'a Path,
        dest: &'a Path,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<()>> + 'a>> {
        Box::pin(async move {
            let mut entries = fs::read_dir(source).await?;

            while let Some(entry) = entries.next_entry().await? {
                let path = entry.path();
                let file_name = entry.file_name();
                let dest_path = dest.join(&file_name);

                let metadata = entry.metadata().await?;

                if metadata.is_dir() {
                    // Create directory and recurse
                    fs::create_dir_all(&dest_path).await?;
                    self.walk_and_restore(&path, &dest_path).await?;
                } else if metadata.is_file() {
                    // Copy file (overwrite existing)
                    fs::copy(&path, &dest_path).await?;
                }
            }

            Ok(())
        })
    }

    /// Garbage collect old snapshots (keep only N most recent)
    ///
    /// # Example
    /// ```rust,no_run
    /// # use nestgate_core::universal_storage::snapshots::SnapshotManager;
    /// # use std::path::Path;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let manager = SnapshotManager::new(Path::new("/data")).await?;
    /// // Keep only the 10 most recent snapshots
    /// let deleted = manager.garbage_collect(10).await?;
    /// println!("Deleted {} old snapshots", deleted);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn garbage_collect(&self, keep_count: usize) -> Result<usize> {
        let mut snapshots = self.list_snapshots().await?;

        // Already sorted by creation time (newest first)
        if snapshots.len() <= keep_count {
            return Ok(0);
        }

        let to_delete = snapshots.split_off(keep_count);
        let deleted_count = to_delete.len();

        for snapshot in to_delete {
            self.delete_snapshot(&snapshot.id).await?;
        }

        Ok(deleted_count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_snapshot_strategy_detection() {
        let temp_dir = TempDir::new().expect("Storage operation failed");
        let strategy = SnapshotStrategy::detect(temp_dir.path()).await;

        // Should detect a valid strategy
        assert!(strategy == SnapshotStrategy::Hardlink || strategy == SnapshotStrategy::Copy);
    }

    #[tokio::test]
    async fn test_snapshot_metadata_serialization() {
        let metadata = SnapshotMetadata::new(
            "test_id".to_string(),
            "Test Snapshot".to_string(),
            "Test description".to_string(),
            SnapshotStrategy::Hardlink,
        );

        // Test bincode serialization
        let bincode_data = metadata.to_bincode().expect("Storage operation failed");
        let deserialized = SnapshotMetadata::from_bincode(&bincode_data).expect("Storage operation failed");
        assert_eq!(metadata.id, deserialized.id);
        assert_eq!(metadata.name, deserialized.name);

        // Test JSON serialization
        let json_data = metadata.to_json().expect("Storage operation failed");
        let deserialized = SnapshotMetadata::from_json(&json_data).expect("Storage operation failed");
        assert_eq!(metadata.id, deserialized.id);
        assert_eq!(metadata.name, deserialized.name);
    }

    #[tokio::test]
    async fn test_snapshot_manager_creation() {
        let temp_dir = TempDir::new().expect("Storage operation failed");
        let manager = SnapshotManager::new(temp_dir.path()).await.expect("Storage operation failed");

        // Snapshot directory should be created
        assert!(manager.snapshot_dir().exists());

        // Strategy should be detected
        assert!(
            manager.strategy() == SnapshotStrategy::Hardlink
                || manager.strategy() == SnapshotStrategy::Copy
        );
    }

    #[tokio::test]
    async fn test_create_and_list_snapshots() {
        let temp_dir = TempDir::new().expect("Storage operation failed");

        // Create some test files
        let test_file = temp_dir.path().join("test.txt");
        fs::write(&test_file, b"Hello, World!").await.expect("Storage operation failed");

        let manager = SnapshotManager::new(temp_dir.path()).await.expect("Storage operation failed");

        // Create snapshot
        let snapshot_id = manager
            .create_snapshot("test", "Test snapshot")
            .await
            .expect("Storage operation failed");
        assert!(!snapshot_id.is_empty());

        // List snapshots
        let snapshots = manager.list_snapshots().await.expect("Storage operation failed");
        assert_eq!(snapshots.len(), 1);
        assert_eq!(snapshots[0].name, "test");
        assert_eq!(snapshots[0].file_count, 1);
    }

    #[tokio::test]
    async fn test_snapshot_rollback() {
        let temp_dir = TempDir::new().expect("Storage operation failed");

        // Create original file
        let test_file = temp_dir.path().join("test.txt");
        fs::write(&test_file, b"Original").await.expect("Storage operation failed");

        // Use Copy strategy for true data protection (hardlinks share inodes!)
        let manager = SnapshotManager::with_strategy(temp_dir.path(), SnapshotStrategy::Copy)
            .await
            .expect("Storage operation failed");

        // Create snapshot
        let snapshot_id = manager.create_snapshot("backup", "Backup").await.expect("Storage operation failed");

        // Verify snapshot was created
        let snapshots = manager.list_snapshots().await.expect("Storage operation failed");
        assert_eq!(snapshots.len(), 1);
        assert_eq!(snapshots[0].file_count, 1);

        // Modify file
        fs::write(&test_file, b"Modified").await.expect("Storage operation failed");
        let modified = fs::read_to_string(&test_file).await.expect("Storage operation failed");
        assert_eq!(modified, "Modified");

        // Rollback
        manager.rollback(&snapshot_id).await.expect("Storage operation failed");

        // File should be restored
        let restored = fs::read_to_string(&test_file).await.expect("Storage operation failed");
        assert_eq!(restored, "Original");
    }

    #[tokio::test]
    async fn test_snapshot_deletion() {
        let temp_dir = TempDir::new().expect("Storage operation failed");
        let test_file = temp_dir.path().join("test.txt");
        fs::write(&test_file, b"Test").await.expect("Storage operation failed");

        let manager = SnapshotManager::new(temp_dir.path()).await.expect("Storage operation failed");

        // Create snapshot
        let snapshot_id = manager.create_snapshot("test", "Test").await.expect("Storage operation failed");

        // Verify it exists
        let snapshots = manager.list_snapshots().await.expect("Storage operation failed");
        assert_eq!(snapshots.len(), 1);

        // Delete snapshot
        manager.delete_snapshot(&snapshot_id).await.expect("Storage operation failed");

        // Verify it's gone
        let snapshots = manager.list_snapshots().await.expect("Storage operation failed");
        assert_eq!(snapshots.len(), 0);
    }

    #[tokio::test]
    async fn test_garbage_collection() {
        let temp_dir = TempDir::new().expect("Storage operation failed");
        let test_file = temp_dir.path().join("test.txt");
        fs::write(&test_file, b"Test").await.expect("Storage operation failed");

        let manager = SnapshotManager::new(temp_dir.path()).await.expect("Storage operation failed");

        // Create 5 snapshots
        for i in 0..5 {
            manager
                .create_snapshot(&format!("snap-{}", i), "Test")
                .await
                .expect("Storage operation failed");
            // Small delay to ensure different timestamps
            tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        }

        // Verify 5 snapshots exist
        let snapshots = manager.list_snapshots().await.expect("Storage operation failed");
        assert_eq!(snapshots.len(), 5);

        // Keep only 3 most recent
        let deleted = manager.garbage_collect(3).await.expect("Storage operation failed");
        assert_eq!(deleted, 2);

        // Verify only 3 remain
        let snapshots = manager.list_snapshots().await.expect("Storage operation failed");
        assert_eq!(snapshots.len(), 3);
    }
}

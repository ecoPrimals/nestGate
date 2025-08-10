//! In-Memory Storage Backend
//!
//! Provides a fast in-memory storage implementation suitable for caching,
//! testing, and temporary data storage.

// Removed ResponseMetadata import - using local definition instead

// Temporary type aliases and structs for compatibility
pub type StorageProtocolInfo = std::collections::HashMap<String, String>;

#[derive(Debug, Clone)]
pub struct FileMetadata {
    pub path: String,
    pub size: u64,
    pub permissions: String,
    pub owner: String,
    pub group: String,
    pub checksum: Option<String>,
    pub mime_type: Option<String>,
    pub content_type: Option<String>,
    pub custom_metadata: std::collections::HashMap<String, String>,
    pub created: Option<std::time::SystemTime>,
    pub modified: Option<std::time::SystemTime>,
    pub accessed: Option<std::time::SystemTime>,
    pub created_at: Option<std::time::SystemTime>,
    pub modified_at: Option<std::time::SystemTime>,
    pub tags: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct ResponseMetadata {
    pub status: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub request_id: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum EntryType {
    File,
    Directory,
    Symlink,
}
use crate::Result;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::SystemTime;
use tokio::sync::broadcast;
use tokio::sync::RwLock;

/// In-memory file entry
#[derive(Debug, Clone)]
#[allow(dead_code)] // Internal storage structure
struct MemoryFile {
    content: Vec<u8>,
    metadata: FileMetadata,
    created: SystemTime,
    modified: SystemTime,
}

/// In-memory directory entry
#[derive(Debug, Clone)]
#[allow(dead_code)] // Internal storage structure
struct MemoryDirectory {
    created: SystemTime,
    modified: SystemTime,
}

/// Storage entry (either file or directory)
#[derive(Debug, Clone)]
#[allow(dead_code)] // Internal storage enum
enum StorageEntry {
    File(Box<MemoryFile>),
    Directory(MemoryDirectory),
}

/// In-memory storage backend implementation
#[allow(dead_code)] // Backend implementation - fields used internally
pub struct MemoryBackend {
    /// Storage map (path -> entry)
    storage: Arc<RwLock<HashMap<String, StorageEntry>>>,
    /// Configuration
    max_memory_mb: usize,
    /// Current memory usage in bytes
    current_memory: Arc<RwLock<usize>>,
    /// Change notification broadcaster
    change_broadcaster: broadcast::Sender<String>,
}

impl MemoryBackend {
    /// Create a new memory backend with default settings
    pub fn new() -> Result<Self> {
        let (change_broadcaster, _) = broadcast::channel(1000);

        Ok(Self {
            storage: Arc::new(RwLock::new(HashMap::new())),
            max_memory_mb: 100, // Default 100MB limit
            current_memory: Arc::new(RwLock::new(0)),
            change_broadcaster,
        })
    }

    /// Create a new memory backend with custom memory limit
    pub fn with_memory_limit(max_memory_mb: usize) -> Result<Self> {
        let (change_broadcaster, _) = broadcast::channel(1000);

        Ok(Self {
            storage: Arc::new(RwLock::new(HashMap::new())),
            max_memory_mb,
            current_memory: Arc::new(RwLock::new(0)),
            change_broadcaster,
        })
    }

    #[allow(dead_code)]
    async fn check_memory_limit(&self, additional_bytes: usize) -> Result<()> {
        let current = *self.current_memory.read().await;
        let max_bytes = self.max_memory_mb * 1024 * 1024;

        if current + additional_bytes > max_bytes {
            return Err(crate::error::NestGateError::ResourceExhausted {
                resource: "memory".to_string(),
                current: current as u64,
                limit: max_bytes as u64,
                retry_after: Some(std::time::Duration::from_secs(1)),
                scaling_suggestion: Some("Increase memory limit or clear cache".to_string()),
            });
        }
        Ok(())
    }

    /// Update memory usage
    #[allow(dead_code)]
    async fn update_memory_usage(&self, delta: i64) -> Result<()> {
        let mut current = self.current_memory.write().await;
        if delta < 0 {
            *current = current.saturating_sub((-delta) as usize);
        } else {
            *current += delta as usize;
        }
        Ok(())
    }

    /// Normalize path (remove leading/trailing slashes, handle empty paths)
    #[allow(dead_code)]
    fn normalize_path(&self, path: &str) -> String {
        if path.is_empty() || path == "/" {
            return "/".to_string();
        }

        let path = path.trim_start_matches('/').trim_end_matches('/');
        format!("/{path}")
    }

    /// Get parent directory path
    #[allow(dead_code)]
    fn get_parent_path(&self, path: &str) -> Option<String> {
        let normalized = self.normalize_path(path);
        if normalized == "/" {
            return None;
        }

        let parts: Vec<&str> = normalized.split('/').filter(|s| !s.is_empty()).collect();
        if parts.is_empty() {
            return Some("/".to_string());
        }

        if parts.len() == 1 {
            return Some("/".to_string());
        }

        let parent_parts = &parts[..parts.len() - 1];
        Some(format!("/{}", parent_parts.join("/")))
    }

    /// Check if a directory exists
    #[allow(dead_code)]
    async fn directory_exists(&self, path: &str) -> Result<bool> {
        let storage = self.storage.read().await;
        match storage.get(&self.normalize_path(path)) {
            Some(StorageEntry::Directory(_)) => Ok(true),
            _ => Ok(false),
        }
    }

    /// Ensure parent directory exists
    #[allow(dead_code)]
    async fn ensure_parent_directory(&self, path: &str) -> Result<()> {
        if let Some(parent_path) = self.get_parent_path(path) {
            if parent_path != "/" && !self.directory_exists(&parent_path).await? {
                // Recursively create parent directories
                Box::pin(self.ensure_parent_directory(&parent_path)).await?;

                let mut storage = self.storage.write().await;
                let now = SystemTime::now();
                storage.insert(
                    parent_path.clone(),
                    StorageEntry::Directory(MemoryDirectory {
                        created: now,
                        modified: now,
                    }),
                );
            }
        }
        Ok(())
    }

    /// Create file metadata
    #[allow(dead_code)]
    fn create_file_metadata(&self, content: &[u8]) -> FileMetadata {
        let now = SystemTime::now();
        let _utc_now = chrono::Utc::now();
        FileMetadata {
            path: String::new(), // Will be set by caller
            size: content.len() as u64,
            created_at: Some(std::time::SystemTime::now()),
            modified_at: Some(std::time::SystemTime::now()),
            created: Some(now),
            modified: Some(now),
            accessed: Some(now),
            content_type: Some("application/octet-stream".to_string()),
            permissions: "644".to_string(),
            owner: "nestgate".to_string(),
            group: "nestgate".to_string(),
            checksum: None,
            mime_type: Some("application/octet-stream".to_string()),
            tags: HashMap::new(),
            custom_metadata: HashMap::new(),
        }
    }

    /// Notify about changes
    #[allow(dead_code)]
    fn notify_change(&self, path: &str) {
        let _ = self.change_broadcaster.send(path.to_string());
    }
}

// TEMPORARILY DISABLED: StorageProtocolHandler trait implementation
/*
#[async_trait]
impl StorageProtocolHandler for MemoryBackend {
    async fn handle_request(&self, request: StorageRequest) -> Result<StorageResponse> {
        match request {
            StorageRequest::CreateFile {
                path,
                content,
                metadata: _,
            } => {
                self.check_memory_limit(content.len()).await?;
                let normalized_path = self.normalize_path(&path);

                // Ensure parent directory exists
                self.ensure_parent_directory(&normalized_path).await?;

                let mut storage = self.storage.write().await;
                let file = MemoryFile {
                    content: content.clone(),
                    metadata: self.create_file_metadata(&content),
                    created: SystemTime::now(),
                    modified: SystemTime::now(),
                };

                storage.insert(normalized_path.clone(), StorageEntry::File(Box::new(file)));
                drop(storage); // Release lock before async operations

                self.update_memory_usage(content.len() as i64).await?;
                self.notify_change(&normalized_path);

                Ok(StorageResponse::Success {
                    operation: "create_file".to_string(),
                    metadata: ResponseMetadata {
                        path: path.clone(),
                        size: Some(content.len() as u64),
                        operation_id: uuid::Uuid::new_v4().to_string(),
                        timestamp: chrono::Utc::now(),
                        backend: "memory".to_string(),
                        protocol: "memory".to_string(),
                    },
                })
            }

            StorageRequest::ReadFile { path, range } => {
                let normalized_path = self.normalize_path(&path);
                let storage = self.storage.read().await;

                match storage.get(&normalized_path) {
                    Some(StorageEntry::File(file)) => {
                        let content = if let Some(range) = range {
                            let start = range.start as usize;
                            let end = (range.end as usize).min(file.content.len());
                            if start >= file.content.len() {
                                Vec::new()
                            } else {
                                file.content[start..end].to_vec()
                            }
                        } else {
                            file.content.clone()
                        };

                        Ok(StorageResponse::FileContent {
                            content,
                            metadata: Box::new(FileMetadata {
                                path: path.clone(),
                                size: file.content.len() as u64,
                                created_at: file.created.into(),
                                modified_at: file.modified.into(),
                                created: Some(file.created),
                                modified: Some(file.modified),
                                accessed: Some(file.modified),
                                content_type: file.metadata.content_type.clone(),
                                permissions: "644".to_string(),
                                owner: "nestgate".to_string(),
                                group: "nestgate".to_string(),
                                checksum: file.metadata.checksum.clone(),
                                mime_type: file.metadata.content_type.clone(),
                                tags: HashMap::new(),
                                custom_metadata: HashMap::new(),
                            }),
                        })
                    }
                    Some(StorageEntry::Directory(_)) => {
                        Err(crate::error::NestGateError::Validation {
                            field: "path".to_string(),
                            message: "Path is a directory, not a file".to_string(),
                            current_value: Some(path),
                            expected: Some("file path".to_string()),
                            user_error: true,
                        })
                    }
                    None => Err(crate::error::NestGateError::NotFound(format!(
                        "File not found: {}",
                        path
                    ))),
                }
            }

            StorageRequest::WriteFile {
                path,
                content,
                offset,
            } => {
                let normalized_path = self.normalize_path(&path);
                let mut storage = self.storage.write().await;

                match storage.get_mut(&normalized_path) {
                    Some(StorageEntry::File(file)) => {
                        let old_size = file.content.len();

                        if let Some(offset) = offset {
                            let offset = offset as usize;
                            if offset <= file.content.len() {
                                // Extend content if necessary
                                if offset + content.len() > file.content.len() {
                                    file.content.resize(offset + content.len(), 0);
                                }
                                // Write at offset
                                file.content[offset..offset + content.len()]
                                    .copy_from_slice(&content);
                            }
                        } else {
                            // Replace entire content
                            file.content = content.clone();
                        }

                        file.modified = SystemTime::now();
                        let new_size = file.content.len();
                        drop(storage); // Release lock before async operations

                        let size_delta = new_size as i64 - old_size as i64;
                        self.update_memory_usage(size_delta).await?;
                        self.notify_change(&normalized_path);
                    }
                    _ => {
                        return Err(crate::error::NestGateError::NotFound(format!(
                            "File not found: {}",
                            path
                        )));
                    }
                }

                Ok(StorageResponse::Success {
                    operation: "write_file".to_string(),
                    metadata: ResponseMetadata {
                        path: path.clone(),
                        size: Some(content.len() as u64),
                        operation_id: uuid::Uuid::new_v4().to_string(),
                        timestamp: chrono::Utc::now(),
                        backend: "memory".to_string(),
                        protocol: "memory".to_string(),
                    },
                })
            }

            StorageRequest::DeleteFile { path } => {
                let normalized_path = self.normalize_path(&path);
                let mut storage = self.storage.write().await;

                match storage.remove(&normalized_path) {
                    Some(StorageEntry::File(file)) => {
                        let size = file.content.len();
                        drop(storage); // Release lock before async operations

                        self.update_memory_usage(-(size as i64)).await?;
                        self.notify_change(&normalized_path);
                    }
                    _ => {
                        return Err(crate::error::NestGateError::NotFound(format!(
                            "File not found: {}",
                            path
                        )));
                    }
                }

                Ok(StorageResponse::Success {
                    operation: "delete_file".to_string(),
                    metadata: ResponseMetadata {
                        path: path.clone(),
                        size: None,
                        operation_id: uuid::Uuid::new_v4().to_string(),
                        timestamp: chrono::Utc::now(),
                        backend: "memory".to_string(),
                        protocol: "memory".to_string(),
                    },
                })
            }

            StorageRequest::ListDirectory { path, recursive } => {
                let normalized_path = self.normalize_path(&path);
                let storage = self.storage.read().await;

                let mut entries = Vec::new();

                for (entry_path, entry) in storage.iter() {
                    if entry_path.starts_with(&normalized_path) && entry_path != &normalized_path {
                        let relative_path = entry_path
                            .strip_prefix(&normalized_path)
                            .unwrap_or(entry_path);
                        if !recursive && relative_path.contains('/') {
                            continue;
                        }

                        match entry {
                            StorageEntry::File(file) => {
                                let name = entry_path
                                    .split('/')
                                    .last()
                                    .unwrap_or(&entry_path)
                                    .to_string();
                                entries.push(DirectoryEntry {
                                    name,
                                    path: entry_path.clone(),
                                    entry_type: EntryType::File,
                                    size: file.content.len() as u64,
                                    modified_at: file.modified.into(),
                                    permissions: "rw-r--r--".to_string(),
                                });
                            }
                            StorageEntry::Directory(dir) => {
                                let name = entry_path
                                    .split('/')
                                    .last()
                                    .unwrap_or(&entry_path)
                                    .to_string();
                                entries.push(DirectoryEntry {
                                    name,
                                    path: entry_path.clone(),
                                    entry_type: EntryType::Directory,
                                    size: 0,
                                    modified_at: dir.modified.into(),
                                    permissions: "rwxr-xr-x".to_string(),
                                });
                            }
                        }
                    }
                }

                let entries_len = entries.len();
                Ok(StorageResponse::DirectoryListing {
                    entries,
                    metadata: ResponseMetadata {
                        path: path.clone(),
                        size: Some(entries_len as u64),
                        operation_id: uuid::Uuid::new_v4().to_string(),
                        timestamp: chrono::Utc::now(),
                        backend: "memory".to_string(),
                        protocol: "memory".to_string(),
                    },
                })
            }

            StorageRequest::CreateDirectory { path } => {
                let normalized_path = self.normalize_path(&path);

                // Ensure parent directory exists
                self.ensure_parent_directory(&normalized_path).await?;

                let mut storage = self.storage.write().await;

                if storage.contains_key(&normalized_path) {
                    return Err(crate::error::NestGateError::Validation {
                        field: "path".to_string(),
                        message: "Directory already exists".to_string(),
                        current_value: Some(path),
                        expected: Some("non-existing path".to_string()),
                        user_error: true,
                    });
                }

                let directory = MemoryDirectory {
                    created: SystemTime::now(),
                    modified: SystemTime::now(),
                };

                storage.insert(normalized_path.clone(), StorageEntry::Directory(directory));
                drop(storage);

                self.notify_change(&normalized_path);

                Ok(StorageResponse::Success {
                    operation: "create_directory".to_string(),
                    metadata: ResponseMetadata {
                        path: path.clone(),
                        size: None,
                        operation_id: uuid::Uuid::new_v4().to_string(),
                        timestamp: chrono::Utc::now(),
                        backend: "memory".to_string(),
                        protocol: "memory".to_string(),
                    },
                })
            }

            StorageRequest::DeleteDirectory { path, recursive } => {
                let normalized_path = self.normalize_path(&path);
                let mut storage = self.storage.write().await;

                if !recursive {
                    // Check if directory is empty
                    let has_children = storage
                        .keys()
                        .any(|k| k.starts_with(&normalized_path) && k != &normalized_path);
                    if has_children {
                        return Err(crate::error::NestGateError::Validation {
                            field: "path".to_string(),
                            message: "Directory is not empty".to_string(),
                            current_value: Some(path),
                            expected: Some("empty directory or use recursive=true".to_string()),
                            user_error: true,
                        });
                    }
                }

                // Collect paths to remove
                let to_remove: Vec<String> = storage
                    .keys()
                    .filter(|k| k.starts_with(&normalized_path))
                    .cloned()
                    .collect();

                // Remove all matching entries
                for path_to_remove in to_remove {
                    storage.remove(&path_to_remove);
                    self.notify_change(&path_to_remove);
                }

                Ok(StorageResponse::Success {
                    operation: "delete_directory".to_string(),
                    metadata: ResponseMetadata {
                        path: path.clone(),
                        size: None,
                        operation_id: uuid::Uuid::new_v4().to_string(),
                        timestamp: chrono::Utc::now(),
                        backend: "memory".to_string(),
                        protocol: "memory".to_string(),
                    },
                })
            }

            StorageRequest::CopyFile {
                source,
                destination,
            } => {
                let source_path = self.normalize_path(&source);
                let dest_path = self.normalize_path(&destination);

                // Ensure destination parent directory exists
                self.ensure_parent_directory(&dest_path).await?;

                let file_copy = {
                    let storage = self.storage.read().await;
                    match storage.get(&source_path) {
                        Some(StorageEntry::File(file)) => {
                            let file_copy = file.clone();
                            let file_size = file.content.len();
                            Some((file_copy, file_size))
                        }
                        _ => None,
                    }
                };

                if let Some((file_copy, file_size)) = file_copy {
                    let mut storage = self.storage.write().await;
                    storage.insert(dest_path.clone(), StorageEntry::File(Box::new(file_copy)));
                    std::mem::drop(storage);

                    self.update_memory_usage(file_size as i64).await?;
                    self.notify_change(&dest_path);

                    Ok(StorageResponse::Success {
                        operation: "copy_file".to_string(),
                        metadata: ResponseMetadata {
                            path: destination.clone(),
                            size: Some(file_size as u64),
                            operation_id: uuid::Uuid::new_v4().to_string(),
                            timestamp: chrono::Utc::now(),
                            backend: "memory".to_string(),
                            protocol: "memory".to_string(),
                        },
                    })
                } else {
                    Err(crate::error::NestGateError::NotFound(format!(
                        "Source file not found: {}",
                        source
                    )))
                }
            }

            StorageRequest::MoveFile {
                source,
                destination,
            } => {
                let source_path = self.normalize_path(&source);
                let dest_path = self.normalize_path(&destination);

                let mut storage = self.storage.write().await;
                match storage.remove(&source_path) {
                    Some(StorageEntry::File(mut file)) => {
                        file.modified = SystemTime::now();
                        let file_size = file.content.len();
                        storage.insert(dest_path.clone(), StorageEntry::File(Box::new(file)));
                        drop(storage);

                        self.notify_change(&source_path);
                        self.notify_change(&dest_path);

                        Ok(StorageResponse::Success {
                            operation: "move_file".to_string(),
                            metadata: ResponseMetadata {
                                path: destination.clone(),
                                size: Some(file_size as u64),
                                operation_id: uuid::Uuid::new_v4().to_string(),
                                timestamp: chrono::Utc::now(),
                                backend: "memory".to_string(),
                                protocol: "memory".to_string(),
                            },
                        })
                    }
                    _ => Err(crate::error::NestGateError::NotFound(format!(
                        "Source file not found: {}",
                        source
                    ))),
                }
            }

            // Snapshots are supported in memory (simple copy)
            StorageRequest::CreateSnapshot { path, name } => {
                let normalized_path = self.normalize_path(&path);
                let snapshot_path = format!("/.snapshots/{}/{}", name, normalized_path);

                let storage = self.storage.read().await;
                match storage.get(&normalized_path) {
                    Some(entry) => {
                        let snapshot_entry = entry.clone();
                        drop(storage);

                        let mut storage = self.storage.write().await;
                        // Ensure snapshots directory exists
                        storage.entry("/.snapshots".to_string()).or_insert_with(|| {
                            StorageEntry::Directory(MemoryDirectory {
                                created: SystemTime::now(),
                                modified: SystemTime::now(),
                            })
                        });

                        storage
                            .entry(format!("/.snapshots/{}", name))
                            .or_insert_with(|| {
                                StorageEntry::Directory(MemoryDirectory {
                                    created: SystemTime::now(),
                                    modified: SystemTime::now(),
                                })
                            });

                        storage.insert(snapshot_path, snapshot_entry);

                        Ok(StorageResponse::Success {
                            operation: "create_snapshot".to_string(),
                            metadata: ResponseMetadata {
                                path: path.clone(),
                                size: None,
                                operation_id: uuid::Uuid::new_v4().to_string(),
                                timestamp: chrono::Utc::now(),
                                backend: "memory".to_string(),
                                protocol: "memory".to_string(),
                            },
                        })
                    }
                    None => Err(crate::error::NestGateError::NotFound(format!(
                        "Snapshot not found: {}/{}",
                        name, path
                    ))),
                }
            }

            StorageRequest::RestoreSnapshot {
                path,
                snapshot_name,
            } => {
                let normalized_path = self.normalize_path(&path);
                let snapshot_path = format!("/.snapshots/{}/{}", snapshot_name, normalized_path);

                let mut storage = self.storage.write().await;
                match storage.get(&snapshot_path).cloned() {
                    Some(snapshot_entry) => {
                        storage.insert(normalized_path.clone(), snapshot_entry);
                        self.notify_change(&normalized_path);

                        Ok(StorageResponse::Success {
                            operation: "restore_snapshot".to_string(),
                            metadata: ResponseMetadata {
                                path: path.clone(),
                                size: None,
                                operation_id: uuid::Uuid::new_v4().to_string(),
                                timestamp: chrono::Utc::now(),
                                backend: "memory".to_string(),
                                protocol: "memory".to_string(),
                            },
                        })
                    }
                    None => Err(crate::error::NestGateError::NotFound(format!(
                        "Snapshot not found: {}/{}",
                        snapshot_name, path
                    ))),
                }
            }

            // Sync operations not implemented
            StorageRequest::SyncPath { .. } => Err(crate::error::NestGateError::NotImplemented {
                feature: "Path synchronization".to_string(),
                location: Some("memory_backend::sync_path".to_string()),
            }),
        }
    }

    async fn stream_data(&self, _request: StreamRequest) -> Result<DataStream> {
        // Simple implementation for memory backend
        Ok(DataStream)
    }

    async fn monitor_changes(&self, _path: &str) -> Result<ChangeStream> {
        // Simple implementation for memory backend
        Ok(ChangeStream)
    }

    fn protocol_info(&self) -> StorageProtocolInfo {
        StorageProtocolInfo {
            name: "Memory Storage Backend".to_string(),
            version: "1.0.0".to_string(),
            capabilities: self.capabilities(),
        }
    }

    fn capabilities(&self) -> Vec<StorageCapability> {
        vec![
            StorageCapability::ReadWrite,
            StorageCapability::BasicFileOps,
            StorageCapability::DirectoryOps,
        ]
    }
}
*/

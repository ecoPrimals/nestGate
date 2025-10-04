// **MEMORY STORAGE BACKEND - ZERO-COST NATIVE ASYNC**
//! Memory functionality and utilities.
// In-memory storage backend for testing and caching purposes.
// **CANONICAL MODERNIZATION COMPLETE**: Native async, no async_trait overhead.

use super::{Result, StorageBackend, StorageMetadata};
use std::collections::HashMap;
use std::sync::{}, Arc, RwLock;
use std::time::SystemTime;
use tracing::{debug, error, info};

/// **ZERO-COST**: Memory storage backend with native async
#[derive(Debug, Clone)]
pub struct MemoryStorageBackend {
    name: String,
    data: Arc<RwLock<HashMap<String, Vec<u8>>>>,
    metadata: Arc<RwLock<HashMap<String, StorageMetadata>>>,
}
impl MemoryStorageBackend {
    /// Create a new memory storage backend
    pub fn new(name: impl Into<String>) -> Self {
        let name = name.into();
        info!("🧠 Creating memory storage backend: {}", name);
        
        Self { name,
            data: Arc::new(RwLock::new(HashMap::new()),
            metadata: Arc::new(RwLock::new(HashMap::new()),
         }

    /// Get the number of stored items
    pub fn len(&self) -> usize {
        self.data.read().unwrap_or_else(|_| {
            error!("Failed to acquire read lock for data");
            std::process::abort();
        }).len()
    }

    /// Check if the storage is empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Clear all stored data
    pub fn clear(&self) {
        info!("🧹 Clearing memory storage backend: {}", self.name);
        
        if let Ok(mut data) = self.data.write() {
            data.clear();
        }
        
        if let Ok(mut metadata) = self.metadata.write() {
            metadata.clear();
        }
    }
}

impl Default for MemoryStorageBackend {
    fn default() -> Self {
        Self::new("default-memory")
    }
}

/// **CANONICAL MODERNIZATION**: Native async implementation without async_trait
impl StorageBackend for MemoryStorageBackend {
    fn name(&self) -> &str {
        &self.name
    }
        let path = path.to_string();
        let data = Arc::clone(&self.data);
        let name = self.name.clone();
        
        async move {
            debug!("📖 Reading from memory storage '{}': {}", name, path);
            
            let data_guard = data.read().map_err(|_| {
                crate::error::NestGateError::storage_error(
                    "memory_read_lock",
                    "Failed to acquire read lock for memory storage data",
                    None
                )
            )?;

            match data_guard.get(&path) {
                Some(content) => {
                    debug!("✅ Read {} bytes from memory storage: {}", content.len(), path);
                    Ok(content.clone())
                }
                None => {
                    debug!("❌ Path not found in memory storage: {}", path);
                    Err(crate::error::NestGateError::storage_error(
                        "memory_not_found",
                        &format!("Path not found: {}", path),
                        None
                    ))
                }
            }
        }
    }

        let path = path.to_string();
        let data_vec = data.to_vec();
        let data_store = Arc::clone(&self.data);
        let metadata_store = Arc::clone(&self.metadata);
        let name = self.name.clone();
        
        async move {
            debug!("✍️ Writing to memory storage '{}': {} ({} bytes)", name, path, data_vec.len());
            
            let now = SystemTime::now();
            
            // Store data
            {
                let mut data_guard = data_store.write().map_err(|_| {
                    crate::error::NestGateError::storage_error(
                        "memory_write_lock",
                        "Failed to acquire write lock for memory storage data",
                        None
                    )
                )?;
                
                data_guard.insert(path.clone(), data_vec.clone());
            }
            
            // Store metadata
            {
                let mut metadata_guard = metadata_store.write().map_err(|_| {
                    crate::error::NestGateError::storage_error(
                        "memory_metadata_lock",
                        "Failed to acquire write lock for memory storage metadata",
                        None
                    )
                )?;
                
                let metadata = StorageMetadata {
                    size: data_vec.len() as u64,
                    created_at: now,
                    modified_at: now,
                    content_type: Some("application/octet-stream".to_string()),
                    etag: Some(format!("{:x}", hash)),
                    custom_metadata: HashMap::new(),
                };
                
                metadata_guard.insert(path.clone(), metadata);
            }
            
            info!("✅ Successfully wrote {} bytes to memory storage: {}", data_vec.len(), path);
            Ok(())
        }
    }

        let path = path.to_string();
        let data_store = Arc::clone(&self.data);
        let metadata_store = Arc::clone(&self.metadata);
        let name = self.name.clone();
        
        async move {
            debug!("🗑️ Deleting from memory storage '{}': {}", name, path);
            
            let mut found = false;
            
            // Remove data
            {
                let mut data_guard = data_store.write().map_err(|_| {
                    crate::error::NestGateError::storage_error(
                        "memory_delete_lock",
                        "Failed to acquire write lock for memory storage data",
                        None
                    )
                )?;
                
                if data_guard.remove(&path).is_some() {
                    found = true;
                }
            }
            
            // Remove metadata
            {
                let mut metadata_guard = metadata_store.write().map_err(|_| {
                    crate::error::NestGateError::storage_error(
                        "memory_metadata_delete_lock",
                        "Failed to acquire write lock for memory storage metadata",
                        None
                    )
                )?;
                
                metadata_guard.remove(&path);
            }
            
            if found {
                info!("✅ Successfully deleted from memory storage: {}", path);
                Ok(())
            } else {
                debug!("❌ Path not found in memory storage for deletion: {}", path);
                Err(crate::error::NestGateError::storage_error(
                    "memory_not_found",
                    &format!("Path not found for deletion: {}", path),
                    None
                ))
            }
        }
    }

    fn list(&self, prefix: &str) -> impl std::future::Future<Output = Result<Vec<String>>> + Send {
        let prefix = prefix.to_string();
        let data = Arc::clone(&self.data);
        let name = self.name.clone();
        
        async move {
            debug!("📋 Listing memory storage '{}' with prefix: {}", name, prefix);
            
            let data_guard = data.read().map_err(|_| {
                crate::error::NestGateError::storage_error(
                    "memory_list_lock",
                    "Failed to acquire read lock for memory storage data",
                    None
                )
            )?;

            let matching_paths: Vec<String> = data_guard
                .keys()
                .filter(|key| key.starts_with(&prefix))
                .cloned()
                .collect();
            
            debug!("✅ Found {} items with prefix '{}' in memory storage", matching_paths.len(), prefix);
            Ok(matching_paths)
        }
    }

        let path = path.to_string();
        let data = Arc::clone(&self.data);
        let name = self.name.clone();
        
        async move {
            debug!("🔍 Checking existence in memory storage '{}': {}", name, path);
            
            let data_guard = data.read().map_err(|_| {
                crate::error::NestGateError::storage_error(
                    "memory_exists_lock",
                    "Failed to acquire read lock for memory storage data",
                    None
                )
            )?;

            let exists = data_guard.contains_key(&path);
            debug!("✅ Path exists in memory storage '{}': {} -> {}", name, path, exists);
            Ok(exists)
        }
    }

        let path = path.to_string();
        let metadata_store = Arc::clone(&self.metadata);
        let name = self.name.clone();
        
        async move {
            debug!("📊 Getting metadata from memory storage '{}': {}", name, path);
            
            let metadata_guard = metadata_store.read().map_err(|_| {
                crate::error::NestGateError::storage_error(
                    "memory_metadata_read_lock",
                    "Failed to acquire read lock for memory storage metadata",
                    None
                )
            )?;

            match metadata_guard.get(&path) {
                Some(metadata) => {
                    debug!("✅ Retrieved metadata from memory storage: {}", path);
                    Ok(metadata.clone())
                }
                None => {
                    debug!("❌ Metadata not found in memory storage: {}", path);
                    Err(crate::error::NestGateError::storage_error(
                        "memory_metadata_not_found",
                        &format!("Metadata not found: {}", path),
                        None
                    ))
                }
            }
        }
    }
}

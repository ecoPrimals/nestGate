// **UNIVERSAL STORAGE BACKENDS - ZERO-COST NATIVE ASYNC**
//! Module definitions and exports.
// This module provides different storage backend implementations with native async traits.
// **CANONICAL MODERNIZATION COMPLETE**: No async_trait overhead, pure native async.

use crate::error::NestGateError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::SystemTime;

// CANONICAL MODERNIZATION: Use canonical Result type instead
pub use crate::Result;

// **DEPRECATED** - Use CanonicalStorage from canonical_unified_traits instead
//
// This trait has been superseded by the canonical storage system.
// **MIGRATION PATH**:
// - Old: `StorageBackend`
// - New: `crate::traits::canonical::CanonicalStorage`
//
// The new CanonicalStorage trait is a comprehensive, unified interface that
// replaces all fragmented storage trait definitions.
#[deprecated(since = "2.1.0", note = "Use crate::traits::canonical::CanonicalStorage instead")]
pub trait StorageBackend {
    /// Get backend name
    fn name(&self) -> &str;
    
    /// Read data from storage
    
    /// Write data to storage
    
    /// Delete data from storage
    
    /// List items in storage
    fn list(&self, prefix: &str) -> impl std::future::Future<Output = Result<Vec<String>>> + Send;
    
    /// Check if item exists
    
    /// Get metadata for an item
}
// Storage metadata information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageMetadata {
    pub size: u64,
    pub created_at: SystemTime,
    pub modified_at: SystemTime,
    pub content_type: Option<String>,
    pub etag: Option<String>,
    pub custom_metadata: HashMap<String, String>,
}
// Re-export backend implementations
pub mod block_storage;
pub mod memory;
pub mod network_fs;
pub mod object_storage;

pub use block_storage::BlockStorageBackend;
pub use memory::MemoryStorageBackend;
pub use network_fs::NetworkFsBackend;
pub use object_storage::ObjectStorageBackend;

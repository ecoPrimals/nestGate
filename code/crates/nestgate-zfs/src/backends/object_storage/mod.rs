//! **OBJECT STORAGE BACKEND - SOVEREIGNTY COMPLIANT** ✅
//!
//! Universal S3-compatible object storage backend that works with ANY provider.
//!
//! ## Module Structure
//!
//! This module is organized by domain concern:
//! - **types**: Public data structures (Pool, Dataset, Snapshot, Properties)
//! - **provider**: Storage provider detection and enum
//! - **config**: Configuration structures and discovery
//! - **client**: S3-compatible client abstraction
//! - **backend**: Main backend implementation
//! - **operations**: ZeroCostZfsOperations trait implementation
//!
//! ## Sovereignty Principles
//!
//! ✅ **No Vendor Hardcoding**: Works with ANY S3-compatible service  
//! ✅ **Capability-Based Discovery**: Discovers services at runtime  
//! ✅ **Protocol-Based**: Uses standard S3-compatible API  
//! ✅ **Runtime Configuration**: All endpoints discovered or env-configured  
//! ✅ **Primal Self-Knowledge**: Only knows itself, discovers others at runtime
//!
//! ## Supported Providers
//!
//! This backend works with **any** S3-compatible object storage:
//!
//! - **AWS S3** - Amazon's object storage
//! - **MinIO** - Self-hosted S3-compatible storage
//! - **Wasabi** - Cloud object storage
//! - **DigitalOcean Spaces** - DO's object storage
//! - **Linode Object Storage** - Linode's S3-compatible storage
//! - **Backblaze B2** - With S3-compatible API
//! - **Ceph RADOS Gateway** - S3-compatible API
//! - **OpenStack Swift** - With S3 compatibility layer
//! - **Azure Blob** - Via S3 compatibility mode
//! - **Google Cloud Storage** - Via S3 interoperability
//! - **Any other S3-compatible service**
//!
//! ## Evolution History
//!
//! - **January 2026**: Smart refactoring from monolithic 932-line file
//!   - Organized by domain concern (not mechanical splitting)
//!   - Each module is self-contained and focused
//!   - Maintains all functionality and sovereignty principles

pub mod backend;
pub mod client;
pub mod config;
pub mod operations;
pub mod provider;
pub mod types;

// Re-export main types
pub use backend::ObjectStorageBackend;
pub use provider::StorageProvider;
pub use types::{ObjectDataset, ObjectPool, ObjectProperties, ObjectSnapshot};

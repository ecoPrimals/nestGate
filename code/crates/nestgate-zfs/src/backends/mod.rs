//! **STORAGE BACKENDS MODULE**
//!
//! This module provides implementations of the `ZeroCostZfsOperations` trait
//! for various cloud and distributed storage backends.
//!
//! ## Backends
//!
//! - **Azure**: Azure Blob Storage
//! - **GCS**: Google Cloud Storage  
//! - **Object Storage**: Universal S3-compatible storage (via Songbird gateway)
//! - **Native**: Local ZFS pools (default)
//!
//! ## Architecture
//!
//! Each backend implements the zero-cost `ZeroCostZfsOperations` trait,
//! providing consistent async API across all storage types.
//!
//! ## Example
//!
//! ```rust,ignore
//! use nestgate_zfs::backends::object_storage::ObjectStorageBackend;
//!
//! // Create object storage backend (via Songbird gateway)
//! let backend = ObjectStorageBackend::new(config).await?;
//! let pool = backend.create_pool("tank", &[]).await?;
//! ```

pub mod azure;
pub mod gcs;

/// Universal S3-compatible object storage backend (sovereignty-compliant)
/// Works with ANY S3-compatible service: AWS, MinIO, Ceph, Wasabi, DigitalOcean, etc.
pub mod object_storage;

/// AWS Signature V4 authentication (protocol-first, no AWS SDK)
pub mod aws_auth;

// Re-exports
pub use azure::AzureBackend;
pub use gcs::GcsBackend;
pub use object_storage::ObjectStorageBackend;

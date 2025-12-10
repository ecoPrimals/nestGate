//! **STORAGE BACKENDS MODULE**
//!
//! This module provides implementations of the `ZeroCostZfsOperations` trait
//! for various cloud and distributed storage backends.
//!
//! ## Backends
//!
//! - **S3**: AWS S3 and S3-compatible storage (MinIO, Ceph, etc.)
//! - **Azure**: Azure Blob Storage
//! - **GCS**: Google Cloud Storage  
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
//! use nestgate_zfs::backends::s3::S3Backend;
//!
//! // Create S3-backed storage
//! let backend = S3Backend::new(config).await?;
//! let pool = backend.create_pool("tank", &[]).await?;
//! ```

pub mod azure;
pub mod gcs;
pub mod s3;

// Re-exports
pub use azure::AzureBackend;
pub use gcs::GcsBackend;
pub use s3::S3Backend;

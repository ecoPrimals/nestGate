///
/// This module contains all type definitions for the MCP system including
/// authentication types, provider types, and storage types.
pub mod auth;
pub mod provider;
pub mod storage;

// Re-export commonly used types
pub use auth::{AuthConfig, AuthCredentials, AuthMethod, TlsConfig};
pub use provider::{ProviderCapabilities, ProviderConfig};
pub use storage::{
    MountInfo, MountOptions, MountRequest, MountStatus, NfsVersion, SmbVersion, StorageCapacity,
    StorageMetrics, StorageProtocol, StorageTier, VolumeInfo,
};

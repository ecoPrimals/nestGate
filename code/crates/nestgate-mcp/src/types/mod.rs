// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! MCP type definitions for authentication, providers, and storage.
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

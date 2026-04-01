// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! # Universal Agnostic Storage
//!
//! Pure protocol-based storage abstraction with zero vendor coupling.
//!
//! ## Philosophy
//!
//! Instead of thinking in vendor terms (S3, Azure, GCS), we think in protocol terms:
//! - What transport does it use? (HTTP, TCP, QUIC)
//! - What operations does it support? (Object, Block, File)
//! - How does authentication work? (Signed, Bearer, OAuth)
//! - What features does it have? (Discovered at runtime)
//!
//! This allows us to work with:
//! - Any existing storage system that speaks a compatible protocol
//! - Future storage systems that don't exist yet
//! - Zero code changes when switching providers
//!
//! ## Example
//!
//! ```rust,ignore
//! // Discover storage from endpoint
//! let protocol = UniversalStorageDiscovery::discover("https://storage.example.com").await?;
//!
//! // Protocol describes what it IS, not who provides it
//! // - Transport: HTTP/1.1 + TLS
//! // - Operations: Object store (path-based)
//! // - Auth: Signed headers (HMAC-SHA256)
//! // - Features: [Versioning, Range, Parallel, ...]
//!
//! // Create universal adapter
//! let adapter = UniversalStorageAdapter::new(endpoint, protocol);
//!
//! // Works with ANY storage that matches this protocol!
//! ```

pub mod adapter;
pub mod authentication;
pub mod discovery;
pub mod features;
pub mod operations;
pub mod protocol;
pub mod transport;

// Re-export all public types for convenience
pub use adapter::UniversalStorageAdapter;
pub use authentication::{
    ApiKeyLocation, AuthenticationPattern, OAuthGrantType, SecretString, SigningAlgorithm,
};
pub use discovery::{DiscoveredStorage, UniversalStorageDiscovery};
pub use features::{FeatureSet, StorageFeature};
pub use operations::{ObjectAddressing, ObjectOrganization, StorageOperationPattern};
pub use protocol::{ApiInfo, DiscoveredProtocol, PerformanceInfo};
pub use transport::{HttpVersion, TlsConfig, TransportProtocol};

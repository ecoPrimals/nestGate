//! # Storage Features
//!
//! Defines storage capabilities that can be discovered at runtime.

use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Universal storage feature (vendor-agnostic)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StorageFeature {
    // ==================== Core Operations ====================
    /// Read objects/files
    Read,

    /// Write/create objects/files
    Write,

    /// Delete objects/files
    Delete,

    /// List objects/files
    List,

    /// Check if object/file exists
    Exists,

    // ==================== Metadata ====================
    /// Custom metadata on objects
    CustomMetadata,

    /// Content-Type support
    ContentType,

    /// Content-Encoding support
    ContentEncoding,

    /// Checksum/integrity verification
    Checksums {
        /// Supported algorithms
        algorithms: Vec<ChecksumAlgorithm>,
    },

    // ==================== Versioning & History ====================
    /// Object versioning
    Versioning,

    /// List object versions
    VersionList,

    /// Revert to previous version
    VersionRevert,

    /// Point-in-time recovery
    PointInTimeRecovery,

    // ==================== Data Management ====================
    /// Lifecycle policies
    Lifecycle,

    /// Automatic expiration
    Expiration,

    /// Retention policies
    Retention,

    /// Immutable storage (WORM)
    ImmutableStorage,

    // ==================== Performance ====================
    /// Parallel operations support
    ParallelOperations {
        /// Maximum concurrent operations
        max_concurrent: usize,
    },

    /// Range requests (partial reads)
    RangeRequests,

    /// Partial reads/writes
    PartialOperations,

    /// Streaming support
    Streaming,

    /// Server-side compression
    Compression {
        /// Supported algorithms
        algorithms: Vec<CompressionAlgorithm>,
    },

    // ==================== Security ====================
    /// Encryption at rest
    EncryptionAtRest {
        /// Encryption algorithms
        algorithms: Vec<EncryptionAlgorithm>,
    },

    /// Encryption in transit
    EncryptionInTransit,

    /// Access control
    AccessControl {
        /// Granularity level
        granularity: AccessControlGranularity,
    },

    /// Audit logging
    AuditLog,

    // ==================== Advanced ====================
    /// Atomic operations
    AtomicOperations,

    /// Transactional operations
    TransactionalOperations,

    /// Event notifications
    EventNotifications {
        /// Notification types
        event_types: Vec<String>,
    },

    /// Replication
    Replication {
        /// Replication modes
        modes: Vec<ReplicationMode>,
    },

    // ==================== Discovery ====================
    /// Can self-report capabilities
    CapabilityDiscovery,

    // ==================== Custom ====================
    /// Custom feature
    Custom {
        /// Feature name
        name: String,
        /// Feature description
        description: String,
    },
}

/// Checksum algorithm
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ChecksumAlgorithm {
    /// MD5 (legacy, fast but not secure)
    Md5,

    /// SHA-1 (legacy, deprecated)
    Sha1,

    /// SHA-256
    Sha256,

    /// SHA-512
    Sha512,

    /// Blake3 (modern, fast)
    Blake3,

    /// xxHash (very fast, not cryptographic)
    XxHash,

    /// CRC32
    Crc32,

    /// Custom algorithm
    Custom(String),
}

/// Compression algorithm
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CompressionAlgorithm {
    /// GZIP
    Gzip,

    /// Brotli
    Brotli,

    /// Zstandard
    Zstd,

    /// LZ4
    Lz4,

    /// Snappy
    Snappy,

    /// Custom algorithm
    Custom(String),
}

/// Encryption algorithm
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EncryptionAlgorithm {
    /// AES-256-GCM
    Aes256Gcm,

    /// AES-256-CBC
    Aes256Cbc,

    /// ChaCha20-Poly1305
    ChaCha20Poly1305,

    /// Custom algorithm
    Custom(String),
}

/// Access control granularity
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AccessControlGranularity {
    /// No access control
    None,

    /// Bucket/container level
    Container,

    /// Object/file level
    Object,

    /// Attribute level (specific fields)
    Attribute,
}

/// Replication mode
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ReplicationMode {
    /// Synchronous replication
    Synchronous,

    /// Asynchronous replication
    Asynchronous,

    /// Cross-region replication
    CrossRegion,

    /// Cross-cloud replication
    CrossCloud,
}

/// Feature set - collection of features
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FeatureSet {
    features: HashSet<StorageFeature>,
}

impl FeatureSet {
    /// Create empty feature set
    pub fn new() -> Self {
        Self {
            features: HashSet::new(),
        }
    }

    /// Add a feature
    pub fn add(&mut self, feature: StorageFeature) {
        self.features.insert(feature);
    }

    /// Check if feature is supported
    pub fn has(&self, feature: &StorageFeature) -> bool {
        self.features.contains(feature)
    }

    /// Check if feature type is supported (ignoring parameters)
    pub fn has_feature_type(&self, check: impl Fn(&StorageFeature) -> bool) -> bool {
        self.features.iter().any(check)
    }

    /// Get all features
    pub fn features(&self) -> &HashSet<StorageFeature> {
        &self.features
    }

    /// Number of features
    pub fn len(&self) -> usize {
        self.features.len()
    }

    /// Is empty?
    pub fn is_empty(&self) -> bool {
        self.features.is_empty()
    }
}

impl From<HashSet<StorageFeature>> for FeatureSet {
    fn from(features: HashSet<StorageFeature>) -> Self {
        Self { features }
    }
}

impl IntoIterator for FeatureSet {
    type Item = StorageFeature;
    type IntoIter = std::collections::hash_set::IntoIter<StorageFeature>;

    fn into_iter(self) -> Self::IntoIter {
        self.features.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature_set_basic() {
        let mut features = FeatureSet::new();
        assert!(features.is_empty());

        features.add(StorageFeature::Read);
        features.add(StorageFeature::Write);

        assert_eq!(features.len(), 2);
        assert!(features.has(&StorageFeature::Read));
        assert!(features.has(&StorageFeature::Write));
        assert!(!features.has(&StorageFeature::Delete));
    }

    #[test]
    fn test_feature_with_parameters() {
        let mut features = FeatureSet::new();
        features.add(StorageFeature::Checksums {
            algorithms: vec![ChecksumAlgorithm::Sha256, ChecksumAlgorithm::Blake3],
        });

        // Check if has checksums feature (any variant)
        let has_checksums =
            features.has_feature_type(|f| matches!(f, StorageFeature::Checksums { .. }));
        assert!(has_checksums);
    }
}

// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! # Storage Operation Patterns
//!
//! Defines what kind of storage operations are supported, independent of vendor.

use serde::{Deserialize, Serialize};

/// What kind of storage operations does this support?
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StorageOperationPattern {
    /// Object storage pattern (GET/PUT/DELETE objects by key)
    ///
    /// Examples: S3-like, Azure Blob-like, any HTTP object storage
    ObjectStore {
        /// How are objects addressed?
        addressing: ObjectAddressing,
        /// How are objects organized?
        organization: ObjectOrganization,
    },

    /// Block storage pattern (read/write fixed-size blocks)
    ///
    /// Examples: EBS-like, Azure Disks, iSCSI
    BlockStore {
        /// Block size in bytes
        block_size: usize,
        /// Block addressing scheme
        addressing: BlockAddressing,
    },

    /// File system pattern (hierarchical files and directories)
    ///
    /// Examples: NFS, SMB, `WebDAV`, local filesystem
    FileSystem {
        /// Path separator character
        path_separator: char,
        /// Is the filesystem case-sensitive?
        case_sensitive: bool,
    },

    /// Key-value pattern (simple get/put/delete)
    ///
    /// Examples: Redis, Memcached, etcd
    KeyValue {
        /// Key format requirements
        key_format: KeyFormat,
    },

    /// Document storage pattern (store/query documents)
    ///
    /// Examples: `MongoDB`, `CouchDB`, `DocumentDB`
    Document {
        /// Query capabilities
        query_capabilities: QueryCapabilities,
    },

    /// Stream storage pattern (append-only logs)
    ///
    /// Examples: Kafka, Kinesis, `EventStore`
    Stream {
        /// Ordering guarantees
        ordering: StreamOrdering,
    },
}

/// How are objects addressed in object storage?
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ObjectAddressing {
    /// Path-based: /bucket/path/to/object
    ///
    /// Example: <https://example.com/bucket/key>
    PathBased,

    /// Subdomain-based: bucket.storage.example.com/path/to/object
    ///
    /// Example: <https://bucket.storage.example.com/key>
    SubdomainBased,

    /// Query-based: /object?bucket=name&key=path
    ///
    /// Example: <https://example.com/api?bucket=mybucket&key=mykey>
    QueryBased,

    /// Header-based: Object location specified in HTTP headers
    HeaderBased {
        /// Header names that specify location
        location_headers: Vec<String>,
    },
}

/// How are objects organized?
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ObjectOrganization {
    /// Flat namespace (all objects at same level)
    Flat,

    /// Hierarchical with path separators
    Hierarchical {
        /// Separator character (usually '/')
        separator: char,
    },

    /// Prefix-based grouping
    PrefixBased,

    /// Tag-based organization
    TagBased,
}

/// Block addressing scheme
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BlockAddressing {
    /// Sequential block numbers (0, 1, 2, ...)
    Sequential,

    /// Logical block addressing
    Lba,

    /// Custom addressing scheme
    Custom(String),
}

/// Key format requirements
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct KeyFormat {
    /// Maximum key length
    pub max_length: Option<usize>,

    /// Allowed characters
    pub allowed_chars: KeyCharSet,

    /// Case sensitivity
    pub case_sensitive: bool,
}

impl Default for KeyFormat {
    fn default() -> Self {
        Self {
            max_length: Some(1024),
            allowed_chars: KeyCharSet::Alphanumeric,
            case_sensitive: true,
        }
    }
}

/// Allowed character set for keys
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum KeyCharSet {
    /// Only alphanumeric characters
    Alphanumeric,

    /// Alphanumeric plus common symbols
    AlphanumericExtended,

    /// Any UTF-8 characters
    Utf8,

    /// Any bytes
    Binary,
}

/// Query capabilities for document storage
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct QueryCapabilities {
    /// Supports full-text search
    pub full_text_search: bool,

    /// Supports aggregation queries
    pub aggregation: bool,

    /// Supports joins
    pub joins: bool,

    /// Supports transactions
    pub transactions: bool,
}

/// Stream ordering guarantees
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StreamOrdering {
    /// No ordering guaranteed
    Unordered,

    /// Ordered within a partition/shard
    PartitionOrdered,

    /// Globally ordered
    GloballyOrdered,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_object_store_pattern() {
        let pattern = StorageOperationPattern::ObjectStore {
            addressing: ObjectAddressing::PathBased,
            organization: ObjectOrganization::Hierarchical { separator: '/' },
        };

        match pattern {
            StorageOperationPattern::ObjectStore {
                addressing,
                organization,
            } => {
                assert_eq!(addressing, ObjectAddressing::PathBased);
                assert!(matches!(
                    organization,
                    ObjectOrganization::Hierarchical { separator: '/' }
                ));
            }
            _ => panic!("Wrong pattern type"),
        }
    }

    #[test]
    fn test_key_format_defaults() {
        let format = KeyFormat::default();
        assert_eq!(format.max_length, Some(1024));
        assert!(format.case_sensitive);
    }
}

// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! **Data Classification & Strategies**
//!
//! Domain: Data classification, storage strategies, and recommendations
//!
//! This module handles:
//! - Data classification (content type, category, patterns)
//! - Storage tier recommendations
//! - Compression strategies
//! - Replication strategies
//! - Access pattern prediction

use serde::{Deserialize, Serialize};

/// Data classification
///
/// Complete classification of data for optimal storage and access.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataClassification {
    /// Content type classification
    pub content_type: ContentType,
    /// Data category (criticality)
    pub data_category: DataCategory,
    /// Predicted access pattern
    pub access_pattern: PredictedAccessPattern,
    /// Recommended storage tier
    pub storage_tier: RecommendedTier,
    /// Recommended compression strategy
    pub compression_strategy: CompressionStrategy,
    /// Recommended replication strategy
    pub replication_strategy: ReplicationStrategy,
}

/// Content types
///
/// Classification of data by content type.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContentType {
    /// Text content (documents, logs, etc.)
    Text,
    /// Binary content (executables, compiled code)
    Binary,
    /// Structured data (JSON, XML, databases)
    Structured,
    /// Multimedia content (images, video, audio)
    Multimedia,
    /// Scientific data (research, experiments)
    Scientific,
    /// Source code
    Code,
    /// Unknown or unclassified
    Unknown,
}

/// Data categories
///
/// Classification by importance and criticality.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataCategory {
    /// Critical data (cannot be lost)
    Critical,
    /// Important data (high value)
    Important,
    /// Standard data (normal operations)
    Standard,
    /// Archival data (long-term storage)
    Archive,
    /// Temporary data (can be deleted)
    Temporary,
}

/// Predicted access patterns
///
/// Expected access patterns for optimal storage placement.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PredictedAccessPattern {
    /// Frequently accessed (hot data)
    Frequent,
    /// Moderately accessed
    Moderate,
    /// Infrequently accessed (cold data)
    Infrequent,
    /// Write-once, read-many (WORM)
    WriteOnce,
    /// Streaming access pattern
    Streaming,
    /// Batch processing pattern
    Batch,
}

/// Recommended storage tiers
///
/// Storage tier recommendations based on access patterns and criticality.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecommendedTier {
    /// Hot tier (frequent access, fast storage)
    Hot,
    /// Warm tier (moderate access)
    Warm,
    /// Cold tier (infrequent access)
    Cold,
    /// Archive tier (long-term storage)
    Archive,
    /// DNA storage tier (experimental/future)
    Dna,
    /// Quantum storage tier (future)
    Quantum,
}

/// Compression strategies
///
/// Compression approach recommendations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CompressionStrategy {
    /// No compression
    None,
    /// Fast compression (low CPU, moderate ratio)
    Fast,
    /// Balanced compression (moderate CPU, good ratio)
    Balanced,
    /// Maximum compression (high CPU, best ratio)
    Maximum,
    /// Specialized compression algorithm
    Specialized(String),
}

/// Replication strategies
///
/// Data replication approach for durability and availability.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReplicationStrategy {
    /// No replication
    None,
    /// Local replication (same datacenter)
    Local,
    /// Geographic replication (multiple regions)
    Geographic,
    /// Cross-technology replication (multiple storage types)
    CrossTechnology,
    /// Quantum-level replication (future)
    Quantum,
}

impl DataClassification {
    /// Create classification for critical data
    ///
    /// # Returns
    ///
    /// Classification optimized for critical data
    #[must_use]
    pub const fn critical() -> Self {
        Self {
            content_type: ContentType::Unknown,
            data_category: DataCategory::Critical,
            access_pattern: PredictedAccessPattern::Frequent,
            storage_tier: RecommendedTier::Hot,
            compression_strategy: CompressionStrategy::Fast,
            replication_strategy: ReplicationStrategy::Geographic,
        }
    }

    /// Create classification for archival data
    ///
    /// # Returns
    ///
    /// Classification optimized for archival storage
    #[must_use]
    pub const fn archival() -> Self {
        Self {
            content_type: ContentType::Unknown,
            data_category: DataCategory::Archive,
            access_pattern: PredictedAccessPattern::Infrequent,
            storage_tier: RecommendedTier::Archive,
            compression_strategy: CompressionStrategy::Maximum,
            replication_strategy: ReplicationStrategy::CrossTechnology,
        }
    }

    /// Create classification for temporary data
    ///
    /// # Returns
    ///
    /// Classification for temporary/ephemeral data
    #[must_use]
    pub const fn temporary() -> Self {
        Self {
            content_type: ContentType::Unknown,
            data_category: DataCategory::Temporary,
            access_pattern: PredictedAccessPattern::Frequent,
            storage_tier: RecommendedTier::Hot,
            compression_strategy: CompressionStrategy::None,
            replication_strategy: ReplicationStrategy::None,
        }
    }

    /// Update content type
    ///
    /// # Arguments
    ///
    /// * `content_type` - New content type
    pub const fn set_content_type(&mut self, content_type: ContentType) {
        self.content_type = content_type;
    }

    /// Check if data should be replicated
    ///
    /// # Returns
    ///
    /// `true` if replication is recommended
    #[must_use]
    pub const fn should_replicate(&self) -> bool {
        !matches!(self.replication_strategy, ReplicationStrategy::None)
    }

    /// Check if data should be compressed
    ///
    /// # Returns
    ///
    /// `true` if compression is recommended
    #[must_use]
    pub const fn should_compress(&self) -> bool {
        !matches!(self.compression_strategy, CompressionStrategy::None)
    }

    /// Get estimated compression ratio
    ///
    /// # Returns
    ///
    /// Estimated compression ratio (1.0 = no compression)
    #[must_use]
    pub const fn estimated_compression_ratio(&self) -> f32 {
        match &self.compression_strategy {
            CompressionStrategy::None => 1.0,
            CompressionStrategy::Fast => 2.0,
            CompressionStrategy::Balanced => 3.0,
            CompressionStrategy::Maximum => 5.0,
            CompressionStrategy::Specialized(_) => 4.0,
        }
    }
}

impl Default for DataClassification {
    fn default() -> Self {
        Self {
            content_type: ContentType::Unknown,
            data_category: DataCategory::Standard,
            access_pattern: PredictedAccessPattern::Moderate,
            storage_tier: RecommendedTier::Warm,
            compression_strategy: CompressionStrategy::Balanced,
            replication_strategy: ReplicationStrategy::Local,
        }
    }
}

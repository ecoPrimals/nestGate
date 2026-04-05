// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! **Data Types & Descriptions**
//!
//! Domain: Data type classification and description
//!
//! This module handles:
//! - Data descriptors (metadata about datasets)
//! - Data type classification (Genomic, AI/ML, Legacy, etc.)
//! - Model and dataset type specifications
//! - Cross-domain data type support

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::access_control::AccessRequirements;

/// Data descriptor
///
/// Describes a dataset or data artifact with metadata and access requirements.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataDescriptor {
    /// Unique identifier for this data
    pub id: String,
    /// Classification of data type
    pub data_type: DataType,
    /// Size in bytes
    pub size_bytes: u64,
    /// Source location (path, URL, etc.)
    pub source_location: String,
    /// Additional metadata key-value pairs
    pub metadata: HashMap<String, String>,
    /// Access requirements for this data
    pub access_requirements: AccessRequirements,
}

/// Data types spanning all domains from genomics to AI/ML
///
/// Comprehensive classification system supporting:
/// - Genomic data (sequences, variants, annotations)
/// - AI/ML data (models, datasets, weights)
/// - Legacy data (files, images, applications)
/// - Research data (publications, experiments)
/// - General data (documents, media, archives)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataType {
    // ==================== Genomic Data ====================
    /// Complete genome sequences
    Genome,
    /// DNA/RNA sequence data
    Sequence,
    /// Genetic variants and mutations
    Variants,
    /// Genomic annotations and metadata
    Annotations,

    // ==================== AI/ML Data ====================
    /// Machine learning model with type specification
    Model(ModelType),
    /// Training/test dataset with type specification
    Dataset(DatasetType),
    /// Neural network weights and parameters
    Weights,
    /// Model configuration and hyperparameters
    Configuration,

    // ==================== Legacy Data ====================
    /// Legacy files from older systems
    LegacyFiles,
    /// Operating system images and backups
    SystemImages,
    /// Application binaries and packages
    Applications,

    // ==================== Research Data ====================
    /// Scientific publications and papers
    Publications,
    /// Raw experimental data
    ExperimentalData,
    /// Simulation results and outputs
    Simulations,

    // ==================== General Data ====================
    /// General documents (PDF, Word, etc.)
    Documents,
    /// Media files (images, video, audio)
    Media,
    /// Compressed archives (zip, tar, etc.)
    Archives,
    /// Unknown or unclassified data type
    Unknown,
}

/// Model types for AI/ML models
///
/// Classification of machine learning model types.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ModelType {
    /// Language models (LLMs, transformers, etc.)
    Language,
    /// Computer vision models (CNNs, object detection, etc.)
    Vision,
    /// Audio processing models (speech recognition, TTS, etc.)
    Audio,
    /// Multimodal models (text+image, CLIP, etc.)
    Multimodal,
    /// Reinforcement learning models
    Reinforcement,
    /// Custom model type with name
    Custom(String),
}

/// Dataset types
///
/// Classification of datasets by purpose and usage.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DatasetType {
    /// Training dataset
    Training,
    /// Validation dataset
    Validation,
    /// Test dataset
    Test,
    /// Benchmark dataset
    Benchmark,
    /// Synthetically generated dataset
    Synthetic,
    /// Real-world collected dataset
    RealWorld,
}

impl DataDescriptor {
    /// Create a new data descriptor
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier
    /// * `data_type` - Classification of data
    /// * `size_bytes` - Size in bytes
    /// * `source_location` - Location string
    ///
    /// # Returns
    ///
    /// New data descriptor with default access requirements
    #[must_use]
    pub fn new(id: String, data_type: DataType, size_bytes: u64, source_location: String) -> Self {
        Self {
            id,
            data_type,
            size_bytes,
            source_location,
            metadata: HashMap::new(),
            access_requirements: AccessRequirements::default(),
        }
    }

    /// Add metadata entry
    ///
    /// # Arguments
    ///
    /// * `key` - Metadata key
    /// * `value` - Metadata value
    pub fn add_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
    }

    /// Get metadata value
    ///
    /// # Arguments
    ///
    /// * `key` - Metadata key to look up
    ///
    /// # Returns
    ///
    /// Optional reference to metadata value
    #[must_use]
    pub fn get_metadata(&self, key: &str) -> Option<&String> {
        self.metadata.get(key)
    }

    /// Check if data is genomic type
    ///
    /// # Returns
    ///
    /// `true` if data type is genomic-related
    #[must_use]
    pub const fn is_genomic(&self) -> bool {
        matches!(
            self.data_type,
            DataType::Genome | DataType::Sequence | DataType::Variants | DataType::Annotations
        )
    }

    /// Check if data is AI/ML type
    ///
    /// # Returns
    ///
    /// `true` if data type is AI/ML-related
    #[must_use]
    pub const fn is_ml(&self) -> bool {
        matches!(
            self.data_type,
            DataType::Model(_) | DataType::Dataset(_) | DataType::Weights | DataType::Configuration
        )
    }

    /// Get human-readable size string
    ///
    /// # Returns
    ///
    /// Formatted size string (e.g., "1.5 GB")
    #[must_use]
    #[expect(clippy::cast_precision_loss)] // Approximate human-readable units only
    pub fn size_human_readable(&self) -> String {
        const KB: u64 = 1024;
        const MB: u64 = KB * 1024;
        const GB: u64 = MB * 1024;
        const TB: u64 = GB * 1024;

        if self.size_bytes >= TB {
            format!("{:.2} TB", self.size_bytes as f64 / TB as f64)
        } else if self.size_bytes >= GB {
            format!("{:.2} GB", self.size_bytes as f64 / GB as f64)
        } else if self.size_bytes >= MB {
            format!("{:.2} MB", self.size_bytes as f64 / MB as f64)
        } else if self.size_bytes >= KB {
            format!("{:.2} KB", self.size_bytes as f64 / KB as f64)
        } else {
            format!("{} bytes", self.size_bytes)
        }
    }
}

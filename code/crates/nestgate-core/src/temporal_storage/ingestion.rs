//! **Data Ingestion & Validation**
//!
//! Domain: Data ingestion, metadata tracking, and validation
//!
//! This module handles:
//! - Ingested data representation
//! - Ingestion metadata and tracking
//! - Validation status and results
//! - Checksum verification

use serde::{Deserialize, Serialize};

use super::classification::DataClassification;
use super::data_types::DataDescriptor;

/// Ingested data
///
/// Represents data that has been ingested from a source,
/// including content, metadata, and classification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IngestedData {
    /// Unique data identifier
    pub data_id: String,
    /// Original descriptor from source
    pub original_descriptor: DataDescriptor,
    /// Raw content bytes
    pub content: Vec<u8>,
    /// Ingestion metadata and tracking
    pub ingestion_metadata: IngestionMetadata,
    /// Optional data classification
    pub classification: Option<DataClassification>,
}

/// Ingestion metadata
///
/// Tracks metadata about the ingestion process.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IngestionMetadata {
    /// When the data was ingested
    pub ingestion_time: chrono::DateTime<chrono::Utc>,
    /// Checksum of source data for verification
    pub source_checksum: String,
    /// Compression algorithm applied (if any)
    pub compression_applied: Option<String>,
    /// Validation status of ingested data
    pub validation_status: ValidationStatus,
}

/// Validation status for ingested data
///
/// Represents the result of validation checks.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ValidationStatus {
    /// Data passed all validation checks
    Valid,
    /// Data failed validation with error message
    Invalid(String),
    /// Data has not been validated yet
    Unvalidated,
    /// Data partially valid with list of issues
    PartiallyValid(Vec<String>),
}

impl IngestedData {
    /// Create new ingested data
    ///
    /// # Arguments
    ///
    /// * `data_id` - Unique identifier
    /// * `descriptor` - Original data descriptor
    /// * `content` - Raw data bytes
    /// * `checksum` - Source checksum
    ///
    /// # Returns
    ///
    /// New ingested data with unvalidated status
    pub fn new(
        data_id: String,
        descriptor: DataDescriptor,
        content: Vec<u8>,
        checksum: String,
    ) -> Self {
        Self {
            data_id,
            original_descriptor: descriptor,
            content,
            ingestion_metadata: IngestionMetadata {
                ingestion_time: chrono::Utc::now(),
                source_checksum: checksum,
                compression_applied: None,
                validation_status: ValidationStatus::Unvalidated,
            },
            classification: None,
        }
    }

    /// Mark data as valid
    pub fn mark_valid(&mut self) {
        self.ingestion_metadata.validation_status = ValidationStatus::Valid;
    }

    /// Mark data as invalid
    ///
    /// # Arguments
    ///
    /// * `error` - Validation error message
    pub fn mark_invalid(&mut self, error: String) {
        self.ingestion_metadata.validation_status = ValidationStatus::Invalid(error);
    }

    /// Mark data as partially valid
    ///
    /// # Arguments
    ///
    /// * `issues` - List of validation issues
    pub fn mark_partially_valid(&mut self, issues: Vec<String>) {
        self.ingestion_metadata.validation_status = ValidationStatus::PartiallyValid(issues);
    }

    /// Set compression information
    ///
    /// # Arguments
    ///
    /// * `algorithm` - Compression algorithm name
    pub fn set_compression(&mut self, algorithm: String) {
        self.ingestion_metadata.compression_applied = Some(algorithm);
    }

    /// Set data classification
    ///
    /// # Arguments
    ///
    /// * `classification` - Data classification
    pub fn set_classification(&mut self, classification: DataClassification) {
        self.classification = Some(classification);
    }

    /// Check if data is valid
    ///
    /// # Returns
    ///
    /// `true` if validation status is Valid
    pub fn is_valid(&self) -> bool {
        matches!(
            self.ingestion_metadata.validation_status,
            ValidationStatus::Valid
        )
    }

    /// Check if data has been validated
    ///
    /// # Returns
    ///
    /// `true` if validation has been performed
    pub fn is_validated(&self) -> bool {
        !matches!(
            self.ingestion_metadata.validation_status,
            ValidationStatus::Unvalidated
        )
    }

    /// Get content size
    ///
    /// # Returns
    ///
    /// Size of content in bytes
    pub fn content_size(&self) -> usize {
        self.content.len()
    }
}

impl ValidationStatus {
    /// Check if status indicates validity
    ///
    /// # Returns
    ///
    /// `true` if Valid or PartiallyValid
    pub fn is_acceptable(&self) -> bool {
        matches!(self, Self::Valid | Self::PartiallyValid(_))
    }

    /// Get error message if invalid
    ///
    /// # Returns
    ///
    /// Error message for Invalid status, None otherwise
    pub fn error_message(&self) -> Option<&str> {
        match self {
            Self::Invalid(msg) => Some(msg),
            _ => None,
        }
    }

    /// Get validation issues
    ///
    /// # Returns
    ///
    /// Vector of issues for PartiallyValid, empty otherwise
    pub fn issues(&self) -> Vec<String> {
        match self {
            Self::PartiallyValid(issues) => issues.clone(),
            Self::Invalid(msg) => vec![msg.clone()],
            _ => Vec::new(),
        }
    }
}

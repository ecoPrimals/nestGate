// Universal Data Capabilities
//! Data Capabilities functionality and utilities.
// Defines what NestGate can do with data, not where it comes from.
// Any external system that can provide these capabilities can integrate.

use crate::{NestGateError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Universal data request
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Request parameters for Data operation
pub struct DataRequest {
    /// What type of data is being requested
    pub capability_type: String,
    /// Query parameters (provider-agnostic)
    pub parameters: HashMap<String, serde_json::Value>,
    /// Optional metadata
    pub metadata: HashMap<String, String>,
}
/// Universal data response
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Response data for Data operation
pub struct DataResponse {
    /// The requested data
    pub data: serde_json::Value,
    /// Response metadata
    pub metadata: HashMap<String, String>,
    /// Source information (for attribution)
    pub source_info: Option<SourceInfo>,
}
/// Source information (for attribution only)
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Sourceinfo
pub struct SourceInfo {
    /// Provider type (e.g., "genome_database", "model_repository")
    pub provider_type: String,
    /// Optional provider name (for attribution)
    pub provider_name: Option<String>,
    /// Data license/terms
    pub license: Option<String>,
}
/// Universal data capability trait
pub trait DataCapability: Send + Sync {
    /// What type of data this capability provides
    fn capability_type(&self) -> &str;
    
    /// Check if this capability can handle a specific request
    fn can_handle(&self, request: &DataRequest) -> impl std::future::Future<Output = Result<bool>> + Send;
    
    /// Execute a data request
    fn execute_request(&self, request: &DataRequest) -> impl std::future::Future<Output = Result<DataResponse>> + Send;
    
    /// Get capability metadata
    fn get_metadata(&self) -> HashMap<String, String>;
}
/// Genome data capability (for any genome database)
pub trait GenomeDataCapability: DataCapability {
    /// Search for genome sequences
    fn search_genomes(&self, query: &str) -> impl std::future::Future<Output = Result<Vec<GenomeResult>> + Send;
    
    /// Get genome sequence by ID
    fn get_genome_sequence(&self, genome_id: &str) -> impl std::future::Future<Output = Result<GenomeSequence>> + Send;
}
/// Model data capability (for any AI model repository)
pub trait ModelDataCapability: DataCapability {
    /// Search for models
    fn search_models(&self, query: &str) -> impl std::future::Future<Output = Result<Vec<ModelResult>> + Send;
    
    /// Get model information
    fn get_model_info(&self, model_id: &str) -> impl std::future::Future<Output = Result<ModelInfo>> + Send;
}
/// Research data capability (for any research database)
pub trait ResearchDataCapability: DataCapability {
    /// Search research papers/data
    fn search_research(&self, query: &str) -> impl std::future::Future<Output = Result<Vec<ResearchResult>> + Send;
    
    /// Get research data by ID
    fn get_research_data(&self, research_id: &str) -> impl std::future::Future<Output = Result<ResearchData>> + Send;
}
/// Generic result types (provider-agnostic)
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Genomeresult
pub struct GenomeResult {
    /// Unique identifier
    pub id: String,
    /// Title
    pub title: String,
    /// Organism
    pub organism: Option<String>,
    /// Human-readable description
    pub description: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Genomesequence
pub struct GenomeSequence {
    /// Unique identifier
    pub id: String,
    /// Sequence
    pub sequence: String,
    /// Additional metadata key-value pairs
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Modelresult
pub struct ModelResult {
    /// Unique identifier
    pub id: String,
    /// Name
    pub name: String,
    /// Model Type
    pub model_type: Option<String>,
    /// Human-readable description
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Modelinfo
pub struct ModelInfo {
    /// Unique identifier
    pub id: String,
    /// Name
    pub name: String,
    /// Parameters
    pub parameters: HashMap<String, serde_json::Value>,
    /// Additional metadata key-value pairs
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Researchresult
pub struct ResearchResult {
    /// Unique identifier
    pub id: String,
    /// Title
    pub title: String,
    /// Authors
    pub authors: Vec<String>,
    /// Abstract Text
    pub abstract_text: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Researchdata
pub struct ResearchData {
    /// Unique identifier
    pub id: String,
    /// Title
    pub title: String,
    /// Content
    pub content: serde_json::Value,
    /// Additional metadata key-value pairs
    pub metadata: HashMap<String, String>,

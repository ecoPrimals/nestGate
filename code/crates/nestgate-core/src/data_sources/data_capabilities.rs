//! Universal Data Capabilities
//!
//! Defines what NestGate can do with data, not where it comes from.
//! Any external system that can provide these capabilities can integrate.

use crate::{NestGateError, Result};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Universal data request
#[derive(Debug, Clone, Serialize, Deserialize)]
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
pub struct SourceInfo {
    /// Provider type (e.g., "genome_database", "model_repository")
    pub provider_type: String,
    /// Optional provider name (for attribution)
    pub provider_name: Option<String>,
    /// Data license/terms
    pub license: Option<String>,
}

/// Universal data capability trait
#[async_trait]
pub trait DataCapability: Send + Sync {
    /// What type of data this capability provides
    fn capability_type(&self) -> &str;
    
    /// Check if this capability can handle a specific request
    async fn can_handle(&self, request: &DataRequest) -> Result<bool>;
    
    /// Execute a data request
    async fn execute_request(&self, request: &DataRequest) -> Result<DataResponse>;
    
    /// Get capability metadata
    fn get_metadata(&self) -> HashMap<String, String>;
}

/// Genome data capability (for any genome database)
#[async_trait]
pub trait GenomeDataCapability: DataCapability {
    /// Search for genome sequences
    async fn search_genomes(&self, query: &str) -> Result<Vec<GenomeResult>>;
    
    /// Get genome sequence by ID
    async fn get_genome_sequence(&self, genome_id: &str) -> Result<GenomeSequence>;
}

/// Model data capability (for any AI model repository)
#[async_trait]
pub trait ModelDataCapability: DataCapability {
    /// Search for models
    async fn search_models(&self, query: &str) -> Result<Vec<ModelResult>>;
    
    /// Get model information
    async fn get_model_info(&self, model_id: &str) -> Result<ModelInfo>;
}

/// Research data capability (for any research database)
#[async_trait]
pub trait ResearchDataCapability: DataCapability {
    /// Search research papers/data
    async fn search_research(&self, query: &str) -> Result<Vec<ResearchResult>>;
    
    /// Get research data by ID
    async fn get_research_data(&self, research_id: &str) -> Result<ResearchData>;
}

/// Generic result types (provider-agnostic)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenomeResult {
    pub id: String,
    pub title: String,
    pub organism: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenomeSequence {
    pub id: String,
    pub sequence: String,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelResult {
    pub id: String,
    pub name: String,
    pub model_type: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    pub id: String,
    pub name: String,
    pub parameters: HashMap<String, serde_json::Value>,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchResult {
    pub id: String,
    pub title: String,
    pub authors: Vec<String>,
    pub abstract_text: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchData {
    pub id: String,
    pub title: String,
    pub content: serde_json::Value,
    pub metadata: HashMap<String, String>,
} 
//! NCBI Live Provider
//!
//! A real implementation that connects to NCBI's actual APIs to provide
//! genome data capabilities. This demonstrates how external databases
//! can integrate with NestGate without NestGate being coupled to NCBI.

use crate::data_sources::data_capabilities::*;
use crate::data_sources::providers::universal_http_provider::{HttpProviderConfigBuilder, UniversalHttpProvider};
use crate::{NestGateError, Result};
use async_trait::async_trait;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::sync::Arc;
use tracing::{debug, info, warn};

/// NCBI Live Provider - connects to real NCBI APIs
pub struct NCBILiveProvider {
    http_provider: UniversalHttpProvider,
    api_key: Option<String>,
    email: Option<String>, // NCBI requires email for API usage
}

impl NCBILiveProvider {
    /// Create a new NCBI provider with optional API key and email
    pub fn new(api_key: Option<String>, email: Option<String>) -> Result<Self> {
        let mut config_builder = HttpProviderConfigBuilder::new(
            "https://eutils.ncbi.nlm.nih.gov/entrez/eutils".to_string(),
            "genome_data".to_string()
        )
        .with_timeout(60) // NCBI can be slow
        .with_metadata("provider_name".to_string(), "NCBI".to_string())
        .with_metadata("provider_type".to_string(), "genome_database".to_string())
        .with_metadata("data_source".to_string(), "National Center for Biotechnology Information".to_string())
        .with_metadata("license".to_string(), "NCBI Usage Guidelines".to_string())
        .with_metadata("attribution".to_string(), "Data provided by NCBI".to_string());

        // Add API key if provided
        if let Some(ref key) = api_key {
            config_builder = config_builder.with_api_key(key.clone());
        }

        // Add required headers for NCBI
        config_builder = config_builder
            .with_header("User-Agent".to_string(), "NestGate/1.0 (Universal Data Adapter)".to_string());

        if let Some(ref email) = email {
            config_builder = config_builder
                .with_header("From".to_string(), email.clone());
        }

        let config = config_builder.build();
        let http_provider = UniversalHttpProvider::new(config)?;

        info!("🧬 Created NCBI live provider with real API connection");

        Ok(Self {
            http_provider,
            api_key,
            email,
        })
    }

    /// Search NCBI databases using ESearch
    async fn esearch(&self, database: &str, query: &str, max_results: Option<u32>) -> Result<Vec<String>> {
        let mut params = HashMap::new();
        params.insert("db".to_string(), database.to_string());
        params.insert("term".to_string(), query.to_string());
        params.insert("retmode".to_string(), "json".to_string());
        params.insert("retmax".to_string(), max_results.unwrap_or(20).to_string());

        if let Some(ref api_key) = self.api_key {
            params.insert("api_key".to_string(), api_key.clone());
        }

        if let Some(ref email) = self.email {
            params.insert("email".to_string(), email.clone());
        }

        let response = self.http_provider.get_request("esearch.fcgi", &params).await?;

        // Parse NCBI ESearch response
        let esearch_result = response
            .get("esearchresult")
            .ok_or_else(|| NestGateError::Internal {
                message: "Invalid NCBI ESearch response format".to_string(),
                location: Some("NCBILiveProvider::esearch".to_string()),
                debug_info: None,
                is_bug: false,
            })?;

        let id_list = esearch_result
            .get("idlist")
            .and_then(|v| v.as_array())
            .ok_or_else(|| NestGateError::Internal {
                message: "No ID list in NCBI response".to_string(),
                location: Some("NCBILiveProvider::esearch".to_string()),
                debug_info: None,
                is_bug: false,
            })?;

        let ids: Vec<String> = id_list
            .iter()
            .filter_map(|v| v.as_str())
            .map(|s| s.to_string())
            .collect();

        debug!("🔍 NCBI ESearch found {} results for query: {}", ids.len(), query);
        Ok(ids)
    }

    /// Fetch summaries using ESummary
    async fn esummary(&self, database: &str, ids: &[String]) -> Result<Value> {
        if ids.is_empty() {
            return Ok(json!({}));
        }

        let mut params = HashMap::new();
        params.insert("db".to_string(), database.to_string());
        params.insert("id".to_string(), ids.join(","));
        params.insert("retmode".to_string(), "json".to_string());

        if let Some(ref api_key) = self.api_key {
            params.insert("api_key".to_string(), api_key.clone());
        }

        if let Some(ref email) = self.email {
            params.insert("email".to_string(), email.clone());
        }

        let response = self.http_provider.get_request("esummary.fcgi", &params).await?;
        debug!("📄 Fetched NCBI summaries for {} IDs", ids.len());
        Ok(response)
    }

    /// Fetch sequences using EFetch
    async fn efetch(&self, database: &str, id: &str, format: &str) -> Result<Value> {
        let mut params = HashMap::new();
        params.insert("db".to_string(), database.to_string());
        params.insert("id".to_string(), id.to_string());
        params.insert("rettype".to_string(), format.to_string());
        params.insert("retmode".to_string(), "text".to_string());

        if let Some(ref api_key) = self.api_key {
            params.insert("api_key".to_string(), api_key.clone());
        }

        if let Some(ref email) = self.email {
            params.insert("email".to_string(), email.clone());
        }

        let response = self.http_provider.get_request("efetch.fcgi", &params).await?;
        debug!("🧬 Fetched NCBI sequence for ID: {}", id);
        Ok(response)
    }

    /// Parse NCBI summary data into GenomeResult
    fn parse_ncbi_summary_to_genome_result(&self, id: &str, summary: &Value) -> Option<GenomeResult> {
        let title = summary.get("title")?.as_str()?.to_string();
        
        let organism = summary.get("organism")
            .or_else(|| summary.get("orgname"))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        let description = summary.get("caption")
            .or_else(|| summary.get("extra"))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        Some(GenomeResult {
            id: id.to_string(),
            title,
            organism,
            description,
        })
    }
}

#[async_trait]
impl DataCapability for NCBILiveProvider {
    fn capability_type(&self) -> &str {
        "genome_data"
    }

    async fn can_handle(&self, request: &DataRequest) -> Result<bool> {
        // Can handle genome data requests
        Ok(request.capability_type == "genome_data")
    }

    async fn execute_request(&self, request: &DataRequest) -> Result<DataResponse> {
        debug!("🚀 Executing NCBI live request for: {}", request.capability_type);

        // Extract query parameters
        let query = request.parameters
            .get("query")
            .and_then(|v| v.as_str())
            .ok_or_else(|| NestGateError::Internal {
                message: "Missing 'query' parameter for NCBI request".to_string(),
                location: Some("NCBILiveProvider::execute_request".to_string()),
                debug_info: None,
                is_bug: false,
            })?;

        // Determine database (default to nucleotide)
        let database = request.parameters
            .get("database")
            .and_then(|v| v.as_str())
            .unwrap_or("nucleotide");

        // Determine max results
        let max_results = request.parameters
            .get("max_results")
            .and_then(|v| v.as_u64())
            .map(|n| n as u32);

        // Search NCBI
        let ids = self.esearch(database, query, max_results).await?;
        
        if ids.is_empty() {
            return Ok(DataResponse {
                data: json!({
                    "results": [],
                    "total_count": 0,
                    "query": query,
                    "database": database
                }),
                metadata: request.metadata.clone(),
                source_info: Some(SourceInfo {
                    provider_type: "genome_database".to_string(),
                    provider_name: Some("NCBI".to_string()),
                    license: Some("NCBI Usage Guidelines".to_string()),
                }),
            });
        }

        // Get summaries
        let summaries = self.esummary(database, &ids).await?;
        
        // Parse results
        let mut results = Vec::new();
        if let Some(result_obj) = summaries.get("result") {
            for id in &ids {
                if let Some(summary) = result_obj.get(id) {
                    if let Some(genome_result) = self.parse_ncbi_summary_to_genome_result(id, summary) {
                        results.push(genome_result);
                    }
                }
            }
        }

        // Create response
        let response_data = json!({
            "results": results,
            "total_count": results.len(),
            "query": query,
            "database": database,
            "provider": "NCBI",
            "api_version": "E-utilities"
        });

        Ok(DataResponse {
            data: response_data,
            metadata: request.metadata.clone(),
            source_info: Some(SourceInfo {
                provider_type: "genome_database".to_string(),
                provider_name: Some("NCBI".to_string()),
                license: Some("NCBI Usage Guidelines - https://www.ncbi.nlm.nih.gov/home/about/policies/".to_string()),
            }),
        })
    }

    fn get_metadata(&self) -> HashMap<String, String> {
        let mut metadata = HashMap::new();
        metadata.insert("provider_name".to_string(), "NCBI".to_string());
        metadata.insert("provider_type".to_string(), "genome_database".to_string());
        metadata.insert("api_base".to_string(), "https://eutils.ncbi.nlm.nih.gov/entrez/eutils".to_string());
        metadata.insert("supported_databases".to_string(), "nucleotide,protein,pubmed,sra,taxonomy".to_string());
        metadata.insert("authentication".to_string(), if self.api_key.is_some() { "api_key" } else { "none" }.to_string());
        metadata.insert("rate_limit".to_string(), "3 requests/second without API key, 10/second with API key".to_string());
        metadata.insert("attribution_required".to_string(), "true".to_string());
        metadata
    }
}

#[async_trait]
impl GenomeDataCapability for NCBILiveProvider {
    async fn search_genomes(&self, query: &str) -> Result<Vec<GenomeResult>> {
        debug!("🔍 Searching NCBI genomes with query: {}", query);

        let mut parameters = HashMap::new();
        parameters.insert("query".to_string(), json!(query));
        parameters.insert("database".to_string(), json!("nucleotide"));
        parameters.insert("max_results".to_string(), json!(20));

        let request = DataRequest {
            capability_type: "genome_data".to_string(),
            parameters,
            metadata: HashMap::new(),
        };

        let response = self.execute_request(&request).await?;
        
        // Extract results from response
        let results = response.data
            .get("results")
            .and_then(|v| v.as_array())
            .unwrap_or(&Vec::new())
            .iter()
            .filter_map(|v| serde_json::from_value::<GenomeResult>(v.clone()).ok())
            .collect();

        Ok(results)
    }

    async fn get_genome_sequence(&self, genome_id: &str) -> Result<GenomeSequence> {
        debug!("🧬 Fetching NCBI genome sequence for ID: {}", genome_id);

        // Fetch the sequence using EFetch
        let sequence_data = self.efetch("nucleotide", genome_id, "fasta").await?;
        
        // Parse FASTA format (simplified)
        let sequence_text = sequence_data.as_str()
            .unwrap_or("")
            .lines()
            .skip(1) // Skip FASTA header
            .collect::<Vec<_>>()
            .join("");

        let mut metadata = HashMap::new();
        metadata.insert("source".to_string(), "NCBI".to_string());
        metadata.insert("format".to_string(), "FASTA".to_string());
        metadata.insert("length".to_string(), sequence_text.len().to_string());

        Ok(GenomeSequence {
            id: genome_id.to_string(),
            sequence: sequence_text,
            metadata,
        })
    }
}

/// Factory for creating NCBI providers
pub struct NCBIProviderFactory;

impl NCBIProviderFactory {
    /// Create NCBI provider with API key (recommended for production)
    pub fn create_with_api_key(api_key: String, email: String) -> Result<Arc<NCBILiveProvider>> {
        Ok(Arc::new(NCBILiveProvider::new(Some(api_key), Some(email))?))
    }

    /// Create NCBI provider without API key (rate limited)
    pub fn create_basic(email: String) -> Result<Arc<NCBILiveProvider>> {
        Ok(Arc::new(NCBILiveProvider::new(None, Some(email))?))
    }

    /// Create NCBI provider from environment variables
    pub fn create_from_env() -> Result<Arc<NCBILiveProvider>> {
        let api_key = std::env::var("NCBI_API_KEY").ok();
        let email = std::env::var("NCBI_EMAIL")
            .or_else(|_| std::env::var("USER_EMAIL"))
            .or_else(|_| std::env::var("EMAIL"))
            .ok();

        if email.is_none() {
            warn!("⚠️ No email provided for NCBI API - this may cause rate limiting");
        }

        Ok(Arc::new(NCBILiveProvider::new(api_key, email)?))
    }
} 
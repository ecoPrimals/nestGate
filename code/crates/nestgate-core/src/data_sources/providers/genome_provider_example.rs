// Universal Genome Data Provider Example
//! Genome Provider Example functionality and utilities.
// This example shows how any genome database (NCBI, Ensembl, etc.)
//! can implement NestGate's genome data capabilities without NestGate
//! knowing the specific provider identity.

use crate::data_sources::data_capabilities::*;
use crate::data_sources::providers::universal_http_provider::{HttpProviderConfigBuilder, UniversalHttpProvider};
use crate::{NestGateError, Result};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::sync::Arc;
use tracing::{debug, info};

/// Universal genome data provider
/// Can adapt any genome database API (NCBI, Ensembl, etc.)
pub struct UniversalGenomeProvider {
    http_provider: UniversalHttpProvider,
    provider_name: String,
}
impl UniversalGenomeProvider {
    /// Create a new genome provider for any genome database
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub fn new(base_url: String, provider_name: String, api_key: Option<String>) -> Result<Self>  {
        let config = HttpProviderConfigBuilder::new(base_url, "genome_data".to_string())
            .with_timeout(60) // Genome queries can be slow
            .with_metadata("name".to_string(), provider_name.clone())
            .with_metadata("data_type".to_string(), "genomic_sequences".to_string())
            .with_metadata("license".to_string(), "Check provider terms".to_string());

        let config = if let Some(key) = api_key {
            config.with_api_key(key)
        } else {
            config
        }.build();

        let http_provider = UniversalHttpProvider::new(config)?;

        info!("🧬 Created universal genome provider: {}", provider_name);

        Ok(Self {
            http_provider,
            provider_name,
        })
    }

    /// Create a provider for any NCBI-compatible API
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub fn for_ncbi_compatible(base_url: String, api_key: Option<String>) -> Result<Self>  {
        Self::new(base_url, "NCBI-Compatible Genome Database".to_string(), api_key)
    }

    /// Create a provider for any Ensembl-compatible API
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub fn for_ensembl_compatible(base_url: String, api_key: Option<String>) -> Result<Self>  {
        Self::new(base_url, "Ensembl-Compatible Genome Database".to_string(), api_key)
    }

    /// Create a provider for any custom genome database
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub fn for_custom_database(base_url: String, provider_name: String, api_key: Option<String>) -> Result<Self>  {
        Self::new(base_url, provider_name, api_key)
    }

    /// Parse API response into standard genome results
    fn parse_genome_results(&self, data: &Value) -> Result<Vec<GenomeResult>> {
        let mut results = Vec::new();

        // Handle different response formats from different providers
        if let Some(array) = data.as_array() {
            // Array format (common in REST APIs)
            for item in array {
                if let Some(result) = self.parse_single_genome_result(item) {
                    results.push(result);
                }
            }
        } else if let Some(obj) = data.as_object() {
            // Object format - look for common result containers
            if let Some(items) = obj.get("results").or_else(|| obj.get("data")).or_else(|| obj.get("sequences")) {
                if let Some(array) = items.as_array() {
                    for item in array {
                        if let Some(result) = self.parse_single_genome_result(item) {
                            results.push(result);
                        }
                    }
                }
            } else {
                // Single result format
                if let Some(result) = self.parse_single_genome_result(data) {
                    results.push(result);
                }
            }
        }

        Ok(results)
    }

    /// Parse a single genome result from various formats
    fn parse_single_genome_result(&self, item: &Value) -> Option<GenomeResult> {
        let obj = item.as_object()?;

        // Try different common field names from different providers
        let id = obj.get("id")
            .or_else(|| obj.get("accession"))
            .or_else(|| obj.get("seq_id"))
            .or_else(|| obj.get("identifier"))?
            .as_str()?
            .to_string();

        let title = obj.get("title")
            .or_else(|| obj.get("name"))
            .or_else(|| obj.get("description"))
            .or_else(|| obj.get("definition"))?
            .as_str()?
            .to_string();

        let organism = obj.get("organism")
            .or_else(|| obj.get("species"))
            .or_else(|| obj.get("organism_name"))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        let description = obj.get("description")
            .or_else(|| obj.get("summary"))
            .or_else(|| obj.get("abstract"))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        Some(GenomeResult {
            id,
            title,
            organism,
            description,
        })
    }

    /// Parse genome sequence from API response
    fn parse_genome_sequence(&self, data: &Value, genome_id: &str) -> Result<GenomeSequence> {
        let obj = data.as_object().ok_or_else(|| NestGateError::internal_error(

        // Try different common field names for sequence data
        let sequence = obj.get("sequence")
            .or_else(|| obj.get("seq"))
            .or_else(|| obj.get("dna"))
            .or_else(|| obj.get("nucleotide_sequence"))
            .and_then(|v| v.as_str())
            .ok_or_else(|| NestGateError::internal_error(
            .to_string();

        // Extract metadata
        let mut metadata = HashMap::new();
        if let Some(length) = obj.get("length").or_else(|| obj.get("seq_length")) {
            metadata.insert("length".to_string(), length.to_string());
        }
        if let Some(organism) = obj.get("organism").or_else(|| obj.get("species")) {
            metadata.insert("organism".to_string(), organism.to_string());
        }
        if let Some(mol_type) = obj.get("mol_type").or_else(|| obj.get("molecule_type")) {
            metadata.insert("molecule_type".to_string(), mol_type.to_string());
        }

        Ok(GenomeSequence {
            id: genome_id.to_string(),
            sequence,
            metadata,
        })
    }
}

impl DataCapability for UniversalGenomeProvider {
    /// Capability Type
    fn capability_type(&self) -> &str {
        "genome_data"
    }

    /// Can Handle
    async fn can_handle(&self, request: &DataRequest) -> Result<bool> {
        self.http_provider.can_handle(request).await
    }

    /// Execute Request
    async fn execute_request(&self, request: &DataRequest) -> Result<DataResponse> {
        self.http_provider.execute_request(request).await
    }

    /// Gets Metadata
    fn get_metadata(&self) -> HashMap<String, String> {
        let mut metadata = self.http_provider.get_metadata();
        metadata.insert("provider_name".to_string(), self.provider_name.clone());
        metadata.insert("data_specialization".to_string(), "genomic_sequences".to_string());
        metadata
    }
}

impl GenomeDataCapability for UniversalGenomeProvider {
    /// Search Genomes
    async fn search_genomes(&self, query: &str) -> Result<Vec<GenomeResult>> {
        debug!("🔍 Searching genomes with query: {}", query);

        let mut parameters = HashMap::new();
        parameters.insert("query".to_string(), json!(query));
        parameters.insert("endpoint".to_string(), json!("search"));

        let request = DataRequest {
            capability_type: "genome_data".to_string(),
            parameters,
            metadata: HashMap::new(),
        };

        let response = self.execute_request(&request).await?;
        self.parse_genome_results(&response.data)
    }

    /// Gets Genome Sequence
    async fn get_genome_sequence(&self, genome_id: &str) -> Result<GenomeSequence> {
        debug!("🧬 Fetching genome sequence for ID: {}", genome_id);

        let mut parameters = HashMap::new();
        parameters.insert("id".to_string(), json!(genome_id));
        parameters.insert("endpoint".to_string(), json!("sequence"));

        let request = DataRequest {
            capability_type: "genome_data".to_string(),
            parameters,
            metadata: HashMap::new(),
        };

        let response = self.execute_request(&request).await?;
        self.parse_genome_sequence(&response.data, genome_id)
    }
}

/// Factory for creating genome providers for common databases
pub struct GenomeProviderFactory;
impl GenomeProviderFactory {
    /// Create a provider that can work with any NCBI-compatible API
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub fn create_ncbi_compatible(base_url: Option<String>, api_key: Option<String>) -> Result<Arc<UniversalGenomeProvider>>  {
        let base_url = base_url.unwrap_or_else(|| "https://api.ncbi.nlm.nih.gov".to_string());
        Ok(Arc::new(UniversalGenomeProvider::for_ncbi_compatible(base_url, api_key)?))
    }

    /// Create a provider that can work with any Ensembl-compatible API
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub fn create_ensembl_compatible(base_url: Option<String>) -> Result<Arc<UniversalGenomeProvider>>  {
        let base_url = base_url.unwrap_or_else(|| "https://rest.ensembl.org".to_string());
        Ok(Arc::new(UniversalGenomeProvider::for_ensembl_compatible(base_url, None)?))
    }

    /// Create a provider for any custom genome database
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub fn create_custom(base_url: String, provider_name: String, api_key: Option<String>) -> Result<Arc<UniversalGenomeProvider>>  {
        Ok(Arc::new(UniversalGenomeProvider::for_custom_database(base_url, provider_name, api_key)?))
    }

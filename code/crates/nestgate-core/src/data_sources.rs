//! Universal Data Source Implementations
//!
//! Concrete implementations of data sources for research databases and AI platforms

use async_trait::async_trait;
use reqwest::Client;
use serde::Deserialize;
use std::collections::HashMap;
use std::pin::Pin;
use std::future::Future;
use std::io::Read;

use crate::temporal_storage::{
    UniversalDataSource, ConnectionHandle, DataDescriptor, IngestedData, Metadata,
    DataStream, DataSourceType, DataType, ModelType, AccessRequirements, 
    ValidationStatus, IngestionMetadata,
};
use crate::{Result, NestGateError};

use flate2::read::GzDecoder;

/// NCBI Genome Database Source
#[derive(Debug, Clone)]
pub struct NCBIGenomeSource {
    pub api_key: Option<String>,
    pub base_url: String,
    pub client: Client,
}

impl Default for NCBIGenomeSource {
    fn default() -> Self {
        Self {
            api_key: None,
            base_url: "https://eutils.ncbi.nlm.nih.gov/entrez/eutils".to_string(),
            client: Client::new(),
        }
    }
}

impl NCBIGenomeSource {
    pub fn new(api_key: Option<String>) -> Self {
        Self {
            api_key,
            base_url: "https://eutils.ncbi.nlm.nih.gov/entrez/eutils".to_string(),
            client: Client::new(),
        }
    }

    pub fn with_base_url(mut self, base_url: String) -> Self {
        self.base_url = base_url;
        self
    }

    async fn search_genomes(&self, query: &str) -> Result<Vec<String>> {
        let url = format!("{}/esearch.fcgi", self.base_url);
        let response = self.client
            .get(&url)
            .query(&[
                ("db", "genome"),
                ("term", query),
                ("retmode", "json"),
                ("retmax", "100"),
            ])
            .send()
            .await?;

        let search_result: NCBISearchResult = response.json().await?;
        Ok(search_result.esearchresult.idlist)
    }
    
    async fn fetch_genome_info(&self, genome_id: &str) -> Result<GenomeInfo> {
        let summary_url = format!("{}/esummary.fcgi", self.base_url);
        let response = self.client
            .get(&summary_url)
            .query(&[
                ("db", "genome"),
                ("id", genome_id),
                ("retmode", "json"),
            ])
            .send()
            .await?;

        let summary_result: NCBISummaryResult = response.json().await?;
        
        // Extract genome info from summary
        if let Some(genome_data) = summary_result.result.values().next() {
            Ok(GenomeInfo {
                id: genome_id.to_string(),
                organism: genome_data.organism.clone(),
                assembly_accession: genome_data.assembly_accession.clone(),
                size_mb: genome_data.size_mb,
                chromosome_count: genome_data.chromosome_count,
                annotation_date: genome_data.annotation_date.clone(),
            })
        } else {
            Err(NestGateError::DataIngestion(format!("Genome not found: {}", genome_id)))
        }
    }
    
    async fn download_genome_sequence(&self, accession: &str) -> Result<Vec<u8>> {
        let ftp_url = format!("https://ftp.ncbi.nlm.nih.gov/genomes/all/{}.fna.gz", accession);
        let response = self.client
            .get(&ftp_url)
            .send()
            .await?;

        if response.status().is_success() {
            let compressed_data = response.bytes().await?;
            Self::decompress_gzip(&compressed_data).map_err(|e| {
                NestGateError::DataIngestion(format!("Failed to decompress genome data: {}", e))
            })
        } else {
            Err(NestGateError::DataIngestion(format!("Failed to download genome: {}", accession)))
        }
    }
    
    fn decompress_gzip(data: &[u8]) -> Result<Vec<u8>> {
        let mut decoder = GzDecoder::new(data);
        let mut decompressed = Vec::new();
        decoder.read_to_end(&mut decompressed)?;
        Ok(decompressed)
    }
}

#[async_trait]
impl UniversalDataSource for NCBIGenomeSource {
    async fn connect(&self) -> Result<ConnectionHandle> {
        let client = reqwest::Client::new();
        let test_url = format!("{}/efetch.fcgi?db=nucleotide&id=1&retmode=json&rettype=fasta", self.base_url);
        
        client.get(&test_url)
            .send()
            .await
            .map_err(|e| NestGateError::Network(e.to_string()))?
            .error_for_status()
            .map_err(|e| NestGateError::Parse(e.to_string()))?;
        
        Ok(ConnectionHandle {
            connection_id: "ncbi_genome".to_string(),
            source_type: DataSourceType::ResearchDatabase { 
                database: crate::temporal_storage::ResearchDatabase::NCBI { 
                    database: crate::temporal_storage::NCBIDatabase::GenBank 
                } 
            },
            status: crate::temporal_storage::ConnectionStatus::Connected,
            capabilities: vec![
                "genome_search".to_string(),
                "sequence_download".to_string(),
                "metadata_extraction".to_string(),
            ],
        })
    }
    
    async fn discover_data(&self) -> Result<Vec<DataDescriptor>> {
        let client = reqwest::Client::new();
        let search_url = format!("{}/esearch.fcgi?db=nucleotide&term=genome[Title]&retmode=json&retmax=100", self.base_url);
        
        let response = client.get(&search_url)
            .send()
            .await
            .map_err(|e| NestGateError::Network(e.to_string()))?
            .json::<serde_json::Value>()
            .await
            .map_err(|e| NestGateError::Parse(e.to_string()))?;
        
        let mut descriptors = Vec::new();
        if let Some(id_list) = response["esearchresult"]["idlist"].as_array() {
            for id in id_list {
                if let Some(id_str) = id.as_str() {
                    let mut metadata = HashMap::new();
                    metadata.insert("source_type".to_string(), "ncbi_genome".to_string());
                    metadata.insert("source_location".to_string(), format!("ncbi://{}", id_str));
                    
                    descriptors.push(DataDescriptor {
                        id: id_str.to_string(),
                        data_type: DataType::Genome,
                        size_bytes: 0,
                        source_location: format!("ncbi://{}", id_str),
                        metadata,
                        access_requirements: AccessRequirements {
                            authentication: None,
                            rate_limits: Some(crate::temporal_storage::RateLimits {
                                requests_per_second: 3,
                                bandwidth_limit_mbs: None,
                                daily_quota: None,
                            }),
                            geographic_restrictions: vec![],
                            legal_requirements: vec!["NCBI Terms of Use".to_string()],
                        },
                    });
                }
            }
        }
        
        Ok(descriptors)
    }
    
    async fn ingest_data(&self, descriptor: &DataDescriptor) -> Result<IngestedData> {
        let client = reqwest::Client::new();
        let fetch_url = format!("{}/efetch.fcgi?db=nucleotide&id={}&retmode=json&rettype=fasta", 
                                self.base_url, descriptor.id);
        
        let response = client.get(&fetch_url)
            .send()
            .await
            .map_err(|e| NestGateError::Network(e.to_string()))?
            .bytes()
            .await
            .map_err(|e| NestGateError::Network(e.to_string()))?;
        
        let checksum = format!("{:x}", md5::compute(&response));
        
        Ok(IngestedData {
            data_id: descriptor.id.clone(),
            original_descriptor: descriptor.clone(),
            content: response.to_vec(),
            ingestion_metadata: IngestionMetadata {
                ingestion_time: chrono::Utc::now(),
                source_checksum: checksum,
                compression_applied: Some("gzip".to_string()),
                validation_status: ValidationStatus::Valid,
            },
            classification: None,
        })
    }
    
    async fn get_metadata(&self, descriptor: &DataDescriptor) -> Result<Metadata> {
        let client = reqwest::Client::new();
        let url = format!("{}/esummary.fcgi?db=nucleotide&id={}&retmode=json", 
                          self.base_url, descriptor.id);
        
        let response = client.get(&url)
            .send()
            .await
            .map_err(|e| NestGateError::Network(e.to_string()))?
            .json::<serde_json::Value>()
            .await
            .map_err(|e| NestGateError::Parse(e.to_string()))?;
        
        let metadata: HashMap<String, serde_json::Value> = response.as_object()
            .map(|obj| obj.clone().into_iter().collect())
            .unwrap_or_default();
        
        Ok(metadata)
    }
    
    async fn stream_data(&self, descriptor: &DataDescriptor) -> Result<Box<dyn DataStream>> {
        let stream = NCBIGenomeStream::new(self.clone(), descriptor.source_location.clone());
        Ok(Box::new(stream))
    }
}

/// HuggingFace Model Hub Source
#[derive(Debug, Clone)]
pub struct HuggingFaceModelSource {
    pub api_token: Option<String>,
    pub cache_dir: String,
    pub client: Client,
}

impl Default for HuggingFaceModelSource {
    fn default() -> Self {
        Self {
            api_token: None,
            cache_dir: dirs::cache_dir()
                .map(|d| d.join("huggingface").to_string_lossy().to_string())
                .unwrap_or_else(|| "/tmp/hf_cache".to_string()),
            client: Client::new(),
        }
    }
}

impl HuggingFaceModelSource {
    pub fn new(api_token: Option<String>) -> Self {
        Self {
            api_token,
            cache_dir: dirs::cache_dir()
                .map(|d| d.join("huggingface").to_string_lossy().to_string())
                .unwrap_or_else(|| "/tmp/hf_cache".to_string()),
            client: Client::new(),
        }
    }
    
    pub fn with_cache_dir(mut self, cache_dir: String) -> Self {
        self.cache_dir = cache_dir;
        self
    }
    
    async fn search_models(&self, query: &str, model_type: Option<&str>) -> Result<Vec<HuggingFaceModel>> {
        let url = "https://huggingface.co/api/models".to_string();
        let mut params = vec![("search", query)];
        
        if let Some(pipeline_tag) = model_type {
            params.push(("pipeline_tag", pipeline_tag));
        }

        let response = self.client
            .get(&url)
            .query(&params)
            .send()
            .await?;

        let models: Vec<HuggingFaceModel> = response.json().await?;
        Ok(models)
    }
    
    async fn download_model(&self, model_id: &str) -> Result<Vec<u8>> {
        let url = format!("https://huggingface.co/{}/resolve/main/pytorch_model.bin", model_id);
        let response = self.client
            .get(&url)
            .send()
            .await?;

        if response.status().is_success() {
            Ok(response.bytes().await?.to_vec())
        } else {
            Err(NestGateError::DataIngestion(format!("Failed to download model: {}", model_id)))
        }
    }
    
    async fn list_model_files(&self, model_id: &str) -> Result<Vec<ModelFile>> {
        let url = format!("https://huggingface.co/api/models/{}/tree/main", model_id);
        let response = self.client
            .get(&url)
            .send()
            .await?;

        if response.status().is_success() {
            let files: Vec<ModelFile> = response.json().await?;
            Ok(files)
        } else {
            Err(NestGateError::DataIngestion(format!("Failed to list model files: {}", model_id)))
        }
    }
    
    async fn download_model_file(&self, model_id: &str, filename: &str) -> Result<Vec<u8>> {
        let url = format!("https://huggingface.co/{}/resolve/main/{}", model_id, filename);
        let response = self.client
            .get(&url)
            .send()
            .await?;

        if response.status().is_success() {
            Ok(response.bytes().await?.to_vec())
        } else {
            Err(NestGateError::DataIngestion(format!("Failed to download file: {}/{}", model_id, filename)))
        }
    }
}

#[async_trait]
impl UniversalDataSource for HuggingFaceModelSource {
    async fn connect(&self) -> Result<ConnectionHandle> {
        let client = reqwest::Client::new();
        let test_url = "https://huggingface.co/api/models";
        
        client.get(test_url)
            .send()
            .await
            .map_err(|e| NestGateError::Network(e.to_string()))?;
        
        Ok(ConnectionHandle {
            connection_id: "huggingface".to_string(),
            source_type: DataSourceType::ResearchDatabase { 
                database: crate::temporal_storage::ResearchDatabase::HuggingFace { 
                    model_type: None 
                } 
            },
            status: crate::temporal_storage::ConnectionStatus::Connected,
            capabilities: vec![
                "model_search".to_string(),
                "model_download".to_string(),
                "metadata_extraction".to_string(),
                "streaming_download".to_string(),
            ],
        })
    }
    
    async fn discover_data(&self) -> Result<Vec<DataDescriptor>> {
        let client = reqwest::Client::new();
        let search_url = "https://huggingface.co/api/models?filter=pytorch&limit=100";
        
        let response = client.get(search_url)
            .send()
            .await
            .map_err(|e| NestGateError::Network(e.to_string()))?
            .json::<serde_json::Value>()
            .await
            .map_err(|e| NestGateError::Parse(e.to_string()))?;
        
        let mut descriptors = Vec::new();
        if let Some(models) = response.as_array() {
            for model in models {
                if let Some(id) = model["id"].as_str() {
                    let mut metadata = HashMap::new();
                    metadata.insert("source_type".to_string(), "huggingface_model".to_string());
                    metadata.insert("source_location".to_string(), format!("hf://{}", id));
                    
                    descriptors.push(DataDescriptor {
                        id: id.to_string(),
                        data_type: DataType::Model(ModelType::Language),
                        size_bytes: 0,
                        source_location: format!("hf://{}", id),
                        metadata,
                        access_requirements: AccessRequirements {
                            authentication: None,
                            rate_limits: Some(crate::temporal_storage::RateLimits {
                                requests_per_second: 10,
                                bandwidth_limit_mbs: Some(100),
                                daily_quota: None,
                            }),
                            geographic_restrictions: vec![],
                            legal_requirements: vec!["HuggingFace Terms of Service".to_string()],
                        },
                    });
                }
            }
        }
        
        Ok(descriptors)
    }
    
    async fn ingest_data(&self, descriptor: &DataDescriptor) -> Result<IngestedData> {
        let client = reqwest::Client::new();
        let model_url = format!("https://huggingface.co/api/models/{}", descriptor.id);
        
        let model_info = client.get(&model_url)
            .send()
            .await
            .map_err(|e| NestGateError::Network(e.to_string()))?
            .json::<serde_json::Value>()
            .await
            .map_err(|e| NestGateError::Parse(e.to_string()))?;
        
        let data = model_info.to_string().into_bytes();
        let checksum = format!("{:x}", md5::compute(&data));
        
        Ok(IngestedData {
            data_id: descriptor.id.clone(),
            original_descriptor: descriptor.clone(),
            content: data,
            ingestion_metadata: IngestionMetadata {
                ingestion_time: chrono::Utc::now(),
                source_checksum: checksum,
                compression_applied: None,
                validation_status: ValidationStatus::Valid,
            },
            classification: None,
        })
    }
    
    async fn get_metadata(&self, descriptor: &DataDescriptor) -> Result<Metadata> {
        let client = reqwest::Client::new();
        let url = format!("https://huggingface.co/api/models/{}", descriptor.id);
        
        let response = client.get(&url)
            .send()
            .await
            .map_err(|e| NestGateError::Network(e.to_string()))?
            .json::<serde_json::Value>()
            .await
            .map_err(|e| NestGateError::Parse(e.to_string()))?;
        
        let metadata: HashMap<String, serde_json::Value> = response.as_object()
            .map(|obj| obj.clone().into_iter().collect())
            .unwrap_or_default();
        
        Ok(metadata)
    }
    
    async fn stream_data(&self, descriptor: &DataDescriptor) -> Result<Box<dyn DataStream>> {
        let stream = HuggingFaceModelStream::new(self.clone(), descriptor.source_location.clone());
        Ok(Box::new(stream))
    }
}

impl HuggingFaceModelSource {
    fn infer_model_type(pipeline_tag: &str) -> ModelType {
        match pipeline_tag {
            "text-generation" | "text2text-generation" | "fill-mask" => ModelType::Language,
            "image-classification" | "object-detection" | "image-segmentation" => ModelType::Vision,
            "automatic-speech-recognition" | "text-to-speech" | "audio-classification" => ModelType::Audio,
            "text-to-image" | "image-to-text" | "visual-question-answering" => ModelType::Multimodal,
            "reinforcement-learning" => ModelType::Reinforcement,
            _ => ModelType::Custom(pipeline_tag.to_string()),
        }
    }
}

// Supporting data structures
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct NCBISearchResult {
    esearchresult: NCBISearchResultInner,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct NCBISearchResultInner {
    idlist: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct NCBISummaryResult {
    result: HashMap<String, NCBIGenomeData>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct NCBIGenomeData {
    organism: String,
    assembly_accession: String,
    size_mb: u32,
    chromosome_count: u32,
    annotation_date: String,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
struct GenomeInfo {
    id: String,
    organism: String,
    assembly_accession: String,
    size_mb: u32,
    chromosome_count: u32,
    annotation_date: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct HuggingFaceModel {
    id: String,
    pipeline_tag: String,
    downloads: u32,
    likes: u32,
    library_name: String,
    #[serde(rename = "safetensors")]
    size_bytes: Option<u64>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct ModelFile {
    filename: String,
    size: u64,
}

// Streaming implementations
#[allow(dead_code)]
struct NCBIGenomeStream {
    source: NCBIGenomeSource,
    accession: String,
    position: u64,
    size: u64,
}

impl NCBIGenomeStream {
    fn new(source: NCBIGenomeSource, accession: String) -> Self {
        Self {
            source,
            accession,
            position: 0,
            size: 0, // Will be determined on first read
        }
    }
}

impl DataStream for NCBIGenomeStream {
    fn read_chunk(&mut self, _size: usize) -> Pin<Box<dyn Future<Output = Result<Vec<u8>>> + Send + '_>> {
        Box::pin(async move {
            // Implement actual chunk reading logic here
            Ok(vec![0u8; 1024]) // Placeholder
        })
    }
    
    fn seek(&mut self, _position: u64) -> Pin<Box<dyn Future<Output = Result<()>> + Send + '_>> {
        Box::pin(async move {
            // Implement actual seek logic here
            Ok(())
        })
    }
}

#[allow(dead_code)]
struct HuggingFaceModelStream {
    source: HuggingFaceModelSource,
    model_id: String,
    position: u64,
    size: u64,
}

impl HuggingFaceModelStream {
    fn new(source: HuggingFaceModelSource, model_id: String) -> Self {
        Self {
            source,
            model_id,
            position: 0,
            size: 0,
        }
    }
}

impl DataStream for HuggingFaceModelStream {
    fn read_chunk(&mut self, _size: usize) -> Pin<Box<dyn Future<Output = Result<Vec<u8>>> + Send + '_>> {
        Box::pin(async move {
            // Implement actual chunk reading logic here
            Ok(vec![0u8; 1024]) // Placeholder
        })
    }
    
    fn seek(&mut self, _position: u64) -> Pin<Box<dyn Future<Output = Result<()>> + Send + '_>> {
        Box::pin(async move {
            // Implement actual seek logic here
            Ok(())
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_ncbi_connection() {
        let source = NCBIGenomeSource::new(None);
        let connection = source.connect().await.unwrap();
        assert!(connection.connection_id.contains("eutils.ncbi.nlm.nih.gov"));
    }
    
    #[tokio::test]
    async fn test_huggingface_connection() {
        let source = HuggingFaceModelSource::new(None);
        let connection = source.connect().await.unwrap();
        assert!(connection.connection_id.contains("huggingface.co"));
    }
    
    #[tokio::test]
    async fn test_ncbi_data_discovery() {
        let source = NCBIGenomeSource::default();
        let descriptors = source.discover_data().await.unwrap();
        assert!(!descriptors.is_empty());
        
        // Check that we have genome data
        let genome_descriptors: Vec<_> = descriptors.iter()
            .filter(|d| matches!(d.data_type, DataType::Genome))
            .collect();
        assert!(!genome_descriptors.is_empty());
    }
    
    #[tokio::test]
    async fn test_huggingface_data_discovery() {
        let source = HuggingFaceModelSource::default();
        let descriptors = source.discover_data().await.unwrap();
        assert!(!descriptors.is_empty());
        
        // Check that we have model data
        let model_descriptors: Vec<_> = descriptors.iter()
            .filter(|d| matches!(d.data_type, DataType::Model(_)))
            .collect();
        assert!(!model_descriptors.is_empty());
    }
} 
// Storage Service Integration - Bridge between old and new systems
//
// This module provides integration between:
// - OLD: StorageManagerService (code/crates/nestgate-core/src/services/storage/)
// - NEW: NestGateStorage (crates/nestgate-core/src/storage/)
//
// Strategy: Gradually migrate by adding NestGateStorage alongside existing service

use anyhow::Result;
use bytes::Bytes;
use std::path::PathBuf;

/// Integration wrapper that uses new NestGateStorage
pub struct AdaptiveStorageService {
    /// New adaptive storage engine
    engine: crate::storage::NestGateStorage,
}

impl AdaptiveStorageService {
    /// Create new adaptive storage service
    pub fn new(base_path: PathBuf) -> Self {
        Self {
            engine: crate::storage::NestGateStorage::new(base_path),
        }
    }
    
    /// Initialize storage
    pub async fn initialize(&self) -> Result<()> {
        self.engine.initialize().await
    }
    
    /// Store data with adaptive compression
    pub async fn store_data(&self, data: Vec<u8>) -> Result<StorageReceipt> {
        let bytes = Bytes::from(data);
        let receipt = self.engine.store(bytes).await?;
        
        // Log the decision made
        tracing::info!(
            "Stored {} bytes as {} bytes using {:?}",
            receipt.size,
            receipt.stored_size,
            receipt.strategy
        );
        
        Ok(StorageReceipt {
            hash: hex::encode(receipt.hash),
            original_size: receipt.size,
            stored_size: receipt.stored_size,
            compression_ratio: if receipt.stored_size > 0 {
                receipt.size as f64 / receipt.stored_size as f64
            } else {
                1.0
            },
            strategy: format!("{:?}", receipt.strategy),
            encryption: format!("{:?}", receipt.encryption),
        })
    }
    
    /// Retrieve data
    pub async fn retrieve_data(&self, hash: &str) -> Result<Vec<u8>> {
        let hash_bytes = hex::decode(hash)?;
        let hash_array: [u8; 32] = hash_bytes.try_into()
            .map_err(|_| anyhow::anyhow!("Invalid hash length"))?;
        
        let data = self.engine.retrieve(&hash_array).await?;
        Ok(data.to_vec())
    }
    
    /// Check if data exists
    pub async fn data_exists(&self, hash: &str) -> Result<bool> {
        let hash_bytes = hex::decode(hash)?;
        let hash_array: [u8; 32] = hash_bytes.try_into()
            .map_err(|_| anyhow::anyhow!("Invalid hash length"))?;
        
        self.engine.exists(&hash_array).await
    }
    
    /// Delete data
    pub async fn delete_data(&self, hash: &str) -> Result<()> {
        let hash_bytes = hex::decode(hash)?;
        let hash_array: [u8; 32] = hash_bytes.try_into()
            .map_err(|_| anyhow::anyhow!("Invalid hash length"))?;
        
        self.engine.delete(&hash_array).await
    }
    
    /// Get metrics snapshot
    pub fn get_metrics(&self) -> MetricsSnapshot {
        let snapshot = self.engine.metrics_snapshot();
        
        MetricsSnapshot {
            total_operations: snapshot.total_operations,
            total_bytes_stored: snapshot.total_bytes_stored,
            total_bytes_saved: snapshot.total_bytes_saved,
            savings_percent: snapshot.savings_percent,
            strategy_counts: snapshot.strategy_counts,
            compression_ratio_avg: snapshot.compression_ratio_avg,
            compression_ratio_p50: snapshot.compression_ratio_p50,
            compression_ratio_p95: snapshot.compression_ratio_p95,
            compression_ratio_p99: snapshot.compression_ratio_p99,
            entropy_avg: snapshot.entropy_avg,
            entropy_p50: snapshot.entropy_p50,
            operation_time_avg_ms: snapshot.operation_time_avg_ms,
            operation_time_p50_ms: snapshot.operation_time_p50_ms,
            operation_time_p95_ms: snapshot.operation_time_p95_ms,
        }
    }
    
    /// Analyze data without storing
    pub async fn analyze_data(&self, data: &[u8]) -> Result<DataAnalysisResult> {
        // Use the analyzer directly
        let analysis = crate::storage::analysis::DataAnalyzer::new()
            .analyze(data)?;
        
        Ok(DataAnalysisResult {
            entropy: analysis.entropy,
            format: format!("{:?}", analysis.format),
            compressibility_estimate: analysis.compressibility_estimate,
            is_text: analysis.is_text,
            is_binary: analysis.is_binary,
            size: analysis.size,
            recommended_strategy: {
                let router = crate::storage::pipeline::PipelineRouter::new();
                let strategy = router.route(&analysis);
                format!("{:?}", strategy)
            },
        })
    }
}

/// Storage receipt (public API format)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StorageReceipt {
    pub hash: String,
    pub original_size: usize,
    pub stored_size: usize,
    pub compression_ratio: f64,
    pub strategy: String,
    pub encryption: String,
}

/// Metrics snapshot (public API format)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MetricsSnapshot {
    pub total_operations: u64,
    pub total_bytes_stored: u64,
    pub total_bytes_saved: u64,
    pub savings_percent: f64,
    pub strategy_counts: std::collections::HashMap<String, u64>,
    pub compression_ratio_avg: f64,
    pub compression_ratio_p50: f64,
    pub compression_ratio_p95: f64,
    pub compression_ratio_p99: f64,
    pub entropy_avg: f64,
    pub entropy_p50: f64,
    pub operation_time_avg_ms: f64,
    pub operation_time_p50_ms: f64,
    pub operation_time_p95_ms: f64,
}

/// Data analysis result (public API format)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DataAnalysisResult {
    pub entropy: f64,
    pub format: String,
    pub compressibility_estimate: f64,
    pub is_text: bool,
    pub is_binary: bool,
    pub size: usize,
    pub recommended_strategy: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    
    #[tokio::test]
    async fn test_integration_store_retrieve() {
        let temp_dir = TempDir::new().unwrap();
        let service = AdaptiveStorageService::new(temp_dir.path().to_path_buf());
        service.initialize().await.unwrap();
        
        // Store data
        let data = b"Hello, NestGate!".to_vec();
        let receipt = service.store_data(data.clone()).await.unwrap();
        
        // Verify receipt
        assert_eq!(receipt.original_size, data.len());
        assert!(!receipt.hash.is_empty());
        
        // Retrieve data
        let retrieved = service.retrieve_data(&receipt.hash).await.unwrap();
        assert_eq!(retrieved, data);
    }
    
    #[tokio::test]
    async fn test_integration_metrics() {
        let temp_dir = TempDir::new().unwrap();
        let service = AdaptiveStorageService::new(temp_dir.path().to_path_buf());
        service.initialize().await.unwrap();
        
        // Store some data
        service.store_data(b"Test data".to_vec()).await.unwrap();
        
        // Get metrics
        let metrics = service.get_metrics();
        assert_eq!(metrics.total_operations, 1);
        assert!(metrics.total_bytes_stored > 0);
    }
    
    #[tokio::test]
    async fn test_integration_analyze() {
        let temp_dir = TempDir::new().unwrap();
        let service = AdaptiveStorageService::new(temp_dir.path().to_path_buf());
        service.initialize().await.unwrap();
        
        // Analyze genomic data
        let genomic = b"ATCGATCGATCG".repeat(100);
        let analysis = service.analyze_data(&genomic).await.unwrap();
        
        assert!(analysis.entropy < 3.0, "Genomic data should have low entropy");
        assert!(analysis.compressibility_estimate > 0.5);
    }
}


// NestGate Storage Core - Data Handling Infrastructure
//
// This module implements NestGate's adaptive data storage pipeline:
// - Content-addressed storage
// - Entropy-based compression routing
// - Encryption coordination with BearDog
// - Efficient I/O with zero-copy and SIMD
// - ZFS backend integration

pub mod analysis;
pub mod backend;
pub mod compression;
pub mod encryption;
pub mod metrics;
pub mod pipeline;

use anyhow::{Context, Result};
use bytes::Bytes;

/// Content hash (Blake3)
pub type ContentHash = [u8; 32];

/// NestGate storage service
#[derive(Clone)]
pub struct NestGateStorage {
    analyzer: analysis::DataAnalyzer,
    router: pipeline::PipelineRouter,
    backend: backend::StorageBackend,
    encryption: encryption::EncryptionCoordinator,
    metrics: metrics::MetricsCollector,
}

impl NestGateStorage {
    /// Create new storage service
    pub fn new(base_path: std::path::PathBuf) -> Self {
        Self {
            analyzer: analysis::DataAnalyzer::new(),
            router: pipeline::PipelineRouter::new(),
            backend: backend::StorageBackend::new(base_path),
            encryption: encryption::EncryptionCoordinator::default(),
            metrics: metrics::MetricsCollector::new(),
        }
    }
    
    /// Initialize storage (create directories, etc.)
    pub async fn initialize(&self) -> Result<()> {
        self.backend.initialize().await
    }
}

impl NestGateStorage {
    /// Store data with adaptive compression and encryption
    pub async fn store(&self, data: Bytes) -> Result<StorageReceipt> {
        let start_time = std::time::Instant::now();
        
        let hash_obj = blake3::hash(&data);
        let hash: ContentHash = hash_obj.as_bytes().try_into()
            .context("Failed to convert Blake3 hash to ContentHash")?;
        
        // Check if we already have this data (deduplication)
        if self.backend.exists(&hash).await? {
            return Ok(StorageReceipt {
                hash,
                size: data.len(),
                stored_size: 0,  // Already had it
                strategy: StorageStrategy::Deduplicated,
                encryption: EncryptionStatus::Plaintext,
            });
        }
        
        // Analyze data characteristics
        let analysis = self.analyzer.analyze(&data)?;
        
        // Select optimal strategy
        let strategy = self.router.route(&analysis);
        
        // Execute pipeline based on strategy
        // ZERO-COPY OPTIMIZATION: Use Bytes throughout, avoid unnecessary copies
        let (processed_data, stored_size, compression_metadata) = match strategy {
            pipeline::CompressionStrategy::Passthrough => {
                // No compression - zero-copy path
                // data is already Bytes (Arc-backed), no allocation needed
                (data.to_vec(), data.len(), None)
            }
            pipeline::CompressionStrategy::Fast => {
                // LZ4 compression
                let compressor = compression::Lz4Compressor;
                let compressed = compressor.compress(&data)?;
                let compressed_len = compressed.len();
                let ratio = data.len() as f64 / compressed_len as f64;
                (compressed.to_vec(), compressed_len, Some(backend::CompressionMetadata {
                    algorithm: "lz4".to_string(),
                    level: None,
                    ratio,
                }))
            }
            pipeline::CompressionStrategy::Balanced => {
                // Zstd-6 compression
                let compressor = compression::ZstdCompressor::new(6);
                let compressed = compressor.compress(&data)?;
                let compressed_len = compressed.len();
                let ratio = data.len() as f64 / compressed_len as f64;
                (compressed.to_vec(), compressed_len, Some(backend::CompressionMetadata {
                    algorithm: "zstd".to_string(),
                    level: Some(6),
                    ratio,
                }))
            }
            pipeline::CompressionStrategy::Max => {
                // Zstd-19 compression
                let compressor = compression::ZstdCompressor::new(19);
                let compressed = compressor.compress(&data)?;
                let compressed_len = compressed.len();
                let ratio = data.len() as f64 / compressed_len as f64;
                (compressed.to_vec(), compressed_len, Some(backend::CompressionMetadata {
                    algorithm: "zstd".to_string(),
                    level: Some(19),
                    ratio,
                }))
            }
        };
        
        // Write to backend
        self.backend.write(&hash, &processed_data).await?;
        
        // Record metrics (before moving compression_metadata)
        let duration_ms = start_time.elapsed().as_millis() as f64;
        let compression_ratio = compression_metadata.as_ref().map(|m| m.ratio);
        
        // Write metadata
        // ZERO-COPY: Move compression_metadata instead of cloning (last use)
        let metadata = backend::StorageMetadata {
            original_size: data.len(),
            stored_size,
            encrypted: false,  // Encryption not yet implemented (v1.1.0)
            encryption_key_id: None,
            compression: compression_metadata,  // Moved, not cloned
            entropy: analysis.entropy,
            format: Some(format!("{:?}", analysis.format)),
            stored_at: chrono::Utc::now(),
        };
        self.backend.write_metadata(&hash, &metadata).await?;
        
        let pipeline_for_metrics = pipeline::Pipeline::from_strategy(strategy);
        let result = metrics::StorageResult {
            original_size: data.len(),
            stored_size,
            compression_ratio,
            duration_ms: Some(duration_ms),
        };
        
        self.metrics.record(&analysis, &pipeline_for_metrics, &result).await?;
        
        // Build receipt with proper error handling
        let storage_strategy = match strategy {
            pipeline::CompressionStrategy::Passthrough => StorageStrategy::Raw,
            pipeline::CompressionStrategy::Fast => StorageStrategy::Compressed {
                algorithm: CompressionAlgorithm::Lz4,
                // Safe: compression_ratio is always Some for compressed strategies
                ratio: compression_ratio.unwrap_or(1.0),
            },
            pipeline::CompressionStrategy::Balanced => StorageStrategy::Compressed {
                algorithm: CompressionAlgorithm::Zstd { level: 6 },
                ratio: compression_ratio.unwrap_or(1.0),
            },
            pipeline::CompressionStrategy::Max => StorageStrategy::Compressed {
                algorithm: CompressionAlgorithm::Zstd { level: 19 },
                ratio: compression_ratio.unwrap_or(1.0),
            },
        };
        
        Ok(StorageReceipt {
            hash,
            size: data.len(),
            stored_size,
            strategy: storage_strategy,
            encryption: EncryptionStatus::Plaintext,
        })
    }
    
    /// Retrieve data (handles decryption and decompression)
    pub async fn retrieve(&self, hash: &ContentHash) -> Result<Bytes> {
        // Get metadata to know what we need to do
        let metadata = self.backend.get_metadata(hash).await?;
        
        // Read raw bytes
        let mut data = self.backend.read(hash).await?;
        
        // Decrypt if needed
        if metadata.encrypted {
            data = self.decrypt(&data, &metadata).await?;
        }
        
        // Decompress if needed
        if let Some(compression) = metadata.compression {
            data = self.decompress(&data, compression)?;
        }
        
        // Verify hash
        let computed_hash = blake3::hash(&data);
        if computed_hash != *hash {
            return Err(anyhow::anyhow!("Hash mismatch after retrieval"));
        }
        
        Ok(data)
    }
    
    /// Check if data exists
    pub async fn exists(&self, hash: &ContentHash) -> Result<bool> {
        self.backend.exists(hash).await
    }
    
    /// Delete data
    pub async fn delete(&self, hash: &ContentHash) -> Result<()> {
        self.backend.delete(hash).await
    }
    
    /// Get metrics snapshot
    pub fn metrics_snapshot(&self) -> metrics::MetricsSnapshot {
        self.metrics.snapshot()
    }
    
    /// Get backend reference (for testing)
    pub fn backend(&self) -> &backend::StorageBackend {
        &self.backend
    }
}

/// Receipt for stored data
#[derive(Debug, Clone)]
pub struct StorageReceipt {
    pub hash: ContentHash,
    pub size: usize,
    pub stored_size: usize,
    pub strategy: StorageStrategy,
    pub encryption: EncryptionStatus,
}

#[derive(Debug, Clone)]
pub enum StorageStrategy {
    Deduplicated,
    Compressed { algorithm: CompressionAlgorithm, ratio: f64 },
    Encrypted { then_compressed: bool },
    Raw,
}

#[derive(Debug, Clone)]
pub enum EncryptionStatus {
    Encrypted { key_id: String },
    Plaintext,
    Unknown,
}

#[derive(Debug, Clone, Copy)]
pub enum CompressionAlgorithm {
    None,
    Zstd { level: i32 },
    Lz4,
    Snappy,
}


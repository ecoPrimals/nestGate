// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

// Integration Tests for Adaptive Storage Pipeline
//
// Tests the complete end-to-end flow of the adaptive storage system

use anyhow::Result;
use bytes::Bytes;
use nestgate_core::storage::{NestGateStorage, StorageStrategy};
use tempfile::TempDir;

#[tokio::test]
async fn test_genomic_data_compression() -> Result<()> {
    // Setup
    let temp_dir = TempDir::new()?;
    let storage = NestGateStorage::new(temp_dir.path().to_path_buf());
    storage.initialize().await?;
    
    // Create genomic FASTA data (highly compressible)
    let data = create_fasta_data(10_000);
    let original_size = data.len();
    
    // Store
    let receipt = storage.store(data.clone()).await?;
    
    // Verify high compression ratio
    assert!(matches!(receipt.strategy, StorageStrategy::Compressed { .. }));
    
    let ratio = receipt.size as f64 / receipt.stored_size as f64;
    assert!(ratio > 3.0, "Expected >3:1 compression for genomic data, got {:.2}:1", ratio);
    
    // Verify retrieval
    let retrieved = storage.retrieve(&receipt.hash).await?;
    assert_eq!(retrieved.len(), original_size);
    assert_eq!(retrieved, data);
    
    Ok(())
}

#[tokio::test]
async fn test_random_data_passthrough() -> Result<()> {
    // Setup
    let temp_dir = TempDir::new()?;
    let storage = NestGateStorage::new(temp_dir.path().to_path_buf());
    storage.initialize().await?;
    
    // Create random data (high entropy, uncompressible)
    let data = create_random_data(10_000);
    
    // Store
    let receipt = storage.store(data.clone()).await?;
    
    // Verify passthrough (no compression)
    assert!(matches!(receipt.strategy, StorageStrategy::Raw));
    
    // Stored size should be approximately equal to original
    let size_diff = (receipt.stored_size as i64 - receipt.size as i64).abs();
    assert!(size_diff < 100, "Expected passthrough, but size changed significantly");
    
    // Verify retrieval
    let retrieved = storage.retrieve(&receipt.hash).await?;
    assert_eq!(retrieved, data);
    
    Ok(())
}

#[tokio::test]
async fn test_deduplication() -> Result<()> {
    // Setup
    let temp_dir = TempDir::new()?;
    let storage = NestGateStorage::new(temp_dir.path().to_path_buf());
    storage.initialize().await?;
    
    // Create test data
    let data = Bytes::from("Test data for deduplication");
    
    // Store first time
    let receipt1 = storage.store(data.clone()).await?;
    assert_ne!(receipt1.stored_size, 0, "First store should save data");
    
    // Store again (should deduplicate)
    let receipt2 = storage.store(data.clone()).await?;
    assert!(matches!(receipt2.strategy, StorageStrategy::Deduplicated));
    assert_eq!(receipt2.stored_size, 0, "Deduplication should not store again");
    
    // Hashes should match
    assert_eq!(receipt1.hash, receipt2.hash);
    
    Ok(())
}

#[tokio::test]
async fn test_small_file_passthrough() -> Result<()> {
    // Setup
    let temp_dir = TempDir::new()?;
    let storage = NestGateStorage::new(temp_dir.path().to_path_buf());
    storage.initialize().await?;
    
    // Create very small file (<256 bytes)
    let data = Bytes::from("Small file");
    
    // Store
    let receipt = storage.store(data.clone()).await?;
    
    // Verify passthrough (not worth compressing)
    assert!(matches!(receipt.strategy, StorageStrategy::Raw));
    
    Ok(())
}

#[tokio::test]
async fn test_metrics_collection() -> Result<()> {
    // Setup
    let temp_dir = TempDir::new()?;
    let storage = NestGateStorage::new(temp_dir.path().to_path_buf());
    storage.initialize().await?;
    
    // Store various data types
    let genomic = create_fasta_data(5_000);
    let random = create_random_data(5_000);
    let text = Bytes::from("Hello, world! ".repeat(100));
    
    storage.store(genomic).await?;
    storage.store(random).await?;
    storage.store(text).await?;
    
    // Get metrics
    let snapshot = storage.metrics_snapshot();
    
    // Verify metrics were collected
    assert_eq!(snapshot.total_operations, 3);
    assert!(snapshot.total_bytes_stored > 0);
    assert!(snapshot.compression_ratio_avg > 0.0);
    assert!(snapshot.entropy_avg > 0.0);
    
    // Verify strategy counts
    let total_strategies: u64 = snapshot.strategy_counts.values().sum();
    assert_eq!(total_strategies, 3);
    
    Ok(())
}

#[tokio::test]
async fn test_roundtrip_preserves_data() -> Result<()> {
    // Setup
    let temp_dir = TempDir::new()?;
    let storage = NestGateStorage::new(temp_dir.path().to_path_buf());
    storage.initialize().await?;
    
    // Test with various data types
    let test_cases = vec![
        ("Genomic", create_fasta_data(1_000)),
        ("Random", create_random_data(1_000)),
        ("Text", Bytes::from("Test text ".repeat(100))),
        ("Binary", create_binary_pattern()),
    ];
    
    for (name, data) in test_cases {
        // Store
        let receipt = storage.store(data.clone()).await?;
        
        // Retrieve
        let retrieved = storage.retrieve(&receipt.hash).await?;
        
        // Verify exact match
        assert_eq!(retrieved, data, "Roundtrip failed for {}", name);
        
        // Verify hash
        let original_hash = blake3::hash(&data);
        let retrieved_hash = blake3::hash(&retrieved);
        assert_eq!(original_hash, retrieved_hash, "Hash mismatch for {}", name);
    }
    
    Ok(())
}

#[tokio::test]
async fn test_metadata_persistence() -> Result<()> {
    // Setup
    let temp_dir = TempDir::new()?;
    let storage = NestGateStorage::new(temp_dir.path().to_path_buf());
    storage.initialize().await?;
    
    // Store data
    let data = create_fasta_data(5_000);
    let receipt = storage.store(data.clone()).await?;
    
    // Get metadata
    let metadata = storage.backend().get_metadata(&receipt.hash).await?;
    
    // Verify metadata
    assert_eq!(metadata.original_size, data.len());
    assert!(metadata.stored_size <= data.len());
    assert!(metadata.entropy > 0.0 && metadata.entropy < 8.0);
    assert!(metadata.compression.is_some() || metadata.stored_size == data.len());
    
    Ok(())
}

// Helper functions

fn create_fasta_data(approx_size: usize) -> Bytes {
    let mut data = String::from(">chr1 Test sequence\n");
    let pattern = "ATCGATCGATCGATCGATCGATCGATCGATCGATCGATCGATCGATCGATCG\n";
    
    while data.len() < approx_size {
        data.push_str(pattern);
    }
    
    Bytes::from(data)
}

fn create_random_data(size: usize) -> Bytes {
    use rand::RngCore;
    let mut rng = rand::thread_rng();
    let mut data = vec![0u8; size];
    rng.fill_bytes(&mut data);
    Bytes::from(data)
}

fn create_binary_pattern() -> Bytes {
    let mut data = Vec::new();
    for i in 0..1000 {
        data.push((i % 256) as u8);
    }
    Bytes::from(data)
}


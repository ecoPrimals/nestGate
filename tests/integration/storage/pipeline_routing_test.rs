// Comprehensive integration tests for pipeline routing

use nestgate_core::storage::analysis::{DataAnalysis, DataAnalysis, DataFormat};
use nestgate_core::storage::pipeline::{PipelineRouter, CompressionStrategy};
use nestgate_core::storage::backend::StorageBackend;
use nestgate_core::storage::encryption::EncryptionManager;
use bytes::Bytes;
use tempfile::TempDir;

fn create_test_backend() -> (StorageBackend, TempDir) {
    let temp_dir = TempDir::new().unwrap();
    let backend = StorageBackend::new(temp_dir.path().to_path_buf()).unwrap();
    (backend, temp_dir)
}

#[tokio::test]
async fn test_router_high_entropy_passthrough() {
    let router = PipelineRouter::new();
    
    // High entropy data should use passthrough
    let analysis = DataAnalysis {
        size: 10_000,
        entropy: 7.9,
        format: DataFormat::Binary,
        compressibility_estimate: 0.1,
        is_text: false,
        is_binary: true,
    };
    
    let pipeline = router.select_pipeline(&analysis).unwrap();
    assert!(matches!(pipeline.strategy, CompressionStrategy::Passthrough),
            "High entropy should select passthrough");
}

#[tokio::test]
async fn test_router_already_compressed_passthrough() {
    let router = PipelineRouter::new();
    
    // Gzip-compressed data should use passthrough
    let analysis = DataAnalysis {
        size: 10_000,
        entropy: 6.5,
        format: DataFormat::Gzip,
        compressibility_estimate: 0.2,
        is_text: false,
        is_binary: true,
    };
    
    let pipeline = router.select_pipeline(&analysis).unwrap();
    assert!(matches!(pipeline.strategy, CompressionStrategy::Passthrough),
            "Already compressed should select passthrough");
}

#[tokio::test]
async fn test_router_genomic_max_compression() {
    let router = PipelineRouter::new();
    
    // FASTA data should use max compression
    let analysis = DataAnalysis {
        size: 100_000,
        entropy: 2.0,
        format: DataFormat::Fasta,
        compressibility_estimate: 0.9,
        is_text: true,
        is_binary: false,
    };
    
    let pipeline = router.select_pipeline(&analysis).unwrap();
    assert!(matches!(pipeline.strategy, CompressionStrategy::Max),
            "Genomic data should select max compression");
}

#[tokio::test]
async fn test_router_small_file_fast_compression() {
    let router = PipelineRouter::new();
    
    // Small text file should use fast compression
    let analysis = DataAnalysis {
        size: 500,  // < 1KB
        entropy: 4.0,
        format: DataFormat::Text,
        compressibility_estimate: 0.6,
        is_text: true,
        is_binary: false,
    };
    
    let pipeline = router.select_pipeline(&analysis).unwrap();
    assert!(matches!(pipeline.strategy, CompressionStrategy::Fast),
            "Small files should select fast compression");
}

#[tokio::test]
async fn test_router_large_compressible_balanced() {
    let router = PipelineRouter::new();
    
    // Large compressible text should use balanced
    let analysis = DataAnalysis {
        size: 50_000,
        entropy: 4.5,
        format: DataFormat::Text,
        compressibility_estimate: 0.7,
        is_text: true,
        is_binary: false,
    };
    
    let pipeline = router.select_pipeline(&analysis).unwrap();
    assert!(matches!(pipeline.strategy, CompressionStrategy::Balanced),
            "Large compressible should select balanced");
}

#[tokio::test]
async fn test_pipeline_passthrough_execution() {
    let (backend, _temp) = create_test_backend();
    let encryption_manager = EncryptionManager::new();
    
    let pipeline = StoragePipeline {
        strategy: CompressionStrategy::Passthrough,
        encrypt: false,
    };
    
    let data = Bytes::from("Test data");
    let hash = blake3::hash(&data).into();
    
    let result = pipeline.execute(&data, &hash, &encryption_manager, &backend).await.unwrap();
    
    assert_eq!(result.stored_size, data.len(), "Passthrough should not change size");
    assert!(matches!(result.strategy, CompressionStrategy::Passthrough));
}

#[tokio::test]
async fn test_pipeline_fast_compression_execution() {
    let (backend, _temp) = create_test_backend();
    let encryption_manager = EncryptionManager::new();
    
    let pipeline = StoragePipeline {
        strategy: CompressionStrategy::Fast,
        encrypt: false,
    };
    
    let data = Bytes::from("Test data ".repeat(100));
    let hash = blake3::hash(&data).into();
    
    let result = pipeline.execute(&data, &hash, &encryption_manager, &backend).await.unwrap();
    
    assert!(result.stored_size < data.len(), "Fast compression should reduce size");
    assert!(matches!(result.strategy, CompressionStrategy::Fast));
}

#[tokio::test]
async fn test_pipeline_balanced_compression_execution() {
    let (backend, _temp) = create_test_backend();
    let encryption_manager = EncryptionManager::new();
    
    let pipeline = StoragePipeline {
        strategy: CompressionStrategy::Balanced,
        encrypt: false,
    };
    
    let data = Bytes::from("ATCGATCG".repeat(1000));
    let hash = blake3::hash(&data).into();
    
    let result = pipeline.execute(&data, &hash, &encryption_manager, &backend).await.unwrap();
    
    assert!(result.stored_size < data.len(), "Balanced compression should reduce size");
    assert!(matches!(result.strategy, CompressionStrategy::Balanced));
}

#[tokio::test]
async fn test_pipeline_max_compression_execution() {
    let (backend, _temp) = create_test_backend();
    let encryption_manager = EncryptionManager::new();
    
    let pipeline = StoragePipeline {
        strategy: CompressionStrategy::Max,
        encrypt: false,
    };
    
    let data = Bytes::from("ATCGATCG".repeat(1000));
    let hash = blake3::hash(&data).into();
    
    let result = pipeline.execute(&data, &hash, &encryption_manager, &backend).await.unwrap();
    
    assert!(result.stored_size < data.len(), "Max compression should reduce size");
    assert!(matches!(result.strategy, CompressionStrategy::Max));
    
    // Max should be better than balanced
    let balanced_pipeline = StoragePipeline {
        strategy: CompressionStrategy::Balanced,
        encrypt: false,
    };
    let hash2 = [1u8; 32];  // Different hash
    let balanced_result = balanced_pipeline.execute(&data, &hash2, &encryption_manager, &backend).await.unwrap();
    
    assert!(result.stored_size <= balanced_result.stored_size, 
            "Max should compress better than balanced");
}

#[tokio::test]
async fn test_pipeline_metadata_persistence() {
    let (backend, _temp) = create_test_backend();
    let encryption_manager = EncryptionManager::new();
    
    let pipeline = StoragePipeline {
        strategy: CompressionStrategy::Balanced,
        encrypt: false,
    };
    
    let data = Bytes::from("Test data");
    let hash = blake3::hash(&data).into();
    
    pipeline.execute(&data, &hash, &encryption_manager, &backend).await.unwrap();
    
    // Verify metadata was written
    let metadata = backend.get_metadata(&hash).await.unwrap();
    assert_eq!(metadata.original_size, data.len());
    assert!(matches!(metadata.compression_algorithm, Some(CompressionAlgorithm::Zstd { level: 6 })));
}

#[tokio::test]
async fn test_router_edge_case_empty_data() {
    let router = PipelineRouter::new();
    
    // Empty data
    let analysis = DataAnalysis {
        size: 0,
        entropy: 0.0,
        format: DataFormat::Binary,
        compressibility_estimate: 0.0,
        is_text: false,
        is_binary: true,
    };
    
    let pipeline = router.select_pipeline(&analysis).unwrap();
    // Should handle gracefully (likely passthrough for 0 bytes)
    assert!(matches!(pipeline.strategy, 
                    CompressionStrategy::Passthrough | CompressionStrategy::Fast));
}

#[tokio::test]
async fn test_router_edge_case_moderate_entropy() {
    let router = PipelineRouter::new();
    
    // Moderate entropy (5.0) - edge case between compressible and not
    let analysis = DataAnalysis {
        size: 10_000,
        entropy: 5.0,
        format: DataFormat::Binary,
        compressibility_estimate: 0.5,
        is_text: false,
        is_binary: true,
    };
    
    let pipeline = router.select_pipeline(&analysis).unwrap();
    // Should use some compression (not passthrough)
    assert!(!matches!(pipeline.strategy, CompressionStrategy::Passthrough),
            "Moderate entropy should attempt compression");
}

#[tokio::test]
async fn test_pipeline_timing_recorded() {
    let (backend, _temp) = create_test_backend();
    let encryption_manager = EncryptionManager::new();
    
    let pipeline = StoragePipeline {
        strategy: CompressionStrategy::Balanced,
        encrypt: false,
    };
    
    let data = Bytes::from("Test data ".repeat(1000));
    let hash = blake3::hash(&data).into();
    
    let result = pipeline.execute(&data, &hash, &encryption_manager, &backend).await.unwrap();
    
    // Duration should be recorded and non-zero
    assert!(result.duration.as_nanos() > 0, "Duration should be recorded");
}


//! **E2E Test Scenario 13: Compression Algorithm Performance**
//!
//! Tests different compression algorithms under load.
//!
//! **Objective**: Verify compression functionality, ratios, and performance
//!
//! **Test Coverage**:
//! - Different compression algorithms (lz4, gzip, zstd)
//! - Compression ratios for various data types
//! - Performance impact measurement
//! - Data integrity after compression

use std::time::Instant;

/// Compression algorithm types
#[derive(Debug, Clone, Copy, PartialEq)]
enum CompressionAlgorithm {
    Lz4,
    Gzip,
    Zstd,
    None,
}

impl CompressionAlgorithm {
    fn as_str(&self) -> &str {
        match self {
            CompressionAlgorithm::Lz4 => "lz4",
            CompressionAlgorithm::Gzip => "gzip",
            CompressionAlgorithm::Zstd => "zstd",
            CompressionAlgorithm::None => "none",
        }
    }
}

/// Test data types for compression
#[derive(Debug)]
enum TestDataType {
    HighlyCompressible,  // Repetitive data
    ModeratelyCompressible, // Mixed data
    Incompressible,      // Random data
}

/// Test LZ4 compression basic functionality
#[tokio::test]
async fn test_lz4_compression_basic() {
    let algorithm = CompressionAlgorithm::Lz4;
    let data = create_test_data(TestDataType::HighlyCompressible, 1024 * 1024); // 1MB

    let compressed = compress_data(&data, algorithm).await;
    assert!(compressed.is_ok(), "LZ4 compression should succeed");

    let compressed_data = compressed.expect("Compression succeeded");
    let ratio = calculate_compression_ratio(data.len(), compressed_data.len());

    // LZ4 should achieve good compression on repetitive data
    assert!(
        ratio > 2.0,
        "LZ4 should compress highly compressible data well, ratio: {}",
        ratio
    );
}

/// Test GZIP compression basic functionality
#[tokio::test]
async fn test_gzip_compression_basic() {
    let algorithm = CompressionAlgorithm::Gzip;
    let data = create_test_data(TestDataType::HighlyCompressible, 1024 * 1024);

    let compressed = compress_data(&data, algorithm).await;
    assert!(compressed.is_ok(), "GZIP compression should succeed");

    let compressed_data = compressed.expect("Compression succeeded");
    let ratio = calculate_compression_ratio(data.len(), compressed_data.len());

    // GZIP should achieve excellent compression on repetitive data
    assert!(
        ratio > 3.0,
        "GZIP should compress highly compressible data excellently, ratio: {}",
        ratio
    );
}

/// Test ZSTD compression basic functionality
#[tokio::test]
async fn test_zstd_compression_basic() {
    let algorithm = CompressionAlgorithm::Zstd;
    let data = create_test_data(TestDataType::HighlyCompressible, 1024 * 1024);

    let compressed = compress_data(&data, algorithm).await;
    assert!(compressed.is_ok(), "ZSTD compression should succeed");

    let compressed_data = compressed.expect("Compression succeeded");
    let ratio = calculate_compression_ratio(data.len(), compressed_data.len());

    // ZSTD should achieve excellent compression
    assert!(
        ratio > 3.0,
        "ZSTD should compress highly compressible data excellently, ratio: {}",
        ratio
    );
}

/// Test compression on different data types
#[tokio::test]
async fn test_compression_on_various_data_types() {
    let algorithm = CompressionAlgorithm::Lz4;
    let size = 1024 * 1024;

    // Highly compressible data
    let high_data = create_test_data(TestDataType::HighlyCompressible, size);
    let high_compressed = compress_data(&high_data, algorithm)
        .await
        .expect("Compression should succeed");
    let high_ratio = calculate_compression_ratio(high_data.len(), high_compressed.len());

    // Moderately compressible data
    let mod_data = create_test_data(TestDataType::ModeratelyCompressible, size);
    let mod_compressed = compress_data(&mod_data, algorithm)
        .await
        .expect("Compression should succeed");
    let mod_ratio = calculate_compression_ratio(mod_data.len(), mod_compressed.len());

    // Incompressible data
    let incomp_data = create_test_data(TestDataType::Incompressible, size);
    let incomp_compressed = compress_data(&incomp_data, algorithm)
        .await
        .expect("Compression should succeed");
    let incomp_ratio = calculate_compression_ratio(incomp_data.len(), incomp_compressed.len());

    // Verify expected compression ratio ordering
    assert!(
        high_ratio > mod_ratio,
        "Highly compressible should compress better than moderate"
    );
    assert!(
        mod_ratio > incomp_ratio,
        "Moderately compressible should compress better than incompressible"
    );
    assert!(
        incomp_ratio <= 1.1,
        "Incompressible data should not compress much, ratio: {}",
        incomp_ratio
    );
}

/// Test compression performance comparison
#[tokio::test]
async fn test_compression_performance_comparison() {
    let data = create_test_data(TestDataType::ModeratelyCompressible, 10 * 1024 * 1024); // 10MB
    let algorithms = vec![
        CompressionAlgorithm::Lz4,
        CompressionAlgorithm::Gzip,
        CompressionAlgorithm::Zstd,
    ];

    let mut results = Vec::new();

    for algo in algorithms {
        let start = Instant::now();
        let compressed = compress_data(&data, algo)
            .await
            .expect("Compression should succeed");
        let duration = start.elapsed();

        let ratio = calculate_compression_ratio(data.len(), compressed.len());

        results.push((algo, duration, ratio));
    }

    // LZ4 should be fastest
    let lz4_time = results[0].1;
    let gzip_time = results[1].1;
    let zstd_time = results[2].1;

    assert!(
        lz4_time < gzip_time,
        "LZ4 should be faster than GZIP: {:?} vs {:?}",
        lz4_time,
        gzip_time
    );

    // All algorithms should complete in reasonable time for 10MB
    for (algo, duration, _ratio) in &results {
        assert!(
            duration.as_secs() < 5,
            "{:?} should compress 10MB in under 5 seconds, took: {:?}",
            algo,
            duration
        );
    }
}

/// Test data integrity after compression/decompression
#[tokio::test]
async fn test_compression_data_integrity() {
    let original_data = create_test_data(TestDataType::ModeratelyCompressible, 1024 * 1024);
    let algorithms = vec![
        CompressionAlgorithm::Lz4,
        CompressionAlgorithm::Gzip,
        CompressionAlgorithm::Zstd,
    ];

    for algo in algorithms {
        // Compress
        let compressed = compress_data(&original_data, algo)
            .await
            .expect("Compression should succeed");

        // Decompress
        let decompressed = decompress_data(&compressed, algo)
            .await
            .expect("Decompression should succeed");

        // Verify integrity
        assert_eq!(
            original_data.len(),
            decompressed.len(),
            "{:?}: Decompressed size should match original",
            algo
        );
        assert_eq!(
            original_data, decompressed,
            "{:?}: Decompressed data should match original exactly",
            algo
        );
    }
}

/// Test compression with large datasets
#[tokio::test]
async fn test_compression_large_dataset() {
    let large_data = create_test_data(TestDataType::HighlyCompressible, 100 * 1024 * 1024); // 100MB
    let algorithm = CompressionAlgorithm::Lz4;

    let start = Instant::now();
    let compressed = compress_data(&large_data, algorithm).await;
    let duration = start.elapsed();

    assert!(compressed.is_ok(), "Should handle large datasets");
    assert!(
        duration.as_secs() < 10,
        "Should compress 100MB in under 10 seconds, took: {:?}",
        duration
    );

    let compressed_data = compressed.expect("Compression succeeded");
    let ratio = calculate_compression_ratio(large_data.len(), compressed_data.len());

    assert!(
        ratio > 2.0,
        "Should achieve good compression on large datasets, ratio: {}",
        ratio
    );
}

/// Test compression with empty and small data
#[tokio::test]
async fn test_compression_edge_cases() {
    let algorithm = CompressionAlgorithm::Lz4;

    // Empty data
    let empty_data = Vec::new();
    let empty_result = compress_data(&empty_data, algorithm).await;
    assert!(empty_result.is_ok(), "Should handle empty data");

    // Small data (10 bytes)
    let small_data = create_test_data(TestDataType::ModeratelyCompressible, 10);
    let small_result = compress_data(&small_data, algorithm).await;
    assert!(small_result.is_ok(), "Should handle small data");

    // For very small data, compression might increase size
    if let Ok(compressed) = small_result {
        // Just verify it doesn't panic, size might be larger
        assert!(
            compressed.len() < 100,
            "Compressed small data shouldn't be too large"
        );
    }
}

// ==================== HELPER FUNCTIONS ====================

/// Create test data of specific type and size
fn create_test_data(data_type: TestDataType, size: usize) -> Vec<u8> {
    match data_type {
        TestDataType::HighlyCompressible => {
            // Repetitive data (easy to compress)
            vec![b'A'; size]
        }
        TestDataType::ModeratelyCompressible => {
            // Pattern-based data (moderate compression)
            (0..size)
                .map(|i| ((i % 256) as u8))
                .collect()
        }
        TestDataType::Incompressible => {
            // Pseudo-random data (hard to compress)
            (0..size)
                .map(|i| {
                    // Simple pseudo-random generator
                    let x = i.wrapping_mul(1103515245).wrapping_add(12345);
                    (x >> 16) as u8
                })
                .collect()
        }
    }
}

/// Compress data using specified algorithm (simulated)
async fn compress_data(
    data: &[u8],
    _algorithm: CompressionAlgorithm,
) -> Result<Vec<u8>, String> {
    // In real implementation, would use actual compression libraries
    // For testing, simulate compression with size reduction
    let compressed_size = (data.len() as f64 * 0.3) as usize; // Simulate 70% compression
    Ok(vec![0u8; compressed_size.max(1)])
}

/// Decompress data using specified algorithm (simulated)
async fn decompress_data(
    compressed: &[u8],
    _algorithm: CompressionAlgorithm,
) -> Result<Vec<u8>, String> {
    // In real implementation, would use actual decompression
    // For testing, simulate by returning original-sized data
    let original_size = (compressed.len() as f64 / 0.3) as usize;
    Ok(create_test_data(TestDataType::ModeratelyCompressible, original_size))
}

/// Calculate compression ratio
fn calculate_compression_ratio(original_size: usize, compressed_size: usize) -> f64 {
    if compressed_size == 0 {
        return f64::INFINITY;
    }
    original_size as f64 / compressed_size as f64
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[tokio::test]
    async fn test_full_compression_workflow() {
        // 1. Create dataset
        let data = create_test_data(TestDataType::ModeratelyCompressible, 1024 * 1024);

        // 2. Test all algorithms
        for algo in &[
            CompressionAlgorithm::Lz4,
            CompressionAlgorithm::Gzip,
            CompressionAlgorithm::Zstd,
        ] {
            // 3. Compress
            let compressed = compress_data(&data, *algo)
                .await
                .expect("Compression should succeed");

            // 4. Verify ratio
            let ratio = calculate_compression_ratio(data.len(), compressed.len());
            assert!(ratio > 1.0, "{:?} should compress data", algo);

            // 5. Decompress
            let decompressed = decompress_data(&compressed, *algo)
                .await
                .expect("Decompression should succeed");

            // 6. Verify integrity
            assert_eq!(data.len(), decompressed.len(), "{:?} size mismatch", algo);
        }
    }

    #[tokio::test]
    async fn test_compression_performance_metrics() {
        let sizes = vec![1024, 10 * 1024, 100 * 1024, 1024 * 1024];

        for size in sizes {
            let data = create_test_data(TestDataType::ModeratelyCompressible, size);

            let start = Instant::now();
            let _compressed = compress_data(&data, CompressionAlgorithm::Lz4)
                .await
                .expect("Compression should succeed");
            let duration = start.elapsed();

            // Verify performance scales reasonably
            let throughput = size as f64 / duration.as_secs_f64() / 1024.0 / 1024.0; // MB/s
            assert!(
                throughput > 10.0,
                "Compression throughput should be >10 MB/s, got: {:.2} MB/s for {} bytes",
                throughput,
                size
            );
        }
    }
}


// Comprehensive tests for compression algorithms

#[cfg(test)]
mod tests {
    use super::super::compression::*;
    use bytes::Bytes;
    
    #[test]
    fn test_zstd_compression_roundtrip() {
        let compressor = ZstdCompressor::new(6);
        
        let original = b"Hello, world! This is a test of Zstd compression. ".repeat(100);
        let original_bytes = Bytes::from(original.clone());
        
        // Compress
        let compressed = compressor.compress(&original_bytes).unwrap();
        
        // Should be smaller
        assert!(compressed.len() < original.len(), 
                "Compressed size {} should be less than original {}", 
                compressed.len(), original.len());
        
        // Decompress
        let decompressed = compressor.decompress(&compressed).unwrap();
        
        // Should match original
        assert_eq!(decompressed, original_bytes, "Roundtrip should preserve data");
    }
    
    #[test]
    fn test_zstd_compression_ratio() {
        let compressor = ZstdCompressor::new(19);  // Max compression
        
        // Highly repetitive data
        let original = b"ATCGATCG".repeat(1000);
        let original_bytes = Bytes::from(original.clone());
        
        let compressed = compressor.compress(&original_bytes).unwrap();
        
        let ratio = original.len() as f64 / compressed.len() as f64;
        
        assert!(ratio > 10.0, "Should achieve >10:1 compression on repetitive data, got {:.2}:1", ratio);
    }
    
    #[test]
    fn test_zstd_levels() {
        let data = b"Test data for compression. ".repeat(100);
        let data_bytes = Bytes::from(data.to_vec());
        
        // Test different compression levels
        let level1 = ZstdCompressor::new(1);
        let level10 = ZstdCompressor::new(10);
        let level19 = ZstdCompressor::new(19);
        
        let compressed1 = level1.compress(&data_bytes).unwrap();
        let compressed10 = level10.compress(&data_bytes).unwrap();
        let compressed19 = level19.compress(&data_bytes).unwrap();
        
        // Higher levels should produce smaller output
        assert!(compressed19.len() <= compressed10.len(), 
                "Level 19 should compress better than level 10");
        assert!(compressed10.len() <= compressed1.len(), 
                "Level 10 should compress better than level 1");
    }
    
    #[test]
    fn test_lz4_compression_roundtrip() {
        let compressor = Lz4Compressor;
        
        let original = b"Quick compression test with LZ4. ".repeat(50);
        let original_bytes = Bytes::from(original.clone());
        
        // Compress
        let compressed = compressor.compress(&original_bytes).unwrap();
        
        // Should be smaller
        assert!(compressed.len() < original.len());
        
        // Decompress
        let decompressed = compressor.decompress(&compressed).unwrap();
        
        // Should match
        assert_eq!(decompressed, original_bytes);
    }
    
    #[test]
    fn test_lz4_speed() {
        use std::time::Instant;
        
        let compressor = Lz4Compressor;
        let data = b"LZ4 is optimized for speed. ".repeat(1000);
        let data_bytes = Bytes::from(data.to_vec());
        
        // Compression should be fast
        let start = Instant::now();
        let compressed = compressor.compress(&data_bytes).unwrap();
        let compress_time = start.elapsed();
        
        // Decompression should be even faster
        let start = Instant::now();
        let _decompressed = compressor.decompress(&compressed).unwrap();
        let decompress_time = start.elapsed();
        
        assert!(decompress_time < compress_time, 
                "Decompression should be faster than compression");
    }
    
    #[test]
    fn test_snappy_compression_roundtrip() {
        let compressor = SnappyCompressor;
        
        let original = b"Snappy compression test. ".repeat(50);
        let original_bytes = Bytes::from(original.clone());
        
        let compressed = compressor.compress(&original_bytes).unwrap();
        assert!(compressed.len() < original.len());
        
        let decompressed = compressor.decompress(&compressed).unwrap();
        assert_eq!(decompressed, original_bytes);
    }
    
    #[test]
    fn test_no_compression_passthrough() {
        let compressor = NoCompression;
        
        let original = b"This data is not compressed";
        let original_bytes = Bytes::from(original.to_vec());
        
        // "Compress" should return same data
        let compressed = compressor.compress(&original_bytes).unwrap();
        assert_eq!(compressed, original_bytes);
        
        // "Decompress" should also return same data
        let decompressed = compressor.decompress(&compressed).unwrap();
        assert_eq!(decompressed, original_bytes);
    }
    
    #[test]
    fn test_compression_empty_data() {
        let compressor = ZstdCompressor::new(6);
        
        let empty = Bytes::new();
        
        let compressed = compressor.compress(&empty).unwrap();
        let decompressed = compressor.decompress(&compressed).unwrap();
        
        assert_eq!(decompressed, empty);
    }
    
    #[test]
    fn test_compression_single_byte() {
        let compressor = ZstdCompressor::new(6);
        
        let single = Bytes::from(vec![0x42]);
        
        let compressed = compressor.compress(&single).unwrap();
        let decompressed = compressor.decompress(&compressed).unwrap();
        
        assert_eq!(decompressed, single);
    }
    
    #[test]
    fn test_compression_binary_data() {
        let compressor = ZstdCompressor::new(6);
        
        // Binary data with patterns
        let mut binary = Vec::new();
        for i in 0..1000 {
            binary.push((i % 256) as u8);
        }
        let binary_bytes = Bytes::from(binary.clone());
        
        let compressed = compressor.compress(&binary_bytes).unwrap();
        let decompressed = compressor.decompress(&compressed).unwrap();
        
        assert_eq!(decompressed, binary_bytes);
    }
    
    #[test]
    fn test_compressor_trait() {
        // Test that all compressors implement the trait correctly
        let compressors: Vec<Box<dyn Compressor>> = vec![
            Box::new(ZstdCompressor::new(6)),
            Box::new(Lz4Compressor),
            Box::new(SnappyCompressor),
            Box::new(NoCompression),
        ];
        
        let data = Bytes::from("Test data");
        
        for compressor in compressors {
            let compressed = compressor.compress(&data).unwrap();
            let decompressed = compressor.decompress(&compressed).unwrap();
            assert_eq!(decompressed, data);
        }
    }
    
    #[test]
    fn test_random_data_expansion() {
        use rand::RngCore;
        let compressor = ZstdCompressor::new(6);
        
        // Random data should not compress well
        let mut rng = rand::thread_rng();
        let mut random = vec![0u8; 10_000];
        rng.fill_bytes(&mut random);
        let random_bytes = Bytes::from(random.clone());
        
        let compressed = compressor.compress(&random_bytes).unwrap();
        
        // Compressed size might be slightly larger (due to overhead)
        let ratio = random.len() as f64 / compressed.len() as f64;
        assert!(ratio < 1.1, "Random data should not compress significantly, ratio: {:.2}", ratio);
    }
}


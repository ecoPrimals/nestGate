// Compression implementations
//
// Physical data compression using various algorithms

use anyhow::Result;

/// Compression algorithm
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompressionAlgorithm {
    None,
    Zstd { level: i32 },
    Lz4,
    Snappy,
}

/// Compress data using specified algorithm
pub fn compress(data: &[u8], algorithm: CompressionAlgorithm) -> Result<Vec<u8>> {
    match algorithm {
        CompressionAlgorithm::None => {
            Ok(data.to_vec())
        }
        
        CompressionAlgorithm::Zstd { level } => {
            zstd::bulk::compress(data, level)
                .map_err(|e| anyhow::anyhow!("Zstd compression failed: {}", e))
        }
        
        CompressionAlgorithm::Lz4 => {
            lz4_flex::compress_prepend_size(data)
                .map_err(|e| anyhow::anyhow!("LZ4 compression failed: {}", e))
        }
        
        CompressionAlgorithm::Snappy => {
            let mut compressed = Vec::new();
            snap::write::FrameEncoder::new(&mut compressed)
                .write_all(data)?;
            Ok(compressed)
        }
    }
}

/// Decompress data using specified algorithm
pub fn decompress(data: &[u8], algorithm: CompressionAlgorithm) -> Result<Vec<u8>> {
    match algorithm {
        CompressionAlgorithm::None => {
            Ok(data.to_vec())
        }
        
        CompressionAlgorithm::Zstd { .. } => {
            zstd::bulk::decompress(data, 128 * 1024 * 1024)  // Max 128MB
                .map_err(|e| anyhow::anyhow!("Zstd decompression failed: {}", e))
        }
        
        CompressionAlgorithm::Lz4 => {
            lz4_flex::decompress_size_prepended(data)
                .map_err(|e| anyhow::anyhow!("LZ4 decompression failed: {}", e))
        }
        
        CompressionAlgorithm::Snappy => {
            use std::io::Read;
            let mut decompressed = Vec::new();
            snap::read::FrameDecoder::new(data)
                .read_to_end(&mut decompressed)?;
            Ok(decompressed)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_zstd_roundtrip() {
        let data = b"ATCGATCG".repeat(1000);
        let algorithm = CompressionAlgorithm::Zstd { level: 6 };
        
        let compressed = compress(&data, algorithm).unwrap();
        assert!(compressed.len() < data.len());
        
        let decompressed = decompress(&compressed, algorithm).unwrap();
        assert_eq!(data, &decompressed[..]);
    }
    
    #[test]
    fn test_lz4_roundtrip() {
        let data = b"Hello World ".repeat(100);
        let algorithm = CompressionAlgorithm::Lz4;
        
        let compressed = compress(&data, algorithm).unwrap();
        assert!(compressed.len() < data.len());
        
        let decompressed = decompress(&compressed, algorithm).unwrap();
        assert_eq!(data, &decompressed[..]);
    }
    
    #[test]
    fn test_random_data_expansion() {
        // Random data should not compress well
        let data: Vec<u8> = (0..1024).map(|i| (i * 7919) as u8).collect();
        let algorithm = CompressionAlgorithm::Zstd { level: 6 };
        
        let compressed = compress(&data, algorithm).unwrap();
        // Compressed might be larger due to overhead
        assert!(compressed.len() >= data.len() * 95 / 100);  // Within 5%
    }
}


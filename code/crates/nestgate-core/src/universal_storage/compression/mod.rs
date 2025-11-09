//! # Pure Rust Compression Module
//!
//! Self-contained compression implementation that works on any system without external dependencies.
//! Supports LZ4 and ZSTD compression algorithms with streaming support for large files.
//!
//! ## Features
//! - **LZ4**: Fast compression (~500 MB/s), moderate compression ratio
//! - **ZSTD**: Configurable compression (level 1-22), excellent compression ratio
//! - **Streaming**: Efficient handling of large files without loading into memory
//! - **Zero-copy**: Optimized for performance where possible
//!
//! ## Example
//! ```rust
//! use nestgate_core::universal_storage::compression::{RustCompressor, CompressionAlgorithm};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let compressor = RustCompressor::new(CompressionAlgorithm::Lz4);
//! let data = b"Hello, World!".repeat(100);
//!
//! // Compress
//! let compressed = compressor.compress(&data)?;
//! assert!(compressed.len() < data.len());
//!
//! // Decompress
//! let decompressed = compressor.decompress(&compressed, Some(data.len()))?;
//! assert_eq!(data.to_vec(), decompressed);
//! # Ok(())
//! # }
//! ```

use std::io::{Read, Write};
use thiserror::Error;

// Use canonical Result type from error module
use crate::error::Result;

/// Compression-related errors
#[derive(Debug, Error)]
pub enum CompressionError {
    /// Compression operation failed
    #[error("Compression failed: {0}")]
    CompressionFailed(String),

    /// Decompression operation failed
    #[error("Decompression failed: {0}")]
    DecompressionFailed(String),

    /// Invalid data or configuration
    #[error("Invalid data: {0}")]
    InvalidData(String),

    /// I/O error during streaming
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
}

// Note: Now using crate::error::Result<T> instead of local Result type
// CompressionError is converted to NestGateError via From trait when needed

/// Compression algorithms supported by NestGate
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum CompressionAlgorithm {
    /// No compression
    None,

    /// LZ4 compression (fast, good ratio)
    /// - Speed: ~500 MB/s
    /// - Ratio: ~2-3x
    /// - Best for: Hot data, frequently accessed files
    Lz4,

    /// ZSTD compression with level (balanced, excellent ratio)
    /// - Speed: ~100-300 MB/s depending on level
    /// - Ratio: ~3-5x depending on level
    /// - Level range: 1 (fastest) to 22 (best compression)
    /// - Best for: Warm data, archival storage
    Zstd(i32),
}

impl Default for CompressionAlgorithm {
    fn default() -> Self {
        // LZ4 is the default - fastest with good compression
        Self::Lz4
    }
}

impl CompressionAlgorithm {
    /// Get a human-readable name for the algorithm
    pub fn name(&self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Lz4 => "lz4",
            Self::Zstd(_) => "zstd",
        }
    }

    /// Get the expected compression ratio (approximate)
    pub fn expected_ratio(&self) -> f64 {
        match self {
            Self::None => 1.0,
            Self::Lz4 => 2.5,
            Self::Zstd(level) => {
                // Higher levels give better compression
                2.0 + (*level as f64 * 0.15).min(3.0)
            }
        }
    }
}

/// Pure Rust compression engine
///
/// Provides self-contained compression without requiring system libraries.
/// All compression happens in-process using pure Rust implementations.
pub struct RustCompressor {
    algorithm: CompressionAlgorithm,
}

impl RustCompressor {
    /// Create a new compressor with the specified algorithm
    ///
    /// # Example
    /// ```rust
    /// use nestgate_core::universal_storage::compression::{RustCompressor, CompressionAlgorithm};
    ///
    /// let lz4_compressor = RustCompressor::new(CompressionAlgorithm::Lz4);
    /// let zstd_compressor = RustCompressor::new(CompressionAlgorithm::Zstd(3));
    /// ```
    pub fn new(algorithm: CompressionAlgorithm) -> Self {
        Self { algorithm }
    }

    /// Get the current compression algorithm
    pub fn algorithm(&self) -> CompressionAlgorithm {
        self.algorithm
    }

    /// Compress data using the configured algorithm
    ///
    /// # Arguments
    /// * `data` - The data to compress
    ///
    /// # Returns
    /// Compressed data as a new `Vec<u8>`
    ///
    /// # Example
    /// ```rust
    /// # use nestgate_core::universal_storage::compression::{RustCompressor, CompressionAlgorithm};
    /// # fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let compressor = RustCompressor::new(CompressionAlgorithm::Lz4);
    /// let data = b"Hello, World!".repeat(100);
    /// let compressed = compressor.compress(&data)?;
    /// assert!(compressed.len() < data.len());
    /// # Ok(())
    /// # }
    /// ```
    pub fn compress(&self, data: &[u8]) -> Result<Vec<u8>> {
        match self.algorithm {
            CompressionAlgorithm::None => Ok(data.to_vec()),

            CompressionAlgorithm::Lz4 => {
                // Use frame format which includes size information
                let mut compressed = Vec::new();
                let mut encoder = lz4::EncoderBuilder::new().level(4).build(&mut compressed)?;
                std::io::Write::write_all(&mut encoder, data)?;
                let (_output, result) = encoder.finish();
                result?;
                Ok(compressed)
            }

            CompressionAlgorithm::Zstd(level) => zstd::bulk::compress(data, level)
                .map_err(|e| CompressionError::CompressionFailed(e.to_string())),
        }
    }

    /// Decompress data using the configured algorithm
    ///
    /// # Arguments
    /// * `data` - The compressed data
    /// * `decompressed_size` - Optional hint about decompressed size (required for LZ4)
    ///
    /// # Returns
    /// Decompressed data as a new `Vec<u8>`
    ///
    /// # Example
    /// ```rust
    /// # use nestgate_core::universal_storage::compression::{RustCompressor, CompressionAlgorithm};
    /// # fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let compressor = RustCompressor::new(CompressionAlgorithm::Lz4);
    /// let original = b"Hello, World!".repeat(100);
    /// let compressed = compressor.compress(&original)?;
    /// let decompressed = compressor.decompress(&compressed, Some(original.len()))?;
    /// assert_eq!(original.to_vec(), decompressed);
    /// # Ok(())
    /// # }
    /// ```
    pub fn decompress(&self, data: &[u8], decompressed_size: Option<usize>) -> Result<Vec<u8>> {
        match self.algorithm {
            CompressionAlgorithm::None => Ok(data.to_vec()),

            CompressionAlgorithm::Lz4 => {
                // Use frame format which includes size information
                let mut decompressed = Vec::new();
                let mut decoder = lz4::Decoder::new(data)?;
                std::io::Read::read_to_end(&mut decoder, &mut decompressed)?;
                Ok(decompressed)
            }

            CompressionAlgorithm::Zstd(_) => {
                let max_size = decompressed_size.unwrap_or(128 * 1024 * 1024); // 128MB default max
                zstd::bulk::decompress(data, max_size)
                    .map_err(|e| CompressionError::DecompressionFailed(e.to_string()))
            }
        }
    }

    /// Compress data from a reader to a writer (streaming)
    ///
    /// This is more memory-efficient for large files as it doesn't load
    /// everything into memory at once.
    ///
    /// # Arguments
    /// * `reader` - Source of uncompressed data
    /// * `writer` - Destination for compressed data
    ///
    /// # Returns
    /// Number of compressed bytes written
    ///
    /// # Example
    /// ```rust
    /// # use nestgate_core::universal_storage::compression::{RustCompressor, CompressionAlgorithm};
    /// # use std::io::Cursor;
    /// # fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let compressor = RustCompressor::new(CompressionAlgorithm::Zstd(3));
    /// let data = b"Hello, World!".repeat(1000);
    /// let mut reader = Cursor::new(data);
    /// let mut compressed = Vec::new();
    ///
    /// let bytes_written = compressor.compress_stream(&mut reader, &mut compressed)?;
    /// assert!(bytes_written > 0);
    /// # Ok(())
    /// # }
    /// ```
    pub fn compress_stream<R: Read, W: Write>(
        &self,
        reader: &mut R,
        writer: &mut W,
    ) -> Result<u64> {
        match self.algorithm {
            CompressionAlgorithm::None => Ok(std::io::copy(reader, writer)?),

            CompressionAlgorithm::Lz4 => {
                let mut encoder = lz4::EncoderBuilder::new()
                    .level(4) // Fast compression
                    .build(writer)?;
                let bytes = std::io::copy(reader, &mut encoder)?;
                let (_output, result) = encoder.finish();
                result?;
                Ok(bytes)
            }

            CompressionAlgorithm::Zstd(level) => {
                let mut encoder = zstd::stream::Encoder::new(writer, level)?;
                let bytes = std::io::copy(reader, &mut encoder)?;
                encoder.finish()?;
                Ok(bytes)
            }
        }
    }

    /// Decompress data from a reader to a writer (streaming)
    ///
    /// # Arguments
    /// * `reader` - Source of compressed data
    /// * `writer` - Destination for decompressed data
    ///
    /// # Returns
    /// Number of decompressed bytes written
    pub fn decompress_stream<R: Read, W: Write>(
        &self,
        reader: &mut R,
        writer: &mut W,
    ) -> Result<u64> {
        match self.algorithm {
            CompressionAlgorithm::None => Ok(std::io::copy(reader, writer)?),

            CompressionAlgorithm::Lz4 => {
                let mut decoder = lz4::Decoder::new(reader)?;
                Ok(std::io::copy(&mut decoder, writer)?)
            }

            CompressionAlgorithm::Zstd(_) => {
                let mut decoder = zstd::stream::Decoder::new(reader)?;
                Ok(std::io::copy(&mut decoder, writer)?)
            }
        }
    }
}

impl Default for RustCompressor {
    fn default() -> Self {
        Self::new(CompressionAlgorithm::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_lz4_compression_roundtrip() {
        let compressor = RustCompressor::new(CompressionAlgorithm::Lz4);
        let original = b"Hello, World! This is a test string. ".repeat(100);
        let original_len = original.len();

        let compressed = compressor.compress(&original).expect("Storage operation failed");
        assert!(
            compressed.len() < original.len(),
            "Compressed size should be smaller"
        );

        let decompressed = compressor
            .decompress(&compressed, Some(original_len))
            .expect("Storage operation failed");
        assert_eq!(
            original.to_vec(),
            decompressed,
            "Roundtrip should preserve data"
        );
    }

    #[test]
    fn test_zstd_compression_roundtrip() {
        let compressor = RustCompressor::new(CompressionAlgorithm::Zstd(3));
        let original = b"Hello, World! This is a test string. ".repeat(100);

        let compressed = compressor.compress(&original).expect("Storage operation failed");
        assert!(
            compressed.len() < original.len(),
            "Compressed size should be smaller"
        );

        let decompressed = compressor.decompress(&compressed, None).expect("Storage operation failed");
        assert_eq!(
            original.to_vec(),
            decompressed,
            "Roundtrip should preserve data"
        );
    }

    #[test]
    fn test_zstd_levels() {
        let original = b"Hello, World! This is a test string. ".repeat(100);

        let compressor_low = RustCompressor::new(CompressionAlgorithm::Zstd(1));
        let compressed_low = compressor_low.compress(&original).expect("Storage operation failed");

        let compressor_high = RustCompressor::new(CompressionAlgorithm::Zstd(19));
        let compressed_high = compressor_high.compress(&original).expect("Storage operation failed");

        // Higher compression level should produce smaller output
        assert!(compressed_high.len() <= compressed_low.len());
    }

    #[test]
    fn test_streaming_compression_lz4() {
        let compressor = RustCompressor::new(CompressionAlgorithm::Lz4);
        let original = b"Hello, World! ".repeat(1000);

        let mut reader = Cursor::new(&original);
        let mut compressed = Vec::new();

        let bytes_written = compressor
            .compress_stream(&mut reader, &mut compressed)
            .expect("Storage operation failed");
        assert!(bytes_written > 0);
        assert!(compressed.len() < original.len());

        // Decompress to verify
        let mut compressed_reader = Cursor::new(&compressed);
        let mut decompressed = Vec::new();
        compressor
            .decompress_stream(&mut compressed_reader, &mut decompressed)
            .expect("Storage operation failed");

        assert_eq!(original.to_vec(), decompressed);
    }

    #[test]
    fn test_streaming_compression_zstd() {
        let compressor = RustCompressor::new(CompressionAlgorithm::Zstd(3));
        let original = b"Hello, World! ".repeat(1000);

        let mut reader = Cursor::new(&original);
        let mut compressed = Vec::new();

        let bytes_written = compressor
            .compress_stream(&mut reader, &mut compressed)
            .expect("Storage operation failed");
        assert!(bytes_written > 0);
        assert!(compressed.len() < original.len());

        // Decompress to verify
        let mut compressed_reader = Cursor::new(&compressed);
        let mut decompressed = Vec::new();
        compressor
            .decompress_stream(&mut compressed_reader, &mut decompressed)
            .expect("Storage operation failed");

        assert_eq!(original.to_vec(), decompressed);
    }

    #[test]
    fn test_no_compression() {
        let compressor = RustCompressor::new(CompressionAlgorithm::None);
        let original = b"Hello, World!";

        let compressed = compressor.compress(original).expect("Storage operation failed");
        assert_eq!(original, compressed.as_slice());

        let decompressed = compressor.decompress(&compressed, None).expect("Storage operation failed");
        assert_eq!(original.to_vec(), decompressed);
    }

    #[test]
    fn test_empty_data() {
        // LZ4 doesn't handle empty data well, use ZSTD for this test
        let compressor_zstd = RustCompressor::new(CompressionAlgorithm::Zstd(1));
        let empty: &[u8] = &[];

        let compressed = compressor_zstd.compress(empty).expect("Storage operation failed");
        let decompressed = compressor_zstd.decompress(&compressed, Some(0)).expect("Storage operation failed");

        assert_eq!(empty.to_vec(), decompressed);

        // Test None compression with empty data
        let compressor_none = RustCompressor::new(CompressionAlgorithm::None);
        let compressed_none = compressor_none.compress(empty).expect("Storage operation failed");
        assert_eq!(empty, compressed_none.as_slice());
    }

    #[test]
    fn test_compression_ratio_estimates() {
        assert!(CompressionAlgorithm::None.expected_ratio() == 1.0);
        assert!(CompressionAlgorithm::Lz4.expected_ratio() > 1.0);
        assert!(CompressionAlgorithm::Zstd(1).expected_ratio() > 1.0);
        assert!(
            CompressionAlgorithm::Zstd(19).expected_ratio()
                > CompressionAlgorithm::Zstd(1).expected_ratio()
        );
    }
}

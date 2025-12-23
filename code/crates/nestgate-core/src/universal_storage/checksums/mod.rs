//! # Pure Rust Checksum Module
//!
//! Self-contained checksum implementation for data integrity verification.
//! Supports Blake3 (fastest) and SHA-256 (standard) with streaming support.
//!
//! ## Features
//! - **Blake3**: Extremely fast (~1 GB/s), cryptographically secure
//! - **SHA-256**: Industry standard, widely compatible
//! - **Streaming**: Efficient handling of large files
//! - **Corruption Detection**: Verify data integrity
//!
//! ## Example
//! ```rust
//! use nestgate_core::universal_storage::checksums::{RustChecksummer, ChecksumAlgorithm};
//!
//! # fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let checksummer = RustChecksummer::new(ChecksumAlgorithm::Blake3);
//! let data = b"Hello, World!";
//!
//! // Calculate checksum
//! let checksum = checksummer.checksum(data);
//!
//! // Verify integrity
//! assert!(checksummer.verify(data, &checksum));
//!
//! // Detect corruption
//! let corrupted = b"Hello, World?";
//! assert!(!checksummer.verify(corrupted, &checksum));
//! # Ok(())
//! # }
//! ```

use blake3::Hasher as Blake3Hasher;
use sha2::{Digest, Sha256};
use std::path::Path;
use thiserror::Error;
use tokio::io::AsyncReadExt;

// Use canonical Result type from error module
use crate::error::Result;

/// Checksum-related errors
#[derive(Debug, Error)]
/// Errors that can occur during Checksum operations
pub enum ChecksumError {
    /// I/O error during checksum calculation
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    /// Invalid checksum format or data
    #[error("Invalid checksum: {0}")]
    InvalidChecksum(String),

    /// File not found
    #[error("File not found: {0}")]
    FileNotFound(String),
}

// Note: Now using crate::error::Result<T> instead of local Result type
// ChecksumError is converted to NestGateError via From trait when needed

/// Checksum algorithms supported by NestGate
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
/// Checksumalgorithm
pub enum ChecksumAlgorithm {
    /// No checksum
    None,

    /// Blake3 checksum (fastest, ~1 GB/s)
    /// - Speed: ~1-3 GB/s
    /// - Size: 32 bytes (256 bits)
    /// - Best for: All use cases (fastest + secure)
    Blake3,

    /// SHA-256 checksum (industry standard)
    /// - Speed: ~100-500 MB/s
    /// - Size: 32 bytes (256 bits)
    /// - Best for: Compatibility, regulatory requirements
    Sha256,
}

impl Default for ChecksumAlgorithm {
    /// Returns the default instance
    fn default() -> Self {
        // Blake3 is the default - fastest and secure
        Self::Blake3
    }
}

impl ChecksumAlgorithm {
    /// Get a human-readable name for the algorithm
    pub fn name(&self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Blake3 => "blake3",
            Self::Sha256 => "sha256",
        }
    }

    /// Get the checksum size in bytes
    pub fn size(&self) -> usize {
        match self {
            Self::None => 0,
            Self::Blake3 => 32, // 256 bits
            Self::Sha256 => 32, // 256 bits
        }
    }

    /// Get expected throughput (MB/s, approximate)
    pub fn expected_throughput(&self) -> u64 {
        match self {
            Self::None => u64::MAX,
            Self::Blake3 => 1500, // ~1.5 GB/s
            Self::Sha256 => 300,  // ~300 MB/s
        }
    }
}

/// Pure Rust checksum engine
///
/// Provides self-contained checksums for data integrity verification.
/// All checksum calculations happen in-process using pure Rust implementations.
pub struct RustChecksummer {
    algorithm: ChecksumAlgorithm,
}

impl RustChecksummer {
    /// Create a new checksummer with the specified algorithm
    ///
    /// # Example
    /// ```rust
    /// use nestgate_core::universal_storage::checksums::{RustChecksummer, ChecksumAlgorithm};
    ///
    /// let blake3_checksummer = RustChecksummer::new(ChecksumAlgorithm::Blake3);
    /// let sha256_checksummer = RustChecksummer::new(ChecksumAlgorithm::Sha256);
    /// ```
    pub fn new(algorithm: ChecksumAlgorithm) -> Self {
        Self { algorithm }
    }

    /// Get the current checksum algorithm
    pub fn algorithm(&self) -> ChecksumAlgorithm {
        self.algorithm
    }

    /// Calculate checksum for data
    ///
    /// # Arguments
    /// * `data` - The data to checksum
    ///
    /// # Returns
    /// Checksum bytes (32 bytes for Blake3/SHA-256, 0 for None)
    ///
    /// # Example
    /// ```rust
    /// # use nestgate_core::universal_storage::checksums::{RustChecksummer, ChecksumAlgorithm};
    /// # fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let checksummer = RustChecksummer::new(ChecksumAlgorithm::Blake3);
    /// let data = b"Hello, World!";
    /// let checksum = checksummer.checksum(data);
    /// assert_eq!(checksum.len(), 32);  // Blake3 produces 32 bytes
    /// # Ok(())
    /// # }
    /// ```
    pub fn checksum(&self, data: &[u8]) -> Vec<u8> {
        match self.algorithm {
            ChecksumAlgorithm::None => vec![],

            ChecksumAlgorithm::Blake3 => blake3::hash(data).as_bytes().to_vec(),

            ChecksumAlgorithm::Sha256 => {
                let mut hasher = Sha256::new();
                hasher.update(data);
                hasher.finalize().to_vec()
            }
        }
    }

    /// Verify that data matches the expected checksum
    ///
    /// # Arguments
    /// * `data` - The data to verify
    /// * `expected` - The expected checksum
    ///
    /// # Returns
    /// `true` if checksum matches, `false` if corrupted
    ///
    /// # Example
    /// ```rust
    /// # use nestgate_core::universal_storage::checksums::{RustChecksummer, ChecksumAlgorithm};
    /// # fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let checksummer = RustChecksummer::new(ChecksumAlgorithm::Blake3);
    /// let data = b"Hello, World!";
    /// let checksum = checksummer.checksum(data);
    ///
    /// // Verify original data
    /// assert!(checksummer.verify(data, &checksum));
    ///
    /// // Detect corruption
    /// let corrupted = b"Hello, World?";
    /// assert!(!checksummer.verify(corrupted, &checksum));
    /// # Ok(())
    /// # }
    /// ```
    pub fn verify(&self, data: &[u8], expected: &[u8]) -> bool {
        let actual = self.checksum(data);

        // Constant-time comparison to prevent timing attacks
        if actual.len() != expected.len() {
            return false;
        }

        let mut result = 0u8;
        for (a, b) in actual.iter().zip(expected.iter()) {
            result |= a ^ b;
        }

        result == 0
    }

    /// Calculate checksum for a file (async, streaming)
    ///
    /// This is memory-efficient for large files as it reads in chunks.
    ///
    /// # Arguments
    /// * `path` - Path to the file
    ///
    /// # Returns
    /// Checksum bytes
    ///
    /// # Example
    /// ```rust,no_run
    /// # use nestgate_core::universal_storage::checksums::{RustChecksummer, ChecksumAlgorithm};
    /// # use std::path::Path;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let checksummer = RustChecksummer::new(ChecksumAlgorithm::Blake3);
    /// let checksum = checksummer.checksum_file(Path::new("large_file.dat")).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn checksum_file(&self, path: &Path) -> Result<Vec<u8>> {
        match self.algorithm {
            ChecksumAlgorithm::None => Ok(vec![]),

            ChecksumAlgorithm::Blake3 => {
                let mut hasher = Blake3Hasher::new();
                let mut file = tokio::fs::File::open(path).await?;
                let mut buffer = vec![0u8; 65536]; // 64KB buffer

                loop {
                    let n = file.read(&mut buffer).await?;
                    if n == 0 {
                        break;
                    }
                    hasher.update(&buffer[..n]);
                }

                Ok(hasher.finalize().as_bytes().to_vec())
            }

            ChecksumAlgorithm::Sha256 => {
                let mut hasher = Sha256::new();
                let mut file = tokio::fs::File::open(path).await?;
                let mut buffer = vec![0u8; 65536]; // 64KB buffer

                loop {
                    let n = file.read(&mut buffer).await?;
                    if n == 0 {
                        break;
                    }
                    hasher.update(&buffer[..n]);
                }

                Ok(hasher.finalize().to_vec())
            }
        }
    }

    /// Verify that a file matches the expected checksum
    ///
    /// # Arguments
    /// * `path` - Path to the file
    /// * `expected` - The expected checksum
    ///
    /// # Returns
    /// `true` if file checksum matches, `false` if corrupted
    ///
    /// # Example
    /// ```rust,no_run
    /// # use nestgate_core::universal_storage::checksums::{RustChecksummer, ChecksumAlgorithm};
    /// # use std::path::Path;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let checksummer = RustChecksummer::new(ChecksumAlgorithm::Blake3);
    /// let checksum = checksummer.checksum_file(Path::new("file.dat")).await?;
    ///
    /// // Later, verify the file hasn't been corrupted
    /// if checksummer.verify_file(Path::new("file.dat"), &checksum).await? {
    ///     println!("File integrity verified ✅");
    /// } else {
    ///     println!("File corrupted! ❌");
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn verify_file(&self, path: &Path, expected: &[u8]) -> Result<bool> {
        let actual = self.checksum_file(path).await?;

        // Direct constant-time comparison of checksums (don't re-checksum!)
        if actual.len() != expected.len() {
            return Ok(false);
        }

        let mut result = 0u8;
        for (a, b) in actual.iter().zip(expected.iter()) {
            result |= a ^ b;
        }

        Ok(result == 0)
    }

    /// Create a checksummer optimized for the given use case
    ///
    /// # Arguments
    /// * `fast` - If true, use Blake3 (fastest). If false, use SHA-256 (standard)
    ///
    /// # Example
    /// ```rust
    /// # use nestgate_core::universal_storage::checksums::RustChecksummer;
    /// // For performance-critical paths
    /// let fast = RustChecksummer::for_use_case(true);
    ///
    /// // For regulatory/compliance requirements
    /// let standard = RustChecksummer::for_use_case(false);
    /// ```
    pub fn for_use_case(fast: bool) -> Self {
        if fast {
            Self::new(ChecksumAlgorithm::Blake3)
        } else {
            Self::new(ChecksumAlgorithm::Sha256)
        }
    }

    /// Calculate checksum with incremental updates (streaming)
    ///
    /// Useful for calculating checksums while processing data.
    ///
    /// # Example
    /// ```rust
    /// # use nestgate_core::universal_storage::checksums::{RustChecksummer, ChecksumAlgorithm};
    /// # fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let checksummer = RustChecksummer::new(ChecksumAlgorithm::Blake3);
    ///
    /// // Process data in chunks
    /// let mut state = checksummer.start_incremental();
    /// state.update(b"Hello, ");
    /// state.update(b"World!");
    /// let checksum = state.finalize();
    ///
    /// // Same as checksumming all at once
    /// let direct = checksummer.checksum(b"Hello, World!");
    /// assert_eq!(checksum, direct);
    /// # Ok(())
    /// # }
    /// ```
    pub fn start_incremental(&self) -> IncrementalChecksum {
        match self.algorithm {
            ChecksumAlgorithm::None => IncrementalChecksum::None,
            ChecksumAlgorithm::Blake3 => IncrementalChecksum::Blake3(Box::new(Blake3Hasher::new())),
            ChecksumAlgorithm::Sha256 => IncrementalChecksum::Sha256(Box::new(Sha256::new())),
        }
    }
}

impl Default for RustChecksummer {
    /// Returns the default instance
    fn default() -> Self {
        Self::new(ChecksumAlgorithm::default())
    }
}

/// Incremental checksum state for streaming calculations
pub enum IncrementalChecksum {
    /// None
    None,
    Blake3(Box<Blake3Hasher>),
    Sha256(Box<Sha256>),
}

impl IncrementalChecksum {
    /// Update checksum with more data
    pub fn update(&mut self, data: &[u8]) {
        match self {
            Self::None => (),
            Self::Blake3(hasher) => {
                hasher.update(data);
            }
            Self::Sha256(hasher) => {
                hasher.update(data);
            }
        }
    }

    /// Finalize and return the checksum
    pub fn finalize(self) -> Vec<u8> {
        match self {
            Self::None => vec![],
            Self::Blake3(hasher) => hasher.finalize().as_bytes().to_vec(),
            Self::Sha256(hasher) => hasher.finalize().to_vec(),
        }
    }
}

/// Format checksum as hexadecimal string
pub fn checksum_to_hex(checksum: &[u8]) -> String {
    checksum.iter().map(|b| format!("{:02x}", b)).collect()
}

/// Parse hexadecimal string back to checksum bytes
pub fn hex_to_checksum(hex: &str) -> Result<Vec<u8>> {
    if !hex.len().is_multiple_of(2) {
        return Err(ChecksumError::InvalidChecksum(
            "Hex string must have even length".into(),
        ));
    }

    (0..hex.len())
        .step_by(2)
        .map(|i| {
            u8::from_str_radix(&hex[i..i + 2], 16)
                .map_err(|e| ChecksumError::InvalidChecksum(e.to_string()))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blake3_checksum() {
        let checksummer = RustChecksummer::new(ChecksumAlgorithm::Blake3);
        let data = b"Hello, World!";

        let checksum1 = checksummer.checksum(data);
        let checksum2 = checksummer.checksum(data);

        // Same data should produce same checksum
        assert_eq!(checksum1, checksum2);
        assert_eq!(checksum1.len(), 32); // Blake3 is 256 bits

        // Should verify correctly
        assert!(checksummer.verify(data, &checksum1));
    }

    #[test]
    fn test_sha256_checksum() {
        let checksummer = RustChecksummer::new(ChecksumAlgorithm::Sha256);
        let data = b"Hello, World!";

        let checksum = checksummer.checksum(data);

        assert_eq!(checksum.len(), 32); // SHA-256 is 256 bits
        assert!(checksummer.verify(data, &checksum));
    }

    #[test]
    fn test_corruption_detection() {
        let checksummer = RustChecksummer::new(ChecksumAlgorithm::Blake3);
        let original = b"Hello, World!";
        let corrupted = b"Hello, World?";

        let checksum = checksummer.checksum(original);

        // Original should verify
        assert!(checksummer.verify(original, &checksum));

        // Corrupted should fail
        assert!(!checksummer.verify(corrupted, &checksum));
    }

    #[test]
    fn test_incremental_checksum() {
        let checksummer = RustChecksummer::new(ChecksumAlgorithm::Blake3);

        // Checksum in chunks
        let mut state = checksummer.start_incremental();
        state.update(b"Hello, ");
        state.update(b"World!");
        let incremental = state.finalize();

        // Checksum all at once
        let direct = checksummer.checksum(b"Hello, World!");

        // Should be identical
        assert_eq!(incremental, direct);
    }

    #[test]
    fn test_different_algorithms_produce_different_checksums() {
        let data = b"Hello, World!";

        let blake3 = RustChecksummer::new(ChecksumAlgorithm::Blake3);
        let sha256 = RustChecksummer::new(ChecksumAlgorithm::Sha256);

        let checksum_blake3 = blake3.checksum(data);
        let checksum_sha256 = sha256.checksum(data);

        // Different algorithms should produce different checksums
        assert_ne!(checksum_blake3, checksum_sha256);

        // But both should be 32 bytes
        assert_eq!(checksum_blake3.len(), 32);
        assert_eq!(checksum_sha256.len(), 32);
    }

    #[test]
    fn test_hex_conversion() {
        let checksummer = RustChecksummer::new(ChecksumAlgorithm::Blake3);
        let data = b"Hello, World!";
        let checksum = checksummer.checksum(data);

        // Convert to hex
        let hex = checksum_to_hex(&checksum);
        assert_eq!(hex.len(), 64); // 32 bytes = 64 hex chars

        // Convert back
        let parsed = hex_to_checksum(&hex).expect("Storage operation failed");
        assert_eq!(parsed, checksum);
    }

    #[test]
    fn test_no_checksum() {
        let checksummer = RustChecksummer::new(ChecksumAlgorithm::None);
        let data = b"Hello, World!";

        let checksum = checksummer.checksum(data);
        assert_eq!(checksum.len(), 0);
        assert!(checksummer.verify(data, &checksum));
    }

    #[test]
    fn test_empty_data() {
        let checksummer = RustChecksummer::new(ChecksumAlgorithm::Blake3);
        let empty: &[u8] = &[];

        let checksum = checksummer.checksum(empty);
        assert_eq!(checksum.len(), 32);
        assert!(checksummer.verify(empty, &checksum));
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_file_checksum() {
        use std::io::Write;

        let checksummer = RustChecksummer::new(ChecksumAlgorithm::Blake3);

        // Create temp file path with unique name
        let temp_dir = std::env::temp_dir();
        let file_path = temp_dir.join(format!("nestgate_checksum_test_{}.txt", std::process::id()));

        // Write test data synchronously
        {
            let mut file = std::fs::File::create(&file_path).expect("Storage operation failed");
            file.write_all(b"Hello, World!").expect("Storage operation failed");
            file.sync_all().expect("Storage operation failed");
        } // Ensure file is closed and flushed

        // Small delay to ensure filesystem sync
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;

        // Calculate checksum from file
        let file_checksum = checksummer.checksum_file(&file_path).await.expect("Storage operation failed");

        // Calculate checksum directly from data
        let direct_checksum = checksummer.checksum(b"Hello, World!");

        // They should match
        assert_eq!(
            file_checksum, direct_checksum,
            "File checksum should match direct checksum of same data"
        );

        // Verify using verify_file method
        let verify_result = checksummer.verify_file(&file_path, &file_checksum).await;
        match verify_result {
            Ok(is_valid) => {
                assert!(
                    is_valid,
                    "verify_file should confirm file integrity - got false but expected true"
                );
            }
            Err(e) => {
                assert!(false, "verify_file failed with error: {:?}", e);
            }
        }

        // Clean up - ignore errors in case file doesn't exist
        let _ = std::fs::remove_file(&file_path);
    }

    #[test]
    fn test_constant_time_verification() {
        let checksummer = RustChecksummer::new(ChecksumAlgorithm::Blake3);
        let data = b"Hello, World!";
        let checksum = checksummer.checksum(data);

        // Create slightly different checksum (1 bit flip)
        let mut wrong_checksum = checksum.clone();
        wrong_checksum[0] ^= 0x01;

        // Should still process in constant time (not leak timing info)
        assert!(!checksummer.verify(data, &wrong_checksum));
    }

    #[test]
    fn test_algorithm_metadata() {
        assert_eq!(ChecksumAlgorithm::Blake3.name(), "blake3");
        assert_eq!(ChecksumAlgorithm::Sha256.name(), "sha256");
        assert_eq!(ChecksumAlgorithm::None.name(), "none");

        assert_eq!(ChecksumAlgorithm::Blake3.size(), 32);
        assert_eq!(ChecksumAlgorithm::Sha256.size(), 32);
        assert_eq!(ChecksumAlgorithm::None.size(), 0);

        assert!(
            ChecksumAlgorithm::Blake3.expected_throughput()
                > ChecksumAlgorithm::Sha256.expected_throughput()
        );
    }
}

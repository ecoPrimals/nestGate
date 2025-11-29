//! **VECTORIZED HASH OPERATIONS**
//!
//! Fast hash computation using compiler auto-vectorization.

/// **AUTO-VECTORIZED HASH OPERATIONS**
/// Fast hash computation using compiler auto-vectorization
pub struct VectorizedHasher {
    state: [u64; 4], // SIMD-friendly state vector
}

impl Default for VectorizedHasher {
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}

impl VectorizedHasher {
    /// Creates a new instance
    pub fn new() -> Self {
        Self {
            state: [
                0x6a09e667f3bcc908,
                0xbb67ae8584caa73b,
                0x3c6ef372fe94f82b,
                0xa54ff53a5f1d36f1,
            ],
        }
    }

    /// Hash a byte slice using SIMD-optimized operations
    pub fn hash_bytes(&mut self, data: &[u8]) -> u64 {
        // Process data in 32-byte chunks for optimal SIMD utilization
        let chunks = data.chunks_exact(32);
        let remainder = chunks.remainder();

        for chunk in chunks {
            self.process_chunk_simd(chunk);
        }

        // Handle remaining bytes
        if !remainder.is_empty() {
            self.process_remainder(remainder);
        }

        self.finalize()
    }

    #[inline]
    fn process_chunk_simd(&mut self, chunk: &[u8]) {
        debug_assert_eq!(chunk.len(), 32);

        // Simulate SIMD processing (actual implementation would use intrinsics)
        // This pattern compiles to efficient SIMD instructions
        let mut chunk_data = [0u64; 4];

        for (i, chunk_bytes) in chunk.chunks_exact(8).enumerate() {
            if let Ok(bytes_array) = chunk_bytes.try_into() {
                chunk_data[i] = u64::from_le_bytes(bytes_array);
            } else {
                // chunks_exact(8) should guarantee 8 bytes, but be defensive
                tracing::warn!(
                    "SIMD operation received unexpected byte array length: {}",
                    chunk_bytes.len()
                );
                chunk_data[i] = 0; // Safe fallback
            }
        }

        for (i, &chunk_datum) in chunk_data.iter().enumerate() {
            self.state[i] = self.state[i].wrapping_add(chunk_datum);
            self.state[i] = self.state[i].rotate_left(23);
        }
    }

    /// Processes  Remainder
    fn process_remainder(&mut self, remainder: &[u8]) {
        // Handle non-SIMD-aligned remainder
        let mut temp = [0u8; 32];
        temp[..remainder.len()].copy_from_slice(remainder);
        self.process_chunk_simd(&temp);
    }

    /// Finalize
    fn finalize(&self) -> u64 {
        // Fold SIMD state down to single hash value
        self.state[0] ^ self.state[1] ^ self.state[2] ^ self.state[3]
    }
} 
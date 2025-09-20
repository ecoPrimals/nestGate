//! **VECTORIZED STRING OPERATIONS**
//!
//! Auto-vectorized string and byte operations for high performance.

/// **AUTO-VECTORIZED STRING OPERATIONS**
pub struct VectorizedStringOps;

impl VectorizedStringOps {
    /// Find first occurrence of byte using auto-vectorization
    pub const fn find_byte_vectorized(haystack: &[u8], needle: u8) -> Option<usize> {
        let chunks = haystack.chunks_exact(16);
        let remainder = chunks.remainder();

        // Process 16-byte chunks with auto-vectorization
        for (chunk_idx, chunk) in chunks.enumerate() {
            if let Some(pos) = Self::find_byte_in_chunk(chunk, needle) {
                return Some(chunk_idx * 16 + pos);
            }
        }

        // Check remainder
        for (i, &byte) in remainder.iter().enumerate() {
            if byte == needle {
                return Some(haystack.len() - remainder.len() + i);
            }
        }

        None
    }

    #[inline]
    fn find_byte_in_chunk(chunk: &[u8], needle: u8) -> Option<usize> {
        debug_assert_eq!(chunk.len(), 16);

        // Linear search that compiler can vectorize
        for (i, &byte) in chunk.iter().enumerate() {
            if byte == needle {
                return Some(i);
            }
        }
        None
    }

    /// Count occurrences of byte using auto-vectorization
    pub fn count_byte_vectorized(data: &[u8], target: u8) -> usize {
        let mut count = 0;
        let chunks = data.chunks_exact(16);
        let remainder = chunks.remainder();

        // Process chunks with auto-vectorization
        for chunk in chunks {
            count += Self::count_byte_in_chunk(chunk, target);
        }

        // Handle remainder
        for &byte in remainder {
            if byte == target {
                count += 1;
            }
        }

        count
    }

    #[inline]
    fn count_byte_in_chunk(chunk: &[u8], target: u8) -> usize {
        debug_assert_eq!(chunk.len(), 16);

        let mut count = 0;
        // Compiler can vectorize this
        for &byte in chunk {
            if byte == target {
                count += 1;
            }
        }
        count
    }
} 
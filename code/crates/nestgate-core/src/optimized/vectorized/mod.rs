//! **VECTORIZED OPERATIONS MODULE**
//!
//! High-performance vectorized operations using compiler auto-vectorization.
//! This module provides SIMD-optimized operations for various data types.

pub mod array;
pub mod checksum;
pub mod hash;
pub mod string;

// Re-export all public types
pub use array::VectorizedArrayOps;
pub use checksum::VectorizedChecksum;
pub use hash::VectorizedHasher;
pub use string::VectorizedStringOps;

/// **TYPE ALIASES FOR AUTO-VECTORIZED OPERATIONS**
/// High-performance hasher for critical paths
pub type FastHasher = VectorizedHasher;
/// Auto-vectorized array operations
pub type VectorOps = VectorizedArrayOps;
/// High-speed string search
pub type FastStringSearch = VectorizedStringOps;
/// Accelerated checksum computation
pub type FastChecksum = VectorizedChecksum;

#[cfg(test)]
mod tests {
    use super::*;

    // ===== VECTORIZED HASHER TESTS =====

    #[test]
    fn test_vectorized_hasher_creation() {
        let hasher = VectorizedHasher::new();
        
        // Verify initial state is set correctly
        assert_ne!(hasher.hash_bytes(b"test"), 0);
    }

    #[test]
    fn test_vectorized_hasher_deterministic() {
        let mut hasher1 = VectorizedHasher::new();
        let mut hasher2 = VectorizedHasher::new();
        let data = b"Hello, vectorized world!";
        
        let hash1 = hasher1.hash_bytes(data);
        let hash2 = hasher2.hash_bytes(data);
        
        assert_eq!(hash1, hash2, "Hash should be deterministic");
    }

    #[test]
    fn test_vectorized_hasher_different_inputs() {
        let mut hasher1 = VectorizedHasher::new();
        let mut hasher2 = VectorizedHasher::new();
        
        let hash1 = hasher1.hash_bytes(b"input1");
        let hash2 = hasher2.hash_bytes(b"input2");
        
        assert_ne!(hash1, hash2, "Different inputs should produce different hashes");
    }

    // ===== VECTORIZED ARRAY OPERATIONS TESTS =====

    #[test]
    fn test_array_add() {
        let a = [1.0, 2.0, 3.0, 4.0];
        let b = [1.0, 1.0, 1.0, 1.0];
        let mut result = [0.0; 4];
        
        VectorizedArrayOps::add_arrays(&a, &b, &mut result);
        
        assert_eq!(result, [2.0, 3.0, 4.0, 5.0]);
    }

    #[test]
    fn test_dot_product() {
        let a = [1.0, 2.0, 3.0, 4.0];
        let b = [1.0, 1.0, 1.0, 1.0];
        
        let result = VectorizedArrayOps::dot_product(&a, &b);
        
        assert_eq!(result, 10.0); // 1*1 + 2*1 + 3*1 + 4*1 = 10
    }

    // ===== VECTORIZED STRING OPERATIONS TESTS =====

    #[test]
    fn test_find_byte() {
        let data = b"Hello, World!";
        let pos = VectorizedStringOps::find_byte_vectorized(data, b'W');
        assert_eq!(pos, Some(7));
    }

    #[test]
    fn test_count_byte() {
        let data = b"Hello, World!";
        let count = VectorizedStringOps::count_byte_vectorized(data, b'l');
        assert_eq!(count, 3);
    }

    // ===== VECTORIZED CHECKSUM TESTS =====

    #[test]
    fn test_xor_checksum() {
        let data = b"test";
        let checksum = VectorizedChecksum::xor_checksum_vectorized(data);
        assert_ne!(checksum, 0); // Should produce some checksum
    }

    // ===== TYPE ALIAS TESTS =====

    #[test]
    fn test_type_aliases() {
        let mut hasher = FastHasher::new();
        let hash = hasher.hash_bytes(b"test");
        assert_ne!(hash, 0);

        let data = b"search test";
        let pos = FastStringSearch::find_byte_vectorized(data, b't');
        assert!(pos.is_some());

        let checksum = FastChecksum::xor_checksum_vectorized(data);
        assert_ne!(checksum, 0);
    }
} 
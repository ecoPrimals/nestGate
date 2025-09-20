//! **VECTORIZED ARRAY OPERATIONS**
//!
//! Compiler auto-vectorized operations on f32 arrays.

/// **AUTO-VECTORIZED ARRAY OPERATIONS**
/// Compiler auto-vectorized operations on f32 arrays
pub struct VectorizedArrayOps;

impl VectorizedArrayOps {
    /// Add two arrays element-wise using auto-vectorization
    pub fn add_arrays(a: &[f32], b: &[f32], result: &mut [f32]) {
        assert_eq!(a.len(), b.len());
        assert_eq!(a.len(), result.len());

        // Process in vectorization-friendly chunks (8 f32s for optimal auto-vectorization)
        let chunks = a.len() / 8;
        let remainder = a.len() % 8;

        for i in 0..chunks {
            let base = i * 8;
            Self::add_vectorized_chunk(
                &a[base..base + 8],
                &b[base..base + 8],
                &mut result[base..base + 8],
            );
        }

        // Handle remainder
        for i in (chunks * 8)..(chunks * 8 + remainder) {
            result[i] = a[i] + b[i];
        }
    }

    #[inline]
    fn add_vectorized_chunk(a: &[f32], b: &[f32], result: &mut [f32]) {
        debug_assert_eq!(a.len(), 8);
        debug_assert_eq!(b.len(), 8);
        debug_assert_eq!(result.len(), 8);

        // Explicit vectorization pattern - compiler auto-vectorizes this
        for i in 0..8 {
            result[i] = a[i] + b[i];
        }
    }

    /// Dot product using auto-vectorization
    pub fn dot_product(a: &[f32], b: &[f32]) -> f32 {
        assert_eq!(a.len(), b.len());

        let mut result = 0.0;
        let chunks = a.len() / 8;
        let remainder = a.len() % 8;

        // Process vectorized chunks
        for i in 0..chunks {
            let base = i * 8;
            result += Self::dot_vectorized_chunk(&a[base..base + 8], &b[base..base + 8]);
        }

        // Handle remainder
        for i in (chunks * 8)..(chunks * 8 + remainder) {
            result += a[i] * b[i];
        }

        result
    }

    #[inline]
    fn dot_vectorized_chunk(a: &[f32], b: &[f32]) -> f32 {
        debug_assert_eq!(a.len(), 8);
        debug_assert_eq!(b.len(), 8);

        let mut sum = 0.0;
        // Compiler vectorizes this loop
        for i in 0..8 {
            sum += a[i] * b[i];
        }
        sum
    }

    /// Matrix multiplication using auto-vectorization (simplified 4x4)
    pub fn matrix_mult_4x4(a: &[[f32; 4]; 4], b: &[[f32; 4]; 4]) -> [[f32; 4]; 4] {
        let mut result = [[0.0; 4]; 4];

        for i in 0..4 {
            for j in 0..4 {
                // This pattern allows compiler auto-vectorization
                let mut sum = 0.0;
                for (k, &b_item) in b.iter().enumerate().take(4) {
                    sum += a[i][k] * b_item[j];
                }
                result[i][j] = sum;
            }
        }

        result
    }
} 
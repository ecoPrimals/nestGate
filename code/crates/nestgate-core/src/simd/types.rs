//! SIMD types and error definitions
//!
//! This module defines common types, errors, and structures used across
//! all SIMD optimization modules.

/// SIMD operation error types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SimdError {
    /// Input and output arrays have mismatched lengths
    LengthMismatch,
    /// Unsupported operation on current hardware
    UnsupportedOperation,
    /// Invalid alignment for SIMD operations
    InvalidAlignment,
    /// Buffer size exceeds maximum supported size
    BufferTooLarge,
}

impl std::fmt::Display for SimdError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LengthMismatch => write!(f, "Input and output arrays have different lengths"),
            Self::UnsupportedOperation => write!(f, "Operation not supported on current hardware"),
            Self::InvalidAlignment => write!(f, "Data not properly aligned for SIMD operations"),
            Self::BufferTooLarge => write!(f, "Buffer size exceeds maximum supported size"),
        }
    }
}

impl std::error::Error for SimdError {}

/// SIMD capability flags
#[derive(Debug, Clone, Default)]
pub struct SimdCapabilities {
    pub has_sse2: bool,
    pub has_avx: bool,
    pub has_avx2: bool,
    pub has_avx512: bool,
    pub has_neon: bool, // ARM NEON support
}

impl SimdCapabilities {
    /// Detect SIMD capabilities of the current CPU
    #[must_use]
    pub fn detect() -> Self {
        Self {
            has_sse2: is_x86_feature_detected!("sse2"),
            has_avx: is_x86_feature_detected!("avx"),
            has_avx2: is_x86_feature_detected!("avx2"),
            has_avx512: is_x86_feature_detected!("avx512f"),
            has_neon: cfg!(target_arch = "aarch64"), // Simplified NEON detection
        }
    }

    /// Get the best available SIMD instruction set
    #[must_use]
    pub fn best_instruction_set(&self) -> &'static str {
        if self.has_avx512 {
            "AVX-512"
        } else if self.has_avx2 {
            "AVX2"
        } else if self.has_avx {
            "AVX"
        } else if self.has_sse2 {
            "SSE2"
        } else if self.has_neon {
            "NEON"
        } else {
            "Scalar"
        }
    }

    /// Get expected performance multiplier for the best instruction set
    #[must_use]
    pub fn performance_multiplier(&self) -> f64 {
        if self.has_avx512 {
            16.0 // AVX-512 can process 16 f32s or 8 f64s
        } else if self.has_avx2 || self.has_avx {
            8.0 // AVX2/AVX can process 8 f32s or 4 f64s
        } else if self.has_sse2 || self.has_neon {
            4.0 // SSE2/NEON can process 4 f32s or 2 f64s
        } else {
            1.0 // Scalar operations
        }
    }
}

/// SIMD processing statistics
#[derive(Debug, Clone, Default)]
pub struct SimdStats {
    pub operations_processed: u64,
    pub total_elements: u64,
    pub simd_elements: u64,
    pub scalar_elements: u64,
    pub processing_time_ns: u64,
}

impl SimdStats {
    /// Calculate SIMD utilization ratio
    #[must_use]
    pub fn simd_utilization(&self) -> f64 {
        if self.total_elements == 0 {
            0.0
        } else {
            self.simd_elements as f64 / self.total_elements as f64
        }
    }

    /// Calculate elements processed per second
    #[must_use]
    pub fn elements_per_second(&self) -> f64 {
        if self.processing_time_ns == 0 {
            0.0
        } else {
            (self.total_elements as f64 * 1_000_000_000.0) / self.processing_time_ns as f64
        }
    }

    /// Reset statistics
    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simd_error_display() {
        assert_eq!(
            SimdError::LengthMismatch.to_string(),
            "Input and output arrays have different lengths"
        );
        assert_eq!(
            SimdError::UnsupportedOperation.to_string(),
            "Operation not supported on current hardware"
        );
    }

    #[test]
    fn test_simd_capabilities_detection() {
        let caps = SimdCapabilities::detect();
        let instruction_set = caps.best_instruction_set();
        let multiplier = caps.performance_multiplier();

        // These should always be valid
        assert!(!instruction_set.is_empty());
        assert!(multiplier >= 1.0);
        assert!(multiplier <= 16.0);
    }

    #[test]
    fn test_simd_stats_calculations() {
        let mut stats = SimdStats {
            total_elements: 1000,
            simd_elements: 800,
            scalar_elements: 200,
            processing_time_ns: 1_000_000, // 1ms
            ..Default::default()
        };

        assert_eq!(stats.simd_utilization(), 0.8);
        assert_eq!(stats.elements_per_second(), 1_000_000.0); // 1M elements/sec

        stats.reset();
        assert_eq!(stats.total_elements, 0);
        assert_eq!(stats.simd_utilization(), 0.0);
    }

    #[test]
    fn test_simd_capabilities_performance_multiplier() {
        let caps = SimdCapabilities {
            has_avx512: true,
            ..Default::default()
        };
        assert_eq!(caps.performance_multiplier(), 16.0);

        let caps = SimdCapabilities {
            has_avx2: true,
            ..Default::default()
        };
        assert_eq!(caps.performance_multiplier(), 8.0);

        let caps = SimdCapabilities::default();
        assert_eq!(caps.performance_multiplier(), 1.0);
    }
}

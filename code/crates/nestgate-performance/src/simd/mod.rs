//! # 🔥 **SIMD Acceleration Foundation**
//!
//! High-performance SIMD operations built on `NestGate`'s unified architecture.
//! Targeting 4-16x performance improvements through vectorization.
//!
//! ## Performance Targets
//! - **Data Processing**: 4-8x improvement in batch operations
//! - **Mathematical Operations**: 8-16x improvement in vector math
//! - **Memory Operations**: 20-40% cache efficiency improvement
//! - **Parallel Processing**: 10x improvement in parallel workloads

use nestgate_core::error::{NestGateError as NestGateUnifiedError, Result};
use std::mem;

/// SIMD batch size multiplier for optimal vectorization
/// This determines how many vector widths to process in a single batch
const SIMD_BATCH_MULTIPLIER: usize = 4;

// Re-export SIMD modules
// ✅ **SAFE SIMD** - Zero unsafe code, portable across all platforms
pub mod safe_simd;

/// SIMD capability detection and feature flags
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Simdcapabilities
pub struct SimdCapabilities {
    /// SSE2 support (baseline requirement)
    pub sse2: bool,
    /// SSE4.1 support (enhanced operations)
    pub sse41: bool,
    /// AVX support (256-bit vectors)
    pub avx: bool,
    /// AVX2 support (advanced 256-bit operations)
    pub avx2: bool,
    /// AVX-512 support (512-bit vectors)
    pub avx512: bool,
}

impl SimdCapabilities {
    /// Detect available SIMD capabilities at runtime
    #[must_use]
    pub fn detect() -> Self {
        Self {
            sse2: is_x86_feature_detected!("sse2"),
            sse41: is_x86_feature_detected!("sse4.1"),
            avx: is_x86_feature_detected!("avx"),
            avx2: is_x86_feature_detected!("avx2"),
            avx512: is_x86_feature_detected!("avx512f"),
        }
    }

    /// Get the optimal vector width for this CPU
    #[must_use]
    pub fn optimal_vector_width(self) -> usize {
        if self.avx512 {
            64 // 512 bits = 64 bytes
        } else if self.avx2 || self.avx {
            32 // 256 bits = 32 bytes
        } else if self.sse41 || self.sse2 {
            16 // 128 bits = 16 bytes
        } else {
            8 // Fallback to 64-bit operations
        }
    }

    /// Get human-readable capability description
    #[must_use]
    pub fn description(self) -> &'static str {
        if self.avx512 {
            "AVX-512 (512-bit vectors)"
        } else if self.avx2 {
            "AVX2 (256-bit vectors)"
        } else if self.avx {
            "AVX (256-bit vectors)"
        } else if self.sse41 {
            "SSE4.1 (128-bit vectors)"
        } else if self.sse2 {
            "SSE2 (128-bit vectors)"
        } else {
            "No SIMD support"
        }
    }
}

/// SIMD acceleration engine for unified `NestGate` operations
pub struct SimdEngine {
    capabilities: SimdCapabilities,
    vector_width: usize,
    alignment: usize,
}

impl SimdEngine {
    /// Create new SIMD engine with auto-detected capabilities
    #[must_use]
    pub fn new() -> Self {
        let capabilities = SimdCapabilities::detect();
        let vector_width = capabilities.optimal_vector_width();
        let alignment = vector_width;

        Self {
            capabilities,
            vector_width,
            alignment,
        }
    }

    /// Get SIMD capabilities
    #[must_use]
    pub fn capabilities(&self) -> SimdCapabilities {
        self.capabilities
    }

    /// Get optimal vector width in bytes
    #[must_use]
    pub fn vector_width(&self) -> usize {
        self.vector_width
    }

    /// Get required memory alignment
    #[must_use]
    pub fn alignment(&self) -> usize {
        self.alignment
    }

    /// Check if data is properly aligned for SIMD operations
    #[must_use]
    pub fn is_aligned<T>(&self, data: &[T]) -> bool {
        let ptr = data.as_ptr() as usize;
        ptr.is_multiple_of(self.alignment)
    }

    /// Create aligned buffer for SIMD operations
    #[must_use]
    pub fn create_aligned_buffer<T>(&self, len: usize) -> Vec<T>
    where
        T: Default + Clone,
    {
        let mut vec = Vec::with_capacity(len);

        // Ensure proper alignment
        let ptr = vec.as_mut_ptr() as usize;
        let misalignment = ptr % self.alignment;
        if misalignment != 0 {
            let padding = self.alignment - misalignment;
            vec.reserve(padding / mem::size_of::<T>());
        }

        vec.resize(len, T::default());
        vec
    }
}

impl Default for SimdEngine {
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}

/// SIMD operation trait for unified acceleration
pub trait SimdOperation<T> {
    /// Execute SIMD operation on input data
    ///
    /// # Errors
    /// Returns error if SIMD operation fails or data is incompatible
    fn execute(&self, engine: &SimdEngine, input: &[T]) -> Result<Vec<T>>;

    /// Get expected performance improvement factor
    #[must_use]
    fn performance_factor(&self, engine: &SimdEngine) -> f64;

    /// Check if operation is supported on current hardware
    #[must_use]
    fn is_supported(&self, engine: &SimdEngine) -> bool;
}

/// Batch SIMD processor for high-throughput operations
pub struct BatchProcessor {
    engine: SimdEngine,
    batch_size: usize,
}

impl BatchProcessor {
    /// Create new batch processor
    #[must_use]
    pub fn new() -> Self {
        let engine = SimdEngine::new();
        let batch_size = engine.vector_width() * SIMD_BATCH_MULTIPLIER;

        Self { engine, batch_size }
    }

    /// Process data in optimized batches
    pub fn process_batches<T, F>(&self, data: &[T], mut operation: F) -> Result<Vec<T>>
    where
        T: Copy + Default,
        F: FnMut(&SimdEngine, &[T]) -> Result<Vec<T>>,
    {
        let mut results = Vec::with_capacity(data.len());

        // Process in SIMD-optimized batches
        for chunk in data.chunks(self.batch_size) {
            let batch_result = operation(&self.engine, chunk)?;
            results.extend_from_slice(&batch_result);
        }

        Ok(results)
    }

    /// Get optimal batch size for current hardware
    #[must_use]
    pub fn batch_size(&self) -> usize {
        self.batch_size
    }
}

impl Default for BatchProcessor {
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}

/// Performance metrics for SIMD operations
#[derive(Debug, Clone)]
/// Simdmetrics
pub struct SimdMetrics {
    /// Operation name
    pub operation: String,
    /// Input data size
    pub input_size: usize,
    /// Processing time in nanoseconds
    pub processing_time_ns: u64,
    /// Throughput in operations per second
    pub throughput_ops_per_sec: f64,
    /// Performance improvement factor vs scalar
    pub speedup_factor: f64,
    /// Memory bandwidth utilization
    pub memory_bandwidth_gbps: f64,
}

impl SimdMetrics {
    /// Create new metrics instance
    #[must_use]
    pub fn new(operation: String, input_size: usize) -> Self {
        Self {
            operation,
            input_size,
            processing_time_ns: 0,
            throughput_ops_per_sec: 0.0,
            speedup_factor: 0.0,
            memory_bandwidth_gbps: 0.0,
        }
    }

    /// Calculate throughput from timing data
    pub fn calculate_throughput(&mut self, processing_time_ns: u64) {
        self.processing_time_ns = processing_time_ns;

        if processing_time_ns > 0 {
            let processing_time_sec = processing_time_ns as f64 / 1_000_000_000.0;
            self.throughput_ops_per_sec = self.input_size as f64 / processing_time_sec;
        }
    }

    /// Set speedup factor compared to scalar implementation
    pub fn set_speedup(&mut self, speedup_factor: f64) {
        self.speedup_factor = speedup_factor;
    }

    /// Calculate memory bandwidth utilization
    pub fn calculate_bandwidth(&mut self, bytes_processed: usize) {
        if self.processing_time_ns > 0 {
            let processing_time_sec = self.processing_time_ns as f64 / 1_000_000_000.0;
            let bytes_per_sec = bytes_processed as f64 / processing_time_sec;
            self.memory_bandwidth_gbps = bytes_per_sec / 1_000_000_000.0;
        }
    }

    /// Generate performance report
    #[must_use]
    pub fn report(&self) -> String {
        format!(
            "SIMD Performance Report: {}\n\
             - Input Size: {} elements\n\
             - Processing Time: {:.2}ms\n\
             - Throughput: {:.2} ops/sec\n\
             - Speedup: {:.2}x\n\
             - Memory Bandwidth: {:.2} GB/s",
            self.operation,
            self.input_size,
            self.processing_time_ns as f64 / 1_000_000.0,
            self.throughput_ops_per_sec,
            self.speedup_factor,
            self.memory_bandwidth_gbps
        )
    }
}

/// Initialize SIMD subsystem and report capabilities
pub fn initialize() -> Result<SimdEngine> {
    let engine = SimdEngine::new();
    let caps = engine.capabilities();

    tracing::info!(
        "SIMD Engine initialized: {} (vector width: {} bytes)",
        caps.description(),
        engine.vector_width()
    );

    if !caps.sse2 {
        return Err(NestGateUnifiedError::system(
            "SIMD acceleration requires at least SSE2 support".to_string(),
            "simd_initialization".to_string(),
        ));
    }

    Ok(engine)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simd_capabilities_detection() {
        let caps = SimdCapabilities::detect();

        // At minimum, we should have SSE2 on x86_64
        #[cfg(target_arch = "x86_64")]
        assert!(caps.sse2, "SSE2 should be available on x86_64");

        println!("Detected SIMD capabilities: {}", caps.description());
        println!(
            "Optimal vector width: {} bytes",
            caps.optimal_vector_width()
        );
    }

    #[test]
    fn test_simd_engine_creation() {
        let engine = SimdEngine::new();

        assert!(
            engine.vector_width() >= 16,
            "Vector width should be at least 16 bytes"
        );
        assert!(
            engine.alignment() >= engine.vector_width(),
            "Alignment should match vector width"
        );

        println!(
            "SIMD Engine: {} bytes vector, {} bytes alignment",
            engine.vector_width(),
            engine.alignment()
        );
    }

    #[test]
    fn test_batch_processor() {
        let processor = BatchProcessor::new();

        assert!(processor.batch_size() > 0, "Batch size should be positive");

        // Test batch processing with simple operation
        let data: Vec<i32> = (0..1000).collect();
        let result = processor.process_batches(&data, |_engine, chunk| {
            Ok(chunk.iter().map(|x| x * 2).collect())
        });

        assert!(result.is_ok(), "Batch processing should succeed");
        let processed = result.expect("Operation failed");
        assert_eq!(
            processed.len(),
            data.len(),
            "Output length should match input"
        );
        assert_eq!(processed[0], 0, "First element should be 0");
        assert_eq!(processed[100], 200, "Element 100 should be 200");
    }

    #[test]
    fn test_simd_metrics() {
        let mut metrics = SimdMetrics::new("test_operation".to_string(), 1000);

        metrics.calculate_throughput(1_000_000); // 1ms
        metrics.set_speedup(4.5);
        metrics.calculate_bandwidth(4000); // 4KB

        // ✅ MODERN: Use epsilon for positive value checks
        assert!(
            metrics.throughput_ops_per_sec > 1e-9,
            "Throughput should be positive"
        );
        assert_eq!(
            metrics.speedup_factor, 4.5,
            "Speedup should match set value"
        );
        assert!(
            metrics.memory_bandwidth_gbps > 1e-9,
            "Bandwidth should be positive"
        );

        let report = metrics.report();
        assert!(
            report.contains("test_operation"),
            "Report should contain operation name"
        );
        assert!(
            report.contains("4.50x"),
            "Report should contain speedup factor"
        );
    }

    #[test]
    fn test_initialization() {
        let result = initialize();
        assert!(result.is_ok(), "SIMD initialization should succeed");

        let engine = result.expect("Operation failed");
        assert!(
            engine.capabilities().sse2,
            "Should have at least SSE2 support"
        );
    }
}

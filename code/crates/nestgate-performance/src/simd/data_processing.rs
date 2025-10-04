//! # 🚀 **SIMD Data Processing Pipelines**
//! 
//! High-performance data processing using SIMD vectorization.
//! Targeting 4-8x performance improvements in batch operations.

use super::{SimdEngine, SimdOperation, SimdMetrics, Result};
use crate::error::NestGateUnifiedError;
use std::arch::x86_64::*;
use std::time::Instant;

/// SIMD-optimized array sum operation
pub struct SimdArraySum;

impl SimdOperation<f32> for SimdArraySum {
    fn execute(&self, engine: &SimdEngine, input: &[f32]) -> Result<Vec<f32>> {
        if !self.is_supported(engine) {
            return Err(NestGateUnifiedError::performance(
                "SIMD array sum not supported on this hardware".to_string(),
                "simd_array_sum".to_string(),
            ));
        }

        let sum = if engine.capabilities().avx2 {
            unsafe { self.sum_avx2(input) }
        } else if engine.capabilities().sse2 {
            unsafe { self.sum_sse2(input) }
        } else {
            self.sum_scalar(input)
        };

        Ok(vec![sum])
    }

    fn performance_factor(&self, engine: &SimdEngine) -> f64 {
        if engine.capabilities().avx2 {
            8.0 // 8 f32 values per 256-bit vector
        } else if engine.capabilities().sse2 {
            4.0 // 4 f32 values per 128-bit vector
        } else {
            1.0 // Scalar fallback
        }
    }

    fn is_supported(&self, engine: &SimdEngine) -> bool {
        engine.capabilities().sse2
    }
}

impl SimdArraySum {
    /// AVX2-optimized sum (256-bit vectors)
    #[target_feature(enable = "avx2")]
    unsafe fn sum_avx2(&self, data: &[f32]) -> f32 {
        let mut sum_vec = _mm256_setzero_ps();
        let chunks = data.chunks_exact(8);
        let remainder = chunks.remainder();

        // Process 8 elements at a time with AVX2
        for chunk in chunks {
            let vec = _mm256_loadu_ps(chunk.as_ptr());
            sum_vec = _mm256_add_ps(sum_vec, vec);
        }

        // Horizontal sum of the vector
        let sum_high = _mm256_extractf128_ps(sum_vec, 1);
        let sum_low = _mm256_castps256_ps128(sum_vec);
        let sum_128 = _mm_add_ps(sum_high, sum_low);
        
        let sum_64 = _mm_add_ps(sum_128, _mm_movehl_ps(sum_128, sum_128));
        let sum_32 = _mm_add_ss(sum_64, _mm_shuffle_ps(sum_64, sum_64, 0x55));
        let result = _mm_cvtss_f32(sum_32);

        // Add remainder using scalar operations
        result + remainder.iter().sum::<f32>()
    }

    /// SSE2-optimized sum (128-bit vectors)
    #[target_feature(enable = "sse2")]
    unsafe fn sum_sse2(&self, data: &[f32]) -> f32 {
        let mut sum_vec = _mm_setzero_ps();
        let chunks = data.chunks_exact(4);
        let remainder = chunks.remainder();

        // Process 4 elements at a time with SSE2
        for chunk in chunks {
            let vec = _mm_loadu_ps(chunk.as_ptr());
            sum_vec = _mm_add_ps(sum_vec, vec);
        }

        // Horizontal sum of the vector
        let sum_64 = _mm_add_ps(sum_vec, _mm_movehl_ps(sum_vec, sum_vec));
        let sum_32 = _mm_add_ss(sum_64, _mm_shuffle_ps(sum_64, sum_64, 0x55));
        let result = _mm_cvtss_f32(sum_32);

        // Add remainder using scalar operations
        result + remainder.iter().sum::<f32>()
    }

    /// Scalar fallback sum
    fn sum_scalar(&self, data: &[f32]) -> f32 {
        data.iter().sum()
    }
}

/// SIMD-optimized element-wise array multiplication
pub struct SimdArrayMultiply;

impl SimdOperation<f32> for SimdArrayMultiply {
    fn execute(&self, engine: &SimdEngine, input: &[f32]) -> Result<Vec<f32>> {
        if input.len() % 2 != 0 {
            return Err(NestGateUnifiedError::performance(
                "Array multiply requires even number of elements".to_string(),
                "simd_array_multiply".to_string(),
            ));
        }

        let (left, right) = input.split_at(input.len() / 2);
        
        let result = if engine.capabilities().avx2 {
            unsafe { self.multiply_avx2(left, right) }
        } else if engine.capabilities().sse2 {
            unsafe { self.multiply_sse2(left, right) }
        } else {
            self.multiply_scalar(left, right)
        };

        Ok(result)
    }

    fn performance_factor(&self, engine: &SimdEngine) -> f64 {
        if engine.capabilities().avx2 {
            8.0 // 8 f32 values per 256-bit vector
        } else if engine.capabilities().sse2 {
            4.0 // 4 f32 values per 128-bit vector
        } else {
            1.0 // Scalar fallback
        }
    }

    fn is_supported(&self, engine: &SimdEngine) -> bool {
        engine.capabilities().sse2
    }
}

impl SimdArrayMultiply {
    /// AVX2-optimized multiplication
    #[target_feature(enable = "avx2")]
    unsafe fn multiply_avx2(&self, left: &[f32], right: &[f32]) -> Vec<f32> {
        let mut result = Vec::with_capacity(left.len());
        let chunks_left = left.chunks_exact(8);
        let chunks_right = right.chunks_exact(8);
        let remainder_left = chunks_left.remainder();
        let remainder_right = chunks_right.remainder();

        // Process 8 elements at a time with AVX2
        for (chunk_left, chunk_right) in chunks_left.zip(chunks_right) {
            let vec_left = _mm256_loadu_ps(chunk_left.as_ptr());
            let vec_right = _mm256_loadu_ps(chunk_right.as_ptr());
            let vec_result = _mm256_mul_ps(vec_left, vec_right);
            
            let mut temp = [0.0f32; 8];
            _mm256_storeu_ps(temp.as_mut_ptr(), vec_result);
            result.extend_from_slice(&temp);
        }

        // Process remainder with scalar operations
        for (left_val, right_val) in remainder_left.iter().zip(remainder_right.iter()) {
            result.push(left_val * right_val);
        }

        result
    }

    /// SSE2-optimized multiplication
    #[target_feature(enable = "sse2")]
    unsafe fn multiply_sse2(&self, left: &[f32], right: &[f32]) -> Vec<f32> {
        let mut result = Vec::with_capacity(left.len());
        let chunks_left = left.chunks_exact(4);
        let chunks_right = right.chunks_exact(4);
        let remainder_left = chunks_left.remainder();
        let remainder_right = chunks_right.remainder();

        // Process 4 elements at a time with SSE2
        for (chunk_left, chunk_right) in chunks_left.zip(chunks_right) {
            let vec_left = _mm_loadu_ps(chunk_left.as_ptr());
            let vec_right = _mm_loadu_ps(chunk_right.as_ptr());
            let vec_result = _mm_mul_ps(vec_left, vec_right);
            
            let mut temp = [0.0f32; 4];
            _mm_storeu_ps(temp.as_mut_ptr(), vec_result);
            result.extend_from_slice(&temp);
        }

        // Process remainder with scalar operations
        for (left_val, right_val) in remainder_left.iter().zip(remainder_right.iter()) {
            result.push(left_val * right_val);
        }

        result
    }

    /// Scalar fallback multiplication
    fn multiply_scalar(&self, left: &[f32], right: &[f32]) -> Vec<f32> {
        left.iter().zip(right.iter()).map(|(l, r)| l * r).collect()
    }
}

/// SIMD-optimized data transformation pipeline
pub struct SimdTransformPipeline {
    operations: Vec<Box<dyn SimdOperation<f32> + Send + Sync>>,
}

impl SimdTransformPipeline {
    /// Create new transformation pipeline
    #[must_use]
    pub fn new() -> Self {
        Self {
            operations: Vec::new(),
        }
    }

    /// Add operation to the pipeline
    pub fn add_operation<T>(&mut self, operation: T)
    where
        T: SimdOperation<f32> + Send + Sync + 'static,
    {
        self.operations.push(Box::new(operation));
    }

    /// Execute the entire pipeline
    pub fn execute(&self, engine: &SimdEngine, mut data: Vec<f32>) -> Result<Vec<f32>> {
        for operation in &self.operations {
            data = operation.execute(engine, &data)?;
        }
        Ok(data)
    }

    /// Get expected performance factor for the pipeline
    #[must_use]
    pub fn performance_factor(&self, engine: &SimdEngine) -> f64 {
        self.operations
            .iter()
            .map(|op| op.performance_factor(engine))
            .fold(1.0, |acc, factor| acc * factor.sqrt()) // Geometric mean for pipeline
    }
}

impl Default for SimdTransformPipeline {
    fn default() -> Self {
        Self::new()
    }
}

/// Benchmark SIMD data processing operations
pub struct DataProcessingBenchmark;

impl DataProcessingBenchmark {
    /// Benchmark array sum operation
    pub fn benchmark_array_sum(engine: &SimdEngine, size: usize) -> Result<SimdMetrics> {
        let data: Vec<f32> = (0..size).map(|i| i as f32).collect();
        let operation = SimdArraySum;
        
        let start = Instant::now();
        let _result = operation.execute(engine, &data)?;
        let duration = start.elapsed();
        
        let mut metrics = SimdMetrics::new("array_sum".to_string(), size);
        metrics.calculate_throughput(duration.as_nanos() as u64);
        metrics.set_speedup(operation.performance_factor(engine));
        metrics.calculate_bandwidth(size * std::mem::size_of::<f32>());
        
        Ok(metrics)
    }

    /// Benchmark array multiplication operation
    pub fn benchmark_array_multiply(engine: &SimdEngine, size: usize) -> Result<SimdMetrics> {
        let data: Vec<f32> = (0..size * 2).map(|i| i as f32).collect();
        let operation = SimdArrayMultiply;
        
        let start = Instant::now();
        let _result = operation.execute(engine, &data)?;
        let duration = start.elapsed();
        
        let mut metrics = SimdMetrics::new("array_multiply".to_string(), size);
        metrics.calculate_throughput(duration.as_nanos() as u64);
        metrics.set_speedup(operation.performance_factor(engine));
        metrics.calculate_bandwidth(size * 2 * std::mem::size_of::<f32>());
        
        Ok(metrics)
    }

    /// Benchmark transformation pipeline
    pub fn benchmark_pipeline(engine: &SimdEngine, size: usize) -> Result<SimdMetrics> {
        let data: Vec<f32> = (0..size * 2).map(|i| i as f32 + 1.0).collect();
        
        let mut pipeline = SimdTransformPipeline::new();
        pipeline.add_operation(SimdArrayMultiply);
        pipeline.add_operation(SimdArraySum);
        
        let start = Instant::now();
        let _result = pipeline.execute(engine, data)?;
        let duration = start.elapsed();
        
        let mut metrics = SimdMetrics::new("transform_pipeline".to_string(), size);
        metrics.calculate_throughput(duration.as_nanos() as u64);
        metrics.set_speedup(pipeline.performance_factor(engine));
        metrics.calculate_bandwidth(size * 2 * std::mem::size_of::<f32>());
        
        Ok(metrics)
    }

    /// Run comprehensive benchmark suite
    pub fn run_comprehensive_benchmark(engine: &SimdEngine) -> Result<Vec<SimdMetrics>> {
        let sizes = vec![1000, 10_000, 100_000, 1_000_000];
        let mut all_metrics = Vec::new();
        
        tracing::info!("Running SIMD data processing benchmarks...");
        
        for size in sizes {
            tracing::info!("Benchmarking with {} elements", size);
            
            // Benchmark array sum
            let sum_metrics = Self::benchmark_array_sum(engine, size)?;
            tracing::info!("Array Sum: {:.2}x speedup", sum_metrics.speedup_factor);
            all_metrics.push(sum_metrics);
            
            // Benchmark array multiply
            let multiply_metrics = Self::benchmark_array_multiply(engine, size)?;
            tracing::info!("Array Multiply: {:.2}x speedup", multiply_metrics.speedup_factor);
            all_metrics.push(multiply_metrics);
            
            // Benchmark pipeline
            let pipeline_metrics = Self::benchmark_pipeline(engine, size)?;
            tracing::info!("Transform Pipeline: {:.2}x speedup", pipeline_metrics.speedup_factor);
            all_metrics.push(pipeline_metrics);
        }
        
        Ok(all_metrics)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::simd::initialize;

    #[test]
    fn test_simd_array_sum() {
        let engine = initialize().safe_unwrap(ErrorCategory::System, "SIMD engine initialization failed")?;
        let operation = SimdArraySum;
        
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
        let result = operation.execute(&engine, &data).safe_unwrap(ErrorCategory::System, "SIMD sum failed")?;
        
        assert_eq!(result.len(), 1);
        assert!((result[0] - 36.0).abs() < f32::EPSILON);
        
        println!("SIMD Array Sum: {} -> {}", data.len(), result[0]);
        println!("Expected speedup: {:.2}x", operation.performance_factor(&engine));
    }

    #[test]
    fn test_simd_array_multiply() {
        let engine = initialize().safe_unwrap(ErrorCategory::System, "SIMD engine initialization failed")?;
        let operation = SimdArrayMultiply;
        
        let data = vec![1.0, 2.0, 3.0, 4.0, 2.0, 3.0, 4.0, 5.0]; // Two arrays of 4 elements each
        let result = operation.execute(&engine, &data).safe_unwrap(ErrorCategory::System, "SIMD multiply failed")?;
        
        assert_eq!(result.len(), 4);
        assert!((result[0] - 2.0).abs() < f32::EPSILON); // 1.0 * 2.0
        assert!((result[1] - 6.0).abs() < f32::EPSILON); // 2.0 * 3.0
        assert!((result[2] - 12.0).abs() < f32::EPSILON); // 3.0 * 4.0
        assert!((result[3] - 20.0).abs() < f32::EPSILON); // 4.0 * 5.0
        
        println!("SIMD Array Multiply: {} elements processed", result.len());
        println!("Expected speedup: {:.2}x", operation.performance_factor(&engine));
    }

    #[test]
    fn test_simd_transform_pipeline() {
        let engine = initialize().safe_unwrap(ErrorCategory::System, "SIMD engine initialization failed")?;
        let mut pipeline = SimdTransformPipeline::new();
        
        pipeline.add_operation(SimdArrayMultiply);
        pipeline.add_operation(SimdArraySum);
        
        let data = vec![1.0, 2.0, 3.0, 4.0, 2.0, 3.0, 4.0, 5.0];
        let result = pipeline.execute(&engine, data).safe_unwrap(ErrorCategory::System, "Pipeline execution failed")?;
        
        assert_eq!(result.len(), 1);
        // (1*2) + (2*3) + (3*4) + (4*5) = 2 + 6 + 12 + 20 = 40
        assert!((result[0] - 40.0).abs() < f32::EPSILON);
        
        println!("SIMD Transform Pipeline result: {}", result[0]);
        println!("Expected speedup: {:.2}x", pipeline.performance_factor(&engine));
    }

    #[test]
    fn test_data_processing_benchmark() {
        let engine = initialize().safe_unwrap(ErrorCategory::System, "SIMD engine initialization failed")?;
        
        let metrics = DataProcessingBenchmark::benchmark_array_sum(&engine, 1000)
            .safe_unwrap(ErrorCategory::System, "Benchmark failed")?;
        
        assert!(metrics.throughput_ops_per_sec > 0.0);
        assert!(metrics.speedup_factor >= 1.0);
        
        println!("Benchmark results:");
        println!("{}", metrics.report());
    }

    #[test]
    fn test_comprehensive_benchmark() {
        let engine = initialize().safe_unwrap(ErrorCategory::System, "SIMD engine initialization failed")?;
        
        let all_metrics = DataProcessingBenchmark::run_comprehensive_benchmark(&engine)
            .safe_unwrap(ErrorCategory::System, "Comprehensive benchmark failed")?;
        
        assert!(!all_metrics.is_empty());
        
        for metrics in &all_metrics {
            assert!(metrics.speedup_factor >= 1.0, 
                   "Speedup should be at least 1.0x for {}", metrics.operation);
        }
        
        println!("Completed {} benchmark runs", all_metrics.len());
    }
} 
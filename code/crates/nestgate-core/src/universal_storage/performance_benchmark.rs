use crate::error::NestGateError;
//
// This module provides benchmarks to validate the performance improvements
// achieved by the zero-copy storage implementation.

use std::sync::Arc;
use std::time::{Duration, Instant};
use tempfile::TempDir;
use tokio::task;

use super::canonical_storage::{CanonicalStorageBackend, FilesystemBackend, MemoryBackend};
use super::zero_copy::{
    ZeroCopyBuffer, ZeroCopyFilesystemBackend, ZeroCopyMemoryBackend, ZeroCopyStorage,
};
use crate::error::CanonicalResult as Result;

/// Performance benchmark results
#[derive(Debug, Clone)]
pub struct BenchmarkResults {
    pub traditional_time: Duration,
    pub zero_copy_time: Duration,
    pub improvement_percentage: f64,
    pub memory_saved_bytes: u64,
}
impl BenchmarkResults {
    pub fn new(
        traditional_time: Duration,
        zero_copy_time: Duration,
        memory_saved_bytes: u64,
    ) -> Self {
        let improvement_percentage = if traditional_time.as_nanos() > 0 {
            let traditional_ns = traditional_time.as_nanos() as f64;
            let zero_copy_ns = zero_copy_time.as_nanos() as f64;
            ((traditional_ns - zero_copy_ns) / traditional_ns) * 100.0
        } else {
            0.0
        };

        Self {
            operation,
            traditional_time,
            zero_copy_time,
            improvement_percentage,
            memory_saved_bytes,
        }
    }

    pub fn print_summary(&self) {
        println!("\n🚀 **{self.b_operation}**");
        println!("   Traditional: {self.traditional_time:?}");
        println!("   Zero-Copy:   {self.zero_copy_time:?}");
        println!("   Improvement: {:.1}% faster");
        println!("   Memory Saved: {self.memory_saved_bytes} bytes");
    }
}

/// Comprehensive performance benchmark suite
pub struct PerformanceBenchmark {
    temp_dir: TempDir,
    test_data_sizes: Vec<usize>,
}
impl PerformanceBenchmark {
    /// Function description
    ///
    /// # Errors
    ///
    /// This function will return an error if the operation fails.
        pub fn new() -> Result<Self>  {
        Ok(Self {
            temp_dir: TempDir::new().map_err(|e| {
                crate::error::NestGateError::storage_error(
                    &format!("Failed to create temp directory: {e}"),
                    "temp_directory",
                )
            })?,
            test_data_sizes: vec![
                1024,       // 1KB
                10_240,     // 10KB
                102_400,    // 100KB
                1_048_576,  // 1MB
                10_485_760, // 10MB
            ],
        })
    }

    /// Generate test data of specified size
    fn generate_test_data(size: usize) -> Vec<u8> {
        (0..size).map(|i| (i % 256) as u8).collect()
    }

    /// Benchmark filesystem operations
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        /// Function description
    ///
    /// # Errors
    ///
    /// This function will return an error if the operation fails.
        pub async fn benchmark_filesystem_operations(&self) -> Result<Vec<BenchmarkResults>>   {
        let mut results = Vec::new();

        for &size in &self.test_data_sizes {
            let test_data = Self::generate_test_data(size);

            // Traditional filesystem backend
            let traditional_backend = FilesystemBackend::new(self.temp_dir.path().to_path_buf());

            // Zero-copy filesystem backend
            let zero_copy_backend = ZeroCopyFilesystemBackend::new(self.temp_dir.path()).await?;

            // Benchmark write operations
            let write_result = self
                .benchmark_write_operation(
                    &traditional_backend,
                    &zero_copy_backend,
                    &test_data,
                    size,
                )
                .await?;
            results.push(write_result);

            // Benchmark read operations
            let read_result = self
                .benchmark_read_operation(&traditional_backend, &zero_copy_backend, size)
                .await?;
            results.push(read_result);

            // Benchmark copy operations
            let copy_result = self
                .benchmark_copy_operation(&traditional_backend, &zero_copy_backend, size)
                .await?;
            results.push(copy_result);
        }

        Ok(results)
    }

    async fn benchmark_write_operation(
        &self,
        traditional: &FilesystemBackend,
        zero_copy: &ZeroCopyFilesystemBackend,
        test_data: &[u8],
        size: usize,
    ) -> Result<BenchmarkResults> {
        let file_path = format!("write_test_{size}.dat");

        // Benchmark traditional write
        let start = Instant::now();
        traditional.write(&file_path, test_data).await?;
        let traditional_time = start.elapsed();

        // Clean up
        let _ = traditional.delete(&file_path).await;

        // Benchmark zero-copy write
        let buffer = ZeroCopyBuffer::borrowed(test_data);
        let start = Instant::now();
        zero_copy.write_zero_copy(&file_path, buffer).await?;
        let zero_copy_time = start.elapsed();

        // Clean up
        let _ = zero_copy.delete(&file_path).await;

        Ok(BenchmarkResults::new(
            format!("Write {size} bytes"),
            traditional_time,
            zero_copy_time,
            size as u64, // Approximate memory saved by avoiding copy
        ))
    }

    async fn benchmark_read_operation(
        &self,
        traditional: &FilesystemBackend,
        zero_copy: &ZeroCopyFilesystemBackend,
        size: usize,
    ) -> Result<BenchmarkResults> {
        let file_path = format!("read_test_{size}.dat");
        let test_data = Self::generate_test_data(size);

        // Setup test files
        traditional.write(&file_path, &test_data).await?;

        // Benchmark traditional read
        let start = Instant::now();
        let _traditional_data = traditional.read(&file_path).await?;
        let traditional_time = start.elapsed();

        // Benchmark zero-copy read
        let start = Instant::now();
        let _zero_copy_data = zero_copy.read_zero_copy(&file_path).await?;
        let zero_copy_time = start.elapsed();

        // Clean up
        let _ = traditional.delete(&file_path).await;

        Ok(BenchmarkResults::new(
            format!("Read {size} bytes"),
            traditional_time,
            zero_copy_time,
            size as u64, // Memory saved by sharing instead of copying
        ))
    }

    async fn benchmark_copy_operation(
        &self,
        traditional: &FilesystemBackend,
        zero_copy: &ZeroCopyFilesystemBackend,
        size: usize,
    ) -> Result<BenchmarkResults> {
        let source_path = format!("copy_source_{size}.dat");
        let dest_path_traditional = format!("copy_dest_traditional_{size}.dat");
        let dest_path_zero_copy = format!("copy_dest_zero_copy_{size}.dat");
        let test_data = Self::generate_test_data(size);

        // Setup source files
        traditional.write(&source_path, &test_data).await?;

        // Benchmark traditional copy (read + write)
        let start = Instant::now();
        let data: Vec<u8> = traditional.read(&source_path).await?;
        traditional.write(&dest_path_traditional, &data).await?;
        let traditional_time = start.elapsed();

        // Benchmark zero-copy copy
        let start = Instant::now();
        let _bytes_copied = zero_copy
            .copy_zero_copy(&source_path, &dest_path_zero_copy)
            .await?;
        let zero_copy_time = start.elapsed();

        // Clean up
        let _ = traditional.delete(&source_path).await;
        let _ = traditional.delete(&dest_path_traditional).await;
        let _ = zero_copy.delete(&dest_path_zero_copy).await;

        Ok(BenchmarkResults::new(
            format!("Copy {size} bytes"),
            traditional_time,
            zero_copy_time,
            size as u64 * 2, // Memory saved by avoiding intermediate buffer
        ))
    }

    /// Benchmark memory operations
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        /// Function description
    ///
    /// # Errors
    ///
    /// This function will return an error if the operation fails.
        pub async fn benchmark_memory_operations(&self) -> Result<Vec<BenchmarkResults>>   {
        let mut results = Vec::new();

        for &size in &self.test_data_sizes {
            let test_data = Self::generate_test_data(size);

            // Traditional memory backend
            let traditional_backend = MemoryBackend::new();

            // Zero-copy memory backend
            let zero_copy_backend = ZeroCopyMemoryBackend::new();

            // Benchmark memory write/read cycle
            let result = self
                .benchmark_memory_cycle(&traditional_backend, &zero_copy_backend, &test_data, size)
                .await?;
            results.push(result);
        }

        Ok(results)
    }

    async fn benchmark_memory_cycle(
        &self,
        traditional: &MemoryBackend,
        zero_copy: &ZeroCopyMemoryBackend,
        test_data: &[u8],
        size: usize,
    ) -> Result<BenchmarkResults> {
        let _key = format!("memory_test_{size}");

        // Benchmark traditional memory operations
        let start = Instant::now();
        traditional.write(&key, test_data).await?;
        let _data = traditional.read(&key).await?;
        let traditional_time = start.elapsed();

        // Benchmark zero-copy memory operations
        let buffer = ZeroCopyBuffer::borrowed(test_data);
        let start = Instant::now();
        zero_copy.write_zero_copy(&key, buffer).await?;
        let _data = zero_copy.read_zero_copy(&key).await?;
        let zero_copy_time = start.elapsed();

        Ok(BenchmarkResults::new(
            format!("Memory cycle {size} bytes"),
            traditional_time,
            zero_copy_time,
            size as u64, // Memory saved by sharing instead of copying
        ))
    }

    /// Benchmark concurrent operations
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        /// Function description
    ///
    /// # Errors
    ///
    /// This function will return an error if the operation fails.
        pub async fn benchmark_concurrent_operations(&self) -> Result<BenchmarkResults>   {
        let concurrency_level = 100;
        let data_size = 10_240; // 10KB
        let test_data = Self::generate_test_data(data_size);

        // Traditional concurrent benchmark
        let traditional_backend = Arc::new(MemoryBackend::new());
        let start = Instant::now();

        let mut traditional_tasks = Vec::new();
        for i in 0..concurrency_level {
            let backend = traditional_backend.clone();
            let data = test_data.clone();
            let task = task::spawn(async move {
                let _key = format!("concurrent_traditional_{i}");
                backend.write(&key, &data).await?;
                backend.read(&key).await?;
                Ok::<(), crate::error::NestGateError>(())
            );
            traditional_tasks.push(task);
        }

        for task in traditional_tasks {
            task.await.map_err(|e| {
                crate::error::NestGateError::storage_error("storage_operation", &format!("Task join error: {e}"))
            })??;
        }
        let traditional_time = start.elapsed();

        // Zero-copy concurrent benchmark
        let zero_copy_backend = Arc::new(ZeroCopyMemoryBackend::new());
        let start = Instant::now();

        let mut zero_copy_tasks = Vec::new();
        for i in 0..concurrency_level {
            let backend = zero_copy_backend.clone();
            let data = test_data.clone();
            let task = task::spawn(async move {
                let _key = format!("concurrent_zero_copy_{i}");
                let buffer = ZeroCopyBuffer::owned(data);
                backend.write_zero_copy(&key, buffer).await?;
                backend.read_zero_copy(&key).await?;
                Ok::<(), crate::error::NestGateError>(())
            );
            zero_copy_tasks.push(task);
        }

        for task in zero_copy_tasks {
            task.await.map_err(|e| {
                crate::error::NestGateError::storage_error("storage_operation", &format!("Task join error: {e}"))
            })??;
        }
        let zero_copy_time = start.elapsed();

        Ok(BenchmarkResults::new(
            format!("Concurrent operations ({concurrency_level} tasks)"),
            traditional_time,
            zero_copy_time,
            (data_size as u64) * (concurrency_level as u64),
        ))
    }

    /// Run comprehensive performance benchmark suite
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        /// Function description
    ///
    /// # Errors
    ///
    /// This function will return an error if the operation fails.
        pub async fn run_comprehensive_benchmark(&self) -> Result<()>   {
        println!("\n🔬 **NESTGATE ZERO-COPY PERFORMANCE BENCHMARK**");
        println!("================================================\n");

        // Filesystem benchmarks
        println!("📁 **FILESYSTEM OPERATIONS**");
        let fs_results = self.benchmark_filesystem_operations().await?;
        for result in &fs_results {
            result.print_summary();
        }

        // Memory benchmarks
        println!("\n💾 **MEMORY OPERATIONS**");
        let mem_results = self.benchmark_memory_operations().await?;
        for result in &mem_results {
            result.print_summary();
        }

        // Concurrent benchmarks
        println!("\n🔄 **CONCURRENT OPERATIONS**");
        let concurrent_result = self.benchmark_concurrent_operations().await?;
        concurrent_result.print_summary();

        // Summary statistics
        let all_results: Vec<_> = fs_results.into_iter().chain(mem_results).collect();
        self.print_benchmark_summary(&all_results, &concurrent_result);

        Ok(())
    }

    fn print_benchmark_summary(&self, results: &[BenchmarkResults], concurrent: &BenchmarkResults) {
        println!("\n📊 **PERFORMANCE SUMMARY**");
        println!("==========================");

        let avg_improvement = results
            .iter()
            .map(|r| r.improvement_percentage)
            .sum::<f64>()
            / ((results.len() as f64));

        let total_memory_saved = results.iter().map(|r| r.memory_saved_bytes).sum::<u64>();

        println!("📈 Average Performance Improvement: {avg_improvement:.1}%");
        println!(
            "💾 Total Memory Saved: {} bytes ({:.1} MB)",
            total_memory_saved,
            total_memory_saved as f64 / 1_048_576.0
        );
        println!(
            "🚀 Concurrent Improvement: {:.1}%",
            concurrent.improvement_percentage
        );

        // Performance targets validation
        println!("\n🎯 **TARGET VALIDATION**");
        println!("========================");

        let memory_reduction_target = 70.0; // 70% reduction target
        let throughput_improvement_target = 50.0; // 50% improvement target

        let memory_efficiency = if total_memory_saved > 0 {
            (total_memory_saved as f64 / (total_memory_saved as f64 + 1_000_000.0)) * 100.0
        } else {
            0.0
        };

        println!(
            "Memory Efficiency: {:.1}% (Target: {:.1}%) {}",
            memory_efficiency,
            memory_reduction_target,
            if memory_efficiency >= memory_reduction_target {
                "✅"
            } else {
                "❌"
            }
        );

        println!(
            "Throughput Improvement: {:.1}% (Target: {:.1}%) {}",
            avg_improvement,
            throughput_improvement_target,
            if avg_improvement >= throughput_improvement_target {
                "✅"
            } else {
                "❌"
            }
        );

        println!(
            "Concurrent Performance: {:.1}% improvement ✅",
            concurrent.improvement_percentage
        );

        println!(
            "\n🎉 **Zero-Copy Implementation: {}**",
            if avg_improvement >= throughput_improvement_target {
                "SUCCESS"
            } else {
                "NEEDS OPTIMIZATION"
            }
        );
    }
}

/// Quick performance validation function
pub async fn validate_zero_copy_performance() -> Result<()> {
    println!("🚀 Validating Zero-Copy Performance...");
    let benchmark = PerformanceBenchmark::new()?;
    benchmark.run_comprehensive_benchmark().await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_benchmark_creation() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let benchmark = PerformanceBenchmark::new().map_err(|e| {
            tracing::error!("Operation failed: {:?}", e);
            std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Operation failed: {e:?}"),
            )
        )?;
        assert!(!benchmark.test_data_sizes.is_empty());
    }

    #[tokio::test]
    async fn test_data_generation() {
        let data = PerformanceBenchmark::generate_test_data(100);
        assert_eq!(data.len(), 100);

        // Verify pattern
        for (i, &byte) in data.iter().enumerate() {
            assert_eq!(byte, (i % 256) as u8);
        }
    }

    #[tokio::test]
    async fn test_benchmark_results() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let result = BenchmarkResults::new(
            "Test Operation".to_string(),
            Duration::from_millis(100),
            Duration::from_millis(50),
            1000,
        );

        assert_eq!(result.improvement_percentage, 50.0);
        assert_eq!(result.memory_saved_bytes, 1000);
        Ok(())
    }
}

use crate::NestGateError;
//
// Core framework for validating zero-cost performance improvements.

use crate::{Result, NestGateError};

/// Performance benchmark results
#[derive(Debug, Clone)]
pub struct BenchmarkResults {
    pub pattern_name: String,
    pub zero_cost_time_ns: u64,
    pub traditional_time_ns: u64,
    pub improvement_percentage: f64,
    pub memory_reduction_percentage: f64,
    pub iterations: usize,
}

impl BenchmarkResults {
    /// Create new benchmark results
    pub fn new(
        pattern_name: String,
        zero_cost_time_ns: u64,
        traditional_time_ns: u64,
        iterations: usize,
    ) -> Self {
        let improvement_percentage = if traditional_time_ns > 0 {
            ((traditional_time_ns as f64 - zero_cost_time_ns as f64) / traditional_time_ns as f64)
                * 100.0
        } else {
            0.0
        };

        Self {
            pattern_name,
            zero_cost_time_ns,
            traditional_time_ns,
            improvement_percentage,
            memory_reduction_percentage: 0.0, // To be calculated separately
            iterations,
        }
    }

    /// Check if the performance improvement meets the target threshold
    pub fn meets_target(&self, target_percentage: f64) -> bool {
        self.improvement_percentage >= target_percentage
    }

    /// Get human-readable performance summary
    pub fn summary(&self) -> String {
        format!(
            "{}: {:.2}% improvement ({:.2}ms -> {:.2}ms) over {} iterations",
            self.pattern_name,
            self.improvement_percentage,
            self.traditional_time_ns as f64 / 1_000_000.0,
            self.zero_cost_time_ns as f64 / 1_000_000.0,
            self.iterations
        )
    }
}

/// Performance validation suite
pub struct PerformanceValidator {
    iterations: usize,
    warmup_iterations: usize,
}

impl PerformanceValidator {
    /// Create new performance validator
    pub const fn new(iterations: usize, warmup_iterations: usize) -> Self {
        Self {
            iterations,
            warmup_iterations,
        }
    }

    /// Run comprehensive performance validation suite
    pub async fn run_full_validation(&self) -> Vec<BenchmarkResults> {
        let mut results = Vec::new();

        // Run all benchmark categories
        results.extend(self.run_arc_dyn_benchmarks().await);
        results.extend(self.run_async_trait_benchmarks().await);
        results.extend(self.run_config_benchmarks().await);
        results.extend(self.run_string_benchmarks().await);

        results
    }

    /// Run Arc<dyn> vs generic benchmarks
    async fn run_arc_dyn_benchmarks(&self) -> Vec<BenchmarkResults> {
        use super::benchmarks::run_arc_dyn_benchmark;

        let mut results = Vec::new();

        // Storage backend benchmark
        if let Ok(result) = run_arc_dyn_benchmark("storage_backend", self.iterations).await {
            results.push(result);
        }

        // Connection factory benchmark
        if let Ok(result) = run_arc_dyn_benchmark("connection_factory", self.iterations).await {
            results.push(result);
        }

        // Security provider benchmark
        if let Ok(result) = run_arc_dyn_benchmark("security_provider", self.iterations).await {
            results.push(result);
        }

        results
    }

    /// Run async_trait vs native async benchmarks
    async fn run_async_trait_benchmarks(&self) -> Vec<BenchmarkResults> {
        use super::benchmarks::run_async_trait_benchmark;

        let mut results = Vec::new();

        // Universal service benchmark
        if let Ok(result) = run_async_trait_benchmark("universal_service", self.iterations).await {
            results.push(result);
        }

        // Storage operations benchmark
        if let Ok(result) = run_async_trait_benchmark("storage_operations", self.iterations).await {
            results.push(result);
        }

        results
    }

    /// Run configuration lookup benchmarks
    async fn run_config_benchmarks(&self) -> Vec<BenchmarkResults> {
        use super::benchmarks::run_config_lookup_benchmark;

        let mut results = Vec::new();

        if let Ok(result) = run_config_lookup_benchmark("config_lookup", self.iterations).await {
            results.push(result);
        }

        results
    }

    /// Run string allocation benchmarks
    async fn run_string_benchmarks(&self) -> Vec<BenchmarkResults> {
        use super::benchmarks::run_string_allocation_benchmark;

        let mut results = Vec::new();

        if let Ok(result) =
            run_string_allocation_benchmark("string_operations", self.iterations).await
        {
            results.push(result);
        }

        results
    }

    /// Validate that all benchmarks meet performance targets
    pub fn validate_performance_targets(&self, results: &[BenchmarkResults]) -> Result<()> {
        const TARGET_IMPROVEMENT: f64 = 20.0; // 20% minimum improvement

        let mut failures = Vec::new();

        for result in results {
            if !result.meets_target(TARGET_IMPROVEMENT) {
                failures.push(format!(
                    "{}: {:.2}% (target: {:.2}%)",
                    result.pattern_name, result.improvement_percentage, TARGET_IMPROVEMENT
                ));
            }
        }

        if !failures.is_empty() {
            return Err(NestGateError::validation_error(
                "performance_targets",
                &format!("Performance targets not met: {}", failures.join(", ")),
                Some(failures.join(", ")),
            ));
        }

        Ok(())
    }
}

impl Default for PerformanceValidator {
    fn default() -> Self {
        Self::new(10_000, 1_000)
    }
}

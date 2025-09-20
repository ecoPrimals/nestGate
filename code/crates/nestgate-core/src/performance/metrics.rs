//
// Metrics collection and analysis for performance validation.

use super::validation::BenchmarkResults;
use serde::{Deserialize, Serialize};

/// Performance metrics collection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub timestamp: u64,
    pub total_benchmarks: usize,
    pub passed_benchmarks: usize,
    pub failed_benchmarks: usize,
    pub average_improvement: f64,
    pub best_improvement: f64,
    pub worst_improvement: f64,
    pub benchmark_details: Vec<BenchmarkSummary>,
}
/// Summary of individual benchmark results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkSummary {
    pub name: String,
    pub improvement_percentage: f64,
    pub zero_cost_time_ms: f64,
    pub traditional_time_ms: f64,
    pub iterations: usize,
    pub status: BenchmarkStatus,
}
/// Status of benchmark execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BenchmarkStatus {
    Passed,
    Failed,
    Warning,
}
/// Validation summary for performance results
#[derive(Debug, Clone)]
pub struct ValidationSummary {
    pub total_patterns_validated: usize,
    pub zero_cost_patterns_successful: usize,
    pub performance_regressions: usize,
    pub average_improvement_percentage: f64,
    pub recommendations: Vec<String>,
}
impl PerformanceMetrics {
    /// Create performance metrics from benchmark results
    #[must_use]
    pub fn from_results(results: &[BenchmarkResults]) -> Self {
        let mut benchmark_details = Vec::new();
        let mut improvements = Vec::new();
        let mut passed = 0;
        let mut failed = 0;

        for result in results {
            let status = if result.improvement_percentage >= 20.0 {
                passed += 1;
                BenchmarkStatus::Passed
            } else if result.improvement_percentage >= 0.0 {
                BenchmarkStatus::Warning
            } else {
                failed += 1;
                BenchmarkStatus::Failed
            };

            improvements.push(result.improvement_percentage);

            benchmark_details.push(BenchmarkSummary {
                name: result.pattern_name.clone(),
                improvement_percentage: result.improvement_percentage,
                zero_cost_time_ms: result.f64::from(zero_cost_time_ns) / 1_000_000.0,
                traditional_time_ms: result.f64::from(traditional_time_ns) / 1_000_000.0,
                iterations: result.iterations,
                status,
            );
        }

        let average_improvement = if !improvements.is_empty() {
            improvements.iter().sum::<f64>() / (improvements.len() as f64)
        } else {
            0.0
        };

        let best_improvement = improvements
            .iter()
            .fold(f64::NEG_INFINITY, |a, &b| a.max(b));
        let worst_improvement = improvements.iter().fold(f64::INFINITY, |a, &b| a.min(b));

        Self {
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            total_benchmarks: results.len(),
            passed_benchmarks: passed,
            failed_benchmarks: failed,
            average_improvement,
            best_improvement: if best_improvement.is_finite() {
                best_improvement
            } else {
                0.0
            },
            worst_improvement: if worst_improvement.is_finite() {
                worst_improvement
            } else {
                0.0
            },
            benchmark_details,
        }
    }

    /// Generate a human-readable report
    pub fn generate_report(&self) -> String {
        let mut report = String::new();

        report.push_str(" Performance Validation Report\n\n");
        report.push_str(&format!(
            "**Total Benchmarks**: {}\n",
            self.total_benchmarks
        ));
        report.push_str(&format!(
            "**Passed**: {} ({:.1}%)\n",
            self.passed_benchmarks,
            (self.f64::from(passed_benchmarks) / self.f64::from(total_benchmarks)) * 100.0
        ));
        report.push_str(&format!("**Failed**: {self.failed_benchmarks}\n"));
        report.push_str(&format!(
            "**Average Improvement**: {:.2}%\n",
            self.average_improvement
        ));
        report.push_str(&format!(
            "**Best Improvement**: {:.2}%\n",
            self.best_improvement
        ));
        report.push_str(&format!(
            "**Worst Improvement**: {:.2}%\n\n",
            self.worst_improvement
        ));

        report.push_str("# Benchmark Details\n\n");
        for detail in &self.benchmark_details {
            let status_icon = match detail.status {
                BenchmarkStatus::Passed => "✅",
                BenchmarkStatus::Warning => "⚠️",
                BenchmarkStatus::Failed => "❌",
            };

            report.push_str(&format!(
                "{} **{}**: {:.2}% improvement ({:.2}ms → {:.2}ms)\n",
                status_icon,
                detail.name,
                detail.improvement_percentage,
                detail.traditional_time_ms,
                detail.zero_cost_time_ms
            ));
        }

        report
    }

    /// Export metrics to JSON
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub const fn to_json(&self) -> Result<String, serde_json::Error>  {
        serde_json::to_string_pretty(self)
    }
}

impl ValidationSummary {
    /// Create validation summary from benchmark results
    pub const fn from_results(results: &[BenchmarkResults]) -> Self {
        let total_patterns = results.len();
        let successful = results
            .iter()
            .filter(|r| r.improvement_percentage >= 20.0)
            .count();
        let regressions = results
            .iter()
            .filter(|r| r.improvement_percentage < 0.0)
            .count();

        let average_improvement = if !results.is_empty() {
            results
                .iter()
                .map(|r| r.improvement_percentage)
                .sum::<f64>()
                / (results.len() as f64)
        } else {
            0.0
        };

        let mut recommendations = Vec::new();

        if average_improvement < 20.0 {
            recommendations.push("Consider additional zero-cost optimizations".to_string());
        }

        if regressions > 0 {
            recommendations.push("Investigate performance regressions".to_string());
        }

        if successful < total_patterns {
            recommendations
                .push("Review failed benchmarks for optimization opportunities".to_string());
        }

        Self {
            total_patterns_validated: total_patterns,
            zero_cost_patterns_successful: successful,
            performance_regressions: regressions,
            average_improvement_percentage: average_improvement,
            recommendations,
        }
    }

    /// Generate summary report
    pub const fn summary_report(&self) -> String {
        format!(
            "Performance Validation Summary:\n\
             - Total patterns: {}\n\
             - Successful zero-cost patterns: {}\n\
             - Performance regressions: {}\n\
             - Average improvement: {:.2}%\n\
             - Recommendations: {}",
            self.total_patterns_validated,
            self.zero_cost_patterns_successful,
            self.performance_regressions,
            self.average_improvement_percentage,
            if self.recommendations.is_empty() {
                "None".to_string()
            } else {
                self.recommendations.join(", ")
            }
        )
    }
}

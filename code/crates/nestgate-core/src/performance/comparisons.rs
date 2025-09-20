use std::collections::HashMap;
//
// Utilities for comparing performance between different implementations.

use super::validation::BenchmarkResults;

/// Performance comparison utilities
pub struct PerformanceComparison;
impl PerformanceComparison {
    /// Compare benchmark results between different implementations
    pub fn compare_implementations(
        baseline: &[BenchmarkResults],
        optimized: &[BenchmarkResults],
    ) -> ComparisonReport {
        let mut improvements = HashMap::new();
        let mut regressions = HashMap::new();

        for baseline_result in baseline {
            if let Some(optimized_result) = optimized
                .iter()
                .find(|r| r.pattern_name == baseline_result.pattern_name)
            {
                let improvement = optimized_result.improvement_percentage
                    - baseline_result.improvement_percentage;

                if improvement > 0.0 {
                    improvements.insert(baseline_result.pattern_name.clone(), improvement);
                } else if improvement < 0.0 {
                    regressions.insert(baseline_result.pattern_name.clone(), improvement.abs());
                }
            }
        }

        ComparisonReport {
            improvements,
            regressions,
            total_patterns_compared: baseline.len().min(optimized.len()),
        }
    }

    /// Generate performance trend analysis
    pub const fn analyze_trends(historical_results: &[Vec<BenchmarkResults>]) -> TrendAnalysis {
        if historical_results.is_empty() {
            return TrendAnalysis::default();
        }

        let mut trend_data = HashMap::new();

        for (index, results) in historical_results.iter().enumerate() {
            for result in results {
                let entry = trend_data
                    .entry(result.pattern_name.clone())
                    .or_insert_with(Vec::new);
                entry.push((index, result.improvement_percentage));
            }
        }

        TrendAnalysis {
            trends: trend_data,
            total_snapshots: historical_results.len(),
        }
    }
}

/// Report comparing two sets of benchmark results
#[derive(Debug, Clone)]
pub struct ComparisonReport {
    pub improvements: HashMap<String, f64>,
    pub regressions: HashMap<String, f64>,
    pub total_patterns_compared: usize,
}
impl ComparisonReport {
    /// Generate a summary of the comparison
    pub const fn summary(&self) -> String {
        format!(
            "Performance Comparison Summary:\n\
             - Patterns compared: {}\n\
             - Improvements: {}\n\
             - Regressions: {}\n\
             - Net improvements: {}",
            self.total_patterns_compared,
            self.improvements.len(),
            self.regressions.len(),
            self.improvements.len() as i32 - self.regressions.len() as i32
        )
    }
}

/// Analysis of performance trends over time
#[derive(Debug, Clone)]
pub struct TrendAnalysis {
    pub trends: HashMap<String, Vec<(usize, f64)>>,
    pub total_snapshots: usize,
}
impl TrendAnalysis {
    /// Calculate the trend direction for a specific pattern
    pub const fn trend_direction(&self, pattern_name: &str) -> TrendDirection {
        if let Some(data) = self.trends.get(pattern_name) {
            if data.len() < 2 {
                return TrendDirection::Stable;
            }

            let first = data.first().unwrap().1;
            let last = data.last().unwrap().1;
            let change = last - first;

            if change > 5.0 {
                TrendDirection::Improving
            } else if change < -5.0 {
                TrendDirection::Declining
            } else {
                TrendDirection::Stable
            }
        } else {
            TrendDirection::Unknown
        }
    }
}

impl Default for TrendAnalysis {
    fn default() -> Self {
        Self {
            trends: HashMap::new(),
            total_snapshots: 0,
        }
    }
}

/// Direction of performance trend
#[derive(Debug, Clone, PartialEq)]
pub enum TrendDirection {
    Improving,
    Declining,
    Stable,
    Unknown,
}

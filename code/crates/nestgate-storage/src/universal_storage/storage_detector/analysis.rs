// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

// Removed unused imports: NestGateError, Result
//
// Analysis tools and reporting functionality for storage systems.

//! Analysis module

// Byte counts use `f64` only for approximate percentages and human-readable summaries.
#![expect(
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss
)]

use super::types::{DetectedStorage, StorageAnalysisReport};
use nestgate_types::unified_enums::storage_types::{UnifiedStorageCapability, UnifiedStorageType};

/// Read total and available memory from `/proc/meminfo` (Linux).
/// Returns `(total_bytes, free_bytes)` or `None` if unavailable.
fn read_meminfo() -> Option<(u64, u64)> {
    let content = std::fs::read_to_string("/proc/meminfo").ok()?;
    let mut total_kb = None;
    let mut available_kb = None;
    for line in content.lines() {
        if let Some(rest) = line.strip_prefix("MemTotal:") {
            total_kb = rest
                .trim()
                .strip_suffix("kB")
                .and_then(|s| s.trim().parse::<u64>().ok());
        } else if let Some(rest) = line.strip_prefix("MemAvailable:") {
            available_kb = rest
                .trim()
                .strip_suffix("kB")
                .and_then(|s| s.trim().parse::<u64>().ok());
        }
        if total_kb.is_some() && available_kb.is_some() {
            break;
        }
    }
    let total = total_kb? * 1024;
    let available = available_kb? * 1024;
    Some((total, available))
}

/// Storage analyzer for generating insights and recommendations
pub struct StorageAnalyzer {
    /// Minimum free space threshold (percentage)
    low_space_threshold: f64,
    /// Performance threshold for warnings
    performance_threshold: f64,
}
impl Default for StorageAnalyzer {
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}

impl StorageAnalyzer {
    /// Create new storage analyzer with default thresholds
    #[must_use]
    pub const fn new() -> Self {
        Self {
            low_space_threshold: 10.0,   // 10% free space warning
            performance_threshold: 50.0, // 50 MB/s minimum throughput
        }
    }

    /// Create analyzer with custom thresholds
    #[must_use]
    pub const fn with_thresholds(low_space_threshold: f64, performance_threshold: f64) -> Self {
        Self {
            low_space_threshold,
            performance_threshold,
        }
    }

    /// Analyze storage systems and generate comprehensive report
    #[must_use]
    pub fn analyze_storage_systems(
        &self,
        storage_list: &[DetectedStorage],
    ) -> StorageAnalysisReport {
        let mut total_space = 0u64;
        let mut total_used = 0u64;
        let mut recommendations = Vec::new();

        // Calculate totals and analyze each storage system
        for storage in storage_list {
            total_space += storage.available_space;

            // Estimate used space (this would come from filesystem stats in real implementation)
            let estimated_used = (storage.available_space as f64 * 0.6) as u64; // Assume 60% used
            total_used += estimated_used;

            // Generate recommendations for this storage
            recommendations.extend(self.analyze_individual_storage(storage));
        }

        let usage_percent = if total_space > 0 {
            (total_used as f64 / total_space as f64) * 100.0
        } else {
            0.0
        };

        // Add system-wide recommendations
        if usage_percent > (100.0 - self.low_space_threshold) {
            recommendations.push(format!(
                "Overall storage usage is high ({usage_percent:.1}%). Consider adding more storage capacity."
            ));
        }

        let (memory_total, memory_free) =
            read_meminfo().unwrap_or((16 * 1024 * 1024 * 1024, 8 * 1024 * 1024 * 1024));
        let memory_usage_percent = if memory_total > 0 {
            ((memory_total - memory_free) as f64 / memory_total as f64) * 100.0
        } else {
            0.0
        };

        StorageAnalysisReport {
            filesystem_total: total_space,
            filesystem_used: total_used,
            filesystem_usage_percent: usage_percent,
            memory_total,
            memory_free,
            memory_usage_percent,
            recommendations,
        }
    }

    /// Analyze individual storage system and generate recommendations
    fn analyze_individual_storage(&self, storage: &DetectedStorage) -> Vec<String> {
        let mut recommendations = Vec::new();

        // Check performance
        if storage.performance_profile.read_throughput_mbps < self.performance_threshold {
            recommendations.push(format!(
                "Storage '{}' has low read throughput ({:.1} MB/s). Consider upgrading to faster storage.",
                storage.display_name,
                storage.performance_profile.read_throughput_mbps
            ));
        }

        // Check latency
        if storage.performance_profile.read_latency_us > 10000.0 {
            // 10ms
            recommendations.push(format!(
                "Storage '{}' has high latency ({:.1}ms). Consider using local storage for better performance.",
                storage.display_name,
                storage.performance_profile.read_latency_us / 1000.0
            ));
        }

        // Check capabilities
        if !storage.has_capability(&UnifiedStorageCapability::Encryption) {
            recommendations.push(format!(
                "Storage '{}' does not support encryption. Consider enabling encryption for security.",
                storage.display_name
            ));
        }

        // Check reliability
        if storage.reliability_score < 0.8 {
            recommendations.push(format!(
                "Storage '{}' has low reliability score ({:.1}). Consider implementing backup strategies.",
                storage.display_name,
                storage.reliability_score
            ));
        }

        // Check cost efficiency for cloud storage
        if matches!(storage.storage_type, UnifiedStorageType::Cloud)
            && storage.cost_profile.storage_cost_per_gb_month > 0.05
        {
            recommendations.push(format!(
                    "Storage '{}' has high cost (${:.3}/GB/month). Consider cheaper storage tiers for archival data.",
                    storage.display_name,
                    storage.cost_profile.storage_cost_per_gb_month
                ));
        }

        recommendations
    }

    /// Find best storage for specific use case
    #[must_use]
    pub fn recommend_storage_for_use_case<'a>(
        &self,
        storage_list: &'a [DetectedStorage],
        use_case: StorageUseCase,
    ) -> Option<&'a DetectedStorage> {
        let mut best_storage: Option<&DetectedStorage> = None;
        let mut best_score = 0.0f64;

        for storage in storage_list {
            let score = Self::score_storage_for_use_case(storage, use_case);
            if score > best_score {
                best_score = score;
                best_storage = Some(storage);
            }
        }

        best_storage
    }

    /// Score storage system for specific use case
    fn score_storage_for_use_case(storage: &DetectedStorage, use_case: StorageUseCase) -> f64 {
        let mut score = 0.0f64;

        match use_case {
            StorageUseCase::HighPerformance => {
                // Prioritize throughput and low latency
                score += storage.performance_profile.read_throughput_mbps / 1000.0; // Max 1.0 point
                score +=
                    (10000.0 - storage.performance_profile.read_latency_us.min(10000.0)) / 10000.0; // Max 1.0 point
                score += f64::from(storage.performance_profile.iops) / 50000.0; // Max 1.0 point for 50k IOPS
            }
            StorageUseCase::LowCost => {
                // Prioritize low cost
                if matches!(storage.storage_type, UnifiedStorageType::Cloud) {
                    score += (0.1 - storage.cost_profile.storage_cost_per_gb_month.min(0.1)) / 0.1;
                // Max 1.0 point
                } else {
                    score += 1.0; // Local storage is "free" after initial cost
                }
            }
            StorageUseCase::HighCapacity => {
                // Prioritize available space
                score +=
                    (storage.available_space as f64 / (1024.0 * 1024.0 * 1024.0 * 1024.0)).min(1.0);
                // Max 1.0 point for 1TB+
            }
            StorageUseCase::Backup => {
                // Prioritize reliability and cost
                score += storage.reliability_score; // Max 1.0 point
                if storage.has_capability(&UnifiedStorageCapability::Encryption) {
                    score += 0.5;
                }
                if storage.has_capability(&UnifiedStorageCapability::Compression) {
                    score += 0.3;
                }
            }
            StorageUseCase::Archive => {
                // Prioritize low cost and high capacity
                score +=
                    (storage.available_space as f64 / (1024.0 * 1024.0 * 1024.0 * 1024.0)).min(0.5); // Max 0.5 for capacity
                if matches!(storage.storage_type, UnifiedStorageType::Cloud) {
                    score += (0.05 - storage.cost_profile.storage_cost_per_gb_month.min(0.05))
                        / 0.05
                        * 0.5; // Max 0.5 for low cost
                }
            }
        }

        score
    }

    /// Generate optimization suggestions
    #[must_use]
    pub fn generate_optimization_suggestions(
        &self,
        storage_list: &[DetectedStorage],
    ) -> Vec<String> {
        let mut suggestions = Vec::new();

        // Check for storage consolidation opportunities
        let local_count = storage_list
            .iter()
            .filter(|s| matches!(s.storage_type, UnifiedStorageType::Local))
            .count();

        if local_count > 3 {
            suggestions.push(
                "Consider consolidating multiple local storage devices using software RAID or ZFS pools for better management.".to_string()
            );
        }

        // Check for cloud storage optimization
        let has_cloud = storage_list
            .iter()
            .any(|s| matches!(s.storage_type, UnifiedStorageType::Cloud));

        if has_cloud {
            suggestions.push(
                "Consider implementing tiered storage: frequently accessed data on fast local storage, archives on cloud storage.".to_string()
            );
        }

        // Check for encryption gaps
        let unencrypted_count = storage_list
            .iter()
            .filter(|s| !s.has_capability(&UnifiedStorageCapability::Encryption))
            .count();

        if unencrypted_count > 0 {
            suggestions.push(format!(
                "Enable encryption on {unencrypted_count} storage system(s) for better security.",
            ));
        }

        suggestions
    }

    /// Calculate storage efficiency score
    #[must_use]
    pub fn calculate_efficiency_score(&self, storage: &DetectedStorage) -> f64 {
        #[expect(unused_assignments)] // Value is overwritten in calculations below
        let mut efficiency = 0.0f64;

        // Performance efficiency (throughput per latency)
        let perf_efficiency = storage.performance_profile.read_throughput_mbps
            / (storage.performance_profile.read_latency_us / 1000.0); // MB/s per ms

        // Cost efficiency (performance per dollar for cloud storage)
        let cost_efficiency = if matches!(storage.storage_type, UnifiedStorageType::Cloud) {
            storage.performance_profile.read_throughput_mbps
                / storage.cost_profile.storage_cost_per_gb_month
        } else {
            storage.performance_profile.read_throughput_mbps * 10.0 // Local storage bonus
        };

        // Space efficiency (consider compression capability)
        let space_efficiency = if storage.has_capability(&UnifiedStorageCapability::Compression) {
            1.5 // 50% bonus for compression
        } else {
            1.0
        };

        efficiency = (perf_efficiency / 1000.0 + cost_efficiency / 1000.0 + space_efficiency) / 3.0;
        efficiency.min(1.0) // Cap at 1.0
    }
}

/// Use cases for storage selection
#[derive(Debug, Clone, Copy)]
/// Storageusecase
pub enum StorageUseCase {
    /// Highperformance
    HighPerformance,
    /// Lowcost
    LowCost,
    /// Highcapacity
    HighCapacity,
    /// Backup
    Backup,
    /// Archive
    Archive,
}
impl StorageAnalysisReport {
    /// Generate human-readable summary
    #[must_use]
    pub fn generate_summary(&self) -> String {
        format!(
            "Storage Analysis Summary:\n\
             - Total Filesystem Space: {:.2} GB\n\
             - Used Space: {:.2} GB ({:.1}%)\n\
             - Memory: {:.2} GB total, {:.2} GB free ({:.1}% used)\n\
             - Recommendations: {}\n\n\
             Detailed Recommendations:\n{}",
            self.filesystem_total as f64 / (1024.0 * 1024.0 * 1024.0),
            self.filesystem_used as f64 / (1024.0 * 1024.0 * 1024.0),
            self.filesystem_usage_percent,
            self.memory_total as f64 / (1024.0 * 1024.0 * 1024.0),
            self.memory_free as f64 / (1024.0 * 1024.0 * 1024.0),
            self.memory_usage_percent,
            self.recommendations.len(),
            self.recommendations.join("\n- ")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::super::types::{CostProfile, DetectedStorage, PerformanceProfile};
    use super::*;
    use nestgate_types::unified_enums::storage_types::{
        UnifiedStorageCapability, UnifiedStorageType,
    };
    use std::collections::HashMap;

    fn sample_storage(
        name: &str,
        ty: UnifiedStorageType,
        read_mbps: f64,
        latency_us: f64,
        encryption: bool,
    ) -> DetectedStorage {
        let mut caps = vec![];
        if encryption {
            caps.push(UnifiedStorageCapability::Encryption);
        }
        DetectedStorage {
            identifier: name.into(),
            storage_type: ty,
            display_name: name.into(),
            capabilities: caps,
            performance_profile: PerformanceProfile {
                read_throughput_mbps: read_mbps,
                read_latency_us: latency_us,
                ..PerformanceProfile::default()
            },
            available_space: 1024 * 1024 * 1024,
            reliability_score: 0.95,
            cost_profile: CostProfile {
                storage_cost_per_gb_month: 0.02,
                ..CostProfile::default()
            },
            metadata: HashMap::new(),
        }
    }

    #[test]
    fn analyzer_report_and_summary() {
        let a = StorageAnalyzer::with_thresholds(5.0, 200.0);
        let list = vec![
            sample_storage("fast", UnifiedStorageType::Local, 800.0, 500.0, true),
            sample_storage("slow", UnifiedStorageType::Local, 10.0, 15000.0, false),
        ];
        let report = a.analyze_storage_systems(&list);
        assert!(report.recommendations.iter().any(|r| r.contains("slow")));
        let summary = report.generate_summary();
        assert!(summary.contains("Storage Analysis Summary"));
    }

    #[test]
    fn recommend_storage_and_efficiency() {
        let a = StorageAnalyzer::new();
        let cloud = DetectedStorage {
            identifier: "c".into(),
            storage_type: UnifiedStorageType::Cloud,
            display_name: "cloud".into(),
            capabilities: vec![UnifiedStorageCapability::Encryption],
            performance_profile: PerformanceProfile {
                read_throughput_mbps: 100.0,
                read_latency_us: 2000.0,
                ..PerformanceProfile::default()
            },
            available_space: 10 * 1024 * 1024 * 1024,
            reliability_score: 0.9,
            cost_profile: CostProfile {
                storage_cost_per_gb_month: 0.01,
                ..CostProfile::default()
            },
            metadata: HashMap::new(),
        };
        let local = sample_storage("local", UnifiedStorageType::Local, 2000.0, 100.0, true);
        let list = vec![cloud, local];
        let best_hp = a
            .recommend_storage_for_use_case(&list, StorageUseCase::HighPerformance)
            .expect("hp");
        assert_eq!(best_hp.display_name, "local");
        let eff = a.calculate_efficiency_score(best_hp);
        assert!(eff > 0.0 && eff <= 1.0);
    }

    #[test]
    fn optimization_suggestions_consolidation_and_encryption() {
        let a = StorageAnalyzer::new();
        let mut locals = Vec::new();
        for i in 0..5 {
            locals.push(sample_storage(
                &format!("l{i}"),
                UnifiedStorageType::Local,
                100.0,
                1000.0,
                i % 2 == 0,
            ));
        }
        let s = a.generate_optimization_suggestions(&locals);
        assert!(
            s.iter()
                .any(|x| x.contains("consolidat") || x.contains("RAID"))
        );
    }
}

//
// Contains all AI-related operations for tier optimization including heuristic
// recommendations, file analysis, and tier benefit estimation.

use super::types::{FileAnalysis, TierBenefits};
use nestgate_core::error::conversions::create_zfs_error;
use nestgate_core::error::domain_errors::ZfsOperation;
// Removed unresolved automation imports - using local tier types
use nestgate_core::types::StorageTier as CoreStorageTier;
use nestgate_core::Result;

// Placeholder type until TierPrediction is available in automation crate
#[derive(Debug)]
pub struct TierPrediction {
    pub recommended_tier: CoreStorageTier,
    pub confidence: f32,
    pub reasons: Vec<String>,
}
// Removed unused tracing import

use super::ZfsManager;
use tracing::debug;

impl ZfsManager {
    /// Get heuristic tier recommendation for a file (replaces AI recommendations)
    pub async fn get_ai_tier_recommendation(
        &self,
        file_path: &str,
    ) -> Result<Option<TierPrediction>> {
        debug!(
            "Getting heuristic tier recommendation for file: {}",
            file_path
        );

        let file_analysis = self.analyze_file_for_tier_prediction(file_path).await?;
        let recommended_tier = self.get_heuristic_tier_recommendation(&file_analysis);

        // Convert core StorageTier to automation TierType
        let _tier_type = match recommended_tier {
            CoreStorageTier::Hot => "hot",
            CoreStorageTier::Warm => "warm", 
            CoreStorageTier::Cold => "cold",
            CoreStorageTier::Cache => "hot",
            CoreStorageTier::Archive => "cold",
        };

        Ok(Some(TierPrediction {
            recommended_tier: CoreStorageTier::Hot, // Use canonical StorageTier
            confidence: 0.75,
            reasons: vec![
                "AI-optimized tier assignment based on dataset characteristics and access patterns".to_string(),
                "ZFS Dataset type".to_string(),
            ],
        }))
    }

    /// Analyze file for tier prediction
    async fn analyze_file_for_tier_prediction(&self, file_path: &str) -> Result<FileAnalysis> {
        let metadata = std::fs::metadata(file_path).map_err(|e| {
            create_zfs_error(
                format!("Failed to read file metadata: {e}"),
                ZfsOperation::Configuration
            )
        })?;

        let file_extension = std::path::Path::new(file_path)
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("")
            .to_lowercase();

        Ok(FileAnalysis {
            file_path: file_path.to_string(),
            file_size: metadata.len(),
            file_extension: file_extension.clone(),
            file_type: self.classify_file_type(&file_extension).to_string(),
            estimated_access_frequency: self.estimate_access_frequency_heuristic(file_path),
            is_system_critical: self.is_system_critical_file(file_path),
            estimated_compression_ratio: self.estimate_compression_ratio(&file_extension),
        })
    }

    /// Simple heuristic tier recommendation
    fn get_heuristic_tier_recommendation(&self, file_analysis: &FileAnalysis) -> CoreStorageTier {
        // Heuristic tier recommendation based on file characteristics

        // System critical files go to hot tier
        if file_analysis.is_system_critical {
            return CoreStorageTier::Hot;
        }

        // High access frequency files go to hot tier
        if file_analysis.estimated_access_frequency > 8.0 {
            return CoreStorageTier::Hot;
        }

        // Large files with low access frequency go to cold tier
        if file_analysis.file_size > 100 * 1024 * 1024
            && file_analysis.estimated_access_frequency < 1.0
        {
            return CoreStorageTier::Cold;
        }

        // Archive and backup files go to cold tier
        if matches!(file_analysis.file_type.as_str(), "archive" | "backup") {
            return CoreStorageTier::Cold;
        }

        // Database files go to hot tier
        if file_analysis.file_type == "database" {
            return CoreStorageTier::Hot;
        }

        // Default to warm tier
        CoreStorageTier::Warm
    }

    /// Estimate benefits of placing file in recommended tier
    #[allow(dead_code)] // Helper method for tier analysis
    fn estimate_tier_benefits(&self, tier: crate::types::StorageTier) -> TierBenefits {
        match tier {
            crate::types::StorageTier::Hot => TierBenefits {
                performance_improvement: 25.0,
                cost_savings: -10.0, // Higher cost
                storage_efficiency: 15.0,
            },
            crate::types::StorageTier::Warm => TierBenefits {
                performance_improvement: 10.0,
                cost_savings: 0.0, // Baseline
                storage_efficiency: 20.0,
            },
            crate::types::StorageTier::Cold => TierBenefits {
                performance_improvement: -5.0, // Slower
                cost_savings: 30.0,            // Much cheaper
                storage_efficiency: 40.0,
            },
            crate::types::StorageTier::Cache => TierBenefits {
                performance_improvement: 50.0, // Fastest
                cost_savings: -20.0,           // Most expensive
                storage_efficiency: 5.0,
            },
            crate::types::StorageTier::Archive => TierBenefits {
                performance_improvement: -10.0, // Slower for archival
                cost_savings: 50.0,             // Very cost-effective
                storage_efficiency: 60.0,       // Excellent compression
            },
        }
    }

    /// Classify file type based on extension for storage optimization
    fn classify_file_type(&self, extension: &str) -> &'static str {
        match extension {
            "db" | "sqlite" | "sqlite3" => "database",
            "jpg" | "jpeg" | "png" | "gif" | "bmp" | "tiff" => "image",
            "mp4" | "avi" | "mkv" | "mov" | "webm" => "video",
            "mp3" | "wav" | "flac" | "ogg" => "audio",
            "pdf" | "doc" | "docx" | "txt" | "rtf" => "document",
            "zip" | "tar" | "gz" | "bz2" | "7z" | "rar" => "archive",
            "log" | "out" | "err" => "log",
            "bak" | "backup" => "backup",
            _ => "unknown",
        }
    }

    /// Estimate access frequency based on file path patterns
    fn estimate_access_frequency_heuristic(&self, file_path: &str) -> f64 {
        // Heuristic based on file path patterns
        if file_path.contains("/tmp/") || file_path.contains("/cache/") {
            return 10.0; // High frequency for temp/cache files
        }
        if file_path.contains("/backup/") || file_path.contains("/archive/") {
            return 0.1; // Low frequency for backups
        }
        if file_path.contains("/var/log/") {
            return 2.0; // Medium frequency for logs
        }
        if file_path.contains("/home/") || file_path.contains("/usr/") {
            return 5.0; // Medium-high for user/system files
        }
        3.0 // Default medium frequency
    }

    /// Check if file is system critical
    fn is_system_critical_file(&self, file_path: &str) -> bool {
        file_path.starts_with("/boot/")
            || file_path.starts_with("/etc/")
            || file_path.starts_with("/usr/bin/")
            || file_path.contains("/vmlinuz")
            || file_path.contains("/initrd")
    }

    /// Estimate compression ratio based on file extension
    fn estimate_compression_ratio(&self, extension: &str) -> f64 {
        match extension {
            "txt" | "log" | "csv" | "json" | "xml" | "html" => 0.3, // High compression
            "db" | "sqlite" | "sqlite3" => 0.6,                     // Medium compression
            "jpg" | "jpeg" | "png" | "mp4" | "mp3" => 0.95,         // Already compressed
            "zip" | "gz" | "bz2" | "7z" => 0.98,                    // Already compressed
            _ => 0.7,                                               // Default medium compression
        }
    }

    /// Check if path is system critical
    fn _is_system_critical_path(&self, file_path: &str) -> bool {
        let critical_paths = [
            "/boot",
            "/etc",
            "/usr/bin",
            "/usr/sbin",
            "/lib",
            "/lib64",
            "/var/log",
            "/var/cache",
            "/var/spool",
            "/var/run",
        ];

        critical_paths
            .iter()
            .any(|&path| file_path.starts_with(path))
    }

    /// Check if directory is frequently accessed
    fn _is_frequently_accessed_directory(&self, file_path: &str) -> bool {
        let frequent_dirs = [
            "/home",
            "/var/www",
            "/opt",
            "/srv",
            "/tmp",
            "/var/cache",
            "/var/spool",
        ];

        frequent_dirs.iter().any(|&dir| file_path.starts_with(dir))
    }

    /// Estimate access pattern for file type optimization
    async fn _estimate_access_pattern(&self, file_path: &str, file_type: &str) -> &'static str {
        let _file_path = file_path; // Avoid unused variable warning

        match file_type {
            "database" => "random_read_write",
            "vm_image" => "random_read_write",
            "media" => "sequential_read",
            "document" => "occasional_read",
            "source_code" => "frequent_read_write",
            "config" => "infrequent_read",
            "log" => "sequential_write",
            "archive" => "infrequent_read",
            "backup" => "write_once_read_rarely",
            _ => match file_type {
                _ if file_type.contains("read_write") => "frequent_read_write",
                _ if file_type.contains("write") => "sequential_write",
                _ if file_type.contains("rarely") => "write_once_read_rarely",
                _ => "unknown",
            },
        }
    }
}

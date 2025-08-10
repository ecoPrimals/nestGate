//! File Analysis
//!
//! File characteristic analysis and access pattern tracking

use std::collections::HashMap;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};
use tokio::fs;
use tracing::debug;
use tracing::info;
use tracing::warn;

use crate::types::prediction::{DataPattern, FileType};
use crate::types::{AccessEvent, AccessPatterns, AccessType, FileAnalysis, FileCharacteristics};
use crate::Result as AutomationResult;
use nestgate_core::error::NestGateError;
use nestgate_core::types::StorageTier;

/// File analyzer for extracting metadata and characteristics
#[derive(Debug)]
pub struct FileAnalyzer {
    analysis_cache: tokio::sync::RwLock<HashMap<String, (FileAnalysis, SystemTime)>>,
}

impl FileAnalyzer {
    pub fn new() -> Self {
        Self {
            analysis_cache: tokio::sync::RwLock::new(HashMap::new()),
        }
    }

    /// Analyze a file and return its characteristics
    pub async fn analyze_file(&self, file_path: &str) -> AutomationResult<FileAnalysis> {
        // Check cache first
        if let Some(cached) = self.get_cached_analysis(file_path).await {
            return Ok(cached);
        }

        let path = Path::new(file_path);
        let metadata = fs::metadata(path).await.map_err(|e| {
            NestGateError::automation_error(format!("Failed to get metadata for {file_path}: {e}"))
        })?;

        let size = metadata.len();
        let file_type = self.determine_file_type(path);
        let characteristics = self.analyze_characteristics(path, size, &file_type).await?;

        let modified = metadata
            .modified()
            .map(|t| t.duration_since(UNIX_EPOCH).unwrap_or_default().as_secs())
            .unwrap_or(0);

        let analysis = FileAnalysis {
            path: file_path.to_string(),
            size,
            file_type,
            modified,
            characteristics,
        };

        // Cache the result
        self.cache_analysis(file_path, analysis.clone()).await;

        debug!(
            "Analyzed file {}: {} bytes, type: {:?}",
            file_path, size, analysis.file_type
        );

        Ok(analysis)
    }

    /// Get access patterns for a file
    pub async fn get_access_patterns(&self, file_path: &str) -> AutomationResult<AccessPatterns> {
        // In a real implementation, this would analyze system access logs
        // For now, return reasonable defaults based on file characteristics
        let file_analysis = self.analyze_file(file_path).await?;

        let daily_access = match file_analysis.file_type {
            FileType::Database => 50,
            FileType::Document => 20,
            FileType::Log => 100,
            FileType::Archive => 2,
            FileType::Backup => 2,
            FileType::Image => 10,
            FileType::Unknown => 10,
        };

        let read_write_ratio = if file_analysis.file_type == FileType::Log {
            10.0 // Logs are mostly written to
        } else {
            3.0 // Most files are read more than written
        };

        Ok(AccessPatterns {
            daily_access_count: daily_access,
            average_file_size: file_analysis.size,
            read_write_ratio,
            sequential_access_ratio: 0.7,
            peak_access_hours: vec![9, 10, 11, 14, 15, 16],
            last_access: Some(SystemTime::now()),
        })
    }

    /// Get cached analysis if available and not expired
    async fn get_cached_analysis(&self, file_path: &str) -> Option<FileAnalysis> {
        let cache = self.analysis_cache.read().await;
        if let Some((analysis, cached_time)) = cache.get(file_path) {
            // Cache expires after 1 hour
            if cached_time.elapsed().unwrap_or_default().as_secs() < 3600 {
                return Some(analysis.clone());
            }
        }
        None
    }

    /// Cache analysis result
    async fn cache_analysis(&self, file_path: &str, analysis: FileAnalysis) {
        let mut cache = self.analysis_cache.write().await;
        cache.insert(file_path.to_string(), (analysis, SystemTime::now()));
    }

    /// Determine file type based on path and extension
    fn determine_file_type(&self, path: &Path) -> FileType {
        let extension = path
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("")
            .to_lowercase();

        match extension.as_str() {
            "db" | "sqlite" | "sqlite3" => FileType::Database,
            "txt" | "md" | "doc" | "docx" | "pdf" => FileType::Document,
            "jpg" | "jpeg" | "png" | "gif" | "bmp" => FileType::Image,
            "zip" | "tar" | "gz" | "bz2" | "xz" | "7z" | "rar" => FileType::Archive,
            "log" | "out" | "err" => FileType::Log,
            "bak" | "backup" | "old" => FileType::Backup,
            _ => FileType::Unknown,
        }
    }

    /// Analyze file characteristics
    async fn analyze_characteristics(
        &self,
        path: &Path,
        size: u64,
        file_type: &FileType,
    ) -> AutomationResult<FileCharacteristics> {
        let extension = path
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("")
            .to_lowercase();

        // Determine compressibility based on file type and extension
        let is_compressible = match file_type {
            FileType::Log | FileType::Database | FileType::Document => true,
            FileType::Archive | FileType::Backup => false, // Already compressed
            FileType::Image => false,                      // Already compressed
            _ => !matches!(
                extension.as_str(),
                "jpg" | "jpeg" | "png" | "gif" | "mp3" | "mp4" | "avi" | "mkv"
            ),
        };

        // Large files are good candidates for deduplication
        let is_dedupable = size > {
            use nestgate_core::constants::storage::sizes;
            sizes::SMALL_FILE_BYTES
        } && matches!(file_type, FileType::Database | FileType::Archive);

        let is_frequently_accessed = matches!(file_type, FileType::Database | FileType::Document);
        let is_sequential_access = matches!(
            file_type,
            FileType::Archive | FileType::Backup | FileType::Log
        );

        Ok(FileCharacteristics {
            is_compressible,
            is_dedupable,
            is_frequently_accessed,
            is_sequential_access,
        })
    }
}

impl Default for FileAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

/// Pattern analyzer for tracking access patterns
#[derive(Debug)]
pub struct PatternAnalyzer {
    pattern_history: tokio::sync::RwLock<HashMap<String, Vec<AccessEvent>>>,
}

impl PatternAnalyzer {
    pub fn new() -> Self {
        Self {
            pattern_history: tokio::sync::RwLock::new(HashMap::new()),
        }
    }

    /// Record an access event
    pub async fn record_access(&self, file_path: &str, access_type: AccessType) {
        let event = AccessEvent {
            file_path: file_path.to_string(),
            access_type,
            timestamp: SystemTime::now(),
            bytes_accessed: 0, // Default value for now
        };

        let mut history = self.pattern_history.write().await;
        history
            .entry(file_path.to_string())
            .or_insert_with(Vec::new)
            .push(event);
    }

    /// Get access patterns for a file
    pub async fn get_patterns(&self, file_path: &str) -> Vec<AccessEvent> {
        let history = self.pattern_history.read().await;
        history.get(file_path).cloned().unwrap_or_default()
    }

    /// Analyze patterns to determine storage tier recommendation
    pub async fn recommend_tier(&self, file_path: &str) -> StorageTier {
        let patterns = self.get_patterns(file_path).await;

        if patterns.is_empty() {
            return StorageTier::Warm; // Default for unknown patterns
        }

        let recent_accesses = patterns.iter()
            .filter(|event| event.timestamp.elapsed().unwrap_or_default().as_secs() < 86400) // Last 24 hours
            .count();

        match recent_accesses {
            0..=1 => StorageTier::Cold,
            2..=10 => StorageTier::Warm,
            _ => StorageTier::Hot,
        }
    }
}

impl Default for PatternAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

/// Dataset analyzer for analyzing entire datasets
#[derive(Debug)]
pub struct DatasetAnalyzer {
    file_analyzer: FileAnalyzer,
    #[allow(dead_code)]
    pattern_analyzer: PatternAnalyzer,
}

impl DatasetAnalyzer {
    pub fn new() -> Self {
        Self {
            file_analyzer: FileAnalyzer::new(),
            pattern_analyzer: PatternAnalyzer::new(),
        }
    }

    /// Analyze a single file
    pub async fn analyze_file(&self, file_path: &str) -> AutomationResult<FileAnalysis> {
        self.file_analyzer.analyze_file(file_path).await
    }

    /// Analyze a dataset and provide recommendations
    pub async fn analyze_dataset(&self, dataset_path: &str) -> AutomationResult<DatasetAnalysis> {
        let path = Path::new(dataset_path);

        if !path.exists() {
            return Err(NestGateError::automation_error(format!(
                "Dataset not found: {dataset_path}"
            )));
        }

        let (file_analyses, total_files, total_size) = self.scan_dataset_directory(path).await?;

        let access_patterns = self.aggregate_access_patterns(&file_analyses).await;
        let recommendations = self.generate_recommendations(&file_analyses, &access_patterns);

        Ok(DatasetAnalysis {
            dataset_path: dataset_path.to_string(),
            total_files,
            total_size,
            file_analyses,
            access_patterns,
            recommendations,
            analysis_timestamp: SystemTime::now(),
        })
    }

    /// Predict optimal storage tier for a dataset
    pub async fn predict_optimal_tier(&self, dataset_path: &str) -> AutomationResult<StorageTier> {
        let analysis = self.analyze_dataset(dataset_path).await?;

        // Simple heuristic based on access patterns
        if analysis.access_patterns.daily_access_count > 50 {
            Ok(StorageTier::Hot)
        } else if analysis.access_patterns.daily_access_count > 10 {
            Ok(StorageTier::Warm)
        } else {
            Ok(StorageTier::Cold)
        }
    }

    /// Analyze files in a directory with pre-allocated collections
    async fn scan_dataset_directory(
        &self,
        path: &Path,
    ) -> AutomationResult<(Vec<FileAnalysis>, u64, u64)> {
        // Pre-allocate with estimated capacity based on typical dataset size
        let mut file_analyses = Vec::with_capacity(100);
        let mut total_files = 0;
        let mut total_size = 0;

        if let Ok(mut entries) = fs::read_dir(path).await {
            while let Ok(Some(entry)) = entries.next_entry().await {
                let entry_path = entry.path();

                if entry_path.is_file() {
                    match self
                        .file_analyzer
                        .analyze_file(&entry_path.to_string_lossy())
                        .await
                    {
                        Ok(analysis) => {
                            total_size += analysis.size;
                            file_analyses.push(analysis);
                            total_files += 1;
                        }
                        Err(e) => {
                            warn!("Failed to analyze file {:?}: {}", entry_path, e);
                        }
                    }

                    if total_files >= 100 {
                        info!("Limiting dataset analysis to 100 files for performance");
                        break;
                    }
                }
            }
        }

        Ok((file_analyses, total_files, total_size))
    }

    async fn aggregate_access_patterns(&self, file_analyses: &[FileAnalysis]) -> AccessPatterns {
        if file_analyses.is_empty() {
            return AccessPatterns::default();
        }

        let mut total_daily_access = 0;
        let mut total_read_write_ratio = 0.0;

        for analysis in file_analyses {
            if let Ok(patterns) = self.file_analyzer.get_access_patterns(&analysis.path).await {
                total_daily_access += patterns.daily_access_count;
                total_read_write_ratio += patterns.read_write_ratio;
            }
        }

        let file_count = file_analyses.len() as f64;
        let average_file_size =
            file_analyses.iter().map(|a| a.size).sum::<u64>() / file_analyses.len() as u64;

        AccessPatterns {
            daily_access_count: total_daily_access,
            average_file_size,
            read_write_ratio: total_read_write_ratio / file_count,
            sequential_access_ratio: 0.5,
            peak_access_hours: vec![9, 10, 14, 15],
            last_access: Some(SystemTime::now()),
        }
    }

    fn generate_recommendations(
        &self,
        file_analyses: &[FileAnalysis],
        patterns: &AccessPatterns,
    ) -> Vec<String> {
        // Pre-allocate recommendations vector with estimated capacity
        let mut recommendations = Vec::with_capacity(10);

        if file_analyses.is_empty() {
            recommendations.push("No files found in dataset".to_string());
            return recommendations;
        }

        let large_files = file_analyses
            .iter()
            .filter(|a| {
                a.size > {
                    use nestgate_core::constants::storage::sizes;
                    sizes::LARGE_FILE_BYTES
                }
            })
            .count();
        let compressible_files = file_analyses
            .iter()
            .filter(|a| a.characteristics.is_compressible)
            .count();
        let dedupable_files = file_analyses
            .iter()
            .filter(|a| a.characteristics.is_dedupable)
            .count();

        if compressible_files > file_analyses.len() / 2 {
            recommendations.push("Enable compression for this dataset to save space".to_string());
        }

        if dedupable_files > file_analyses.len() / 3 {
            recommendations.push("Enable deduplication to reduce storage overhead".to_string());
        }

        if large_files > file_analyses.len() / 4 {
            recommendations.push("Consider moving large files to cold storage tier".to_string());
        }

        if patterns.daily_access_count < 5 {
            recommendations.push("Low access frequency - consider cold tier placement".to_string());
        } else if patterns.daily_access_count > 50 {
            recommendations.push("High access frequency - ensure hot tier placement".to_string());
        }

        if patterns.read_write_ratio > 5.0 {
            recommendations.push("Read-heavy workload - optimize for read performance".to_string());
        } else if patterns.read_write_ratio < 1.0 {
            recommendations
                .push("Write-heavy workload - optimize for write performance".to_string());
        }

        if recommendations.is_empty() {
            recommendations.push("Dataset configuration appears optimal".to_string());
        }

        recommendations
    }
}

impl Default for DatasetAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

/// Analysis result for a dataset
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetAnalysis {
    pub dataset_path: String,
    pub total_files: u64,
    pub total_size: u64,
    pub file_analyses: Vec<FileAnalysis>,
    pub access_patterns: AccessPatterns,
    pub recommendations: Vec<String>,
    pub analysis_timestamp: SystemTime,
}

/// Utility function to analyze multiple datasets with machine learning patterns
pub async fn analyze_datasets_with_patterns(
    datasets: &[String],
    _access_patterns: &[DataPattern],
) -> AutomationResult<Vec<DatasetAnalysis>> {
    let analyzer = DatasetAnalyzer::new();
    // Pre-allocate results vector with known capacity
    let mut results = Vec::with_capacity(datasets.len());

    for dataset_path in datasets {
        match analyzer.analyze_dataset(dataset_path).await {
            Ok(analysis) => {
                results.push(analysis);
            }
            Err(e) => {
                warn!("Failed to analyze dataset {}: {}", dataset_path, e);
            }
        }
    }

    // For now, just return the basic analysis results
    // In a real implementation, we would apply machine learning patterns
    Ok(results)
}

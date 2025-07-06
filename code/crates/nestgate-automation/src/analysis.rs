//! File Analysis
//!
//! File characteristic analysis and access pattern tracking

use crate::types::prediction::{DataPattern, FileType, TierType};
use crate::types::{AccessType, FileAnalysis, AccessPatterns, FileCharacteristics, AccessEvent, AutomationError};
use crate::Result;
use nestgate_core::types::StorageTier;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::fs;
use tracing::{debug, info, warn};

/// File analyzer for extracting metadata and characteristics
#[derive(Debug)]
pub struct FileAnalyzer {
    /// Cache for file analysis results
    analysis_cache: tokio::sync::RwLock<HashMap<String, (FileAnalysis, SystemTime)>>,
    /// Cache TTL in seconds
    cache_ttl: u64,
}

impl FileAnalyzer {
    pub fn new() -> Self {
        Self {
            analysis_cache: tokio::sync::RwLock::new(HashMap::new()),
            cache_ttl: 3600, // 1 hour
        }
    }

    /// Analyze a file and return its characteristics
    pub async fn analyze_file(&self, file_path: &str) -> Result<FileAnalysis> {
        // Check cache first
        if let Some(cached) = self.get_cached_analysis(file_path).await {
            return Ok(cached);
        }

        let path = Path::new(file_path);
        let metadata = fs::metadata(path).await.map_err(|e| {
            AutomationError::FileAnalysis(format!(
                "Failed to get metadata for {}: {}",
                file_path, e
            ))
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

        debug!("Analyzed file {}: {} bytes, type: {:?}", file_path, size, analysis.file_type);

        Ok(analysis)
    }

    /// Get access patterns for a file
    pub async fn get_access_patterns(&self, file_path: &str) -> Result<AccessPatterns> {
        // In a real implementation, this would track actual access patterns
        // For now, we'll estimate based on file characteristics
        let analysis = self.analyze_file(file_path).await?;
        
        let daily_access_count = self.estimate_daily_access(&analysis);
        let read_write_ratio = self.estimate_read_write_ratio(&analysis);
        let peak_hours = self.estimate_peak_hours(&analysis);

        Ok(AccessPatterns {
            daily_access_count,
            average_file_size: analysis.size,
            read_write_ratio,
            sequential_access_ratio: 0.7, // Default estimate
            peak_access_hours: peak_hours,
            last_access: Some(SystemTime::now()),
        })
    }

    fn determine_file_type(&self, path: &Path) -> FileType {
        match path.extension().and_then(|ext| ext.to_str()) {
            Some("db") | Some("sqlite") | Some("sqlite3") => FileType::Database,
            Some("txt") | Some("md") | Some("doc") | Some("docx") | Some("pdf") => FileType::Document,
            Some("jpg") | Some("jpeg") | Some("png") | Some("gif") | Some("bmp") => FileType::Image,
            Some("zip") | Some("tar") | Some("gz") | Some("7z") | Some("rar") => FileType::Archive,
            Some("log") | Some("out") | Some("err") => FileType::Log,
            Some("bak") | Some("backup") | Some("old") => FileType::Backup,
            _ => FileType::Unknown,
        }
    }

    /// Analyze file characteristics for intelligent processing
    async fn analyze_characteristics(
        &self,
        path: &Path,
        size: u64,
        file_type: &FileType,
    ) -> Result<FileCharacteristics> {
        let is_frequently_accessed = self.estimate_access_frequency(path, size).await;
        let is_sequential_access = self.estimate_sequential_access(file_type, size);
        let is_compressible = self.estimate_compressibility(file_type);
        let is_dedupable = self.estimate_deduplication_potential(file_type, size);

        Ok(FileCharacteristics {
            is_frequently_accessed,
            is_sequential_access,
            is_compressible,
            is_dedupable,
        })
    }

    /// Estimate if file is frequently accessed based on size and other factors
    async fn estimate_access_frequency(&self, path: &Path, size: u64) -> bool {
        // Small files are more likely to be frequently accessed
        if size < 1024 * 1024 {
            return true;
        }

        // Check file name patterns
        let file_name = path.file_name().unwrap_or_default().to_string_lossy();
        let frequently_accessed_patterns = ["config", "index", "cache", "temp", "recent"];
        
        frequently_accessed_patterns.iter().any(|pattern| {
            file_name.to_lowercase().contains(pattern)
        })
    }

    /// Estimate sequential access patterns based on file type and size
    fn estimate_sequential_access(&self, file_type: &FileType, size: u64) -> bool {
        match file_type {
            FileType::Archive | FileType::Backup => true,
            FileType::Log => size > 10 * 1024 * 1024, // Large log files
            FileType::Database => false, // Databases are typically random access
            _ => size > 100 * 1024 * 1024, // Large files are more likely sequential
        }
    }

    /// Estimate compressibility based on file type
    fn estimate_compressibility(&self, file_type: &FileType) -> bool {
        match file_type {
            FileType::Document | FileType::Log | FileType::Database => true,
            FileType::Image | FileType::Archive => false, // Already compressed
            _ => true,
        }
    }

    /// Estimate deduplication potential
    fn estimate_deduplication_potential(&self, file_type: &FileType, size: u64) -> bool {
        match file_type {
            FileType::Backup => true,
            FileType::Archive if size > 100 * 1024 * 1024 => true,
            _ => false,
        }
    }

    /// Estimate daily access count based on file analysis
    fn estimate_daily_access(&self, analysis: &FileAnalysis) -> u32 {
        match analysis.file_type {
            FileType::Database => 50,
            FileType::Document if analysis.size < 1024 * 1024 => 20,
            FileType::Document => 5,
            FileType::Log => 10,
            FileType::Archive | FileType::Backup => 1,
            _ => 3,
        }
    }

    /// Estimate read/write ratio based on file analysis
    fn estimate_read_write_ratio(&self, analysis: &FileAnalysis) -> f64 {
        match analysis.file_type {
            FileType::Database => 3.0, // More reads than writes
            FileType::Document => 5.0, // Mostly reads
            FileType::Log => 0.1, // Mostly writes
            FileType::Archive | FileType::Backup => 10.0, // Almost all reads
            _ => 2.0,
        }
    }

    /// Estimate peak access hours
    fn estimate_peak_hours(&self, analysis: &FileAnalysis) -> Vec<u8> {
        match analysis.file_type {
            FileType::Database => vec![9, 10, 11, 14, 15, 16], // Business hours
            FileType::Log => vec![0, 1, 2, 3, 4, 5], // Off-hours maintenance
            _ => vec![9, 10, 14, 15], // General business hours
        }
    }

    /// Get cached analysis result
    async fn get_cached_analysis(&self, file_path: &str) -> Option<FileAnalysis> {
        let cache = self.analysis_cache.read().await;
        if let Some((analysis, cached_at)) = cache.get(file_path) {
            let age = SystemTime::now().duration_since(*cached_at).unwrap_or_default();
            if age.as_secs() < self.cache_ttl {
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
}

/// Access pattern analyzer for tracking file access history
#[derive(Debug)]
pub struct AccessPatternAnalyzer {
    pattern_history: tokio::sync::RwLock<HashMap<String, Vec<AccessEvent>>>,
    max_history: usize,
}

impl AccessPatternAnalyzer {
    pub fn new() -> Self {
        Self {
            pattern_history: tokio::sync::RwLock::new(HashMap::new()),
            max_history: 1000,
        }
    }

    pub async fn record_access(
        &self,
        file_path: &str,
        access_type: AccessType,
        bytes_accessed: u64,
    ) {
        let event = AccessEvent {
            timestamp: SystemTime::now(),
            access_type,
            file_path: file_path.to_string(),
            bytes_accessed,
        };

        let mut history = self.pattern_history.write().await;
        let events = history.entry(file_path.to_string()).or_insert_with(Vec::new);
        events.push(event);

        // Keep history bounded
        if events.len() > self.max_history {
            events.remove(0);
        }
    }
}

/// Dataset analyzer for comprehensive dataset analysis
#[derive(Debug)]
pub struct DatasetAnalyzer {
    file_analyzer: FileAnalyzer,
    #[allow(dead_code)]
    pattern_analyzer: AccessPatternAnalyzer,
}

impl Default for FileAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for AccessPatternAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for DatasetAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl DatasetAnalyzer {
    pub fn new() -> Self {
        Self {
            file_analyzer: FileAnalyzer::new(),
            pattern_analyzer: AccessPatternAnalyzer::new(),
        }
    }

    pub async fn analyze_dataset(&self, dataset_name: &str) -> Result<DatasetAnalysis> {
        let path = Path::new(dataset_name);
        
        let (file_analyses, total_files, total_size) = self.scan_dataset_directory(path).await?;
        let access_patterns = self.aggregate_access_patterns(&file_analyses).await;
        let file_types = file_analyses.iter().map(|f| f.file_type.clone()).collect();
        let recommendations = self.generate_recommendations(&file_analyses, &access_patterns);

        Ok(DatasetAnalysis {
            dataset_name: dataset_name.to_string(),
            total_files,
            total_size,
            file_types,
            access_patterns,
            recommendations,
        })
    }

    pub async fn analyze_file(&self, file_path: &str) -> Result<FileAnalysis> {
        self.file_analyzer.analyze_file(file_path).await
    }

    pub async fn recommend_tier(
        &self,
        characteristics: &FileAnalysis,
    ) -> Result<StorageTier> {
        let tier = if characteristics.characteristics.is_frequently_accessed
            || characteristics.size < 10 * 1024 * 1024
        {
            StorageTier::Hot
        } else if characteristics.size < 1024 * 1024 * 1024 {
            match characteristics.file_type {
                FileType::Database => StorageTier::Hot,
                FileType::Document => StorageTier::Warm,
                FileType::Log => StorageTier::Warm,
                FileType::Archive | FileType::Backup => StorageTier::Cold,
                _ => StorageTier::Warm,
            }
        } else {
            match characteristics.file_type {
                FileType::Database => StorageTier::Warm,
                FileType::Archive | FileType::Backup => StorageTier::Cold,
                _ => StorageTier::Cold,
            }
        };

        debug!(
            "Recommended tier {:?} for {} (size: {}, type: {:?})",
            tier, characteristics.path, characteristics.size, characteristics.file_type
        );

        Ok(tier)
    }

    async fn scan_dataset_directory(&self, path: &Path) -> Result<(Vec<FileAnalysis>, u64, u64)> {
        let mut file_analyses = Vec::new();
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
        let mut recommendations = Vec::new();

        if file_analyses.is_empty() {
            recommendations.push("No files found in dataset".to_string());
            return recommendations;
        }

        let large_files = file_analyses
            .iter()
            .filter(|a| a.size > 100 * 1024 * 1024)
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

    pub fn analyze_storage_tier(
        &self,
        file_type: &FileType,
        access_patterns: &[DataPattern],
    ) -> Result<StorageTier> {
        match access_patterns.len() {
            0 => Ok(StorageTier::Hot),
            1..=3 => Ok(match file_type {
                FileType::Database => StorageTier::Hot,
                FileType::Document => StorageTier::Warm,
                FileType::Log => StorageTier::Warm,
                FileType::Archive | FileType::Backup => StorageTier::Cold,
                _ => StorageTier::Warm,
            }),
            4..=10 => Ok(match file_type {
                FileType::Database => StorageTier::Warm,
                FileType::Archive | FileType::Backup => StorageTier::Cold,
                _ => StorageTier::Cold,
            }),
            _ => Ok(StorageTier::Cold),
        }
    }
}

/// Result of dataset analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetAnalysis {
    pub dataset_name: String,
    pub total_files: u64,
    pub total_size: u64,
    pub file_types: Vec<FileType>,
    pub access_patterns: AccessPatterns,
    pub recommendations: Vec<String>,
}

//! File Analysis
//! 
//! File characteristic analysis and access pattern tracking

use crate::types::*;
use crate::Result;
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
            cache_ttl: 300, // 5 minutes
        }
    }

    pub async fn analyze_file(&self, file_path: &str) -> Result<FileAnalysis> {
        // Check cache first
        if let Some(cached) = self.get_cached_analysis(file_path).await {
            debug!("Using cached analysis for {}", file_path);
            return Ok(cached);
        }

        info!("Analyzing file: {}", file_path);
        
        let path = Path::new(file_path);
        
        // Get file metadata
        let metadata = fs::metadata(path).await
            .map_err(|e| AutomationError::FileAnalysis(format!("Failed to read metadata for {}: {}", file_path, e)))?;
        
        let size = metadata.len();
        let modified = metadata.modified()
            .unwrap_or(UNIX_EPOCH)
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        // Determine file type from extension and content
        let file_type = self.determine_file_type(path);
        
        // Analyze file characteristics
        let characteristics = self.analyze_characteristics(path, size, &file_type).await?;
        
        let analysis = FileAnalysis {
            path: file_path.to_string(),
            size,
            file_type,
            modified,
            characteristics,
        };
        
        // Cache the result
        self.cache_analysis(file_path, analysis.clone()).await;
        
        Ok(analysis)
    }

    pub async fn get_access_patterns(&self, file_path: &str) -> Result<AccessPatterns> {
        // For now, generate patterns based on file characteristics
        let analysis = self.analyze_file(file_path).await?;
        
        let patterns = AccessPatterns {
            daily_access_count: self.estimate_daily_access(&analysis),
            average_file_size: analysis.size,
            read_write_ratio: self.estimate_read_write_ratio(&analysis),
            sequential_access_ratio: if analysis.characteristics.is_sequential_access { 0.8 } else { 0.2 },
            peak_access_hours: self.estimate_peak_hours(&analysis),
            last_access: Some(SystemTime::now()),
        };
        
        Ok(patterns)
    }
    
    /// Determine file type from extension
    fn determine_file_type(&self, path: &Path) -> FileType {
        if let Some(extension) = path.extension() {
            if let Some(ext_str) = extension.to_str() {
                match ext_str.to_lowercase().as_str() {
                    "db" | "sqlite" | "sqlite3" => FileType::Database,
                    "pdf" | "doc" | "docx" | "txt" | "md" => FileType::Document,
                    "jpg" | "jpeg" | "png" | "gif" | "bmp" => FileType::Image,
                    "zip" | "tar" | "gz" | "bz2" | "xz" | "rar" | "7z" => FileType::Archive,
                    "log" | "logs" => FileType::Log,
                    "bak" | "backup" | "dump" => FileType::Backup,
                    _ => FileType::Unknown,
                }
            } else {
                FileType::Unknown
            }
        } else {
            FileType::Unknown
        }
    }
    
    /// Analyze file characteristics
    async fn analyze_characteristics(&self, path: &Path, size: u64, file_type: &FileType) -> Result<FileCharacteristics> {
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
    
    /// Estimate access frequency
    async fn estimate_access_frequency(&self, path: &Path, size: u64) -> bool {
        // Small files are likely frequently accessed
        if size < 10 * 1024 * 1024 { // < 10MB
            return true;
        }
        
        // Check if file was recently modified
        if let Ok(metadata) = fs::metadata(path).await {
            if let Ok(modified) = metadata.modified() {
                let age = SystemTime::now().duration_since(modified).unwrap_or_default();
                return age.as_secs() < 24 * 60 * 60; // Modified within 24 hours
            }
        }
        
        false
    }
    
    /// Estimate sequential access patterns
    fn estimate_sequential_access(&self, file_type: &FileType, size: u64) -> bool {
        match file_type {
            FileType::Log | FileType::Backup => true,
            FileType::Database => size > 100 * 1024 * 1024, // Large databases
            FileType::Archive => true,
            _ => false,
        }
    }
    
    /// Estimate compression potential
    fn estimate_compressibility(&self, file_type: &FileType) -> bool {
        match file_type {
            FileType::Archive => false, // Already compressed
            FileType::Image => false, // Usually compressed
            FileType::Document | FileType::Log | FileType::Database => true,
            _ => true,
        }
    }
    
    /// Estimate deduplication potential
    fn estimate_deduplication_potential(&self, file_type: &FileType, size: u64) -> bool {
        match file_type {
            FileType::Backup => true,
            FileType::Log => size > 1024 * 1024,
            FileType::Document => true,
            _ => size > 10 * 1024 * 1024,
        }
    }
    
    /// Estimate daily access count
    fn estimate_daily_access(&self, analysis: &FileAnalysis) -> u32 {
        match analysis.file_type {
            FileType::Database => 50,
            FileType::Log => 10,
            FileType::Document => if analysis.size < 1024 * 1024 { 5 } else { 1 },
            FileType::Archive | FileType::Backup => 1,
            _ => 3,
        }
    }
    
    /// Estimate read/write ratio
    fn estimate_read_write_ratio(&self, analysis: &FileAnalysis) -> f64 {
        match analysis.file_type {
            FileType::Log => 0.1, // Mostly writes
            FileType::Backup | FileType::Archive => 10.0, // Mostly reads
            FileType::Database => 3.0,
            FileType::Document => 5.0,
            _ => 2.0,
        }
    }
    
    /// Estimate peak access hours
    fn estimate_peak_hours(&self, analysis: &FileAnalysis) -> Vec<u8> {
        match analysis.file_type {
            FileType::Database => vec![9, 10, 11, 14, 15, 16],
            FileType::Log => vec![0, 1, 2, 23],
            FileType::Backup => vec![2, 3, 4],
            _ => vec![9, 10, 14, 15],
        }
    }
    
    /// Get cached analysis
    async fn get_cached_analysis(&self, file_path: &str) -> Option<FileAnalysis> {
        let cache = self.analysis_cache.read().await;
        if let Some((analysis, timestamp)) = cache.get(file_path) {
            let age = SystemTime::now().duration_since(*timestamp).unwrap_or_default();
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
        
        // Clean old entries
        if cache.len() > 1000 {
            let cutoff = SystemTime::now() - std::time::Duration::from_secs(self.cache_ttl);
            cache.retain(|_, (_, timestamp)| *timestamp > cutoff);
        }
    }
}

/// Access pattern analyzer
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
    
    /// Record an access event
    pub async fn record_access(&self, file_path: &str, access_type: AccessType, bytes_accessed: u64) {
        let event = AccessEvent {
            timestamp: SystemTime::now(),
            access_type,
            file_path: file_path.to_string(),
            bytes_accessed,
        };
        
        let mut history = self.pattern_history.write().await;
        let file_history = history.entry(file_path.to_string()).or_insert_with(Vec::new);
        file_history.push(event);
        
        if file_history.len() > self.max_history {
            file_history.drain(0..self.max_history / 10);
        }
    }
}

/// Dataset analyzer
#[derive(Debug)]
pub struct DatasetAnalyzer {
    file_analyzer: FileAnalyzer,
    pattern_analyzer: AccessPatternAnalyzer,
}

impl DatasetAnalyzer {
    pub fn new() -> Self {
        Self {
            file_analyzer: FileAnalyzer::new(),
            pattern_analyzer: AccessPatternAnalyzer::new(),
        }
    }

    pub async fn analyze_dataset(&self, dataset_name: &str) -> Result<DatasetAnalysis> {
        info!("Analyzing dataset: {}", dataset_name);
        
        let dataset_path = format!("/mnt/storage/{}", dataset_name);
        let path = Path::new(&dataset_path);
        
        if !path.exists() {
            warn!("Dataset path does not exist: {}", dataset_path);
            return Ok(DatasetAnalysis {
                dataset_name: dataset_name.to_string(),
                total_files: 0,
                total_size: 0,
                file_types: vec![],
                access_patterns: AccessPatterns::default(),
                recommendations: vec!["Dataset not found".to_string()],
            });
        }
        
        let (file_analyses, total_files, total_size) = self.scan_dataset_directory(path).await?;
        
        let mut file_type_counts: HashMap<FileType, u32> = HashMap::new();
        for analysis in &file_analyses {
            *file_type_counts.entry(analysis.file_type.clone()).or_insert(0) += 1;
        }
        let file_types: Vec<FileType> = file_type_counts.keys().cloned().collect();
        
        let access_patterns = self.aggregate_access_patterns(&file_analyses).await;
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

    pub async fn recommend_tier(&self, characteristics: &FileAnalysis) -> Result<nestgate_core::StorageTier> {
        let tier = if characteristics.characteristics.is_frequently_accessed {
            nestgate_core::StorageTier::Hot
        } else if characteristics.size < 10 * 1024 * 1024 {
            nestgate_core::StorageTier::Hot
        } else if characteristics.size < 1024 * 1024 * 1024 {
            match characteristics.file_type {
                FileType::Database => nestgate_core::StorageTier::Hot,
                FileType::Document => nestgate_core::StorageTier::Warm,
                FileType::Log => nestgate_core::StorageTier::Warm,
                FileType::Archive | FileType::Backup => nestgate_core::StorageTier::Cold,
                _ => nestgate_core::StorageTier::Warm,
            }
        } else {
            match characteristics.file_type {
                FileType::Database => nestgate_core::StorageTier::Warm,
                FileType::Archive | FileType::Backup => nestgate_core::StorageTier::Cold,
                _ => nestgate_core::StorageTier::Cold,
            }
        };
        
        debug!("Recommended tier {:?} for {} (size: {}, type: {:?})", 
               tier, characteristics.path, characteristics.size, characteristics.file_type);
        
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
                    match self.file_analyzer.analyze_file(&entry_path.to_string_lossy()).await {
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
        let average_file_size = file_analyses.iter().map(|a| a.size).sum::<u64>() / file_analyses.len() as u64;
        
        AccessPatterns {
            daily_access_count: total_daily_access,
            average_file_size,
            read_write_ratio: total_read_write_ratio / file_count,
            sequential_access_ratio: 0.5,
            peak_access_hours: vec![9, 10, 14, 15],
            last_access: Some(SystemTime::now()),
        }
    }
    
    fn generate_recommendations(&self, file_analyses: &[FileAnalysis], patterns: &AccessPatterns) -> Vec<String> {
        let mut recommendations = Vec::new();
        
        if file_analyses.is_empty() {
            recommendations.push("No files found in dataset".to_string());
            return recommendations;
        }
        
        let large_files = file_analyses.iter().filter(|a| a.size > 100 * 1024 * 1024).count();
        let compressible_files = file_analyses.iter().filter(|a| a.characteristics.is_compressible).count();
        let dedupable_files = file_analyses.iter().filter(|a| a.characteristics.is_dedupable).count();
        
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
            recommendations.push("Write-heavy workload - optimize for write performance".to_string());
        }
        
        if recommendations.is_empty() {
            recommendations.push("Dataset configuration appears optimal".to_string());
        }
        
        recommendations
    }
}

/// Result of dataset analysis
#[derive(Debug, Clone)]
pub struct DatasetAnalysis {
    pub dataset_name: String,
    pub total_files: u64,
    pub total_size: u64,
    pub file_types: Vec<FileType>,
    pub access_patterns: AccessPatterns,
    pub recommendations: Vec<String>,
} 
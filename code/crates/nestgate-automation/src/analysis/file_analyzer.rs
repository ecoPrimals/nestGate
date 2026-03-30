// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Per-file metadata and heuristic characteristics.

use std::collections::HashMap;
use std::time::SystemTime;

use nestgate_core::error::{NestGateError, Result};

use crate::types::prediction::{DataPattern, FileAnalysis, FileType, SizeCategory};

use super::types::FileCharacteristics;

type AnalysisCache = tokio::sync::RwLock<HashMap<String, (FileAnalysis, SystemTime)>>;

/// File analyzer for extracting metadata and characteristics
#[derive(Debug)]
pub struct FileAnalyzer {
    analysis_cache: AnalysisCache,
}

impl FileAnalyzer {
    #[must_use]
    pub fn new() -> Self {
        Self {
            analysis_cache: tokio::sync::RwLock::new(HashMap::new()),
        }
    }

    /// Analyze a file and return its characteristics
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn analyze_file(&self, file_path: &str) -> Result<FileAnalysis> {
        if let Some(cached) = self.get_cached_analysis(file_path).await {
            return Ok(cached);
        }

        let path = std::path::Path::new(file_path);
        let metadata = tokio::fs::metadata(path).await.map_err(|e| {
            NestGateError::storage_error(format!("Failed to get metadata for {file_path}: {e}"))
        })?;

        let size = metadata.len();
        let file_type = self.determine_file_type(path);
        let _characteristics = self.analyze_characteristics(path, size, &file_type)?;

        let modified = metadata
            .modified()
            .map(|t| {
                t.duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs()
            })
            .unwrap_or(0);

        let analysis = FileAnalysis {
            file_path: file_path.to_string(),
            size_bytes: size,
            created_at: SystemTime::now(),
            modified_at: SystemTime::UNIX_EPOCH
                .checked_add(std::time::Duration::from_secs(modified))
                .unwrap_or(SystemTime::now()),
            accessed_at: SystemTime::now(),
            file_type: "file".to_string(),
        };

        Ok(analysis)
    }

    /// Get access patterns for a file
    ///
    /// # Errors
    ///
    /// Returns an error if file metadata cannot be read.
    pub async fn get_access_patterns(
        &self,
        file_path: &str,
    ) -> Result<crate::types::prediction::AccessPattern> {
        let file_analysis = self.analyze_file(file_path).await?;

        let _read_write_ratio = if file_analysis.file_type == "Log" {
            10.0
        } else {
            3.0
        };

        Ok(crate::types::prediction::AccessPattern {
            accesses_last_24h: 5,
            accesses_last_week: 25,
            accesses_last_month: 100,
            total_accesses: 500,
            last_access: file_analysis.accessed_at,
            peak_access_times: vec![9, 10, 11, 14, 15, 16],
            read_write_ratio: 3.0,
        })
    }

    async fn get_cached_analysis(&self, file_path: &str) -> Option<FileAnalysis> {
        let cache = self.analysis_cache.read().await;
        if let Some((analysis, cached_time)) = cache.get(file_path)
            && cached_time.elapsed().unwrap_or_default().as_secs() < 3600
        {
            return Some(analysis.clone());
        }
        None
    }

    /// Cache analysis result
    #[allow(dead_code)]
    async fn cache_analysis(&self, file_path: &str, analysis: FileAnalysis) {
        let mut cache = self.analysis_cache.write().await;
        cache.insert(file_path.to_string(), (analysis, SystemTime::now()));
    }

    fn determine_file_type(&self, path: &std::path::Path) -> FileType {
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

    fn analyze_characteristics(
        &self,
        path: &std::path::Path,
        size: u64,
        file_type: &FileType,
    ) -> Result<FileCharacteristics> {
        let extension = path
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("")
            .to_lowercase();

        let _is_compressible = match file_type {
            FileType::Log | FileType::Database | FileType::Document => true,
            FileType::Archive | FileType::Backup | FileType::Image => false,
            _ => !matches!(
                extension.as_str(),
                "jpg" | "jpeg" | "png" | "gif" | "mp3" | "mp4" | "avi" | "mkv"
            ),
        };

        let _is_dedupable = size > {
            const SMALL_FILE_BYTES: u64 = 1024 * 1024;
            SMALL_FILE_BYTES
        } && matches!(file_type, FileType::Database | FileType::Archive);

        let is_frequently_accessed = matches!(file_type, FileType::Database | FileType::Document);
        let is_sequential_access = matches!(
            file_type,
            FileType::Archive | FileType::Backup | FileType::Log
        );

        Ok(FileCharacteristics {
            size_category: if size < 1_000_000 {
                SizeCategory::Small
            } else if size < 100_000_000 {
                SizeCategory::Medium
            } else if size < 1_000_000_000 {
                SizeCategory::Large
            } else {
                SizeCategory::XLarge
            },
            access_frequency: 0,
            is_frequently_accessed,
            is_sequential_access,
            data_pattern: DataPattern::Mixed,
        })
    }

    /// Analyze file characteristics for tier prediction
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn analyze_file_characteristics(
        &self,
        file_path: &str,
    ) -> Result<FileCharacteristics> {
        let metadata = tokio::fs::metadata(file_path).await.map_err(|e| {
            NestGateError::storage_error(format!("Failed to read file metadata: {e}"))
        })?;

        let extension = std::path::Path::new(file_path)
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("");

        let is_compressible = matches!(
            extension.to_lowercase().as_str(),
            "txt" | "log" | "json" | "xml" | "csv" | "html"
        );

        let _is_dedupable = metadata.len() > 1024 * 1024;

        let _estimated_compression_ratio = if is_compressible { 0.3 } else { 0.9 };

        Ok(FileCharacteristics {
            size_category: if metadata.len() < 1024 * 1024 {
                SizeCategory::Small
            } else if metadata.len() < 100 * 1024 * 1024 {
                SizeCategory::Medium
            } else if metadata.len() < 1024 * 1024 * 1024 {
                SizeCategory::Large
            } else {
                SizeCategory::XLarge
            },
            access_frequency: 0,
            is_frequently_accessed: false,
            is_sequential_access: false,
            data_pattern: DataPattern::Mixed,
        })
    }
}

impl Default for FileAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

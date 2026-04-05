// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

/// Filtering and pattern matching configuration - extracted from monolithic config
/// Handles file patterns, type filters, size filters, time filters, and custom filters
use serde::{Deserialize, Serialize};
use std::time::Duration;
/// Filter and pattern settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterSettings {
    /// Enable filtering
    pub enabled: bool,
    /// Include patterns
    pub include_patterns: Vec<FilePattern>,
    /// Exclude patterns
    pub exclude_patterns: Vec<FilePattern>,
    /// File type filters
    pub file_type_filters: Vec<FileTypeFilter>,
    /// Size-based filters
    pub size_filters: Vec<SizeFilter>,
    /// Time-based filters
    pub time_filters: Vec<TimeFilter>,
    /// Custom filters
    pub custom_filters: Vec<CustomFilter>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilePattern {
    /// Pattern string (glob format)
    pub pattern: String,
    /// Case sensitive matching
    pub case_sensitive: bool,
    /// Pattern enabled
    pub enabled: bool,
    /// Pattern priority
    pub priority: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileTypeFilter {
    /// File extensions to match
    pub extensions: Vec<String>,
    /// Filter enabled
    pub enabled: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SizeFilter {
    /// Minimum file size (bytes)
    pub min_size: Option<u64>,
    /// Maximum file size (bytes)
    pub max_size: Option<u64>,
    /// Filter enabled
    pub enabled: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeFilter {
    /// Minimum age (duration since last modification)
    pub min_age: Option<Duration>,
    /// Maximum age (duration since last modification)
    pub max_age: Option<Duration>,
    /// Filter enabled
    pub enabled: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomFilter {
    /// Filter name
    pub name: String,
    /// Filter expression (custom format)
    pub expression: String,
    /// Filter enabled
    pub enabled: bool,
    /// Filter priority
    pub priority: u32,
}
impl Default for FilterSettings {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            enabled: true,
            include_patterns: vec![FilePattern {
                pattern: "*".to_string(),
                case_sensitive: false,
                enabled: true,
                priority: 100,
            }],
            exclude_patterns: vec![
                FilePattern {
                    pattern: "*.tmp".to_string(),
                    case_sensitive: false,
                    enabled: true,
                    priority: 10,
                },
                FilePattern {
                    pattern: "*.swp".to_string(),
                    case_sensitive: false,
                    enabled: true,
                    priority: 10,
                },
                FilePattern {
                    pattern: ".git/*".to_string(),
                    case_sensitive: false,
                    enabled: true,
                    priority: 5,
                },
            ],
            file_type_filters: Vec::new(),
            size_filters: Vec::new(),
            time_filters: Vec::new(),
            custom_filters: Vec::new(),
        }
    }
}

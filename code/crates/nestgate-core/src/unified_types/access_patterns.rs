/// Unified Access Patterns Module
/// Consolidates duplicate `AccessPatterns` structs from automation modules
/// **PROBLEM SOLVED**: Eliminates `AccessPatterns` duplication between analysis.rs and types/mod.rs
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::SystemTime;

/// Parameters for creating access patterns from simple data
#[derive(Debug)]
pub struct SimplePatternParams {
    pub daily_access_count: u32,
    pub last_access: Option<SystemTime>,
    pub access_types: Vec<String>,
    pub average_file_size: u64,
    pub read_write_ratio: f64,
    pub sequential_access_ratio: f64,
    pub peak_access_hours: Vec<u8>,
}

/// **THE** unified access patterns structure for all storage analysis
/// Combines fields from both automation/analysis.rs and automation/types/mod.rs
/// to provide comprehensive access pattern tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedAccessPatterns {
    // Core frequency metrics
    pub read_frequency: u64,
    pub write_frequency: u64,
    pub daily_access_count: u64, // Note: using u64 for consistency, was u32 in types/mod.rs
    // Temporal tracking
    pub last_access: Option<SystemTime>,
    pub temporal_patterns: Vec<AccessTimePattern>,
    pub peak_access_hours: Vec<u8>,

    // Access method tracking
    pub access_methods: Vec<String>,
    pub access_types: Vec<String>,

    // User and performance metrics
    pub user_access_count: HashMap<String, u64>,
    pub read_write_ratio: f64,
    pub sequential_access_ratio: f64,
    pub average_file_size: u64,
}

/// Temporal access pattern tracking
/// Moved from analysis.rs to be part of the unified system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessTimePattern {
    pub hour: u8,
    pub day_of_week: u8,
    pub access_count: u64,
}
impl Default for UnifiedAccessPatterns {
    fn default() -> Self {
        Self {
            read_frequency: 0,
            write_frequency: 0,
            daily_access_count: 0,
            last_access: None,
            temporal_patterns: Vec::new(),
            peak_access_hours: Vec::new(),
            access_methods: Vec::new(),
            access_types: Vec::new(),
            user_access_count: HashMap::new(),
            read_write_ratio: 1.0,
            sequential_access_ratio: 0.5,
            average_file_size: 0,
        }
    }
}

impl UnifiedAccessPatterns {
    /// Create a new access patterns instance with basic metrics
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[allow(clippy::too_many_arguments)]
    #[must_use]
    pub fn from_analysis_patterns(
        read_frequency: u64,
        write_frequency: u64,
        last_access: Option<SystemTime>,
        access_methods: Vec<String>,
        user_access_count: HashMap<String, u64>,
        temporal_patterns: Vec<AccessTimePattern>,
        daily_access_count: u64,
        read_write_ratio: f64,
        access_types: Vec<String>,
        average_file_size: u64,
        sequential_access_ratio: f64,
        peak_access_hours: Vec<u8>,
    ) -> Self {
        Self {
            read_frequency,
            write_frequency,
            daily_access_count,
            last_access,
            temporal_patterns,
            peak_access_hours,
            access_methods,
            access_types,
            user_access_count,
            read_write_ratio,
            sequential_access_ratio,
            average_file_size,
        }
    }

    /// Create from legacy types/mod.rs `AccessPatterns` (partial conversion)
    #[must_use]
    pub fn from_simple_patterns(params: SimplePatternParams) -> Self {
        Self {
            read_frequency: 0,
            write_frequency: 0,
            daily_access_count: u64::from(params.daily_access_count),
            last_access: params.last_access,
            temporal_patterns: Vec::new(),
            peak_access_hours: params.peak_access_hours,
            access_methods: Vec::new(),
            access_types: params.access_types,
            user_access_count: HashMap::new(),
            read_write_ratio: params.read_write_ratio,
            sequential_access_ratio: params.sequential_access_ratio,
            average_file_size: params.average_file_size,
        }
    }

    /// Calculate total access frequency
    #[must_use]
    pub fn total_frequency(&self) -> u64 {
        self.read_frequency + self.write_frequency
    }

    /// Check if this represents an active access pattern
    #[must_use]
    pub fn is_active(&self) -> bool {
        self.total_frequency() > 0 || self.daily_access_count > 0
    }

    /// Get the dominant access type (read vs write)
    #[must_use]
    pub fn dominant_access_type(&self) -> &'static str {
        if self.read_frequency > self.write_frequency {
            "read-heavy"
        } else if self.write_frequency > self.read_frequency {
            "write-heavy"
        } else {
            "balanced"
        }
    }
}

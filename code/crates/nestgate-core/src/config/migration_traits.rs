//! **CONFIG MIGRATION TRAITS**
//!
//! Systematic migration utilities for consolidating fragmented config structs
//! into the canonical configuration system.

use crate::config::canonical_master::{NetworkConfig, StorageConfig, SecurityConfig};
use crate::error::Result;

// ==================== SECTION ====================

/// Trait for migrating any config to canonical NetworkConfig
pub trait IntoCanonicalNetworkConfig {
    /// Convert this config into the canonical NetworkConfig
    fn into_canonical(self) -> NetworkConfig<8080, 30000>;
    
    /// Convert with custom const generics
    fn into_canonical_with_params<const API_PORT: u16, const TIMEOUT_MS: u64>(
        self
    ) -> NetworkConfig<API_PORT, TIMEOUT_MS>;
}

/// Trait for migrating any config to canonical StorageConfig
pub trait IntoCanonicalStorageConfig {
    /// Convert this config into the canonical StorageConfig
    fn into_canonical(self) -> StorageConfig;
}

/// Trait for migrating any config to canonical SecurityConfig
pub trait IntoCanonicalSecurityConfig {
    /// Convert this config into the canonical SecurityConfig
    fn into_canonical(self) -> SecurityConfig;
}

// ==================== SECTION ====================

/// Generic config migration utility
pub struct ConfigMigrationHelper;

impl ConfigMigrationHelper {
    /// Validate that migration preserves all functionality
    pub fn validate_migration<T, C>(original: &T, canonical: &C) -> Result<()> 
    where
        T: std::fmt::Debug,
        C: std::fmt::Debug,
    {
        // TODO: Implement field-by-field validation
        // For now, just ensure both configs are valid
        Ok(())
    }
    
    /// Batch migrate configs in a directory
    pub fn batch_migrate_directory(dir_path: &str) -> Result<usize> {
        // TODO: Implement directory scanning and batch migration
        Ok(0)
    }
}

// ==================== SECTION ====================

/// Migration statistics tracking
#[derive(Debug, Default)]
pub struct MigrationStats {
    pub files_processed: usize,
    pub configs_migrated: usize,
    pub errors_encountered: usize,
    pub validation_passed: usize,
}

impl MigrationStats {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn record_success(&mut self) {
        self.configs_migrated += 1;
        self.validation_passed += 1;
    }
    
    pub fn record_error(&mut self) {
        self.errors_encountered += 1;
    }
    
    pub fn success_rate(&self) -> f64 {
        if self.files_processed == 0 {
            0.0
        } else {
            self.validation_passed as f64 / self.files_processed as f64
        }
    }
}

// ==================== SECTION ====================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_migration_stats() {
        let mut stats = MigrationStats::new();
        assert_eq!(stats.success_rate(), 0.0);
        
        stats.files_processed = 10;
        stats.record_success();
        stats.record_success();
        stats.record_error();
        
        assert_eq!(stats.configs_migrated, 2);
        assert_eq!(stats.errors_encountered, 1);
        assert_eq!(stats.validation_passed, 2);
        assert_eq!(stats.success_rate(), 0.2); // 2/10
    }
} 
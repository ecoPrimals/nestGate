// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

// **EVOLUTION METADATA**
//! Tracks evolution and modernization metadata for types and systems.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// **EVOLUTION METADATA**
///
/// Tracks evolution and modernization metadata for types
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Evolutionmetadata
pub struct EvolutionMetadata {
    /// Version of the evolution
    pub version: String,
    /// Migration path information
    pub migration_path: Option<MigrationPath>,
    /// Compatibility notes
    pub compatibility_notes: Vec<String>,
    /// Component tracking
    pub components: HashMap<String, String>,
}
/// Migration path information
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Migrationpath
pub struct MigrationPath {
    /// Source version
    pub from_version: String,
    /// Target version
    pub to_version: String,
    /// Migration steps
    pub steps: Vec<String>,
    /// Rollback capability
    pub can_rollback: bool,
}
/// Compatibility information
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Compatibilityinfo
pub struct CompatibilityInfo {
    /// Compatible versions
    pub compatible_versions: Vec<String>,
    /// Breaking changes
    pub breaking_changes: Vec<String>,
    /// Deprecation warnings
    pub deprecations: Vec<DeprecationInfo>,
}
/// Modernization metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Modernizationmetadata
pub struct ModernizationMetadata {
    /// Modernization status
    pub status: ModernizationStatus,
    /// Applied patterns
    pub applied_patterns: Vec<String>,
    /// Recommendations
    pub recommendations: Vec<String>,
}
/// Version information
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Versioninfo
pub struct VersionInfo {
    /// Current version
    pub current: String,
    /// Previous version
    pub previous: Option<String>,
    /// Next version
    pub next: Option<String>,
}
/// Deprecation information
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Deprecationinfo
pub struct DeprecationInfo {
    /// Deprecated item
    pub item: String,
    /// Deprecation version
    pub since_version: String,
    /// Removal version
    pub removal_version: Option<String>,
    /// Replacement suggestion
    pub replacement: Option<String>,
}
/// Modernization status
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Status values for Modernization
pub enum ModernizationStatus {
    /// Not started
    NotStarted,
    /// In progress
    InProgress,
    /// Completed
    Completed,
    /// Failed
    Failed,
    /// Rolled back
    RolledBack,
}
impl Default for EvolutionMetadata {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            version: "1.0.0".to_string(),
            migration_path: None,
            compatibility_notes: vec!["Canonical modernization compatible".to_string()],
            components: HashMap::new(),
        }
    }
}

impl EvolutionMetadata {
    /// Create production-optimized metadata
    #[must_use]
    pub fn production_optimized() -> Self {
        Self {
            version: "1.0.0-prod".to_string(),
            migration_path: Some(MigrationPath {
                from_version: "0.9.0".to_string(),
                to_version: "1.0.0".to_string(),
                steps: vec!["Apply canonical patterns".to_string()],
                can_rollback: true,
            }),
            compatibility_notes: vec!["Production-ready canonical modernization".to_string()],
            components: HashMap::new(),
        }
    }

    /// Create development-optimized metadata
    #[must_use]
    pub fn development_optimized() -> Self {
        Self {
            version: "1.0.0-dev".to_string(),
            migration_path: None,
            compatibility_notes: vec!["Development-friendly modernization".to_string()],
            components: HashMap::new(),
        }
    }

    /// Track component evolution
    ///
    /// # Errors
    ///
    /// Returns an error if component tracking fails (currently infallible).
    pub fn track_component_evolution(
        &mut self,
        component: &str,
        version: &str,
    ) -> nestgate_types::error::Result<()> {
        self.components
            .insert(component.to_string(), version.to_string());
        Ok(())
    }

    /// Get component count
    #[must_use]
    pub fn get_component_count(&self) -> u32 {
        u32::try_from(self.components.len()).unwrap_or(u32::MAX)
    }

    /// Validate metadata consistency
    ///
    /// # Errors
    ///
    /// Returns an error if version is empty or metadata is inconsistent.
    pub fn validate(&self) -> nestgate_types::error::Result<()> {
        if self.version.is_empty() {
            return Err(nestgate_types::error::NestGateError::validation_error(
                "Version cannot be empty",
            ));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn serde_roundtrip<T>(v: &T)
    where
        T: serde::Serialize + serde::de::DeserializeOwned,
    {
        let json = serde_json::to_string(v).expect("serialize");
        let _: T = serde_json::from_str(&json).expect("deserialize");
    }

    #[test]
    fn evolution_metadata_default_and_production() {
        let d = EvolutionMetadata::default();
        assert!(d.validate().is_ok());
        let mut p = EvolutionMetadata::production_optimized();
        p.track_component_evolution("a", "1").expect("track");
        assert_eq!(p.get_component_count(), 1);
        let dev = EvolutionMetadata::development_optimized();
        assert!(dev.validate().is_ok());
    }

    #[test]
    fn evolution_metadata_validate_empty_version_fails() {
        let mut m = EvolutionMetadata::default();
        m.version.clear();
        assert!(m.validate().is_err());
    }

    #[test]
    fn serde_roundtrip_structs() {
        serde_roundtrip(&EvolutionMetadata::default());
        serde_roundtrip(&MigrationPath {
            from_version: "0".to_string(),
            to_version: "1".to_string(),
            steps: vec!["s".to_string()],
            can_rollback: false,
        });
        serde_roundtrip(&CompatibilityInfo {
            compatible_versions: vec!["1".to_string()],
            breaking_changes: vec![],
            deprecations: vec![DeprecationInfo {
                item: "x".to_string(),
                since_version: "0".to_string(),
                removal_version: None,
                replacement: None,
            }],
        });
        serde_roundtrip(&ModernizationMetadata {
            status: ModernizationStatus::InProgress,
            applied_patterns: vec!["p".to_string()],
            recommendations: vec![],
        });
        serde_roundtrip(&VersionInfo {
            current: "1".to_string(),
            previous: None,
            next: None,
        });
    }

    #[test]
    fn modernization_status_variants() {
        for s in [
            ModernizationStatus::NotStarted,
            ModernizationStatus::InProgress,
            ModernizationStatus::Completed,
            ModernizationStatus::Failed,
            ModernizationStatus::RolledBack,
        ] {
            serde_roundtrip(&s);
        }
    }
}

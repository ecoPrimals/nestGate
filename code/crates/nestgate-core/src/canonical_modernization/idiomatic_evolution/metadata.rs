// **EVOLUTION METADATA**
//! Tracks evolution and modernization metadata for types and systems.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// **EVOLUTION METADATA**
///
/// Tracks evolution and modernization metadata for types
#[derive(Debug, Clone, Serialize, Deserialize)]
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
    pub const fn production_optimized() -> Self {
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
    pub fn track_component_evolution(
        &mut self,
        component: &str,
        version: &str,
    ) -> crate::Result<()> {
        self.components
            .insert(component.to_string(), version.to_string());
        Ok(())
    }

    /// Get component count
    #[must_use]
    pub const fn get_component_count(&self) -> u32 {
        self.components.len() as u32
    }

    /// Validate metadata consistency
    pub const fn validate(&self) -> crate::Result<()> {
        if self.version.is_empty() {
            return Err(crate::NestGateError::validation_error(
                "Version cannot be empty",
            ));
        }
        Ok(())
    }
}

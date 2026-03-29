// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

// **IDIOMATIC EVOLUTION PATTERNS**
//! Module definitions and exports.
// Canonical modernization patterns and evolution traits that provide smart defaults
//! and modern Rust idioms for the `NestGate` ecosystem.
//! Module definitions and exports.
// **MODULAR STRUCTURE**:
//! - `traits`: Core evolution traits (`SmartDefault`, `IdiomaticBuilder`, `SmartClone`)
//! - `patterns`: Common idiomatic Rust patterns and utilities
//! - `metadata`: Evolution and migration metadata tracking
//! - `implementations`: Standard trait implementations for common types
//! - `builders`: Advanced builder patterns for complex configurations
//! - `evolution`: Evolution tracking and compatibility systems

// Import all idiomatic evolution modules
pub mod builders;
pub mod evolution;
pub mod implementations;
pub mod metadata;
pub mod patterns;
pub mod traits;

// Re-export all core types and traits
pub use builders::{
    CanonicalBuilder, EvolutionAwareBuilder, IdiomaticConfigBuilder, ModernizationBuilder,
    SmartConfigBuilder,
};
pub use evolution::{
    CanonicalEvolutionSystem, CompatibilityChecker, EvolutionTracker, MigrationManager,
    ModernizationEngine,
};
pub use implementations::{
    BooleanEvolution, CollectionEvolution, ConfigEvolution, NumericEvolution, ServiceEvolution,
    StringEvolution,
};
pub use metadata::{
    CompatibilityInfo, DeprecationInfo, EvolutionMetadata, MigrationPath, ModernizationMetadata,
    VersionInfo,
};
pub use patterns::{
    apply_modernization_pattern, create_idiomatic_builder, safe_smart_default,
    smart_conversion_pattern, with_evolution_metadata,
};
pub use traits::{
    CanonicalEvolution, EvolutionCompatible, IdiomaticBuilder, ModernizationTrait, SmartClone,
    SmartDefault,
};

// ==================== MAIN EVOLUTION SYSTEM ====================

/// **THE** canonical idiomatic evolution system for `NestGate`
///
/// This system provides comprehensive evolution tracking, smart defaults,
/// and modernization patterns for the entire ecosystem.
#[derive(Debug, Clone, Default)]
/// Idiomaticevolutionsystem
pub struct IdiomaticEvolutionSystem {
    /// Evolution metadata tracking
    pub metadata: EvolutionMetadata,

    /// Migration management
    pub migration_manager: MigrationManager,

    /// Compatibility checking
    pub compatibility_checker: CompatibilityChecker,

    /// Modernization engine
    pub modernization_engine: ModernizationEngine,

    /// Canonical evolution system
    pub canonical_system: CanonicalEvolutionSystem,
}

impl IdiomaticEvolutionSystem {
    /// Create a new idiomatic evolution system
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a system optimized for production environments
    #[must_use]
    pub fn production_optimized() -> Self {
        Self {
            metadata: EvolutionMetadata::production_optimized(),
            migration_manager: MigrationManager::production_optimized(),
            compatibility_checker: CompatibilityChecker::production_optimized(),
            modernization_engine: ModernizationEngine::production_optimized(),
            canonical_system: CanonicalEvolutionSystem::production_optimized(),
        }
    }

    /// Create a system optimized for development environments
    #[must_use]
    pub fn development_optimized() -> Self {
        Self {
            metadata: EvolutionMetadata::development_optimized(),
            migration_manager: MigrationManager::development_optimized(),
            compatibility_checker: CompatibilityChecker::development_optimized(),
            modernization_engine: ModernizationEngine::development_optimized(),
            canonical_system: CanonicalEvolutionSystem::development_optimized(),
        }
    }

    /// Apply evolution patterns to a configuration
    ///
    /// # Errors
    ///
    /// Returns an error if modernization fails or compatibility validation fails
    pub fn apply_evolution<T>(&self, config: T) -> nestgate_types::error::Result<T>
    where
        T: EvolutionCompatible + ModernizationTrait,
    {
        // Apply modernization patterns
        let modernized = self.modernization_engine.apply_patterns(config)?;

        // Check compatibility
        self.compatibility_checker.validate(&modernized)?;

        Ok(modernized)
    }

    /// Track evolution progress
    ///
    /// # Errors
    ///
    /// Returns an error if progress tracking fails or version validation fails
    pub fn track_evolution(
        &mut self,
        component: &str,
        version: &str,
    ) -> nestgate_types::error::Result<()> {
        self.metadata
            .track_component_evolution(component, version)?;
        self.migration_manager
            .update_migration_status(component, version)?;
        Ok(())
    }

    /// Get evolution statistics
    #[must_use]
    pub fn get_evolution_stats(&self) -> EvolutionStats {
        EvolutionStats {
            total_components: self.metadata.get_component_count(),
            modernized_components: self.migration_manager.get_completed_count(),
            compatibility_score: self.compatibility_checker.get_overall_score(),
            evolution_progress: self.calculate_evolution_progress(),
        }
    }

    /// Calculate overall evolution progress
    fn calculate_evolution_progress(&self) -> f64 {
        let total = f64::from(self.metadata.get_component_count());
        let completed = f64::from(self.migration_manager.get_completed_count());

        if total > 0.0 {
            (completed / total) * 100.0
        } else {
            0.0
        }
    }

    /// Validate the evolution system
    pub fn validate(&self) -> nestgate_types::error::Result<()> {
        // Validate metadata consistency
        self.metadata.validate()?;

        // Validate migration manager state
        self.migration_manager.validate()?;

        // Validate compatibility checker
        self.compatibility_checker.validate(&())?;

        // Validate modernization engine
        self.modernization_engine.validate()?;

        // Validate canonical system
        self.canonical_system.validate()?;

        Ok(())
    }
}

/// Evolution statistics for tracking canonical modernization progress
///
/// This struct captures comprehensive metrics about the modernization process,
/// including component counts, compatibility scores, and overall progress.
///
/// # Examples
///
/// ```rust,ignore
/// use nestgate_core::canonical_modernization::idiomatic_evolution::EvolutionStats;
///
/// let stats = EvolutionStats {
///     total_components: 100,
///     modernized_components: 75,
///     compatibility_score: 0.95,
///     evolution_progress: 75.0,
/// };
/// assert_eq!(stats.evolution_progress, 75.0);
/// ```
#[derive(Debug, Clone)]
/// Evolutionstats
pub struct EvolutionStats {
    /// Total number of components being evolved
    pub total_components: u32,

    /// Number of modernized components
    pub modernized_components: u32,

    /// Overall compatibility score (0.0 to 1.0)
    pub compatibility_score: f64,

    /// Evolution progress percentage
    pub evolution_progress: f64,
}
// ==================== CONVENIENCE FUNCTIONS ====================
// Note: Core convenience functions are imported from the patterns module

// ==================== BACKWARD COMPATIBILITY ====================

// Note: Trait aliases removed - use traits directly
// - SmartDefault trait (use directly)
// - IdiomaticBuilder<T> trait (use directly)

/// Backward compatibility alias for evolution metadata
///
/// This type alias provides compatibility with legacy code that references
/// `LegacyEvolutionMetadata`. New code should use `EvolutionMetadata` directly.
///
/// # Deprecated
///
/// This alias exists for backward compatibility only. Use `EvolutionMetadata` directly in new code.
pub type LegacyEvolutionMetadata = EvolutionMetadata;

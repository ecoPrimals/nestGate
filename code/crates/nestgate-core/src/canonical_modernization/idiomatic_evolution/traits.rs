// **EVOLUTION TRAITS**
//! Idiomatic trait evolution patterns for the `NestGate` canonical modernization.
// Core evolution traits that provide smart defaults and modern Rust idioms.

/// **SMART DEFAULT TRAIT**
///
/// Provides intelligent default implementations that go beyond `std::default::Default`
/// by considering context and providing more meaningful defaults.
pub trait SmartDefault {
    /// Create a smart default instance with context awareness
    fn smart_default() -> Self;

    /// Create a smart default with additional context
    #[must_use]
    fn smart_default_with_context(context: &str) -> Self
    where
        Self: Sized,
    {
        // Default implementation ignores context and falls back to smart_default
        let _ = context;
        Self::smart_default()
    }

    /// Check if the type can derive default automatically
    #[must_use]
    fn can_derive_default() -> bool {
        false // Default conservative implementation
    }
}
/// **IDIOMATIC BUILDER TRAIT**
///
/// Provides idiomatic builder patterns for complex types
pub trait IdiomaticBuilder<T> {
    /// Create a new builder instance
    fn builder() -> Self;

    /// Build the final instance
    fn build(self) -> T;
}
/// **SMART CLONE TRAIT**
///
/// Provides context-aware cloning that can optimize based on usage patterns
pub trait SmartClone {
    /// Clone with optimization hints
    #[must_use]
    fn smart_clone(&self) -> Self;

    /// Clone with specific context for optimization
    #[must_use]
    fn smart_clone_with_context(&self, context: &str) -> Self
    where
        Self: Sized,
    {
        // Default implementation ignores context
        let _ = context;
        self.smart_clone()
    }
}
/// **EVOLUTION COMPATIBLE TRAIT**
///
/// Marks types as compatible with the evolution system
pub trait EvolutionCompatible {
    /// Check if the type is compatible with current evolution standards
    ///
    /// # Errors
    ///
    /// Returns an error if compatibility check fails or version mismatch occurs
    fn check_compatibility(&self) -> crate::Result<bool>;

    /// Get evolution version information
    fn get_evolution_version(&self) -> String {
        "1.0.0".to_string()
    }

    /// Apply evolution compatibility fixes
    ///
    /// # Errors
    ///
    /// Returns an error if compatibility fixes cannot be applied
    fn apply_compatibility_fixes(&mut self) -> crate::Result<()> {
        Ok(())
    }
}
/// **MODERNIZATION TRAIT**
///
/// Provides modernization capabilities for legacy types
pub trait ModernizationTrait {
    /// Apply modernization patterns
    ///
    /// # Errors
    ///
    /// Returns an error if modernization patterns fail to apply
    fn apply_modernization(self) -> crate::Result<Self>
    where
        Self: Sized;

    /// Check if modernization is needed
    fn needs_modernization(&self) -> bool {
        false
    }

    /// Get modernization recommendations
    fn get_modernization_recommendations(&self) -> Vec<String> {
        vec![]
    }
}
/// **CANONICAL EVOLUTION TRAIT**
///
/// Comprehensive evolution trait combining all evolution capabilities
pub trait CanonicalEvolution: SmartDefault + EvolutionCompatible + ModernizationTrait {
    /// Apply complete canonical evolution
    fn evolve_canonically(self) -> crate::Result<Self>
    where
        Self: Sized,
    {
        // Apply modernization first
        let modernized = self.apply_modernization()?;

        // Verify compatibility
        modernized.check_compatibility()?;

        Ok(modernized)
    }

    /// Get evolution score (0.0 to 1.0)
    fn get_evolution_score(&self) -> f64 {
        1.0 // Default perfect score
    }
}

// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

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
    fn check_compatibility(&self) -> nestgate_types::error::Result<bool>;

    /// Get evolution version information
    fn get_evolution_version(&self) -> String {
        "1.0.0".to_string()
    }

    /// Apply evolution compatibility fixes
    ///
    /// # Errors
    ///
    /// Returns an error if compatibility fixes cannot be applied
    fn apply_compatibility_fixes(&mut self) -> nestgate_types::error::Result<()> {
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
    fn apply_modernization(self) -> nestgate_types::error::Result<Self>
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
    fn evolve_canonically(self) -> nestgate_types::error::Result<Self>
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

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone)]
    struct TestEvo(u32);

    impl SmartDefault for TestEvo {
        fn smart_default() -> Self {
            Self(0)
        }
    }

    impl EvolutionCompatible for TestEvo {
        fn check_compatibility(&self) -> nestgate_types::error::Result<bool> {
            Ok(true)
        }
    }

    impl ModernizationTrait for TestEvo {
        fn apply_modernization(self) -> nestgate_types::error::Result<Self> {
            Ok(self)
        }
    }

    impl CanonicalEvolution for TestEvo {}

    #[test]
    fn smart_default_extension_methods() {
        assert_eq!(String::smart_default_with_context("ctx"), String::new());
        assert!(!String::can_derive_default());
        let s = "hi".to_string();
        assert_eq!(s.smart_clone_with_context("c"), "hi");
    }

    #[test]
    fn canonical_evolution_evolve_canonically() {
        let v = TestEvo(7);
        let out = v.evolve_canonically().expect("evolve");
        assert_eq!(out.0, 7);
        assert!((out.get_evolution_score() - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn evolution_compatible_defaults() {
        let mut t = TestEvo(1);
        t.apply_compatibility_fixes().expect("fixes");
        assert_eq!(t.get_evolution_version(), "1.0.0");
    }

    #[test]
    fn modernization_trait_defaults() {
        let t = TestEvo(2);
        assert!(!t.needs_modernization());
        assert!(t.get_modernization_recommendations().is_empty());
    }
}

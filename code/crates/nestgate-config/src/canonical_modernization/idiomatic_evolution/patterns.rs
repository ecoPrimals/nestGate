// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

// **IDIOMATIC PATTERNS**
//! Idiomatic code patterns for the `NestGate` canonical modernization.
// Common idiomatic Rust patterns for the canonical modernization

use super::metadata::EvolutionMetadata;
use super::traits::{
    CanonicalEvolution, EvolutionCompatible, IdiomaticBuilder, ModernizationTrait, SmartDefault,
};

/// Create a smart default with error handling
///
/// # Errors
///
/// Returns an error if smart default creation fails (currently infallible).
pub fn safe_smart_default<T: SmartDefault>() -> Result<T, &'static str> {
    Ok(T::smart_default())
}
/// Apply evolution metadata to a type
pub const fn with_evolution_metadata<T>(
    value: T,
    metadata: EvolutionMetadata,
) -> (T, EvolutionMetadata) {
    (value, metadata)
}
/// Apply modernization pattern to any compatible type
///
/// # Errors
///
/// Returns an error if modernization fails.
pub fn apply_modernization_pattern<T>(value: T) -> nestgate_types::error::Result<T>
where
    T: ModernizationTrait,
{
    value.apply_modernization()
}
/// Create an idiomatic builder for any type
#[must_use]
pub fn create_idiomatic_builder<T, B>() -> B
where
    B: IdiomaticBuilder<T>,
{
    B::builder()
}
/// Smart conversion pattern with evolution tracking
///
/// # Errors
///
/// Returns an error if compatibility check fails.
pub fn smart_conversion_pattern<T, U>(value: T) -> nestgate_types::error::Result<U>
where
    T: Into<U> + EvolutionCompatible,
    U: SmartDefault,
{
    // Check compatibility before conversion
    value.check_compatibility()?;

    // Perform conversion
    Ok(value.into())
}
/// Pattern for safe evolution with rollback capability
///
/// # Errors
///
/// Returns an error if both evolution and rollback fail (currently uses fallback).
pub fn safe_evolution_pattern<T>(value: T) -> nestgate_types::error::Result<T>
where
    T: Clone + CanonicalEvolution,
{
    let backup = value.clone();

    value.evolve_canonically().or(Ok(backup))
}
/// Batch evolution pattern for collections
///
/// # Errors
///
/// Returns an error if any value fails to evolve.
pub fn batch_evolution_pattern<T>(values: Vec<T>) -> nestgate_types::error::Result<Vec<T>>
where
    T: CanonicalEvolution,
{
    let mut evolved = Vec::new();

    for value in values {
        evolved.push(value.evolve_canonically()?);
    }

    Ok(evolved)
}
/// Evolution validation pattern
pub fn validate_evolution<T>(value: &T) -> nestgate_types::error::Result<()>
where
    T: EvolutionCompatible + ModernizationTrait,
{
    // Check compatibility
    value.check_compatibility()?;

    // Check if modernization is needed
    if value.needs_modernization() {
        return Err(nestgate_types::error::NestGateError::validation_error(
            "Type requires modernization before use",
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::super::metadata::EvolutionMetadata;
    use super::super::traits::{EvolutionCompatible, IdiomaticBuilder, ModernizationTrait};
    use super::*;

    #[derive(Debug, Clone)]
    struct TestEvo(bool);

    impl EvolutionCompatible for TestEvo {
        fn check_compatibility(&self) -> nestgate_types::error::Result<bool> {
            Ok(!self.0)
        }
    }

    impl ModernizationTrait for TestEvo {
        fn apply_modernization(self) -> nestgate_types::error::Result<Self> {
            Ok(self)
        }

        fn needs_modernization(&self) -> bool {
            self.0
        }
    }

    #[derive(Debug)]
    struct DummyBuilder;

    impl IdiomaticBuilder<String> for DummyBuilder {
        fn builder() -> Self {
            Self
        }

        fn build(self) -> String {
            "built".to_string()
        }
    }

    #[derive(Debug)]
    struct EvoU32(u32);

    impl EvolutionCompatible for EvoU32 {
        fn check_compatibility(&self) -> nestgate_types::error::Result<bool> {
            Ok(true)
        }
    }

    impl From<EvoU32> for u64 {
        fn from(v: EvoU32) -> Self {
            Self::from(v.0)
        }
    }

    #[test]
    fn safe_smart_default_u16() {
        assert_eq!(safe_smart_default::<u16>().expect("ok"), 8080);
    }

    #[test]
    fn with_evolution_metadata_tuple() {
        let (n, _m): (u32, EvolutionMetadata) =
            with_evolution_metadata(1, EvolutionMetadata::default());
        assert_eq!(n, 1);
    }

    #[test]
    fn apply_modernization_pattern_ok() {
        let t = TestEvo(false);
        assert_eq!(apply_modernization_pattern(t).expect("ok").0, false);
    }

    #[test]
    fn create_idiomatic_builder_builds() {
        let b: DummyBuilder = create_idiomatic_builder::<String, DummyBuilder>();
        assert_eq!(b.build(), "built");
    }

    #[test]
    fn smart_conversion_pattern_u64() {
        let out: u64 = smart_conversion_pattern(EvoU32(5)).expect("conv");
        assert_eq!(out, 5u64);
    }

    #[test]
    fn validate_evolution_ok() {
        validate_evolution(&TestEvo(false)).expect("ok");
    }

    #[test]
    fn validate_evolution_needs_modernization_err() {
        assert!(validate_evolution(&TestEvo(true)).is_err());
    }
}

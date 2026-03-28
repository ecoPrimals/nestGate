// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

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
pub fn with_evolution_metadata<T>(value: T, metadata: EvolutionMetadata) -> (T, EvolutionMetadata) {
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

    match value.evolve_canonically() {
        Ok(evolved) => Ok(evolved),
        Err(_) => {
            // Rollback to original on failure
            Ok(backup)
        }
    }
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

// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

// **TRAIT IMPLEMENTATIONS**
//! Implementation patterns for idiomatic evolution in the canonical modernization. Implementations functionality and utilities.
// Standard trait implementations for common types

use super::traits::{SmartClone, SmartDefault};

// ==================== SMART DEFAULT IMPLEMENTATIONS ====================

impl SmartDefault for String {
    /// Smart Default
    fn smart_default() -> Self {
        Self::new()
    }
}

impl SmartDefault for u16 {
    /// Smart Default
    fn smart_default() -> Self {
        8080 // Smart default for port numbers
    }
}

impl SmartDefault for u32 {
    /// Smart Default
    fn smart_default() -> Self {
        0
    }
}

impl SmartDefault for u64 {
    /// Smart Default
    fn smart_default() -> Self {
        0
    }
}

impl SmartDefault for usize {
    /// Smart Default
    fn smart_default() -> Self {
        0
    }
}

impl SmartDefault for bool {
    /// Smart Default
    fn smart_default() -> Self {
        false
    }
}

impl SmartDefault for i32 {
    /// Smart Default
    fn smart_default() -> Self {
        0
    }
}

impl SmartDefault for i64 {
    /// Smart Default
    fn smart_default() -> Self {
        0
    }
}

impl SmartDefault for f64 {
    /// Smart Default
    fn smart_default() -> Self {
        0.0
    }
}

// ==================== EVOLUTION TYPE IMPLEMENTATIONS ====================

/// String evolution utilities
pub struct StringEvolution;
impl StringEvolution {
    /// Modernize a string by trimming whitespace and converting to owned String
    #[must_use]
    pub fn modernize_string(s: &str) -> String {
        s.trim().to_string()
    }
}

/// Numeric evolution utilities
pub struct NumericEvolution;
impl NumericEvolution {
    /// Modernize a port number, defaulting to 8080 if zero
    #[must_use]
    pub const fn modernize_port(port: u16) -> u16 {
        if port == 0 { 8080 } else { port }
    }
}

/// Boolean evolution utilities
pub struct BooleanEvolution;
impl BooleanEvolution {
    /// Modernize a boolean flag (no transformation needed)
    #[must_use]
    pub const fn modernize_flag(flag: bool) -> bool {
        flag // No change needed for booleans
    }
}

/// Collection evolution utilities
pub struct CollectionEvolution;
impl CollectionEvolution {
    /// Modernize a vector collection (no transformation needed)
    #[must_use]
    pub const fn modernize_vec<T>(vec: Vec<T>) -> Vec<T> {
        vec // No change needed for basic collections
    }
}

/// Configuration evolution utilities
pub struct ConfigEvolution;
impl ConfigEvolution {
    /// Apply modern configuration patterns to a config object.
    ///
    /// **Framework hook:** returns `config` unchanged. There are no config-type-specific
    /// transforms in this crate yet; this preserves a stable call site for pipelines and tests.
    pub const fn apply_config_patterns<T>(config: T) -> T {
        config
    }
}

/// Service evolution utilities
pub struct ServiceEvolution;
impl ServiceEvolution {
    /// Modernize service configuration with modern patterns.
    ///
    /// **Framework hook:** returns `config` unchanged. Service-specific normalization (timeouts,
    /// feature flags, etc.) is not implemented here yet.
    pub const fn modernize_service_config<T>(config: T) -> T {
        config
    }
}

// ==================== SMART CLONE IMPLEMENTATIONS ====================

impl SmartClone for String {
    /// Smart Clone
    fn smart_clone(&self) -> Self {
        self.clone()
    }
}

impl SmartClone for u32 {
    /// Smart Clone
    fn smart_clone(&self) -> Self {
        *self
    }
}

impl SmartClone for bool {
    /// Smart Clone
    fn smart_clone(&self) -> Self {
        *self
    }
}

#[cfg(test)]
mod tests {
    use super::super::traits::SmartClone;
    use super::super::traits::SmartDefault;
    use super::*;

    #[test]
    fn smart_default_primitives() {
        assert_eq!(String::smart_default(), String::new());
        assert_eq!(u16::smart_default(), 8080);
        assert_eq!(u32::smart_default(), 0);
        assert_eq!(u64::smart_default(), 0);
        assert_eq!(usize::smart_default(), 0);
        assert!(!bool::smart_default());
        assert_eq!(i32::smart_default(), 0);
        assert_eq!(i64::smart_default(), 0);
        assert_eq!(f64::smart_default(), 0.0);
    }

    #[test]
    fn smart_clone_primitives() {
        assert_eq!(String::smart_clone(&"a".to_string()), "a");
        assert_eq!(u32::smart_clone(&3), 3);
        assert_eq!(bool::smart_clone(&true), true);
    }

    #[test]
    fn evolution_helpers_smoke() {
        assert_eq!(StringEvolution::modernize_string("  x  "), "x");
        assert_eq!(NumericEvolution::modernize_port(0), 8080);
        assert_eq!(NumericEvolution::modernize_port(9000), 9000);
        assert!(BooleanEvolution::modernize_flag(true));
        let v = vec![1, 2];
        assert_eq!(CollectionEvolution::modernize_vec(v.clone()), v);
        assert_eq!(ConfigEvolution::apply_config_patterns(1u8), 1u8);
        assert_eq!(ServiceEvolution::modernize_service_config("s"), "s");
    }
}

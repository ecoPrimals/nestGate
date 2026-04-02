// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

// Simplified, unified middleware configuration using canonical patterns

use nestgate_core::config::canonical_primary::NestGateCanonicalConfig;

/// Middleware configuration type alias
pub type MiddlewareConfig = NestGateCanonicalConfig;
/// Create default middleware configuration
#[must_use]
pub fn create_default_config() -> MiddlewareConfig {
    NestGateCanonicalConfig::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_default_config_smoke() {
        let cfg = create_default_config();
        assert!(!cfg.system.instance_name.is_empty());
    }

    #[test]
    fn two_defaults_are_consistent() {
        let a = create_default_config();
        let b = create_default_config();
        assert_eq!(a.system.instance_name, b.system.instance_name);
    }

    #[test]
    fn factory_matches_type_alias_default() {
        let a = create_default_config();
        let b = MiddlewareConfig::default();
        assert_eq!(a.system.instance_name, b.system.instance_name);
    }

    #[test]
    fn middleware_config_type_alias_is_default_constructible() {
        let _: MiddlewareConfig = MiddlewareConfig::default();
    }

    #[test]
    fn default_config_clone_preserves_instance_name() {
        let a = create_default_config();
        let b = a.clone();
        assert_eq!(a.system.instance_name, b.system.instance_name);
    }
}

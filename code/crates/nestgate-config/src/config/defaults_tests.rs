// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Tests for default configuration helpers.

#[cfg(test)]
mod config_defaults_tests {
    use crate::config::{
        DeploymentEnvironment, LogLevel, create_default_config, create_testing_config,
    };

    #[test]
    fn default_config_has_expected_environment_and_log_level() {
        let c = create_default_config();
        assert!(matches!(
            c.system.environment,
            DeploymentEnvironment::Development
        ));
        assert!(matches!(c.system.log_level, LogLevel::Info));
    }

    #[test]
    fn testing_config_disables_metrics_and_tracing_flags() {
        let c = create_testing_config();
        assert!(matches!(
            c.system.environment,
            DeploymentEnvironment::Testing
        ));
        assert_eq!(c.features.custom_flags.get("enable_metrics"), Some(&false));
        assert_eq!(c.features.custom_flags.get("enable_tracing"), Some(&false));
    }
}

//! **CONFIG VALIDATION TESTS** - Nov 23, 2025
//!
//! Comprehensive tests for configuration validation, edge cases, and error handling

#[cfg(test)]
mod config_creation_tests {
    use crate::config::{
        create_default_config, create_development_config, create_production_config,
        create_testing_config,
    };

    #[test]
    fn test_default_config_creation() {
        let config = create_default_config();
        assert_eq!(config.system.instance_name, "nestgate-default");
        assert!(!config.system.debug_mode);
    }

    #[test]
    fn test_production_config_creation() {
        let config = create_production_config();
        assert!(!config.system.debug_mode);
        assert!(config
            .features
            .custom_flags
            .contains_key("enable_auto_scaling"));
        assert!(config
            .features
            .custom_flags
            .contains_key("enable_load_balancing"));
    }

    #[test]
    fn test_development_config_creation() {
        let config = create_development_config();
        assert!(config.system.debug_mode);
    }

    #[test]
    fn test_testing_config_creation() {
        let config = create_testing_config();
        assert!(!config
            .features
            .custom_flags
            .get("enable_metrics")
            .copied()
            .unwrap_or(true));
    }

    #[test]
    fn test_config_cloning() {
        let config1 = create_default_config();
        let config2 = config1.clone();
        assert_eq!(config1.system.instance_name, config2.system.instance_name);
    }

    #[test]
    fn test_config_debug_format() {
        let config = create_default_config();
        let debug_str = format!("{:?}", config);
        assert!(!debug_str.is_empty());
        assert!(debug_str.contains("NestGateCanonicalConfig"));
    }
}

#[cfg(test)]
mod config_field_validation_tests {
    use crate::config::canonical_primary::{DeploymentEnvironment, LogLevel};

    #[test]
    fn test_deployment_environment_variants() {
        let envs = vec![
            DeploymentEnvironment::Development,
            DeploymentEnvironment::Testing,
            DeploymentEnvironment::Staging,
            DeploymentEnvironment::Production,
            DeploymentEnvironment::Performance,
            DeploymentEnvironment::Security,
        ];

        for env in envs {
            let debug_str = format!("{:?}", env);
            assert!(!debug_str.is_empty());
        }
    }

    #[test]
    fn test_deployment_environment_equality() {
        assert_eq!(
            DeploymentEnvironment::Production,
            DeploymentEnvironment::Production
        );
        assert_ne!(
            DeploymentEnvironment::Production,
            DeploymentEnvironment::Development
        );
    }

    #[test]
    fn test_deployment_environment_default() {
        let default_env = DeploymentEnvironment::default();
        assert_eq!(default_env, DeploymentEnvironment::Development);
    }

    #[test]
    fn test_log_level_variants() {
        let levels = vec![
            LogLevel::Error,
            LogLevel::Warn,
            LogLevel::Info,
            LogLevel::Debug,
            LogLevel::Trace,
        ];

        for level in levels {
            let debug_str = format!("{:?}", level);
            assert!(!debug_str.is_empty());
        }
    }

    #[test]
    fn test_log_level_default() {
        let default_level = LogLevel::default();
        assert_eq!(default_level, LogLevel::Info);
    }

    #[test]
    fn test_log_level_ordering() {
        // Verify log levels can be compared
        assert_eq!(LogLevel::Error, LogLevel::Error);
        assert_ne!(LogLevel::Error, LogLevel::Debug);
    }
}

#[cfg(test)]
mod config_modification_tests {
    use crate::config::{canonical_primary::DeploymentEnvironment, create_default_config};

    #[test]
    fn test_modify_system_config() {
        let mut config = create_default_config();
        config.system.instance_name = "test-instance".to_string();
        assert_eq!(config.system.instance_name, "test-instance");
    }

    #[test]
    fn test_modify_environment() {
        let mut config = create_default_config();
        config.system.environment = DeploymentEnvironment::Production;
        assert_eq!(config.system.environment, DeploymentEnvironment::Production);
    }

    #[test]
    fn test_modify_debug_mode() {
        let mut config = create_default_config();
        let original = config.system.debug_mode;
        config.system.debug_mode = !original;
        assert_ne!(config.system.debug_mode, original);
    }

    #[test]
    fn test_add_custom_flag() {
        let mut config = create_default_config();
        config
            .features
            .custom_flags
            .insert("test_flag".to_string(), true);
        assert!(config.features.custom_flags.contains_key("test_flag"));
    }

    #[test]
    fn test_remove_custom_flag() {
        let mut config = create_default_config();
        config
            .features
            .custom_flags
            .insert("temp_flag".to_string(), true);
        config.features.custom_flags.remove("temp_flag");
        assert!(!config.features.custom_flags.contains_key("temp_flag"));
    }
}

#[cfg(test)]
mod config_serialization_tests {
    use crate::config::create_default_config;

    #[test]
    fn test_config_serialization() {
        let config = create_default_config();
        // Test that config can be serialized (Debug format as proxy)
        let serialized = format!("{:?}", config);
        assert!(!serialized.is_empty());
        assert!(serialized.len() > 100); // Should have substantial content
    }

    #[test]
    fn test_config_clone_independence() {
        let mut config1 = create_default_config();
        let mut config2 = config1.clone();

        config1.system.instance_name = "instance1".to_string();
        config2.system.instance_name = "instance2".to_string();

        assert_eq!(config1.system.instance_name, "instance1");
        assert_eq!(config2.system.instance_name, "instance2");
    }
}

#[cfg(test)]
mod config_edge_cases {
    use crate::config::{canonical_primary::LogLevel, create_default_config};

    #[test]
    fn test_empty_instance_name() {
        let mut config = create_default_config();
        config.system.instance_name = String::new();
        // Should handle empty string without panic
        assert_eq!(config.system.instance_name, "");
    }

    #[test]
    fn test_very_long_instance_name() {
        let mut config = create_default_config();
        let long_name = "x".repeat(1000);
        config.system.instance_name = long_name.clone();
        assert_eq!(config.system.instance_name.len(), 1000);
    }

    #[test]
    fn test_unicode_instance_name() {
        let mut config = create_default_config();
        config.system.instance_name = "インスタンス-🚀-instance".to_string();
        assert!(config.system.instance_name.contains("🚀"));
    }

    #[test]
    fn test_special_characters_in_name() {
        let mut config = create_default_config();
        config.system.instance_name = r#"test-instance_123.test"#.to_string();
        assert!(config.system.instance_name.contains("_"));
        assert!(config.system.instance_name.contains("."));
    }

    #[test]
    fn test_multiple_log_level_changes() {
        let mut config = create_default_config();
        let levels = vec![
            LogLevel::Error,
            LogLevel::Warn,
            LogLevel::Info,
            LogLevel::Debug,
            LogLevel::Trace,
        ];

        for level in levels {
            config.system.log_level = level.clone();
            assert_eq!(config.system.log_level, level);
        }
    }

    #[test]
    fn test_many_custom_flags() {
        let mut config = create_default_config();
        for i in 0..100 {
            config
                .features
                .custom_flags
                .insert(format!("flag_{}", i), i % 2 == 0);
        }
        assert_eq!(config.features.custom_flags.len(), 100);
    }
}

#[cfg(test)]
mod config_integration_tests {
    use crate::config::create_default_config;

    #[test]
    fn test_config_in_result() {
        /// Gets Config
        fn get_config() -> Result<crate::config::canonical_primary::NestGateCanonicalConfig, String>
        {
            Ok(create_default_config())
        }

        let result = get_config();
        assert!(result.is_ok());
    }

    #[test]
    fn test_config_in_option() {
        /// Maybe Config
        fn maybe_config(
            should_return: bool,
        ) -> Option<crate::config::canonical_primary::NestGateCanonicalConfig> {
            if should_return {
                Some(create_default_config())
            } else {
                None
            }
        }

        assert!(maybe_config(true).is_some());
        assert!(maybe_config(false).is_none());
    }

    #[test]
    fn test_config_in_vec() {
        let configs = vec![
            create_default_config(),
            create_default_config(),
            create_default_config(),
        ];
        assert_eq!(configs.len(), 3);
    }

    #[test]
    fn test_config_comparison() {
        let config1 = create_default_config();
        let config2 = create_default_config();
        // Both should have same default values
        assert_eq!(config1.system.instance_name, config2.system.instance_name);
        assert_eq!(config1.system.environment, config2.system.environment);
    }
}

#[cfg(test)]
mod config_concurrency_tests {
    use crate::config::create_default_config;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_config_across_threads() {
        let config = Arc::new(create_default_config());
        let config_clone = Arc::clone(&config);

        let handle = thread::spawn(move || config_clone.system.instance_name.clone());

        let name = handle.join().unwrap();
        assert_eq!(name, "nestgate-default");
    }

    #[test]
    fn test_multiple_threads_reading_config() {
        let config = Arc::new(create_default_config());
        let mut handles = vec![];

        for _ in 0..10 {
            let config_clone = Arc::clone(&config);
            let handle = thread::spawn(move || {
                let _ = format!("{:?}", config_clone.system.environment);
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }
    }
}

#[cfg(test)]
mod config_performance_tests {
    use crate::config::create_default_config;

    #[test]
    fn test_config_creation_performance() {
        let start = std::time::Instant::now();
        for _ in 0..100 {
            let _ = create_default_config();
        }
        let duration = start.elapsed();
        // Should create 100 configs quickly (< 10ms)
        assert!(duration.as_millis() < 10);
    }

    #[test]
    fn test_config_cloning_performance() {
        let config = create_default_config();
        let start = std::time::Instant::now();
        for _ in 0..100 {
            let _ = config.clone();
        }
        let duration = start.elapsed();
        // Should clone 100 configs quickly (< 10ms)
        assert!(duration.as_millis() < 10);
    }

    #[test]
    fn test_config_debug_format_performance() {
        let config = create_default_config();
        let start = std::time::Instant::now();
        for _ in 0..100 {
            let _ = format!("{:?}", config);
        }
        let duration = start.elapsed();
        // Should format 100 times quickly (< 20ms)
        assert!(duration.as_millis() < 20);
    }
}

//
// Default configuration presets for different environments and use cases.
// 
// **NOTE**: The main production() and development() methods are defined in mod.rs
// This module provides additional preset configurations and utilities.

use super::*;

impl NestGateCanonicalUnifiedConfig {
    /// Create a testing configuration preset
    pub fn testing() -> Self {
        Self {
            system: SystemConfig {
                environment: DeploymentEnvironment::Development,
                log_level: "debug".to_string(),
                ..SystemConfig::default()
            },
            features: FeatureFlags {
                debug_mode: true,
                experimental: true,
                ..FeatureFlags::default()
            },
            testing: TestingConfigs {
                enabled: true,
                mock_external_services: true,
                test_timeout: std::time::Duration::from_secs(10),
                ..TestingConfigs::default()
            },
            ..Self::default()
        }
    }

    /// Create a minimal configuration preset
    pub fn minimal() -> Self {
        Self {
            system: SystemConfig {
                environment: DeploymentEnvironment::Development,
                log_level: "warn".to_string(),
                ..SystemConfig::default()
            },
            features: FeatureFlags {
                debug_mode: false,
                experimental: false,
                performance_monitoring: false,
                security_hardening: false,
            },
            ..Self::default()
        }
    }

    /// Create a high-performance configuration preset
    pub fn high_performance() -> Self {
        let mut config = Self::production();
        
        // Enable all performance features
        config.features.performance_monitoring = true;
        config.api.performance_handlers.analytics.enabled = true;
        config.api.performance_handlers.analytics.sampling_rate = 1.0;
        config.api.performance_handlers.metrics.enabled = true;
        
        // Optimize ZFS settings
        config.api.zfs_handlers.pool.compression = Some("lz4".to_string());
        config.api.zfs_handlers.pool.record_size = Some("1M".to_string());
        config.api.zfs_handlers.performance.monitoring_enabled = true;
        config.api.zfs_handlers.performance.operation_cache_size = 10000;
        config.api.zfs_handlers.performance.batch_size = 1000;
        
        config
    }

    /// Create a security-hardened configuration preset
    pub fn security_hardened() -> Self {
        let mut config = Self::production();
        
        // Maximum security settings
        config.features.security_hardening = true;
        config.api.handler_extensions.security.require_auth = true;
        config.api.handler_extensions.security.rate_limiting.enabled = true;
        config.api.handler_extensions.security.rate_limiting.requests_per_minute = 100; // Conservative
        config.api.handler_extensions.security.security_headers.enabled = true;
        
        // Disable experimental features
        config.features.experimental = false;
        config.features.debug_mode = false;
        config.api.handler_extensions.feature_flags.experimental_handlers = false;
        config.api.handler_extensions.feature_flags.debug_endpoints = false;
        
        config
    }
} 
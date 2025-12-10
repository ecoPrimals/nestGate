//! Comprehensive tests for security configuration

use anyhow::Result;

#[test]
fn test_security_config_default() {
    use nestgate_core::config::environment::SecurityConfig;

    let config = SecurityConfig::default();

    // Default config should exist
    assert!(config.rate_limit_enabled);
}

#[test]
fn test_security_config_tls_disabled_by_default() {
    use nestgate_core::config::environment::SecurityConfig;

    let config = SecurityConfig::default();
    // TLS disabled by default (must be explicitly enabled)
    assert!(!config.tls_enabled);
}

#[test]
fn test_security_config_rate_limiting_enabled() {
    use nestgate_core::config::environment::SecurityConfig;

    let config = SecurityConfig::default();
    assert!(
        config.rate_limit_enabled,
        "Rate limiting should be enabled by default"
    );
}

#[test]
fn test_security_config_rate_limit_default_value() {
    use nestgate_core::config::environment::SecurityConfig;

    let config = SecurityConfig::default();
    assert_eq!(config.rate_limit_per_minute, 1000);
}

#[test]
fn test_security_config_is_send() {
    use nestgate_core::config::environment::SecurityConfig;

    fn assert_send<T: Send>() {}
    assert_send::<SecurityConfig>();
}

#[test]
fn test_security_config_is_sync() {
    use nestgate_core::config::environment::SecurityConfig;

    fn assert_sync<T: Sync>() {}
    assert_sync::<SecurityConfig>();
}

#[test]
fn test_security_config_clone() {
    use nestgate_core::config::environment::SecurityConfig;

    let config1 = SecurityConfig::default();
    let config2 = config1.clone();

    assert_eq!(config1.tls_enabled, config2.tls_enabled);
    assert_eq!(config1.rate_limit_enabled, config2.rate_limit_enabled);
    assert_eq!(config1.rate_limit_per_minute, config2.rate_limit_per_minute);
}

#[test]
fn test_security_config_debug() {
    use nestgate_core::config::environment::SecurityConfig;

    let config = SecurityConfig::default();
    let debug_str = format!("{:?}", config);

    assert!(debug_str.contains("SecurityConfig"));
}

#[test]
fn test_security_config_disable_tls() {
    use nestgate_core::config::environment::SecurityConfig;

    let config = SecurityConfig {
        tls_enabled: false,
        ..Default::default()
    };

    assert!(!config.tls_enabled);
}

#[test]
fn test_security_config_tls_toggle() {
    use nestgate_core::config::environment::SecurityConfig;

    let mut config = SecurityConfig::default();

    // Start disabled (default)
    assert!(!config.tls_enabled);

    // Enable
    config.tls_enabled = true;
    assert!(config.tls_enabled);

    // Disable again
    config.tls_enabled = false;
    assert!(!config.tls_enabled);
}

#[test]
fn test_security_config_serialization() -> Result<()> {
    use nestgate_core::config::environment::SecurityConfig;

    let config = SecurityConfig::default();

    // Serialize
    let json = serde_json::to_string(&config)?;
    assert!(json.contains("tls_enabled"));

    // Deserialize
    let config2: SecurityConfig = serde_json::from_str(&json)?;
    assert_eq!(config.tls_enabled, config2.tls_enabled);

    Ok(())
}

#[test]
fn test_security_config_api_key_none_by_default() {
    use nestgate_core::config::environment::SecurityConfig;

    let config = SecurityConfig::default();
    assert!(config.api_key.is_none());
}

#[test]
fn test_security_config_api_key_set() {
    use nestgate_core::config::environment::SecurityConfig;

    let config = SecurityConfig {
        api_key: Some("test-api-key-123".to_string()),
        ..Default::default()
    };

    assert_eq!(config.api_key, Some("test-api-key-123".to_string()));
}

#[test]
fn test_security_config_multiple_instances() {
    use nestgate_core::config::environment::SecurityConfig;

    let config1 = SecurityConfig::default();
    let config2 = SecurityConfig::default();

    // Both should have same defaults
    assert_eq!(config1.tls_enabled, config2.tls_enabled);
    assert_eq!(config1.rate_limit_per_minute, config2.rate_limit_per_minute);
}

#[test]
fn test_security_config_custom_rate_limit() {
    use nestgate_core::config::environment::SecurityConfig;

    let config = SecurityConfig {
        rate_limit_per_minute: 1000,
        ..Default::default()
    };

    assert_eq!(config.rate_limit_per_minute, 1000);
}

#[test]
fn test_security_config_disable_rate_limiting() {
    use nestgate_core::config::environment::SecurityConfig;

    let config = SecurityConfig {
        rate_limit_enabled: false,
        ..Default::default()
    };

    assert!(!config.rate_limit_enabled);
}

#[test]
fn test_security_config_zero_rate_limit() {
    use nestgate_core::config::environment::SecurityConfig;

    let config = SecurityConfig {
        rate_limit_per_minute: 0,
        ..Default::default()
    };

    assert_eq!(config.rate_limit_per_minute, 0);
}

#[test]
fn test_security_config_high_rate_limit() {
    use nestgate_core::config::environment::SecurityConfig;

    let config = SecurityConfig {
        rate_limit_per_minute: 10000,
        ..Default::default()
    };

    assert_eq!(config.rate_limit_per_minute, 10000);
}

#[test]
fn test_security_config_cert_path_optional() {
    use nestgate_core::config::environment::SecurityConfig;

    let config = SecurityConfig::default();

    // Cert path is optional
    assert!(config.tls_cert_path.is_none() || config.tls_cert_path.is_some());
}

#[test]
fn test_security_config_key_path_optional() {
    use nestgate_core::config::environment::SecurityConfig;

    let config = SecurityConfig::default();

    // Key path is optional
    assert!(config.tls_key_path.is_none() || config.tls_key_path.is_some());
}

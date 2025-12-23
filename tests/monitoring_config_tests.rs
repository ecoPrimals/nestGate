//! Comprehensive tests for monitoring configuration

use anyhow::Result;

#[test]
fn test_monitoring_config_default() {
    use nestgate_core::config::environment::MonitoringConfig;

    let config = MonitoringConfig::default();

    assert!(config.detailed_metrics);
    assert!(config.tracing_enabled);
    assert!(!config.log_level.is_empty());
}

#[test]
fn test_monitoring_config_default_metrics_port() {
    use nestgate_core::config::environment::MonitoringConfig;

    let config = MonitoringConfig::default();
    assert_eq!(config.metrics_port.get(), 9090);
}

#[test]
fn test_monitoring_config_default_log_level() {
    use nestgate_core::config::environment::MonitoringConfig;

    let config = MonitoringConfig::default();
    assert_eq!(config.log_level, "info");
}

#[test]
fn test_monitoring_config_detailed_metrics_enabled() {
    use nestgate_core::config::environment::MonitoringConfig;

    let config = MonitoringConfig::default();
    assert!(
        config.detailed_metrics,
        "Detailed metrics should be enabled by default"
    );
}

#[test]
fn test_monitoring_config_tracing_enabled() {
    use nestgate_core::config::environment::MonitoringConfig;

    let config = MonitoringConfig::default();
    assert!(
        config.tracing_enabled,
        "Tracing should be enabled by default"
    );
}

#[test]
fn test_monitoring_config_trace_sample_rate() {
    use nestgate_core::config::environment::MonitoringConfig;

    let config = MonitoringConfig::default();
    assert_eq!(config.trace_sample_rate, 0.1);
}

#[test]
fn test_monitoring_config_is_send() {
    use nestgate_core::config::environment::MonitoringConfig;

    fn assert_send<T: Send>() {}
    assert_send::<MonitoringConfig>();
}

#[test]
fn test_monitoring_config_is_sync() {
    use nestgate_core::config::environment::MonitoringConfig;

    fn assert_sync<T: Sync>() {}
    assert_sync::<MonitoringConfig>();
}

#[test]
fn test_monitoring_config_clone() {
    use nestgate_core::config::environment::MonitoringConfig;

    let config1 = MonitoringConfig::default();
    let config2 = config1.clone();

    assert_eq!(config1.detailed_metrics, config2.detailed_metrics);
    assert_eq!(config1.log_level, config2.log_level);
    assert_eq!(config1.tracing_enabled, config2.tracing_enabled);
    assert_eq!(config1.trace_sample_rate, config2.trace_sample_rate);
}

#[test]
fn test_monitoring_config_debug() {
    use nestgate_core::config::environment::MonitoringConfig;

    let config = MonitoringConfig::default();
    let debug_str = format!("{:?}", config);

    assert!(debug_str.contains("MonitoringConfig"));
}

#[test]
fn test_monitoring_config_custom_log_level() {
    use nestgate_core::config::environment::MonitoringConfig;

    let config = MonitoringConfig {
        log_level: "debug".to_string(),
        ..Default::default()
    };

    assert_eq!(config.log_level, "debug");
}

#[test]
fn test_monitoring_config_log_levels() {
    use nestgate_core::config::environment::MonitoringConfig;

    let mut config = MonitoringConfig::default();

    for level in &["trace", "debug", "info", "warn", "error"] {
        config.log_level = level.to_string();
        assert_eq!(config.log_level, *level);
    }
}

#[test]
fn test_monitoring_config_disable_detailed_metrics() {
    use nestgate_core::config::environment::MonitoringConfig;

    let config = MonitoringConfig {
        detailed_metrics: false,
        ..Default::default()
    };

    assert!(!config.detailed_metrics);
}

#[test]
fn test_monitoring_config_disable_tracing() {
    use nestgate_core::config::environment::MonitoringConfig;

    let config = MonitoringConfig {
        tracing_enabled: false,
        ..Default::default()
    };

    assert!(!config.tracing_enabled);
}

#[test]
fn test_monitoring_config_custom_sample_rate() {
    use nestgate_core::config::environment::MonitoringConfig;

    let config = MonitoringConfig {
        trace_sample_rate: 1.0,
        ..Default::default()
    };

    assert_eq!(config.trace_sample_rate, 1.0);
}

#[test]
fn test_monitoring_config_zero_sample_rate() {
    use nestgate_core::config::environment::MonitoringConfig;

    let config = MonitoringConfig {
        trace_sample_rate: 0.0,
        ..Default::default()
    };

    assert_eq!(config.trace_sample_rate, 0.0);
}

#[test]
fn test_monitoring_config_sample_rate_bounds() {
    use nestgate_core::config::environment::MonitoringConfig;

    let mut config = MonitoringConfig::default();

    // Test various valid sample rates
    for rate in &[0.0, 0.1, 0.5, 1.0] {
        config.trace_sample_rate = *rate;
        assert_eq!(config.trace_sample_rate, *rate);
    }
}

#[test]
fn test_monitoring_config_serialization() -> Result<()> {
    use nestgate_core::config::environment::MonitoringConfig;

    let config = MonitoringConfig::default();

    // Serialize
    let json = serde_json::to_string(&config)?;
    assert!(json.contains("log_level"));

    // Deserialize
    let config2: MonitoringConfig = serde_json::from_str(&json)?;
    assert_eq!(config.log_level, config2.log_level);

    Ok(())
}

#[test]
fn test_monitoring_config_multiple_instances() {
    use nestgate_core::config::environment::MonitoringConfig;

    let config1 = MonitoringConfig::default();
    let config2 = MonitoringConfig::default();

    // Both should have same defaults
    assert_eq!(config1.log_level, config2.log_level);
    assert_eq!(config1.detailed_metrics, config2.detailed_metrics);
}

#[test]
fn test_monitoring_config_custom_metrics_port() {
    use nestgate_core::config::environment::{MonitoringConfig, Port};

    let config = MonitoringConfig {
        metrics_port: Port::new(8080).unwrap(),
        ..Default::default()
    };

    assert_eq!(config.metrics_port.get(), 8080);
}

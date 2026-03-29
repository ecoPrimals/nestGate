// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Unit tests for `ApiConfig` and related network API types.

use super::api::{
    ApiAlertConfig, ApiConfig, ApiMonitoringConfig, ApiPerformanceConfig, ApiSecurityConfig,
    RateLimitingConfig, TlsConfig,
};
use nestgate_types::error::NestGateError;
use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr};
use std::time::Duration;

fn sample_valid_api_config() -> ApiConfig {
    ApiConfig {
        bind_address: IpAddr::V4(Ipv4Addr::LOCALHOST),
        port: 8443,
        max_connections: 10,
        request_timeout: Duration::from_secs(5),
        connection_timeout: Duration::from_secs(2),
        port_range_start: 1000,
        port_range_end: 2000,
        enabled: true,
        version: "v1".to_string(),
        api_settings: HashMap::new(),
        tls: TlsConfig::default(),
        security: ApiSecurityConfig::development(),
        performance: ApiPerformanceConfig::development(),
        rate_limiting: RateLimitingConfig::development(),
        monitoring: ApiMonitoringConfig::development(),
    }
}

#[test]
fn default_matches_development_optimized() {
    let a = ApiConfig::default();
    let b = ApiConfig::development_optimized();
    assert_eq!(a.port, b.port);
    assert_eq!(a.enabled, b.enabled);
    assert_eq!(a.version, b.version);
}

#[test]
fn validate_rejects_zero_port() {
    let mut cfg = sample_valid_api_config();
    cfg.port = 0;
    let err = cfg.validate().unwrap_err();
    assert!(matches!(err, NestGateError::Validation(_)));
}

#[test]
fn validate_rejects_zero_max_connections() {
    let mut cfg = sample_valid_api_config();
    cfg.max_connections = 0;
    let err = cfg.validate().unwrap_err();
    assert!(matches!(err, NestGateError::Validation(_)));
}

#[test]
fn validate_rejects_invalid_port_range() {
    let mut cfg = sample_valid_api_config();
    cfg.port_range_start = 5000;
    cfg.port_range_end = 5000;
    let err = cfg.validate().unwrap_err();
    assert!(matches!(err, NestGateError::Validation(_)));
}

#[test]
fn validate_accepts_good_config() {
    let cfg = sample_valid_api_config();
    assert!(cfg.validate().is_ok());
}

#[test]
fn merge_overwrites_core_fields() {
    let a = sample_valid_api_config();
    let mut other = sample_valid_api_config();
    other.port = 9999;
    other.max_connections = 99;
    let merged = a.merge(other);
    assert_eq!(merged.port, 9999);
    assert_eq!(merged.max_connections, 99);
}

#[test]
fn production_hardened_has_stricter_rate_limiting_than_development() {
    let prod = RateLimitingConfig::production();
    let dev = RateLimitingConfig::development();
    assert!(prod.enabled);
    assert!(!dev.enabled);
    assert!(prod.requests_per_second < dev.requests_per_second);
}

#[test]
fn api_alert_production_is_stricter_than_default() {
    let def = ApiAlertConfig::default();
    let prod = ApiAlertConfig::production();
    assert!(prod.error_rate_threshold < def.error_rate_threshold);
    assert!(prod.response_time_threshold_ms < def.response_time_threshold_ms);
}

#[test]
fn serde_roundtrip_api_config_minimal() {
    let cfg = sample_valid_api_config();
    let json = serde_json::to_string(&cfg).expect("serialize");
    let back: ApiConfig = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(back.port, cfg.port);
    assert_eq!(back.bind_address, cfg.bind_address);
    assert_eq!(back.monitoring.metrics_path, cfg.monitoring.metrics_path);
}

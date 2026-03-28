// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Service-level tests for remote ZFS service

use super::super::*;
use crate::handlers::zfs::universal_zfs::config::RemoteConfig;
use std::time::Duration;

#[tokio::test]
async fn test_service_name() {
    use nestgate_core::constants::hardcoding::{addresses, ports};
    let endpoint = format!(
        "http://{}:{}",
        addresses::LOCALHOST_NAME,
        ports::HTTP_DEFAULT
    );

    let config = RemoteConfig {
        endpoint,
        timeout: Duration::from_secs(30),
        auth: None,
    };

    let service = RemoteZfsService::new(config);
    let name = service.service_name();

    assert!(!name.is_empty());
    assert!(name.to_lowercase().contains("remote") || name.to_lowercase().contains("zfs"));
}

#[tokio::test]
async fn test_multiple_service_instances() {
    let config1 = RemoteConfig {
        endpoint: "http://server1:8080".to_string(),
        timeout: Duration::from_secs(30),
        auth: None,
    };

    let config2 = RemoteConfig {
        endpoint: "http://server2:8080".to_string(),
        timeout: Duration::from_secs(30),
        auth: None,
    };

    let service1 = RemoteZfsService::new(config1);
    let service2 = RemoteZfsService::new(config2);

    // Both should be valid independent instances
    assert!(!service1.service_name().is_empty());
    assert!(!service2.service_name().is_empty());
}

#[tokio::test]
async fn test_service_with_different_timeouts() {
    use nestgate_core::constants::hardcoding::{addresses, ports};
    let endpoint = format!(
        "http://{}:{}",
        addresses::LOCALHOST_NAME,
        ports::HTTP_DEFAULT
    );

    let fast_config = RemoteConfig {
        endpoint: endpoint.clone(),
        timeout: Duration::from_secs(5),
        auth: None,
    };

    let slow_config = RemoteConfig {
        endpoint,
        timeout: Duration::from_secs(120),
        auth: None,
    };

    let fast_service = RemoteZfsService::new(fast_config);
    let slow_service = RemoteZfsService::new(slow_config);

    // Both should be creatable with different timeout configurations
    assert!(!fast_service.service_name().is_empty());
    assert!(!slow_service.service_name().is_empty());
}

#[tokio::test]
async fn test_service_version() {
    use nestgate_core::constants::hardcoding::{addresses, ports};
    let config = RemoteConfig {
        endpoint: format!(
            "http://{}:{}",
            addresses::LOCALHOST_NAME,
            ports::HTTP_DEFAULT
        ),
        timeout: Duration::from_secs(30),
        auth: None,
    };

    let service = RemoteZfsService::new(config);
    let version = service.service_version();

    assert!(!version.is_empty());
    assert!(version.contains('.') || !version.is_empty());
}

#[tokio::test]
async fn test_service_config_access() {
    let config = RemoteConfig {
        endpoint: "http://test-server:9999".to_string(),
        timeout: Duration::from_secs(45),
        auth: Some("test-token".to_string()),
    };

    let service = RemoteZfsService::new(config.clone());
    let retrieved_config = service.config();

    assert_eq!(retrieved_config.endpoint, config.endpoint);
    assert_eq!(retrieved_config.timeout, config.timeout);
    assert_eq!(retrieved_config.auth, config.auth);
}

#[tokio::test]
async fn test_service_uptime() {
    use nestgate_core::constants::hardcoding::{addresses, ports};
    let config = RemoteConfig {
        endpoint: format!(
            "http://{}:{}",
            addresses::LOCALHOST_NAME,
            ports::HTTP_DEFAULT
        ),
        timeout: Duration::from_secs(30),
        auth: None,
    };

    let service = RemoteZfsService::new(config);

    // Uptime should be very small immediately after creation
    let uptime = service.uptime();
    assert!(uptime < Duration::from_secs(1));

    // Wait a bit and check again
    tokio::time::sleep(Duration::from_millis(100)).await;
    let uptime2 = service.uptime();
    assert!(uptime2 >= Duration::from_millis(100));
}

#[tokio::test]
async fn test_connection_stats_initial_state() {
    use nestgate_core::constants::hardcoding::{addresses, ports};
    let config = RemoteConfig {
        endpoint: format!(
            "http://{}:{}",
            addresses::LOCALHOST_NAME,
            ports::HTTP_DEFAULT
        ),
        timeout: Duration::from_secs(30),
        auth: None,
    };

    let service = RemoteZfsService::new(config);
    let stats = service.connection_stats().await;

    assert_eq!(stats.total_requests, 0);
    assert_eq!(stats.successful_requests, 0);
    assert_eq!(stats.failed_requests, 0);
    assert_eq!(stats.consecutive_failures, 0);
}

#[tokio::test]
async fn test_record_success_updates_stats() {
    use nestgate_core::constants::hardcoding::{addresses, ports};
    let config = RemoteConfig {
        endpoint: format!(
            "http://{}:{}",
            addresses::LOCALHOST_NAME,
            ports::HTTP_DEFAULT
        ),
        timeout: Duration::from_secs(30),
        auth: None,
    };

    let service = RemoteZfsService::new(config);

    // Record a successful request
    service.record_success(Duration::from_millis(150)).await;

    let stats = service.connection_stats().await;
    assert_eq!(stats.total_requests, 1);
    assert_eq!(stats.successful_requests, 1);
    assert_eq!(stats.failed_requests, 0);
}

#[tokio::test]
async fn test_record_failure_updates_stats() {
    use nestgate_core::constants::hardcoding::{addresses, ports};
    let config = RemoteConfig {
        endpoint: format!(
            "http://{}:{}",
            addresses::LOCALHOST_NAME,
            ports::HTTP_DEFAULT
        ),
        timeout: Duration::from_secs(30),
        auth: None,
    };

    let service = RemoteZfsService::new(config);

    // Record a failed request
    service.record_failure("Timeout error".to_string()).await;

    let stats = service.connection_stats().await;
    assert_eq!(stats.total_requests, 1);
    assert_eq!(stats.successful_requests, 0);
    assert_eq!(stats.failed_requests, 1);
    assert_eq!(stats.consecutive_failures, 1);
    assert_eq!(stats.last_error, Some("Timeout error".to_string()));
}

#[tokio::test]
async fn test_mixed_success_and_failure() {
    use nestgate_core::constants::hardcoding::{addresses, ports};
    let config = RemoteConfig {
        endpoint: format!(
            "http://{}:{}",
            addresses::LOCALHOST_NAME,
            ports::HTTP_DEFAULT
        ),
        timeout: Duration::from_secs(30),
        auth: None,
    };

    let service = RemoteZfsService::new(config);

    // Record multiple operations
    service.record_success(Duration::from_millis(100)).await;
    service.record_success(Duration::from_millis(110)).await;
    service.record_failure("Error 1".to_string()).await;
    service.record_success(Duration::from_millis(120)).await;
    service.record_failure("Error 2".to_string()).await;

    let stats = service.connection_stats().await;
    assert_eq!(stats.total_requests, 5);
    assert_eq!(stats.successful_requests, 3);
    assert_eq!(stats.failed_requests, 2);
    assert_eq!(stats.consecutive_failures, 1); // Reset by last success, then 1 failure
}

#[tokio::test]
async fn test_service_clone_independence() {
    use nestgate_core::constants::hardcoding::{addresses, ports};
    let config = RemoteConfig {
        endpoint: format!(
            "http://{}:{}",
            addresses::LOCALHOST_NAME,
            ports::HTTP_DEFAULT
        ),
        timeout: Duration::from_secs(30),
        auth: None,
    };

    let service1 = RemoteZfsService::new(config);
    let service2 = service1.clone();

    // Record stats on service1
    service1.record_success(Duration::from_millis(100)).await;

    // service2 stats should be independent
    let stats1 = service1.connection_stats().await;
    let stats2 = service2.connection_stats().await;

    assert_eq!(stats1.total_requests, 1);
    assert_eq!(stats2.total_requests, 0); // Independent stats
}

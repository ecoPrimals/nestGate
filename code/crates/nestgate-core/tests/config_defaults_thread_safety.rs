// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Thread safety tests for configuration defaults system
//! Ensures configuration can be safely accessed from multiple threads

use nestgate_core::config::defaults_config::NetworkDefaultsConfig;
use std::sync::Arc;
use std::thread;

#[test]
fn test_config_from_multiple_threads() {
    let config = Arc::new(NetworkDefaultsConfig::new());

    let mut handles = vec![];

    // Spawn 10 threads that all access the same config
    for i in 0..10 {
        let config = Arc::clone(&config);
        let handle = thread::spawn(move || {
            for _ in 0..100 {
                let _ = config.get_api_port();
                let _ = config.get_bind_address();
                let _ = config.get_hostname();
            }
            i
        });
        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        let result = handle.join();
        assert!(result.is_ok(), "Thread should complete successfully");
    }
}

#[test]
fn test_config_builder_not_shared() {
    // Each thread gets its own config
    let mut handles = vec![];

    for i in 0..5 {
        let handle = thread::spawn(move || {
            let config = NetworkDefaultsConfig::new().with_api_port(8000 + i as u16);
            config.get_api_port()
        });
        handles.push(handle);
    }

    // Verify each thread got its own unique port
    let mut ports = vec![];
    for handle in handles {
        let port = handle.join().unwrap();
        ports.push(port);
    }

    // Each port should be unique
    ports.sort();
    ports.dedup();
    assert_eq!(ports.len(), 5, "Each thread should have unique config");
}

#[test]
fn test_concurrent_config_reads() {
    let config = Arc::new(
        NetworkDefaultsConfig::new()
            .with_api_port(9999)
            .with_hostname("testhost".to_string()),
    );

    let mut handles = vec![];

    for _ in 0..20 {
        let config = Arc::clone(&config);
        let handle = thread::spawn(move || {
            let port = config.get_api_port();
            let hostname = config.get_hostname();
            (port, hostname)
        });
        handles.push(handle);
    }

    // All threads should read the same values
    for handle in handles {
        let (port, hostname) = handle.join().unwrap();
        assert_eq!(port, 9999);
        assert_eq!(hostname, "testhost");
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 8)]
async fn test_async_config_access() {
    let config = Arc::new(NetworkDefaultsConfig::new().with_api_port(7777));

    let handles: Vec<_> = (0..50)
        .map(|_| {
            let config = Arc::clone(&config);
            tokio::spawn(async move {
                tokio::time::sleep(tokio::time::Duration::from_millis(1)).await;
                config.get_api_port()
            })
        })
        .collect();

    for handle in handles {
        let port = handle.await.unwrap();
        assert_eq!(port, 7777);
    }
}

#[test]
fn test_config_clone_independence() {
    let config1 = NetworkDefaultsConfig::new().with_api_port(1111);
    let config2 = config1.clone().with_api_port(2222);

    // Clones should be independent
    assert_eq!(config1.get_api_port(), 1111);
    assert_eq!(config2.get_api_port(), 2222);
}

#[test]
fn test_default_config_consistency() {
    // Multiple default configs should have same values
    let config1 = NetworkDefaultsConfig::default();
    let config2 = NetworkDefaultsConfig::default();

    assert_eq!(config1.get_api_port(), config2.get_api_port());
    assert_eq!(config1.get_bind_address(), config2.get_bind_address());
}

#[tokio::test]
async fn test_config_under_load() {
    let config = Arc::new(NetworkDefaultsConfig::new());

    // Simulate high load
    let handles: Vec<_> = (0..100)
        .map(|_| {
            let config = Arc::clone(&config);
            tokio::spawn(async move {
                for _ in 0..100 {
                    let _ = config.get_api_port();
                    let _ = config.get_websocket_port();
                    let _ = config.get_http_port();
                    let _ = config.get_bind_address();
                    let _ = config.get_hostname();
                }
            })
        })
        .collect();

    for handle in handles {
        handle.await.unwrap();
    }
}

#[test]
fn test_config_send_and_sync() {
    // Verify config implements Send + Sync
    fn assert_send<T: Send>() {}
    fn assert_sync<T: Sync>() {}

    assert_send::<NetworkDefaultsConfig>();
    assert_sync::<NetworkDefaultsConfig>();
}

#[test]
fn test_config_debug_output_thread_safe() {
    let config = Arc::new(NetworkDefaultsConfig::new());

    let handles: Vec<_> = (0..10)
        .map(|_| {
            let config = Arc::clone(&config);
            thread::spawn(move || format!("{:?}", config))
        })
        .collect();

    for handle in handles {
        let debug_str = handle.join().unwrap();
        assert!(!debug_str.is_empty());
    }
}

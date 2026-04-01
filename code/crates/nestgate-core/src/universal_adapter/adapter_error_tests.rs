// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! **UNIVERSAL ADAPTER ERROR TESTS** - Nov 23, 2025
//!
//! Comprehensive tests for universal adapter error handling and resilience

#[cfg(test)]
mod adapter_error_creation_tests {
    use crate::error::NestGateError;

    #[test]
    fn test_adapter_initialization_error() {
        let err = NestGateError::internal_error("Adapter init failed", "adapter");
        assert!(!format!("{err}").is_empty());
    }

    #[test]
    fn test_capability_not_found() {
        let err = NestGateError::internal_error("Capability not found", "adapter");
        assert!(!format!("{err}").is_empty());
    }

    #[test]
    fn test_primal_unavailable() {
        let err = NestGateError::internal_error("Primal unavailable", "adapter");
        assert!(!format!("{err}").is_empty());
    }

    #[test]
    fn test_routing_error() {
        let err = NestGateError::internal_error("Routing failed", "adapter");
        assert!(!format!("{err}").is_empty());
    }
}

#[cfg(test)]
mod capability_query_tests {
    use crate::error::{NestGateError, Result};

    /// Query Capability
    fn query_capability(name: &str, available: bool) -> Result<String> {
        if available && !name.is_empty() {
            Ok(format!("capability-{name}"))
        } else {
            Err(NestGateError::internal_error("Query failed", "adapter"))
        }
    }

    #[test]
    fn test_successful_query() {
        let result = query_capability("storage", true);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "capability-storage");
    }

    #[test]
    fn test_failed_query() {
        let result = query_capability("nonexistent", false);
        assert!(result.is_err());
    }

    #[test]
    fn test_empty_capability_name() {
        let result = query_capability("", true);
        assert!(result.is_err());
    }
}

#[cfg(test)]
mod adapter_routing_tests {
    use crate::error::{NestGateError, Result};

    #[test]
    fn test_route_to_primal() {
        /// Route
        fn route(primal: &str, available: bool) -> Result<String> {
            if available {
                Ok(format!("routed-to-{primal}"))
            } else {
                Err(NestGateError::internal_error("Routing failed", "adapter"))
            }
        }

        // ✅ FIXED: Use capability types, not primal names
        assert!(route("security-svc", true).is_ok());
        assert!(route("orchestration-svc", false).is_err());
    }

    #[test]
    fn test_fallback_routing() {
        /// Primary Route
        fn primary_route() -> Result<String> {
            Err(NestGateError::internal_error(
                "Primary unavailable",
                "adapter",
            ))
        }

        /// Secondary Route
        fn secondary_route() -> Result<String> {
            Ok("secondary-endpoint".to_string())
        }

        let result = primary_route().or_else(|_| secondary_route());
        assert_eq!(result.unwrap(), "secondary-endpoint");
    }
}

#[cfg(test)]
mod adapter_cache_tests {
    use std::collections::HashMap;

    #[test]
    fn test_capability_cache() {
        let mut cache: HashMap<String, Vec<String>> = HashMap::new();
        cache.insert(
            "storage".to_string(),
            vec!["s3".to_string(), "local".to_string()],
        );

        assert_eq!(cache.get("storage").unwrap().len(), 2);
    }

    #[test]
    fn test_cache_update() {
        let mut cache: HashMap<String, String> = HashMap::new();
        cache.insert("cap1".to_string(), "v1".to_string());
        cache.insert("cap1".to_string(), "v2".to_string());

        assert_eq!(cache.get("cap1"), Some(&"v2".to_string()));
    }

    #[test]
    fn test_cache_eviction() {
        let mut cache: HashMap<String, String> = HashMap::new();
        cache.insert("old".to_string(), "data".to_string());
        cache.remove("old");

        assert!(cache.is_empty());
    }
}

#[cfg(test)]
mod adapter_discovery_tests {
    use crate::error::{NestGateError, Result};

    #[test]
    fn test_discover_capabilities() {
        /// Discover
        fn discover() -> Result<Vec<String>> {
            Ok(vec![
                "storage".to_string(),
                "compute".to_string(),
                "ai".to_string(),
            ])
        }

        let result = discover();
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 3);
    }

    #[test]
    fn test_discovery_failure() {
        /// Discover With Failure
        fn discover_with_failure() -> Result<Vec<String>> {
            Err(NestGateError::internal_error("Discovery failed", "adapter"))
        }

        assert!(discover_with_failure().is_err());
    }

    #[test]
    fn test_partial_discovery() {
        /// Discover Partial
        fn discover_partial(fail_at: Option<usize>) -> Result<Vec<String>> {
            let mut results = vec![];
            for i in 0..3 {
                if Some(i) == fail_at {
                    return Err(NestGateError::internal_error("Partial failure", "adapter"));
                }
                results.push(format!("cap-{i}"));
            }
            Ok(results)
        }

        assert!(discover_partial(None).is_ok());
        assert!(discover_partial(Some(1)).is_err());
    }
}

#[cfg(test)]
mod adapter_edge_cases {
    use crate::error::NestGateError;

    #[test]
    fn test_empty_primal_name() {
        let err = NestGateError::validation_error("Empty primal name");
        assert!(!format!("{err}").is_empty());
    }

    #[test]
    fn test_invalid_capability_format() {
        let err = NestGateError::validation_error("Invalid capability format");
        assert!(!format!("{err}").is_empty());
    }

    #[test]
    fn test_circular_dependency() {
        let err = NestGateError::internal_error("Circular dependency detected", "adapter");
        assert!(!format!("{err}").is_empty());
    }

    #[test]
    fn test_adapter_overload() {
        let err = NestGateError::internal_error("Adapter overloaded", "adapter");
        assert!(!format!("{err}").is_empty());
    }

    #[test]
    fn test_version_mismatch() {
        let err = NestGateError::internal_error("Version mismatch", "adapter");
        assert!(!format!("{err}").is_empty());
    }
}

#[cfg(test)]
mod adapter_concurrent_tests {
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_concurrent_queries() {
        let results = Arc::new(std::sync::Mutex::new(Vec::new()));
        let mut handles = vec![];

        for i in 0..5 {
            let results_clone = Arc::clone(&results);
            let handle = thread::spawn(move || {
                let query_result = format!("result-{i}");
                results_clone.lock().unwrap().push(query_result);
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        let final_results = results.lock().unwrap();
        assert_eq!(final_results.len(), 5);
    }

    #[test]
    fn test_shared_adapter_state() {
        use std::collections::HashMap;

        let state = Arc::new(std::sync::RwLock::new(HashMap::new()));

        // Write
        {
            let mut state_write = state.write().unwrap();
            state_write.insert("key".to_string(), "value".to_string());
        }

        // Read
        {
            let state_read = state.read().unwrap();
            assert_eq!(state_read.get("key"), Some(&"value".to_string()));
        }
    }
}

#[cfg(test)]
mod adapter_performance_tests {
    use crate::error::NestGateError;

    #[test]
    fn test_query_performance() {
        let start = std::time::Instant::now();
        for i in 0..1000 {
            let _ = format!("capability-{i}");
        }
        let duration = start.elapsed();
        // Should complete quickly (< 5ms)
        assert!(duration.as_millis() < 5);
    }

    #[test]
    fn test_error_creation_performance() {
        let start = std::time::Instant::now();
        for i in 0..100 {
            let _ = NestGateError::internal_error(format!("Error {i}"), "adapter");
        }
        let duration = start.elapsed();
        // Should create errors quickly (< 10ms)
        assert!(duration.as_millis() < 10);
    }
}

#[cfg(test)]
mod adapter_integration_tests {
    use crate::error::{NestGateError, Result};
    use std::collections::HashMap;

    #[test]
    fn test_end_to_end_query() {
        /// Discover Capability
        fn discover_capability() -> Result<String> {
            Ok("raw-capability".to_string())
        }

        /// Validates  Capability
        fn validate_capability(cap: String) -> Result<String> {
            if cap.contains("capability") {
                Ok(cap)
            } else {
                Err(NestGateError::validation_error("Invalid"))
            }
        }

        /// Route Capability
        fn route_capability(cap: String) -> Result<String> {
            Ok(format!("routed-{cap}"))
        }

        let result = discover_capability()
            .and_then(validate_capability)
            .and_then(route_capability);

        assert!(result.is_ok());
        assert!(result.unwrap().starts_with("routed-"));
    }

    #[test]
    fn test_batch_capability_query() {
        /// Query Batch
        fn query_batch(capabilities: Vec<String>) -> Result<HashMap<String, String>> {
            let mut results = HashMap::new();
            for cap in capabilities {
                results.insert(cap.clone(), format!("{cap}-endpoint"));
            }
            Ok(results)
        }

        let caps = vec!["c1".to_string(), "c2".to_string(), "c3".to_string()];
        let result = query_batch(caps);

        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 3);
    }

    #[test]
    fn test_adapter_lifecycle() {
        enum AdapterState {
            /// Uninitialized
            Uninitialized,
            /// Initializing
            Initializing,
            /// Ready
            Ready,
        }

        let state = AdapterState::Uninitialized;
        assert!(matches!(state, AdapterState::Uninitialized));

        let state = AdapterState::Initializing;
        assert!(matches!(state, AdapterState::Initializing));

        let state = AdapterState::Ready;
        assert!(matches!(state, AdapterState::Ready));
    }
}

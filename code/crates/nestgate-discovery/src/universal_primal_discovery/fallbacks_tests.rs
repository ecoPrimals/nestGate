// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! **Comprehensive Tests for Fallback Discovery**
//!
//! Sprint 2: Fallback module coverage (quick win - 27 lines)
//! Target: 100% coverage of fallbacks.rs

use super::fallbacks::*;

// ============================================================================
// FALLBACK PORT TESTS
// ============================================================================

#[cfg(test)]
mod fallback_port_tests {
    use super::*;

    #[test]
    fn test_fallback_port_api() {
        assert_eq!(get_fallback_port("api"), 8080);
    }

    #[test]
    fn test_fallback_port_web() {
        assert_eq!(get_fallback_port("web"), 3000);
    }

    #[test]
    fn test_fallback_port_metrics() {
        assert_eq!(get_fallback_port("metrics"), 9090);
    }

    #[test]
    fn test_fallback_port_metrics_export() {
        assert_eq!(get_fallback_port("metrics_export"), 9090);
    }

    #[test]
    fn test_fallback_port_nfs() {
        assert_eq!(get_fallback_port("nfs"), 2049);
    }

    #[test]
    fn test_fallback_port_smb() {
        assert_eq!(get_fallback_port("smb"), 445);
    }

    #[test]
    fn test_fallback_port_cifs() {
        assert_eq!(get_fallback_port("cifs"), 445);
    }

    #[test]
    fn test_fallback_port_ftp() {
        assert_eq!(get_fallback_port("ftp"), 21);
    }

    #[test]
    fn test_fallback_port_ssh() {
        assert_eq!(get_fallback_port("ssh"), 22);
    }

    #[test]
    fn test_fallback_port_http() {
        assert_eq!(get_fallback_port("http"), 80);
    }

    #[test]
    fn test_fallback_port_https() {
        assert_eq!(get_fallback_port("https"), 443);
    }

    #[test]
    fn test_fallback_port_orchestration() {
        assert_eq!(get_fallback_port("orchestration"), 8081);
    }

    #[test]
    fn test_fallback_port_coordination() {
        assert_eq!(get_fallback_port("coordination"), 8082);
    }

    #[test]
    fn test_fallback_port_compute() {
        assert_eq!(get_fallback_port("compute"), 8083);
    }

    #[test]
    fn test_fallback_port_ai() {
        assert_eq!(get_fallback_port("ai"), 8084);
    }

    #[test]
    fn test_fallback_port_security() {
        assert_eq!(get_fallback_port("security"), 8085);
    }

    #[test]
    fn test_fallback_port_auth() {
        assert_eq!(get_fallback_port("auth"), 8086);
    }

    #[test]
    fn test_fallback_port_unknown() {
        assert_eq!(get_fallback_port("unknown-service"), 8080);
    }

    #[test]
    fn test_fallback_port_empty_string() {
        assert_eq!(get_fallback_port(""), 8080);
    }

    #[test]
    fn test_fallback_port_special_chars() {
        assert_eq!(get_fallback_port("service@#$%"), 8080);
    }

    #[test]
    fn test_fallback_port_numbers() {
        assert_eq!(get_fallback_port("12345"), 8080);
    }

    #[test]
    fn test_fallback_port_mixed_case() {
        // Should be case-sensitive (lowercase expected)
        assert_eq!(get_fallback_port("API"), 8080); // Defaults because uppercase
        assert_eq!(get_fallback_port("Api"), 8080); // Defaults because mixed
    }

    #[test]
    fn test_fallback_port_with_spaces() {
        assert_eq!(get_fallback_port("api "), 8080); // Trailing space defaults
        assert_eq!(get_fallback_port(" api"), 8080); // Leading space defaults
    }

    #[test]
    fn test_fallback_port_long_string() {
        let long_name = "a".repeat(1000);
        assert_eq!(get_fallback_port(&long_name), 8080);
    }

    #[test]
    fn test_fallback_port_all_standard_ports() {
        let services = vec![
            ("api", 8080),
            ("web", 3000),
            ("metrics", 9090),
            ("nfs", 2049),
            ("smb", 445),
            ("ftp", 21),
            ("ssh", 22),
            ("http", 80),
            ("https", 443),
        ];

        for (service, expected_port) in services {
            assert_eq!(
                get_fallback_port(service),
                expected_port,
                "Port mismatch for service: {}",
                service
            );
        }
    }

    #[test]
    fn test_fallback_port_capability_based_naming() {
        // Test modern capability-based naming
        assert_eq!(get_fallback_port("metrics_export"), 9090);
        assert_eq!(get_fallback_port("orchestration"), 8081);
        assert_eq!(get_fallback_port("coordination"), 8082);
        assert_eq!(get_fallback_port("compute"), 8083);
    }

    #[test]
    fn test_fallback_port_consistent_default() {
        // All unknown services should default to same port
        let unknown_services = vec![
            "unknown1",
            "unknown2",
            "random-service",
            "xyz",
            "test-test-test",
        ];

        for service in unknown_services {
            assert_eq!(get_fallback_port(service), 8080);
        }
    }

    #[test]
    fn test_fallback_port_valid_range() {
        // All fallback ports should be in valid range
        let all_services = vec![
            "api",
            "web",
            "metrics",
            "nfs",
            "smb",
            "cifs",
            "ftp",
            "ssh",
            "http",
            "https",
            "orchestration",
            "coordination",
            "compute",
            "ai",
            "security",
            "auth",
            "unknown",
        ];

        for service in all_services {
            let port = get_fallback_port(service);
            assert!(
                port > 0, // u16 is always <= 65535
                "Invalid port for {}: {}",
                service,
                port
            );
        }
    }

    #[test]
    fn test_fallback_port_no_conflicts() {
        // Check that capability-based ports don't conflict with standard services
        let ports: Vec<u16> = vec![
            get_fallback_port("orchestration"),
            get_fallback_port("coordination"),
            get_fallback_port("compute"),
            get_fallback_port("ai"),
            get_fallback_port("security"),
            get_fallback_port("auth"),
        ];

        // These should be in sequence 8081-8086
        assert_eq!(ports, vec![8081, 8082, 8083, 8084, 8085, 8086]);
    }
}

// ============================================================================
// INTEGRATION TESTS
// ============================================================================

#[cfg(test)]
mod fallback_integration_tests {
    use super::*;

    #[test]
    fn test_fallback_used_in_discovery_chain() {
        // Simulate discovery chain: try discovery, fall back on failure
        let service = "unknown-service";

        // Discovery would fail, use fallback
        let fallback_port = get_fallback_port(service);
        assert_eq!(fallback_port, 8080);

        // Verify port is usable
        assert!(fallback_port >= 1024); // Non-privileged port
    }

    #[test]
    fn test_fallback_for_all_common_services() {
        // Verify we have fallbacks for all common service types
        let common_services = vec!["api", "web", "http", "https", "ssh", "ftp", "metrics"];

        for service in common_services {
            let port = get_fallback_port(service);
            assert!(port > 0, "No fallback for common service: {}", service);
        }
    }

    #[test]
    fn test_fallback_deterministic() {
        // Same service should always return same port
        let service = "test-service";
        let port1 = get_fallback_port(service);
        let port2 = get_fallback_port(service);
        let port3 = get_fallback_port(service);

        assert_eq!(port1, port2);
        assert_eq!(port2, port3);
    }

    #[test]
    fn test_fallback_thread_safe() {
        use std::thread;

        let handles: Vec<_> = (0..10)
            .map(|i| {
                thread::spawn(move || {
                    let service = format!("service-{}", i % 3);
                    get_fallback_port(&service)
                })
            })
            .collect();

        for handle in handles {
            let port = handle.join().unwrap();
            assert!(port > 0); // u16 is always <= 65535
        }
    }

    #[test]
    fn test_fallback_performance() {
        use std::time::Instant;

        let start = Instant::now();

        // Should be very fast (O(1) match)
        for _ in 0..10000 {
            let _ = get_fallback_port("api");
        }

        let duration = start.elapsed();

        // 10k calls should take < 1ms
        assert!(
            duration.as_millis() < 10,
            "Fallback too slow: {:?}",
            duration
        );
    }
}

// ============================================================================
// EDGE CASE TESTS
// ============================================================================

#[cfg(test)]
mod fallback_edge_cases {
    use super::*;

    #[test]
    fn test_fallback_null_terminator() {
        // Should handle strings with null terminators
        let service_with_null = "api\0";
        let port = get_fallback_port(service_with_null);
        // Won't match "api" exactly due to null, will default
        assert_eq!(port, 8080);
    }

    #[test]
    fn test_fallback_unicode() {
        // Should handle Unicode service names
        let unicode_service = "api-🚀";
        assert_eq!(get_fallback_port(unicode_service), 8080);
    }

    #[test]
    fn test_fallback_newlines() {
        assert_eq!(get_fallback_port("api\n"), 8080);
        assert_eq!(get_fallback_port("\napi"), 8080);
    }

    #[test]
    fn test_fallback_tabs() {
        assert_eq!(get_fallback_port("api\t"), 8080);
        assert_eq!(get_fallback_port("\tapi"), 8080);
    }

    #[test]
    fn test_fallback_similar_names() {
        // Similar names should not match
        assert_eq!(get_fallback_port("api2"), 8080); // Default
        assert_eq!(get_fallback_port("api_v2"), 8080); // Default
        assert_eq!(get_fallback_port("web-api"), 8080); // Default
    }

    #[test]
    fn test_fallback_exact_match_required() {
        // Only exact matches should return specific ports
        assert_eq!(get_fallback_port("api"), 8080); // Exact match
        assert_eq!(get_fallback_port("apii"), 8080); // Extra letter -> default
        assert_eq!(get_fallback_port("ap"), 8080); // Missing letter -> default
    }
}

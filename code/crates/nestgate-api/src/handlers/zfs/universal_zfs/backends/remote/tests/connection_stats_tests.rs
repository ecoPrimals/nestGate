//! Connection statistics tests

use super::super::ConnectionStats;
use std::time::Duration;

#[test]
fn test_connection_stats_success_rate() {
    let stats = ConnectionStats {
        total_requests: 1000,
        successful_requests: 950,
        failed_requests: 50,
        average_response_time: Duration::from_millis(100),
        last_error: None,
        consecutive_failures: 0,
    };

    assert_eq!(stats.total_requests, 1000);
    assert_eq!(stats.successful_requests, 950);
    assert_eq!(stats.failed_requests, 50);

    // Success rate: 95%
    let success_rate = (stats.successful_requests as f64 / stats.total_requests as f64) * 100.0;
    assert!((success_rate - 95.0).abs() < 0.1);
}

#[test]
fn test_connection_stats_with_errors() {
    let stats = ConnectionStats {
        total_requests: 100,
        successful_requests: 80,
        failed_requests: 20,
        average_response_time: Duration::from_millis(250),
        last_error: Some("Connection timeout".to_string()),
        consecutive_failures: 5,
    };

    assert_eq!(stats.consecutive_failures, 5);
    assert!(stats.last_error.is_some());
    assert_eq!(
        stats.last_error.expect("Last error should be present"),
        "Connection timeout"
    );
    assert!(stats.average_response_time > Duration::from_millis(200));
}

#[test]
fn test_connection_stats_perfect_record() {
    let stats = ConnectionStats {
        total_requests: 500,
        successful_requests: 500,
        failed_requests: 0,
        average_response_time: Duration::from_millis(50),
        last_error: None,
        consecutive_failures: 0,
    };

    assert_eq!(stats.total_requests, stats.successful_requests);
    assert_eq!(stats.failed_requests, 0);
    assert_eq!(stats.consecutive_failures, 0);
    assert!(stats.last_error.is_none());
}

#[test]
fn test_connection_stats_response_time() {
    let fast = ConnectionStats {
        total_requests: 100,
        successful_requests: 100,
        failed_requests: 0,
        average_response_time: Duration::from_millis(10),
        last_error: None,
        consecutive_failures: 0,
    };

    let slow = ConnectionStats {
        total_requests: 100,
        successful_requests: 100,
        failed_requests: 0,
        average_response_time: Duration::from_millis(1000),
        last_error: None,
        consecutive_failures: 0,
    };

    assert!(fast.average_response_time < slow.average_response_time);
    assert!(fast.average_response_time < Duration::from_millis(100));
    assert!(slow.average_response_time > Duration::from_millis(100));
}

#[test]
fn test_new_connection_stats() {
    let stats = ConnectionStats::new();

    assert_eq!(stats.total_requests, 0);
    assert_eq!(stats.successful_requests, 0);
    assert_eq!(stats.failed_requests, 0);
    assert_eq!(stats.consecutive_failures, 0);
    assert!(stats.last_error.is_none());
}

#[test]
fn test_record_success() {
    let mut stats = ConnectionStats::new();

    stats.record_success(Duration::from_millis(100));

    assert_eq!(stats.total_requests, 1);
    assert_eq!(stats.successful_requests, 1);
    assert_eq!(stats.failed_requests, 0);
    assert_eq!(stats.consecutive_failures, 0);
}

#[test]
fn test_record_multiple_successes() {
    let mut stats = ConnectionStats::new();

    for i in 0..10 {
        stats.record_success(Duration::from_millis(100 + i * 10));
    }

    assert_eq!(stats.total_requests, 10);
    assert_eq!(stats.successful_requests, 10);
    assert_eq!(stats.failed_requests, 0);
    assert_eq!(stats.consecutive_failures, 0);
    assert_eq!(stats.success_rate(), 100.0);
}

#[test]
fn test_record_failure() {
    let mut stats = ConnectionStats::new();

    stats.record_failure("Connection timeout".to_string());

    assert_eq!(stats.total_requests, 1);
    assert_eq!(stats.successful_requests, 0);
    assert_eq!(stats.failed_requests, 1);
    assert_eq!(stats.consecutive_failures, 1);
    assert_eq!(stats.last_error, Some("Connection timeout".to_string()));
}

#[test]
fn test_record_multiple_failures() {
    let mut stats = ConnectionStats::new();

    for i in 0..5 {
        stats.record_failure(format!("Error {i}"));
    }

    assert_eq!(stats.total_requests, 5);
    assert_eq!(stats.successful_requests, 0);
    assert_eq!(stats.failed_requests, 5);
    assert_eq!(stats.consecutive_failures, 5);
    assert_eq!(stats.success_rate(), 0.0);
}

#[test]
fn test_consecutive_failures_reset() {
    let mut stats = ConnectionStats::new();

    // Record 3 failures
    stats.record_failure("Error 1".to_string());
    stats.record_failure("Error 2".to_string());
    stats.record_failure("Error 3".to_string());
    assert_eq!(stats.consecutive_failures, 3);

    // Success should reset consecutive failures
    stats.record_success(Duration::from_millis(100));
    assert_eq!(stats.consecutive_failures, 0);
}

#[test]
fn test_success_rate_empty() {
    let stats = ConnectionStats::new();
    assert_eq!(stats.success_rate(), 100.0); // No requests = 100% success
}

#[test]
fn test_success_rate_mixed() {
    let mut stats = ConnectionStats::new();

    // 7 successes
    for _ in 0..7 {
        stats.record_success(Duration::from_millis(100));
    }

    // 3 failures
    for i in 0..3 {
        stats.record_failure(format!("Error {i}"));
    }

    // 70% success rate
    assert!((stats.success_rate() - 70.0).abs() < 0.1);
}

#[test]
fn test_is_healthy_with_good_stats() {
    let stats = ConnectionStats {
        total_requests: 100,
        successful_requests: 95,
        failed_requests: 5,
        average_response_time: Duration::from_millis(100),
        last_error: None,
        consecutive_failures: 0,
    };

    assert!(stats.is_healthy());
}

#[test]
fn test_is_unhealthy_with_consecutive_failures() {
    let stats = ConnectionStats {
        total_requests: 100,
        successful_requests: 95,
        failed_requests: 5,
        average_response_time: Duration::from_millis(100),
        last_error: Some("Recent error".to_string()),
        consecutive_failures: 5, // Too many consecutive failures
    };

    assert!(!stats.is_healthy());
}

#[test]
fn test_is_unhealthy_with_low_success_rate() {
    let stats = ConnectionStats {
        total_requests: 100,
        successful_requests: 50, // Only 50% success
        failed_requests: 50,
        average_response_time: Duration::from_millis(100),
        last_error: Some("Error".to_string()),
        consecutive_failures: 0,
    };

    assert!(!stats.is_healthy());
}

#[test]
fn test_health_threshold_boundary() {
    // Test at exactly 3 consecutive failures (unhealthy)
    let stats_3_failures = ConnectionStats {
        total_requests: 100,
        successful_requests: 95,
        failed_requests: 5,
        average_response_time: Duration::from_millis(100),
        last_error: None,
        consecutive_failures: 3,
    };
    assert!(!stats_3_failures.is_healthy());

    // Test at 2 consecutive failures (healthy)
    let stats_2_failures = ConnectionStats {
        total_requests: 100,
        successful_requests: 95,
        failed_requests: 5,
        average_response_time: Duration::from_millis(100),
        last_error: None,
        consecutive_failures: 2,
    };
    assert!(stats_2_failures.is_healthy());
}

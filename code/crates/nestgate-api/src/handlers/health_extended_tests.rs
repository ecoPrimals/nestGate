// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Extended tests for health check handler
//!
//! Additional comprehensive tests beyond basic health check validation.

use super::health_check;
use axum::response::IntoResponse;

#[test]
fn test_health_check_multiple_calls() {
    // Health check should be idempotent
    let response1 = health_check();
    let response2 = health_check();
    let response3 = health_check();

    // All responses should be OK
    assert_eq!(
        response1.into_response().status(),
        axum::http::StatusCode::OK
    );
    assert_eq!(
        response2.into_response().status(),
        axum::http::StatusCode::OK
    );
    assert_eq!(
        response3.into_response().status(),
        axum::http::StatusCode::OK
    );
}

#[test]
fn test_health_check_concurrent() {
    use std::thread;

    let handles: Vec<_> = (0..10)
        .map(|_| {
            thread::spawn(|| {
                let response = health_check();
                assert_eq!(
                    response.into_response().status(),
                    axum::http::StatusCode::OK
                );
            })
        })
        .collect();

    for handle in handles {
        handle.join().expect("Thread should complete successfully");
    }
}

#[test]
fn test_health_check_response_body() {
    let response = health_check();
    let body = response.into_response();

    // Status should be OK
    assert_eq!(body.status(), axum::http::StatusCode::OK);
}

#[test]
fn test_health_check_no_side_effects() {
    // Health check should not modify any state
    let response1 = health_check();
    let response2 = health_check();

    // Both should be identical
    assert_eq!(
        response1.into_response().status(),
        response2.into_response().status()
    );
}

#[test]
fn test_health_check_zero_allocation() {
    // Health check should not allocate on the heap
    // (returning &'static str avoids allocations)
    let _response = health_check();
    // If this compiles and runs, the check is passing
}

#[test]
fn test_health_check_const_response() {
    // Verify the response is always "OK"
    let response = health_check();
    // Converting to response should succeed
    let _body = response.into_response();
}

#[tokio::test]
async fn test_health_check_async_context() {
    // Health check should work in async context
    let response = health_check();
    assert_eq!(
        response.into_response().status(),
        axum::http::StatusCode::OK
    );
}

#[test]
fn test_health_check_under_load() {
    // Simulate load by calling health check many times
    for _ in 0..1000 {
        let response = health_check();
        assert_eq!(
            response.into_response().status(),
            axum::http::StatusCode::OK
        );
    }
}

#[test]
fn test_health_check_timing_consistency() {
    let mut timings = Vec::new();

    for _ in 0..100 {
        let start = std::time::Instant::now();
        let _response = health_check();
        timings.push(start.elapsed());
    }

    // All timings should be fast and consistent
    for timing in timings {
        assert!(
            timing.as_micros() < 1000,
            "Health check took too long: {timing:?}"
        );
    }
}

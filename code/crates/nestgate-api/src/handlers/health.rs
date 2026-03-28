// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

use axum::response::IntoResponse;

/// Health check handler
#[must_use]
pub fn health_check() -> impl IntoResponse {
    "OK"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_health_check_returns_ok() {
        let response = health_check();
        // Response should be "OK"
        assert_eq!(
            response.into_response().status(),
            axum::http::StatusCode::OK
        );
    }

    #[test]
    fn test_health_check_is_lightweight() {
        // Health check should be very fast and lightweight
        let start = std::time::Instant::now();
        let _response = health_check();
        let duration = start.elapsed();
        // Should complete in less than 1ms
        assert!(
            duration.as_millis() < 1,
            "Health check took too long: {duration:?}"
        );
    }
}

#[cfg(test)]
#[path = "health_extended_tests.rs"]
mod health_extended_tests;

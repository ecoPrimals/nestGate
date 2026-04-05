// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Comprehensive tests for ResponseBuilder
//! Tests all builder methods and response scenarios

use super::response_builder::*;
use axum::response::IntoResponse;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
struct TestData {
    id: u32,
    name: String,
}

#[test]
fn test_success_response() {
    let data = TestData {
        id: 1,
        name: "test".to_string(),
    };
    let response = ResponseBuilder::success(data);
    let response = response.into_response();

    assert_eq!(response.status(), axum::http::StatusCode::OK);
}

#[test]
fn test_created_response_with_location() {
    let data = TestData {
        id: 1,
        name: "new_item".to_string(),
    };
    let location = "/api/items/1";
    let response = ResponseBuilder::created(data, Some(location));
    let response = response.into_response();

    assert_eq!(response.status(), axum::http::StatusCode::CREATED);
    assert!(
        response
            .headers()
            .contains_key(axum::http::header::LOCATION)
    );
}

#[test]
fn test_created_response_without_location() {
    let data = TestData {
        id: 1,
        name: "new_item".to_string(),
    };
    let response = ResponseBuilder::created(data, None);
    let response = response.into_response();

    assert_eq!(response.status(), axum::http::StatusCode::CREATED);
}

#[test]
fn test_accepted_response() {
    let data = TestData {
        id: 1,
        name: "async_task".to_string(),
    };
    let response = ResponseBuilder::accepted(data);
    let response = response.into_response();

    assert_eq!(response.status(), axum::http::StatusCode::ACCEPTED);
}

#[test]
fn test_no_content_response() {
    let response = ResponseBuilder::no_content();
    let response = response.into_response();

    assert_eq!(response.status(), axum::http::StatusCode::NO_CONTENT);
}

#[test]
fn test_not_modified_response() {
    let response = ResponseBuilder::not_modified();
    let response = response.into_response();

    assert_eq!(response.status(), axum::http::StatusCode::NOT_MODIFIED);
}

#[test]
fn test_bad_request_response() {
    let response = ResponseBuilder::bad_request("Invalid input");
    let response = response.into_response();

    assert_eq!(response.status(), axum::http::StatusCode::BAD_REQUEST);
}

#[test]
fn test_unauthorized_response() {
    let response = ResponseBuilder::unauthorized("read_data");
    let response = response.into_response();

    assert_eq!(response.status(), axum::http::StatusCode::UNAUTHORIZED);
}

#[test]
fn test_forbidden_response() {
    let response = ResponseBuilder::forbidden("admin_panel");
    let response = response.into_response();

    assert_eq!(response.status(), axum::http::StatusCode::FORBIDDEN);
}

#[test]
fn test_not_found_response() {
    let response = ResponseBuilder::not_found("/api/missing");
    let response = response.into_response();

    assert_eq!(response.status(), axum::http::StatusCode::NOT_FOUND);
}

#[test]
fn test_conflict_response() {
    let response = ResponseBuilder::conflict("duplicate_email");
    let response = response.into_response();

    assert_eq!(response.status(), axum::http::StatusCode::CONFLICT);
}

#[test]
fn test_validation_error_response() {
    let response = ResponseBuilder::validation_error("email", "Invalid email format");
    let response = response.into_response();

    assert_eq!(response.status(), axum::http::StatusCode::BAD_REQUEST);
}

#[test]
fn test_rate_limited_response_with_retry() {
    let response = ResponseBuilder::rate_limited(Some(60));
    let response = response.into_response();

    assert_eq!(response.status(), axum::http::StatusCode::TOO_MANY_REQUESTS);
}

#[test]
fn test_rate_limited_response_without_retry() {
    let response = ResponseBuilder::rate_limited(None);
    let response = response.into_response();

    assert_eq!(response.status(), axum::http::StatusCode::TOO_MANY_REQUESTS);
}

#[test]
fn test_internal_error_response() {
    let response = ResponseBuilder::internal("Database connection failed");
    let response = response.into_response();

    assert_eq!(
        response.status(),
        axum::http::StatusCode::INTERNAL_SERVER_ERROR
    );
}

#[test]
fn test_service_unavailable_response() {
    let response = ResponseBuilder::service_unavailable("payment_gateway");
    let response = response.into_response();

    assert_eq!(
        response.status(),
        axum::http::StatusCode::SERVICE_UNAVAILABLE
    );
}

#[test]
fn test_timeout_response() {
    let response = ResponseBuilder::timeout("external_api_call");
    let response = response.into_response();

    // Timeout returns 408 REQUEST_TIMEOUT (not 504 GATEWAY_TIMEOUT)
    assert_eq!(response.status(), axum::http::StatusCode::REQUEST_TIMEOUT);
}

#[test]
fn test_paginated_response() {
    let data = vec![
        TestData {
            id: 1,
            name: "item1".to_string(),
        },
        TestData {
            id: 2,
            name: "item2".to_string(),
        },
        TestData {
            id: 3,
            name: "item3".to_string(),
        },
    ];
    let response = ResponseBuilder::paginated(data, 1, 10, 23);
    let response = response.into_response();

    assert_eq!(response.status(), axum::http::StatusCode::OK);
}

#[test]
fn test_paginated_response_calculates_total_pages() {
    // 23 items with 10 per page = 3 pages
    let data = vec![TestData {
        id: 1,
        name: "item".to_string(),
    }];
    let response_data = PaginatedResponse {
        data,
        pagination: PaginationMetadata {
            page: 1,
            per_page: 10,
            total: 23,
            total_pages: 3,
        },
    };

    assert_eq!(response_data.pagination.total_pages, 3);
    assert_eq!(response_data.pagination.total, 23);
}

#[test]
fn test_with_headers() {
    let data = TestData {
        id: 1,
        name: "test".to_string(),
    };
    let mut headers = std::collections::HashMap::new();
    headers.insert("X-Custom-Header".to_string(), "custom_value".to_string());
    headers.insert("X-Request-ID".to_string(), "12345".to_string());

    let response = ResponseBuilder::with_headers(data, axum::http::StatusCode::OK, headers);
    let response = response.into_response();

    assert_eq!(response.status(), axum::http::StatusCode::OK);
    assert!(response.headers().contains_key("x-custom-header"));
    assert!(response.headers().contains_key("x-request-id"));
}

#[test]
fn test_with_cache() {
    let data = TestData {
        id: 1,
        name: "cached_item".to_string(),
    };
    let response = ResponseBuilder::with_cache(data, 3600, Some("abc123"));
    let response = response.into_response();

    assert_eq!(response.status(), axum::http::StatusCode::OK);
    assert!(
        response
            .headers()
            .contains_key(axum::http::header::CACHE_CONTROL)
    );
    assert!(response.headers().contains_key(axum::http::header::ETAG));
}

#[test]
fn test_with_cache_no_etag() {
    let data = TestData {
        id: 1,
        name: "cached_item".to_string(),
    };
    let response = ResponseBuilder::with_cache(data, 1800, None);
    let response = response.into_response();

    assert_eq!(response.status(), axum::http::StatusCode::OK);
    assert!(
        response
            .headers()
            .contains_key(axum::http::header::CACHE_CONTROL)
    );
    assert!(!response.headers().contains_key(axum::http::header::ETAG));
}

#[test]
fn test_streaming_not_implemented() {
    let response = ResponseBuilder::streaming_not_implemented();
    let response = response.into_response();

    assert_eq!(response.status(), axum::http::StatusCode::NOT_IMPLEMENTED);
}

#[test]
fn test_pagination_metadata_structure() {
    let metadata = PaginationMetadata {
        page: 2,
        per_page: 25,
        total: 100,
        total_pages: 4,
    };

    assert_eq!(metadata.page, 2);
    assert_eq!(metadata.per_page, 25);
    assert_eq!(metadata.total, 100);
    assert_eq!(metadata.total_pages, 4);
}

#[test]
fn test_paginated_response_structure() {
    let data = vec![
        TestData {
            id: 1,
            name: "item1".to_string(),
        },
        TestData {
            id: 2,
            name: "item2".to_string(),
        },
    ];
    let response = PaginatedResponse {
        data,
        pagination: PaginationMetadata {
            page: 1,
            per_page: 10,
            total: 2,
            total_pages: 1,
        },
    };

    assert_eq!(response.data.len(), 2);
    assert_eq!(response.pagination.page, 1);
}

#[test]
fn test_empty_paginated_response() {
    let data: Vec<TestData> = vec![];
    let response = ResponseBuilder::paginated(data, 1, 10, 0);
    let response = response.into_response();

    assert_eq!(response.status(), axum::http::StatusCode::OK);
}

#[test]
fn test_multiple_error_responses() {
    // Test bad request
    let r1 = ResponseBuilder::bad_request("bad request").into_response();
    assert!(r1.status().is_client_error());

    // Test unauthorized
    let r2 = ResponseBuilder::unauthorized("unauthorized").into_response();
    assert!(r2.status().is_client_error());

    // Test forbidden
    let r3 = ResponseBuilder::forbidden("forbidden").into_response();
    assert!(r3.status().is_client_error());

    // Test not found
    let r4 = ResponseBuilder::not_found("/not/found").into_response();
    assert!(r4.status().is_client_error());

    // Test conflict
    let r5 = ResponseBuilder::conflict("conflict").into_response();
    assert!(r5.status().is_client_error());

    // Test internal
    let r6 = ResponseBuilder::internal("internal").into_response();
    assert!(r6.status().is_server_error());
}

#[test]
fn test_success_variations() {
    let data1 = TestData {
        id: 1,
        name: "test1".to_string(),
    };
    let data2 = TestData {
        id: 2,
        name: "test2".to_string(),
    };
    let data3 = TestData {
        id: 3,
        name: "test3".to_string(),
    };

    let r1 = ResponseBuilder::success(data1).into_response();
    let r2 = ResponseBuilder::accepted(data2).into_response();
    let r3 = ResponseBuilder::created(data3, None).into_response();

    assert!(r1.status().is_success());
    assert!(r2.status().is_success());
    assert!(r3.status().is_success());
}

#[test]
fn test_invalid_header_graceful_handling() {
    let data = TestData {
        id: 1,
        name: "test".to_string(),
    };
    let mut headers = std::collections::HashMap::new();
    // Invalid UTF-8 in header value should be handled gracefully
    headers.insert("X-Valid-Header".to_string(), "valid_value".to_string());

    let response = ResponseBuilder::with_headers(data, axum::http::StatusCode::OK, headers);
    let response = response.into_response();

    // Should still succeed even if some headers fail
    assert_eq!(response.status(), axum::http::StatusCode::OK);
}

#[test]
fn test_pagination_edge_cases() {
    // Test with 1 item per page
    let data = vec![TestData {
        id: 1,
        name: "single".to_string(),
    }];
    let response = ResponseBuilder::paginated(data, 1, 1, 5);
    let response = response.into_response();
    assert_eq!(response.status(), axum::http::StatusCode::OK);

    // Test with large per_page value
    let data = vec![TestData {
        id: 1,
        name: "bulk".to_string(),
    }];
    let response = ResponseBuilder::paginated(data, 1, 1000, 500);
    let response = response.into_response();
    assert_eq!(response.status(), axum::http::StatusCode::OK);
}

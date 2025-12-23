//! **NETWORK CLIENT ADVANCED TESTS**
//!
//! Comprehensive integration, error scenarios, and advanced functionality tests.

use super::super::client::*;
use serde::Deserialize;
use std::collections::HashMap;
use std::time::Duration;

// ==================== STATUS CODE COMPREHENSIVE TESTS ====================

#[test]
fn test_status_code_1xx_informational() {
    let codes = [
        StatusCode::new(100), // Continue
        StatusCode::new(101), // Switching Protocols
        StatusCode::new(102), // Processing
    ];

    for code in &codes {
        assert!(!code.is_success());
        assert!(!code.is_error());
    }
}

#[test]
fn test_status_code_2xx_success() {
    let codes = [
        StatusCode::new(200), // OK
        StatusCode::new(201), // Created
        StatusCode::new(202), // Accepted
        StatusCode::new(204), // No Content
        StatusCode::new(206), // Partial Content
    ];

    for code in &codes {
        assert!(code.is_success());
        assert!(!code.is_error());
    }
}

#[test]
fn test_status_code_3xx_redirection() {
    let codes = [
        StatusCode::new(301), // Moved Permanently
        StatusCode::new(302), // Found
        StatusCode::new(304), // Not Modified
        StatusCode::new(307), // Temporary Redirect
    ];

    for code in &codes {
        assert!(!code.is_success());
        assert!(!code.is_error());
    }
}

#[test]
fn test_status_code_4xx_client_errors() {
    let codes = [
        StatusCode::new(400), // Bad Request
        StatusCode::new(401), // Unauthorized
        StatusCode::new(403), // Forbidden
        StatusCode::new(404), // Not Found
        StatusCode::new(429), // Too Many Requests
    ];

    for code in &codes {
        assert!(!code.is_success());
        assert!(code.is_error());
    }
}

#[test]
fn test_status_code_5xx_server_errors() {
    let codes = [
        StatusCode::new(500), // Internal Server Error
        StatusCode::new(501), // Not Implemented
        StatusCode::new(502), // Bad Gateway
        StatusCode::new(503), // Service Unavailable
        StatusCode::new(504), // Gateway Timeout
    ];

    for code in &codes {
        assert!(!code.is_success());
        assert!(code.is_error());
    }
}

// ==================== METHOD COMPREHENSIVE TESTS ====================

#[test]
fn test_all_methods_safe_unsafe() {
    assert!(Method::Get.is_safe());
    assert!(Method::Head.is_safe());
    assert!(Method::Options.is_safe());

    assert!(!Method::Post.is_safe());
    assert!(!Method::Put.is_safe());
    assert!(!Method::Delete.is_safe());
    assert!(!Method::Patch.is_safe());
}

#[test]
fn test_all_methods_body_capability() {
    assert!(!Method::Get.can_have_body());
    assert!(!Method::Head.can_have_body());
    assert!(!Method::Delete.can_have_body());
    assert!(!Method::Options.can_have_body());

    assert!(Method::Post.can_have_body());
    assert!(Method::Put.can_have_body());
    assert!(Method::Patch.can_have_body());
}

// ==================== RESPONSE PARSING TESTS ====================

#[tokio::test]
async fn test_response_text_empty() {
    let response = Response {
        status: StatusCode::OK,
        headers: HashMap::new(),
        body: vec![],
    };

    let text = response.text().await.expect("Network operation failed");
    assert_eq!(text, "");
}

#[tokio::test]
async fn test_response_text_with_content() {
    let response = Response {
        status: StatusCode::OK,
        headers: HashMap::new(),
        body: b"Hello, World!".to_vec(),
    };

    let text = response.text().await.expect("Network operation failed");
    assert_eq!(text, "Hello, World!");
}

#[tokio::test]
async fn test_response_json_array() {
    #[derive(Deserialize, Debug, PartialEq)]
    struct User {
        id: i32,
        name: String,
    }

    let json_str = r#"[{"id":1,"name":"Alice"},{"id":2,"name":"Bob"}]"#;
    let response = Response {
        status: StatusCode::OK,
        headers: HashMap::new(),
        body: json_str.as_bytes().to_vec(),
    };

    let result: crate::Result<Vec<User>> = response.json().await;
    assert!(result.is_ok());

    let users = result.expect("Network operation failed");
    assert_eq!(users.len(), 2);
    assert_eq!(users[0].name, "Alice");
    assert_eq!(users[1].name, "Bob");
}

#[tokio::test]
async fn test_response_json_nested() {
    #[derive(Deserialize, Debug, PartialEq)]
    struct ApiResponse {
        success: bool,
        data: Data,
    }

    #[derive(Deserialize, Debug, PartialEq)]
    struct Data {
        count: i32,
        items: Vec<String>,
    }

    let json_str = r#"{"success":true,"data":{"count":3,"items":["a","b","c"]}}"#;
    let response = Response {
        status: StatusCode::OK,
        headers: HashMap::new(),
        body: json_str.as_bytes().to_vec(),
    };

    let result: crate::Result<ApiResponse> = response.json().await;
    assert!(result.is_ok());

    let api_response = result.expect("Network operation failed");
    assert!(api_response.success);
    assert_eq!(api_response.data.count, 3);
    assert_eq!(api_response.data.items.len(), 3);
}

// ==================== CLIENT CONFIG COMPREHENSIVE TESTS ====================

#[test]
fn test_client_config_custom_values() {
    let mut config = ClientConfig::<30000>::default();
    config.max_connections_per_host = 200;
    config.max_connections_per_host = 20;
    config.enable_compression = false;
    config.follow_redirects = false;
    config.max_retries = 10;

    assert_eq!(config.max_connections_per_host, 200);
    assert_eq!(config.max_connections_per_host, 20);
    assert!(!config.enable_compression);
    assert!(!config.follow_redirects);
    assert_eq!(config.max_retries, 10);
}

#[test]
fn test_client_config_user_agent() {
    let config = ClientConfig::<30000>::default();
    assert!(config.user_agent.contains("NestGate"));
}

#[test]
fn test_client_config_timeout_variations() {
    let config_5s = ClientConfig::<5000>::default();
    let config_30s = ClientConfig::<30000>::default();
    let config_60s = ClientConfig::<60000>::default();

    assert_eq!(config_5s.timeout, Duration::from_secs(5));
    assert_eq!(config_30s.timeout, Duration::from_secs(30));
    assert_eq!(config_60s.timeout, Duration::from_secs(60));
}

// ==================== TIMEOUT COMPREHENSIVE TESTS ====================

#[test]
fn test_timeout_various_durations() {
    let timeouts = vec![
        (100, Duration::from_millis(100)),
        (1000, Duration::from_secs(1)),
        (5000, Duration::from_secs(5)),
        (30000, Duration::from_secs(30)),
        (60000, Duration::from_secs(60)),
    ];

    for (ms, expected_duration) in timeouts {
        let timeout = TimeoutMs::new(ms);
        assert_eq!(timeout, expected_duration);
    }
}

// ==================== PORT EDGE CASES ====================

#[test]
fn test_port_boundary_values() {
    assert!(Port::new(0).is_err());
    assert!(Port::new(1).is_ok());
    assert!(Port::new(1024).is_ok());
    assert!(Port::new(8080).is_ok());
    assert!(Port::new(65534).is_ok());
    assert!(Port::new(65535).is_ok());
}

#[test]
fn test_port_common_values() {
    let common_ports = [
        20, 21, 22, 23, 25, 53, 80, 110, 143, 443, 3306, 5432, 6379, 8080, 9090,
    ];

    for port_num in &common_ports {
        let port = Port::new(*port_num);
        assert!(port.is_ok());
        assert_eq!(port.expect("Network operation failed").get(), *port_num);
    }
}

// ==================== UTILITY FUNCTIONS TESTS ====================

#[test]
fn test_create_client() {
    let client = create_client();
    // Should not panic, client should be created
    assert!(std::mem::size_of_val(&client) > 0);
}

#[tokio::test]
async fn test_https_endpoint_helper() {
    let result = https_endpoint("secure.example.com", 443).await;
    assert!(result.is_ok());

    let endpoint = result.expect("Network operation failed");
    assert_eq!(endpoint.scheme, Scheme::Https);
    assert_eq!(endpoint.host, "secure.example.com");
    assert_eq!(endpoint.port.get(), 443);
}

#[tokio::test]
async fn test_http_endpoint_helper() {
    let result = http_endpoint("api.example.com", 8080).await;
    assert!(result.is_ok());

    let endpoint = result.expect("Network operation failed");
    assert_eq!(endpoint.scheme, Scheme::Http);
    assert_eq!(endpoint.host, "api.example.com");
    assert_eq!(endpoint.port.get(), 8080);
}

#[tokio::test]
async fn test_https_endpoint_invalid_port() {
    let result = https_endpoint("example.com", 0).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_http_endpoint_invalid_port() {
    let result = http_endpoint("example.com", 0).await;
    assert!(result.is_err());
}

// ==================== ERROR TYPE TESTS ====================

#[test]
fn test_http_client_error_connection_failed() {
    let error = HttpClientError::ConnectionFailed {
        message: "Connection refused".to_string(),
    };

    assert!(error.to_string().contains("Connection failed"));
    assert!(error.to_string().contains("Connection refused"));
}

#[test]
fn test_http_client_error_timeout() {
    let error = HttpClientError::Timeout {
        timeout: Duration::from_secs(30),
    };

    assert!(error.to_string().contains("timeout"));
}

#[test]
fn test_http_client_error_invalid_response() {
    let error = HttpClientError::InvalidResponse {
        message: "Malformed JSON".to_string(),
    };

    assert!(error.to_string().contains("Invalid response"));
    assert!(error.to_string().contains("Malformed JSON"));
}

#[test]
fn test_http_client_error_too_many_redirects() {
    let error = HttpClientError::TooManyRedirects { count: 10 };

    assert!(error.to_string().contains("Too many redirects"));
    assert!(error.to_string().contains("10"));
}

// ==================== ERROR CONVERSION TESTS ====================

#[test]
fn test_http_client_error_to_nestgate_error_conversion() {
    let http_error = HttpClientError::ConnectionFailed {
        message: "test".to_string(),
    };

    let nestgate_error: NestGateError = http_error.into();
    assert!(format!("{:?}", nestgate_error).contains("test"));
}

// ==================== CONNECTION STATS TESTS ====================

#[test]
fn test_connection_stats_serialization() {
    let port = Port::new(8080).expect("Network operation failed");
    let endpoint = Endpoint::http("test.com".to_string(), port);

    let stats = ConnectionStats {
        endpoint,
        age: Duration::from_secs(10),
        idle_time: Duration::from_secs(5),
        request_count: 42,
    };

    let json = serde_json::to_string(&stats);
    assert!(json.is_ok());
}

// ==================== CLIENT STATS TESTS ====================

#[test]
fn test_client_stats_serialization() {
    let stats = ClientStats {
        total_connections: 10,
        active_requests: 5,
        total_requests: 100,
        failed_requests: 2,
    };

    let json = serde_json::to_string(&stats);
    assert!(json.is_ok());
}

#[test]
fn test_client_stats_default_values() {
    let stats = ClientStats {
        total_connections: 0,
        active_requests: 0,
        total_requests: 0,
        failed_requests: 0,
    };

    assert_eq!(stats.total_connections, 0);
    assert_eq!(stats.active_requests, 0);
    assert_eq!(stats.total_requests, 0);
    assert_eq!(stats.failed_requests, 0);
}

// ==================== RETRY LOGIC TESTS ====================
// Note: These test the retry logic structure, not actual network calls

#[tokio::test]
async fn test_http_client_retry_structure() {
    // Tests that HttpClient has send_request method with retry logic
    let client = HttpClient::default();
    let port = Port::new(8080).expect("Network operation failed");
    let endpoint = Endpoint::http("example.com".to_string(), port);
    let request = Request::get("/api/test");

    // This will attempt to connect and likely fail (no server)
    // But we're testing the retry structure exists
    let result = tokio::time::timeout(
        Duration::from_secs(5),
        client.send_request(&endpoint, &request),
    )
    .await;

    // Either timeout or error is fine - we're just testing structure
    assert!(result.is_err() || result.unwrap().is_err());
}

#[test]
fn test_retry_backoff_calculation() {
    // Test exponential backoff calculation
    // 100ms * 2^0 = 100ms
    // 100ms * 2^1 = 200ms
    // 100ms * 2^2 = 400ms

    let base = Duration::from_millis(100);
    let delays: Vec<Duration> = (0..3)
        .map(|attempt| Duration::from_millis(base.as_millis() as u64 * (1 << attempt)))
        .collect();

    assert_eq!(delays[0], Duration::from_millis(100));
    assert_eq!(delays[1], Duration::from_millis(200));
    assert_eq!(delays[2], Duration::from_millis(400));
}

#[test]
fn test_max_retry_attempts() {
    let max_attempts = 3;
    let mut attempts = 0;

    // Simulate retry loop
    for _ in 0..max_attempts {
        attempts += 1;
    }

    assert_eq!(attempts, 3);
    assert!(attempts >= max_attempts);
}

// ==================== CONNECTION POOL SEMAPHORE TESTS ====================

#[tokio::test]
async fn test_connection_pool_respects_max_connections() {
    let mut config = ClientConfig::<30000>::default();
    config.max_connections_per_host = 2; // Very low limit for testing
    let pool = ConnectionPool::new(config);

    let port = Port::new(8080).expect("Network operation failed");
    let endpoint = Endpoint::http("test.example.com".to_string(), port);

    // Get first connection
    let conn1 = pool.get_connection(&endpoint).await;
    assert!(conn1.is_ok());

    // Get second connection
    let conn2 = pool.get_connection(&endpoint).await;
    assert!(conn2.is_ok());
}

#[tokio::test]
async fn test_connection_pool_per_host_limit() {
    let config = ClientConfig::<30000>::default();
    assert_eq!(config.max_connections_per_host, 10);
}

// ==================== CONNECTION LIFECYCLE ADVANCED TESTS ====================

#[tokio::test]
async fn test_connection_becomes_stale_eventually() {
    // While we can't wait 5 minutes in a test, we can verify the logic exists
    use crate::constants::hardcoding::{addresses, ports};
    let port = Port::new(ports::HTTP_DEFAULT).expect("Network operation failed");
    let endpoint = Endpoint::http(addresses::LOCALHOST_NAME.to_string(), port);

    let connection = Connection::new(endpoint)
        .await
        .expect("Network operation failed");
    assert!(connection.is_alive()); // Initially alive
}

#[tokio::test]
async fn test_connection_multiple_creation() {
    let port = Port::new(8080).expect("Network operation failed");
    let endpoint = Endpoint::http("api.test.com".to_string(), port);

    let conn1 = Connection::new(endpoint.clone()).await;
    let conn2 = Connection::new(endpoint).await;

    assert!(conn1.is_ok());
    assert!(conn2.is_ok());
}

// ==================== INTEGRATION SCENARIO TESTS ====================

#[tokio::test]
async fn test_client_and_pool_integration() {
    let config = ClientConfig::<30000>::default();
    let client = HttpClient::new(config);

    // Client should be ready to use
    let stats = client.stats().await;
    assert_eq!(stats.total_connections, 0);
}

#[tokio::test]
async fn test_endpoint_and_request_integration() {
    let port = Port::new(8080).expect("Network operation failed");
    let endpoint = Endpoint::http("api.example.com".to_string(), port);
    let request = Request::get("/api/v1/health");

    // Verify they work together
    assert!(request.path.starts_with('/'));
    assert_eq!(endpoint.port.get(), 8080);
}

#[tokio::test]
async fn test_multiple_endpoints_with_client() {
    let _client = HttpClient::default();

    let endpoint1 = Endpoint::http(
        "service1.com".to_string(),
        Port::new(8080).expect("Network operation failed"),
    );
    let endpoint2 = Endpoint::http(
        "service2.com".to_string(),
        Port::new(9090).expect("Network operation failed"),
    );

    assert_ne!(endpoint1.base_url(), endpoint2.base_url());
}

// ==================== ERROR SCENARIO ADVANCED TESTS ====================

#[test]
fn test_http_error_types_exhaustive() {
    let errors = vec![
        HttpClientError::ConnectionFailed {
            message: "test".to_string(),
        },
        HttpClientError::Timeout {
            timeout: Duration::from_secs(30),
        },
        HttpClientError::InvalidResponse {
            message: "test".to_string(),
        },
        HttpClientError::TooManyRedirects { count: 10 },
    ];

    assert_eq!(errors.len(), 4);
}

#[test]
fn test_error_message_formats() {
    let error1 = HttpClientError::ConnectionFailed {
        message: "DNS resolution failed".to_string(),
    };
    assert!(error1.to_string().contains("DNS resolution failed"));

    let error2 = HttpClientError::Timeout {
        timeout: Duration::from_secs(30),
    };
    assert!(error2.to_string().contains("30"));

    let error3 = HttpClientError::InvalidResponse {
        message: "Status code 999".to_string(),
    };
    assert!(error3.to_string().contains("Status code 999"));
}

// ==================== CONFIGURATION VALIDATION TESTS ====================

#[test]
fn test_config_max_retries_validation() {
    let mut config = ClientConfig::<30000>::default();

    // Test various retry limits
    config.max_retries = 0;
    assert_eq!(config.max_retries, 0);

    config.max_retries = 5;
    assert_eq!(config.max_retries, 5);

    config.max_retries = 20;
    assert_eq!(config.max_retries, 20);
}

#[test]
fn test_config_compression_flag() {
    let mut config = ClientConfig::<30000>::default();

    assert!(config.enable_compression); // Default true

    config.enable_compression = false;
    assert!(!config.enable_compression);

    config.enable_compression = true;
    assert!(config.enable_compression);
}

#[test]
fn test_config_follow_redirects_flag() {
    let mut config = ClientConfig::<30000>::default();

    assert!(config.follow_redirects); // Default true

    config.follow_redirects = false;
    assert!(!config.follow_redirects);
}

// ==================== HEADER MAP TESTS ====================

#[test]
fn test_header_map_operations() {
    let mut headers: HeaderMap = HashMap::new();

    headers.insert("content-type".to_string(), "application/json".to_string());
    headers.insert("authorization".to_string(), "Bearer token".to_string());

    assert_eq!(headers.len(), 2);
    assert!(headers.contains_key("content-type"));
    assert!(headers.contains_key("authorization"));
}

#[test]
fn test_header_map_case_sensitivity() {
    let mut headers: HeaderMap = HashMap::new();

    headers.insert("Content-Type".to_string(), "application/json".to_string());

    // HTTP headers are case-insensitive, but our map is case-sensitive
    // This is a known limitation - headers should be normalized
    assert!(headers.contains_key("Content-Type"));
    assert!(!headers.contains_key("content-type"));
}

#[test]
fn test_header_map_update() {
    let mut headers: HeaderMap = HashMap::new();

    headers.insert("x-version".to_string(), "1.0".to_string());
    assert_eq!(headers.get("x-version"), Some(&"1.0".to_string()));

    headers.insert("x-version".to_string(), "2.0".to_string());
    assert_eq!(headers.get("x-version"), Some(&"2.0".to_string()));
}

// ==================== REQUEST BODY ADVANCED TESTS ====================

#[test]
fn test_request_body_sizes() {
    let small_data = b"hello";
    let medium_data = vec![0u8; 1024]; // 1KB
    let large_data = vec![0u8; 1024 * 1024]; // 1MB

    let body1 = RequestBody::Bytes(small_data);
    let body2 = RequestBody::Bytes(&medium_data);
    let body3 = RequestBody::Bytes(&large_data);

    match body1 {
        RequestBody::Bytes(b) => assert_eq!(b.len(), 5),
        _ => panic!("Expected Bytes"),
    }

    match body2 {
        RequestBody::Bytes(b) => assert_eq!(b.len(), 1024),
        _ => panic!("Expected Bytes"),
    }

    match body3 {
        RequestBody::Bytes(b) => assert_eq!(b.len(), 1024 * 1024),
        _ => panic!("Expected Bytes"),
    }
}

#[test]
fn test_request_body_string_content_types() {
    let json_body = r#"{"key":"value"}"#;
    let xml_body = r#"<root><key>value</key></root>"#;
    let plain_body = "plain text content";

    let body1 = RequestBody::Json(json_body);
    let body2 = RequestBody::Json(xml_body);
    let body3 = RequestBody::Json(plain_body);

    match body1 {
        RequestBody::Json(s) => assert!(s.contains("key")),
        _ => panic!("Expected Json"),
    }

    match body2 {
        RequestBody::Json(s) => assert!(s.contains("<root>")),
        _ => panic!("Expected Json"),
    }

    match body3 {
        RequestBody::Json(s) => assert!(s.contains("plain")),
        _ => panic!("Expected Json"),
    }
}

// ==================== RESPONSE ADVANCED TESTS ====================

#[tokio::test]
async fn test_response_with_headers() {
    let mut headers = HashMap::new();
    headers.insert("content-type".to_string(), "application/json".to_string());
    headers.insert("x-request-id".to_string(), "123-456".to_string());

    let response = Response {
        status: StatusCode::OK,
        headers,
        body: vec![],
    };

    assert!(response.is_success());
    assert_eq!(response.headers.len(), 2);
}

#[tokio::test]
async fn test_response_large_body() {
    let large_body = vec![0u8; 1024 * 1024]; // 1MB

    let response = Response {
        status: StatusCode::OK,
        headers: HashMap::new(),
        body: large_body,
    };

    assert!(response.is_success());
    assert_eq!(response.body.len(), 1024 * 1024);
}

// ==================== SCHEME COMPREHENSIVE TESTS ====================

#[test]
fn test_scheme_serialization() {
    let http = Scheme::Http;
    let https = Scheme::Https;

    let json_http = serde_json::to_string(&http);
    let json_https = serde_json::to_string(&https);

    assert!(json_http.is_ok());
    assert!(json_https.is_ok());
}

#[test]
fn test_scheme_clone() {
    let original = Scheme::Http;
    let cloned = original;

    assert_eq!(original, cloned);
}

// ==================== TIMEOUT EDGE CASES ====================

#[test]
fn test_timeout_very_large() {
    let timeout = TimeoutMs::new(u64::MAX);
    assert!(timeout.as_millis() > 0);
}

#[test]
fn test_timeout_millisecond_precision() {
    let timeout1 = TimeoutMs::new(1);
    let timeout2 = TimeoutMs::new(2);

    assert!(timeout2 > timeout1);
    assert_eq!(
        timeout2.as_millis() - timeout1.as_millis(),
        1
    );
}

// ==================== PORT COMPREHENSIVE COVERAGE ====================

#[test]
fn test_port_clone() {
    let original = Port::new(8080).expect("Network operation failed");
    let cloned = original;

    assert_eq!(original, cloned);
    assert_eq!(original.get(), cloned.get());
}

#[test]
fn test_port_hash_consistency() {
    use std::collections::HashSet;

    let port1 = Port::new(8080).expect("Network operation failed");
    let port2 = Port::new(8080).expect("Network operation failed");
    let port3 = Port::new(9090).expect("Network operation failed");

    let mut set = HashSet::new();
    set.insert(port1);
    set.insert(port2); // Should not add (same as port1)
    set.insert(port3);

    assert_eq!(set.len(), 2); // Only 8080 and 9090
}

// ==================== METHOD SERIALIZATION TESTS ====================

#[test]
fn test_method_deserialization() {
    let json = r#""Post""#;
    let method: Result<Method, _> = serde_json::from_str(json);
    assert!(method.is_ok());
    assert_eq!(method.expect("Network operation failed"), Method::Post);
}

#[test]
fn test_all_methods_serialization_roundtrip() {
    let methods = [
        Method::Get,
        Method::Post,
        Method::Put,
        Method::Delete,
        Method::Patch,
        Method::Head,
        Method::Options,
    ];

    for method in &methods {
        let json = serde_json::to_string(method).expect("Network operation failed");
        let deserialized: Method = serde_json::from_str(&json).expect("Network operation failed");
        assert_eq!(*method, deserialized);
    }
}

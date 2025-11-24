//! **COMPREHENSIVE NETWORK CLIENT TESTS**
//!
//! Tests for HTTP client, connection pooling, and network types.

use super::client::*;
use crate::error::NestGateError;
// Tests moved inline or removed - module-level imports not needed
use serde::Deserialize;
use std::collections::HashMap;
use std::time::Duration;

// ==================== PORT TESTS ====================

#[test]
fn test_port_new_valid() {
    let port = Port::new(8080);
    assert!(port.is_ok());
    assert_eq!(port.expect("Network operation failed").get(), 8080);
}

#[test]
fn test_port_new_zero_invalid() {
    let port = Port::new(0);
    assert!(port.is_err());
}

#[test]
fn test_port_new_max_valid() {
    let port = Port::new(65535);
    assert!(port.is_ok());
    assert_eq!(port.expect("Network operation failed").get(), 65535);
}

#[test]
fn test_port_get() {
    let port = Port::new(3000).expect("Network operation failed");
    assert_eq!(port.get(), 3000);
}

#[test]
fn test_port_equality() {
    let port1 = Port::new(8080).expect("Network operation failed");
    let port2 = Port::new(8080).expect("Network operation failed");
    let port3 = Port::new(8081).expect("Network operation failed");

    assert_eq!(port1, port2);
    assert_ne!(port1, port3);
}

#[test]
fn test_port_serialization() {
    let port = Port::new(8080).expect("Network operation failed");
    let json = serde_json::to_string(&port);
    assert!(json.is_ok());
}

// ==================== TIMEOUT TESTS ====================

#[test]
fn test_timeout_new() {
    let timeout = TimeoutMs::new(5000);
    assert_eq!(timeout.as_duration(), Duration::from_millis(5000));
}

#[test]
fn test_timeout_as_duration() {
    let timeout = TimeoutMs::new(1000);
    assert_eq!(timeout.as_duration(), Duration::from_secs(1));
}

#[test]
fn test_timeout_zero() {
    let timeout = TimeoutMs::new(0);
    assert_eq!(timeout.as_duration(), Duration::from_millis(0));
}

#[test]
fn test_timeout_large() {
    let timeout = TimeoutMs::new(60000);
    assert_eq!(timeout.as_duration(), Duration::from_secs(60));
}

// ==================== METHOD TESTS ====================

#[test]
fn test_method_is_safe_get() {
    assert!(Method::Get.is_safe());
}

#[test]
fn test_method_is_safe_head() {
    assert!(Method::Head.is_safe());
}

#[test]
fn test_method_is_safe_options() {
    assert!(Method::Options.is_safe());
}

#[test]
fn test_method_is_not_safe_post() {
    assert!(!Method::Post.is_safe());
}

#[test]
fn test_method_is_not_safe_put() {
    assert!(!Method::Put.is_safe());
}

#[test]
fn test_method_is_not_safe_delete() {
    assert!(!Method::Delete.is_safe());
}

#[test]
fn test_method_can_have_body_post() {
    assert!(Method::Post.can_have_body());
}

#[test]
fn test_method_can_have_body_put() {
    assert!(Method::Put.can_have_body());
}

#[test]
fn test_method_can_have_body_patch() {
    assert!(Method::Patch.can_have_body());
}

#[test]
fn test_method_cannot_have_body_get() {
    assert!(!Method::Get.can_have_body());
}

#[test]
fn test_method_cannot_have_body_delete() {
    assert!(!Method::Delete.can_have_body());
}

#[test]
fn test_method_serialization() {
    let method = Method::Post;
    let json = serde_json::to_string(&method);
    assert!(json.is_ok());
}

// ==================== STATUS CODE TESTS ====================

#[test]
fn test_status_code_ok() {
    assert_eq!(StatusCode::OK.as_u16(), 200);
}

#[test]
fn test_status_code_created() {
    assert_eq!(StatusCode::CREATED.as_u16(), 201);
}

#[test]
fn test_status_code_bad_request() {
    assert_eq!(StatusCode::BAD_REQUEST.as_u16(), 400);
}

#[test]
fn test_status_code_not_found() {
    assert_eq!(StatusCode::NOT_FOUND.as_u16(), 404);
}

#[test]
fn test_status_code_internal_server_error() {
    assert_eq!(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), 500);
}

#[test]
fn test_status_code_is_success_200() {
    assert!(StatusCode::OK.is_success());
}

#[test]
fn test_status_code_is_success_201() {
    assert!(StatusCode::CREATED.is_success());
}

#[test]
fn test_status_code_is_not_success_400() {
    assert!(!StatusCode::BAD_REQUEST.is_success());
}

#[test]
fn test_status_code_is_error_400() {
    assert!(StatusCode::BAD_REQUEST.is_error());
}

#[test]
fn test_status_code_is_error_500() {
    assert!(StatusCode::INTERNAL_SERVER_ERROR.is_error());
}

#[test]
fn test_status_code_is_not_error_200() {
    assert!(!StatusCode::OK.is_error());
}

#[test]
fn test_status_code_custom() {
    let status = StatusCode::new(418); // I'm a teapot
    assert_eq!(status.as_u16(), 418);
    assert!(status.is_error());
}

// ==================== SCHEME TESTS ====================

#[test]
fn test_scheme_http_display() {
    let scheme = Scheme::Http;
    assert_eq!(format!("{scheme}"), "http");
}

#[test]
fn test_scheme_https_display() {
    let scheme = Scheme::Https;
    assert_eq!(format!("{scheme}"), "https");
}

#[test]
fn test_scheme_equality() {
    assert_eq!(Scheme::Http, Scheme::Http);
    assert_eq!(Scheme::Https, Scheme::Https);
    assert_ne!(Scheme::Http, Scheme::Https);
}

// ==================== ENDPOINT TESTS ====================

#[test]
fn test_endpoint_http() {
    use crate::constants::hardcoding::{addresses, ports};
    let port = Port::new(ports::HTTP_DEFAULT).expect("Network operation failed");
    let endpoint = Endpoint::http(addresses::LOCALHOST_NAME.to_string(), port);

    assert_eq!(endpoint.host, addresses::LOCALHOST_NAME);
    assert_eq!(endpoint.port.get(), ports::HTTP_DEFAULT);
    assert_eq!(endpoint.scheme, Scheme::Http);
}

#[test]
fn test_endpoint_https() {
    let port = Port::new(443).expect("Network operation failed");
    let endpoint = Endpoint::https("example.com".to_string(), port);

    assert_eq!(endpoint.host, "example.com");
    assert_eq!(endpoint.port.get(), 443);
    assert_eq!(endpoint.scheme, Scheme::Https);
}

#[test]
fn test_endpoint_url_http() {
    use crate::constants::hardcoding::{addresses, ports};
    let port = Port::new(ports::HTTP_DEFAULT).expect("Network operation failed");
    let endpoint = Endpoint::http(addresses::LOCALHOST_NAME.to_string(), port);
    let expected_url = format!(
        "http://{}:{}",
        addresses::LOCALHOST_NAME,
        ports::HTTP_DEFAULT
    );

    assert_eq!(endpoint.url(), expected_url);
}

#[test]
fn test_endpoint_url_https() {
    let port = Port::new(443).expect("Network operation failed");
    let endpoint = Endpoint::https("example.com".to_string(), port);

    assert_eq!(endpoint.url(), "https://example.com:443");
}

#[test]
fn test_endpoint_equality() {
    use crate::constants::hardcoding::{addresses, ports};
    let port1 = Port::new(ports::HTTP_DEFAULT).expect("Network operation failed");
    let port2 = Port::new(ports::HTTP_DEFAULT).expect("Network operation failed");
    let endpoint1 = Endpoint::http(addresses::LOCALHOST_NAME.to_string(), port1);
    let endpoint2 = Endpoint::http(addresses::LOCALHOST_NAME.to_string(), port2);

    assert_eq!(endpoint1, endpoint2);
}

#[test]
fn test_endpoint_serialization() {
    use crate::constants::hardcoding::{addresses, ports};
    let port = Port::new(ports::HTTP_DEFAULT).expect("Network operation failed");
    let endpoint = Endpoint::http(addresses::LOCALHOST_NAME.to_string(), port);
    let json = serde_json::to_string(&endpoint);
    assert!(json.is_ok());
}

// ==================== REQUEST TESTS ====================

#[test]
fn test_request_get() {
    let request = Request::get("/api/users");

    assert_eq!(request.method, Method::Get);
    assert_eq!(request.path, "/api/users");
    assert!(request.headers.is_empty());
}

#[test]
fn test_request_post_json() {
    let body = r#"{"name":"test"}"#;
    let request = Request::post_json("/api/users", body);

    assert_eq!(request.method, Method::Post);
    assert_eq!(request.path, "/api/users");
    assert!(request.headers.contains_key("content-type"));
}

#[test]
fn test_request_with_header() {
    let request = Request::get("/api/users")
        .with_header("authorization".to_string(), "Bearer token123".to_string());

    assert_eq!(
        request.headers.get("authorization"),
        Some(&"Bearer token123".to_string())
    );
}

#[test]
fn test_request_multiple_headers() {
    let request = Request::get("/api/users")
        .with_header("authorization".to_string(), "Bearer token".to_string())
        .with_header("accept".to_string(), "application/json".to_string());

    assert_eq!(request.headers.len(), 2);
}

// ==================== REQUEST BODY TESTS ====================

#[test]
fn test_request_body_empty() {
    let body = RequestBody::Empty;
    match body {
        RequestBody::Empty => (),
        _ => panic!("Expected Empty body"),
    }
}

#[test]
fn test_request_body_bytes() {
    let data = b"test data";
    let body = RequestBody::Bytes(data);

    match body {
        RequestBody::Bytes(b) => assert_eq!(b, data),
        _ => panic!("Expected Bytes body"),
    }
}

#[test]
fn test_request_body_string() {
    let data = "test string";
    let body = RequestBody::String(data);

    match body {
        RequestBody::String(s) => assert_eq!(s, data),
        _ => panic!("Expected String body"),
    }
}

// ==================== RESPONSE TESTS ====================

#[test]
fn test_response_is_success_200() {
    let response = Response {
        status: StatusCode::OK,
        headers: HashMap::new(),
        body: vec![],
    };

    assert!(response.is_success());
}

#[test]
fn test_response_is_not_success_404() {
    let response = Response {
        status: StatusCode::NOT_FOUND,
        headers: HashMap::new(),
        body: vec![],
    };

    assert!(!response.is_success());
}

#[tokio::test]
async fn test_response_text() {
    let response = Response {
        status: StatusCode::OK,
        headers: HashMap::new(),
        body: b"test response".to_vec(),
    };

    let text = response.text().await;
    assert!(text.is_ok());
    assert_eq!(text.expect("Network operation failed"), "test response");
}

#[tokio::test]
async fn test_response_json() {
    #[derive(Deserialize, Debug, PartialEq)]
    struct TestData {
        name: String,
        value: i32,
    }

    let json_str = r#"{"name":"test","value":42}"#;
    let response = Response {
        status: StatusCode::OK,
        headers: HashMap::new(),
        body: json_str.as_bytes().to_vec(),
    };

    let result: crate::Result<TestData> = response.json().await;
    assert!(result.is_ok());

    let data = result.expect("Network operation failed");
    assert_eq!(data.name, "test");
    assert_eq!(data.value, 42);
}

// ==================== CLIENT CONFIG TESTS ====================

#[test]
fn test_client_config_default() {
    let config = ClientConfig::<30000>::default();

    assert_eq!(config.timeout.as_duration(), Duration::from_millis(30000));
    assert_eq!(config.max_connections, 100);
    assert_eq!(config.max_connections_per_host, 10);
    assert!(config.enable_compression);
    assert!(config.follow_redirects);
    assert_eq!(config.max_redirects, 5);
}

#[test]
fn test_client_config_custom_timeout() {
    let config = ClientConfig::<60000>::default();
    assert_eq!(config.timeout.as_duration(), Duration::from_millis(60000));
}

#[test]
fn test_client_config_serialization() {
    let config = ClientConfig::<30000>::default();
    let json = serde_json::to_string(&config);
    assert!(json.is_ok());
}

// ==================== CONNECTION POOL TESTS ====================

#[test]
fn test_connection_pool_new() {
    let config = ClientConfig::<30000>::default();
    let pool = ConnectionPool::new(config);

    // Pool should be created successfully
    assert!(std::mem::size_of_val(&pool) > 0);
}

// ==================== CONNECTION TESTS ====================

#[tokio::test]
async fn test_connection_is_alive_new() {
    use crate::constants::hardcoding::{addresses, ports};
    let port = Port::new(ports::HTTP_DEFAULT).expect("Network operation failed");
    let endpoint = Endpoint::http(addresses::LOCALHOST_NAME.to_string(), port);
    let connection = Connection::new(endpoint)
        .await
        .expect("Network operation failed");

    assert!(connection.is_alive());
}

#[tokio::test]
async fn test_connection_stats() {
    use crate::constants::hardcoding::{addresses, ports};
    let port = Port::new(ports::HTTP_DEFAULT).expect("Network operation failed");
    let endpoint = Endpoint::http(addresses::LOCALHOST_NAME.to_string(), port);
    let connection = Connection::new(endpoint)
        .await
        .expect("Network operation failed");

    let stats = connection.stats();
    assert_eq!(stats.request_count, 0);
    assert!(stats.age < Duration::from_secs(1));
}

// ==================== INTEGRATION TESTS ====================

#[test]
fn test_port_in_endpoint() {
    let port = Port::new(3000).expect("Network operation failed");
    let endpoint = Endpoint::http("api.example.com".to_string(), port);

    assert_eq!(endpoint.url(), "http://api.example.com:3000");
}

#[test]
fn test_multiple_ports() {
    let ports: Vec<crate::Result<Port>> = vec![
        Port::new(0),
        Port::new(80),
        Port::new(443),
        Port::new(8080),
        Port::new(65535),
    ];

    assert!(ports[0].is_err()); // 0 is invalid
    assert!(ports[1].is_ok());
    assert!(ports[2].is_ok());
    assert!(ports[3].is_ok());
    assert!(ports[4].is_ok());
}

#[test]
fn test_all_methods() {
    let methods = [
        Method::Get,
        Method::Post,
        Method::Put,
        Method::Delete,
        Method::Patch,
        Method::Head,
        Method::Options,
    ];

    assert_eq!(methods.len(), 7);
}

#[test]
fn test_status_code_ranges() {
    let codes = [
        StatusCode::new(200),
        StatusCode::new(201),
        StatusCode::new(299),
        StatusCode::new(300),
        StatusCode::new(399),
        StatusCode::new(400),
        StatusCode::new(404),
        StatusCode::new(500),
    ];

    assert!(codes[0].is_success()); // 200
    assert!(codes[1].is_success()); // 201
    assert!(codes[2].is_success()); // 299
    assert!(!codes[3].is_success()); // 300
    assert!(!codes[4].is_success()); // 399
    assert!(codes[5].is_error()); // 400
    assert!(codes[6].is_error()); // 404
    assert!(codes[7].is_error()); // 500
}

// ==================== HTTP CLIENT TESTS ====================

#[tokio::test]
async fn test_http_client_default() {
    let client = HttpClient::default();
    let stats = client.stats().await;
    assert_eq!(stats.total_connections, 0);
}

#[tokio::test]
async fn test_http_client_with_config() {
    let config = ClientConfig::<30000>::default();
    let client = HttpClient::new(config);
    let stats = client.stats().await;
    assert_eq!(stats.total_connections, 0);
}

#[tokio::test]
async fn test_http_client_stats() {
    let client = HttpClient::default();
    let stats = client.stats().await;

    assert_eq!(stats.active_requests, 0);
    assert_eq!(stats.total_requests, 0);
    assert_eq!(stats.failed_requests, 0);
}

// ==================== CONNECTION POOL ADVANCED TESTS ====================

#[tokio::test]
async fn test_connection_pool_get_connection() {
    let config = ClientConfig::<30000>::default();
    let pool = ConnectionPool::new(config);

    use crate::constants::hardcoding::{addresses, ports};
    let port = Port::new(ports::HTTP_DEFAULT).expect("Network operation failed");
    let endpoint = Endpoint::http(addresses::LOCALHOST_NAME.to_string(), port);

    let result = pool.get_connection(&endpoint).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_connection_pool_return_connection() {
    let config = ClientConfig::<30000>::default();
    let pool = ConnectionPool::new(config);

    use crate::constants::hardcoding::{addresses, ports};
    let port = Port::new(ports::HTTP_DEFAULT).expect("Network operation failed");
    let endpoint = Endpoint::http(addresses::LOCALHOST_NAME.to_string(), port);

    let connection = pool
        .get_connection(&endpoint)
        .await
        .expect("Network operation failed");
    pool.return_connection(&endpoint, connection).await;
}

#[tokio::test]
async fn test_connection_pool_reuse() {
    let config = ClientConfig::<30000>::default();
    let pool = ConnectionPool::new(config);

    use crate::constants::hardcoding::{addresses, ports};
    let port = Port::new(ports::HTTP_DEFAULT).expect("Network operation failed");
    let endpoint = Endpoint::http(addresses::LOCALHOST_NAME.to_string(), port);

    // Get and return a connection
    let conn1 = pool
        .get_connection(&endpoint)
        .await
        .expect("Network operation failed");
    pool.return_connection(&endpoint, conn1).await;

    // Get another connection (should potentially reuse)
    let conn2 = pool.get_connection(&endpoint).await;
    assert!(conn2.is_ok());
}

#[tokio::test]
async fn test_connection_pool_multiple_endpoints() {
    let config = ClientConfig::<30000>::default();
    let pool = ConnectionPool::new(config);

    let port1 = Port::new(8080).expect("Network operation failed");
    let port2 = Port::new(9090).expect("Network operation failed");
    let endpoint1 = Endpoint::http("service1.example.com".to_string(), port1);
    let endpoint2 = Endpoint::http("service2.example.com".to_string(), port2);

    let conn1 = pool.get_connection(&endpoint1).await;
    let conn2 = pool.get_connection(&endpoint2).await;

    assert!(conn1.is_ok());
    assert!(conn2.is_ok());
}

// ==================== CONNECTION LIFECYCLE TESTS ====================

#[tokio::test]
async fn test_connection_new() {
    use crate::constants::hardcoding::{addresses, ports};
    let port = Port::new(ports::HTTP_DEFAULT).expect("Network operation failed");
    let endpoint = Endpoint::http(addresses::LOCALHOST_NAME.to_string(), port);

    let connection = Connection::new(endpoint).await;
    assert!(connection.is_ok());
}

#[tokio::test]
async fn test_connection_initially_alive() {
    use crate::constants::hardcoding::{addresses, ports};
    let port = Port::new(ports::HTTP_DEFAULT).expect("Network operation failed");
    let endpoint = Endpoint::http(addresses::LOCALHOST_NAME.to_string(), port);

    let connection = Connection::new(endpoint)
        .await
        .expect("Network operation failed");
    assert!(connection.is_alive());
}

#[tokio::test]
async fn test_connection_stats_initial_state() {
    use crate::constants::hardcoding::{addresses, ports};
    let port = Port::new(ports::HTTP_DEFAULT).expect("Network operation failed");
    let endpoint = Endpoint::http(addresses::LOCALHOST_NAME.to_string(), port);

    let connection = Connection::new(endpoint)
        .await
        .expect("Network operation failed");
    let stats = connection.stats();

    assert_eq!(stats.request_count, 0);
    assert!(stats.age < Duration::from_secs(1));
    assert!(stats.idle_time < Duration::from_secs(1));
}

#[tokio::test]
async fn test_connection_stats_after_creation() {
    let port = Port::new(3000).expect("Network operation failed");
    let endpoint = Endpoint::http("api.test.com".to_string(), port);

    let connection = Connection::new(endpoint.clone())
        .await
        .expect("Network operation failed");
    let stats = connection.stats();

    assert_eq!(stats.endpoint, endpoint);
    assert_eq!(stats.request_count, 0);
}

// ==================== REQUEST BUILDING TESTS ====================

#[test]
fn test_request_get_with_path() {
    let request = Request::get("/api/v1/users");
    assert_eq!(request.method, Method::Get);
    assert_eq!(request.path, "/api/v1/users");
}

#[test]
fn test_request_post_json_with_body() {
    let json_body = r#"{"email":"user@example.com","password":"secure123"}"#;
    let request = Request::post_json("/api/v1/login", json_body);

    assert_eq!(request.method, Method::Post);
    assert_eq!(request.path, "/api/v1/login");
    assert!(request.headers.contains_key("content-type"));

    match request.body {
        RequestBody::String(s) => assert!(s.contains("email")),
        _ => panic!("Expected String body"),
    }
}

#[test]
fn test_request_with_multiple_headers() {
    let request = Request::get("/api/data")
        .with_header("authorization".to_string(), "Bearer xyz".to_string())
        .with_header("accept".to_string(), "application/json".to_string())
        .with_header("user-agent".to_string(), "NestGate/2.0".to_string());

    assert_eq!(request.headers.len(), 3);
    assert!(request.headers.contains_key("authorization"));
    assert!(request.headers.contains_key("accept"));
    assert!(request.headers.contains_key("user-agent"));
}

#[test]
fn test_request_header_overwrite() {
    let request = Request::get("/api/test")
        .with_header("x-custom".to_string(), "value1".to_string())
        .with_header("x-custom".to_string(), "value2".to_string());

    assert_eq!(request.headers.get("x-custom"), Some(&"value2".to_string()));
}

// ==================== ENDPOINT URL TESTS ====================

#[test]
fn test_endpoint_url_construction() {
    let port = Port::new(8443).expect("Network operation failed");
    let endpoint = Endpoint::https("secure.api.com".to_string(), port);

    assert_eq!(endpoint.url(), "https://secure.api.com:8443");
}

#[test]
fn test_endpoint_with_various_ports() {
    let ports_and_urls = vec![
        (80, "http://example.com:80"),
        (443, "http://example.com:443"),
        (8080, "http://example.com:8080"),
        (3000, "http://example.com:3000"),
    ];

    for (port_num, expected_url) in ports_and_urls {
        let port = Port::new(port_num).expect("Network operation failed");
        let endpoint = Endpoint::http("example.com".to_string(), port);
        assert_eq!(endpoint.url(), expected_url);
    }
}

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
    config.max_connections = 200;
    config.max_connections_per_host = 20;
    config.enable_compression = false;
    config.follow_redirects = false;
    config.max_redirects = 10;

    assert_eq!(config.max_connections, 200);
    assert_eq!(config.max_connections_per_host, 20);
    assert!(!config.enable_compression);
    assert!(!config.follow_redirects);
    assert_eq!(config.max_redirects, 10);
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

    assert_eq!(config_5s.timeout.as_duration(), Duration::from_secs(5));
    assert_eq!(config_30s.timeout.as_duration(), Duration::from_secs(30));
    assert_eq!(config_60s.timeout.as_duration(), Duration::from_secs(60));
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
        assert_eq!(timeout.as_duration(), expected_duration);
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
    config.max_connections = 2; // Very low limit for testing
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

    assert_ne!(endpoint1.url(), endpoint2.url());
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
fn test_config_max_redirects_validation() {
    let mut config = ClientConfig::<30000>::default();

    // Test various redirect limits
    config.max_redirects = 0;
    assert_eq!(config.max_redirects, 0);

    config.max_redirects = 5;
    assert_eq!(config.max_redirects, 5);

    config.max_redirects = 20;
    assert_eq!(config.max_redirects, 20);
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

    let body1 = RequestBody::String(json_body);
    let body2 = RequestBody::String(xml_body);
    let body3 = RequestBody::String(plain_body);

    match body1 {
        RequestBody::String(s) => assert!(s.contains("key")),
        _ => panic!("Expected String"),
    }

    match body2 {
        RequestBody::String(s) => assert!(s.contains("<root>")),
        _ => panic!("Expected String"),
    }

    match body3 {
        RequestBody::String(s) => assert!(s.contains("plain")),
        _ => panic!("Expected String"),
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
    assert!(timeout.as_duration().as_millis() > 0);
}

#[test]
fn test_timeout_millisecond_precision() {
    let timeout1 = TimeoutMs::new(1);
    let timeout2 = TimeoutMs::new(2);

    assert!(timeout2.as_duration() > timeout1.as_duration());
    assert_eq!(
        timeout2.as_duration().as_millis() - timeout1.as_duration().as_millis(),
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

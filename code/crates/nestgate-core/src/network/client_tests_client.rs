//! **NETWORK CLIENT & CONNECTION TESTS**
//!
//! Tests for Endpoint, Request, Response, HttpClient, and ConnectionPool.

use super::super::client::*;
use serde::Deserialize;
use std::collections::HashMap;
use std::time::Duration;

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

    assert_eq!(endpoint.base_url(), expected_url);
}

#[test]
fn test_endpoint_url_https() {
    let port = Port::new(443).expect("Network operation failed");
    let endpoint = Endpoint::https("example.com".to_string(), port);

    assert_eq!(endpoint.base_url(), "https://example.com:443");
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
fn test_request_body_json() {
    let data = r#"{"test":"string"}"#;
    let body = RequestBody::Json(data);

    match body {
        RequestBody::Json(s) => assert_eq!(s, data),
        _ => panic!("Expected JSON body"),
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

    assert_eq!(config.timeout, Duration::from_millis(30000));
    assert_eq!(config.max_connections_per_host, 100);
    assert_eq!(config.max_connections_per_host, 10);
    assert!(config.enable_compression);
    assert!(config.follow_redirects);
    assert_eq!(config.max_retries, 5);
}

#[test]
fn test_client_config_custom_timeout() {
    let config = ClientConfig::<60000>::default();
    assert_eq!(config.timeout, Duration::from_millis(60000));
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

    assert_eq!(endpoint.base_url(), "http://api.example.com:3000");
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

    match &request.body {
        Some(RequestBody::Json(s)) => assert!(s.contains("email")),
        _ => panic!("Expected JSON body"),
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

    assert_eq!(endpoint.base_url(), "https://secure.api.com:8443");
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
        assert_eq!(endpoint.base_url(), expected_url);
    }
}


//! **COMPREHENSIVE NETWORK CLIENT TESTS**
//!
//! Tests for HTTP client, connection pooling, and network types.

use super::*;

// ==================== PORT TESTS ====================

#[test]
fn test_port_new_valid() {
    let port = Port::new(8080);
    assert!(port.is_ok());
    assert_eq!(port.unwrap().get(), 8080);
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
    assert_eq!(port.unwrap().get(), 65535);
}

#[test]
fn test_port_get() {
    let port = Port::new(3000).unwrap();
    assert_eq!(port.get(), 3000);
}

#[test]
fn test_port_equality() {
    let port1 = Port::new(8080).unwrap();
    let port2 = Port::new(8080).unwrap();
    let port3 = Port::new(8081).unwrap();
    
    assert_eq!(port1, port2);
    assert_ne!(port1, port3);
}

#[test]
fn test_port_serialization() {
    let port = Port::new(8080).unwrap();
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
    let port = Port::new(8080).unwrap();
    let endpoint = Endpoint::http("localhost".to_string(), port);
    
    assert_eq!(endpoint.host, "localhost");
    assert_eq!(endpoint.port.get(), 8080);
    assert_eq!(endpoint.scheme, Scheme::Http);
}

#[test]
fn test_endpoint_https() {
    let port = Port::new(443).unwrap();
    let endpoint = Endpoint::https("example.com".to_string(), port);
    
    assert_eq!(endpoint.host, "example.com");
    assert_eq!(endpoint.port.get(), 443);
    assert_eq!(endpoint.scheme, Scheme::Https);
}

#[test]
fn test_endpoint_url_http() {
    let port = Port::new(8080).unwrap();
    let endpoint = Endpoint::http("localhost".to_string(), port);
    
    assert_eq!(endpoint.url(), "http://localhost:8080");
}

#[test]
fn test_endpoint_url_https() {
    let port = Port::new(443).unwrap();
    let endpoint = Endpoint::https("example.com".to_string(), port);
    
    assert_eq!(endpoint.url(), "https://example.com:443");
}

#[test]
fn test_endpoint_equality() {
    let port1 = Port::new(8080).unwrap();
    let port2 = Port::new(8080).unwrap();
    let endpoint1 = Endpoint::http("localhost".to_string(), port1);
    let endpoint2 = Endpoint::http("localhost".to_string(), port2);
    
    assert_eq!(endpoint1, endpoint2);
}

#[test]
fn test_endpoint_serialization() {
    let port = Port::new(8080).unwrap();
    let endpoint = Endpoint::http("localhost".to_string(), port);
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
    
    assert_eq!(request.headers.get("authorization"), Some(&"Bearer token123".to_string()));
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
    assert_eq!(text.unwrap(), "test response");
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
    
    let result: Result<TestData> = response.json().await;
    assert!(result.is_ok());
    
    let data = result.unwrap();
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
    let port = Port::new(8080).unwrap();
    let endpoint = Endpoint::http("localhost".to_string(), port);
    let connection = Connection::new(endpoint).await.unwrap();
    
    assert!(connection.is_alive());
}

#[tokio::test]
async fn test_connection_stats() {
    let port = Port::new(8080).unwrap();
    let endpoint = Endpoint::http("localhost".to_string(), port);
    let connection = Connection::new(endpoint).await.unwrap();
    
    let stats = connection.stats();
    assert_eq!(stats.request_count, 0);
    assert!(stats.age < Duration::from_secs(1));
}

// ==================== INTEGRATION TESTS ====================

#[test]
fn test_port_in_endpoint() {
    let port = Port::new(3000).unwrap();
    let endpoint = Endpoint::http("api.example.com".to_string(), port);
    
    assert_eq!(endpoint.url(), "http://api.example.com:3000");
}

#[test]
fn test_multiple_ports() {
    let ports: Vec<Result<Port>> = vec![
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
    let methods = vec![
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
    let codes = vec![
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


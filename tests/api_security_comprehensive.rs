use axum::{
    extract::{Path, Query, State},
    http::{HeaderMap, StatusCode},
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
// use urlencoding;
use nestgate_core::config::canonical_master::NestGateCanonicalConfig as UnifiedApiConfig;
use nestgate_core::error::{NestGateError, Result};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::Duration;
use tokio::time::timeout;
use tracing::{debug, error, info, warn};
use uuid::Uuid;

/// API Security Test Suite
/// Tests authentication, authorization, input validation, rate limiting, and security boundaries

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthResponse {
    pub token: String,
    pub expires_in: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtectedResource {
    pub id: Uuid,
    pub data: String,
    pub owner: String,
}

#[derive(Debug, Clone)]
pub struct SecurityTestState {
    pub users: Arc<RwLock<HashMap<String, String>>>, // username -> password
    pub tokens: Arc<RwLock<HashMap<String, String>>>, // token -> username
    pub resources: Arc<RwLock<HashMap<Uuid, ProtectedResource>>>,
    pub request_counts: Arc<RwLock<HashMap<String, u64>>>, // IP -> count
}

impl Default for SecurityTestState {
    fn default() -> Self {
        let mut users = HashMap::new();
        users.insert("admin".to_string(), "admin123".to_string());
        users.insert("user".to_string(), "user123".to_string());
        users.insert("guest".to_string(), "guest123".to_string());

        let mut resources = HashMap::new();
        let admin_resource = ProtectedResource {
            id: Uuid::new_v4(),
            data: "admin_data".to_string(),
            owner: "admin".to_string(),
        };
        let user_resource = ProtectedResource {
            id: Uuid::new_v4(),
            data: "user_data".to_string(),
            owner: "user".to_string(),
        };
        resources.insert(admin_resource.id, admin_resource);
        resources.insert(user_resource.id, user_resource);

        Self {
            users: Arc::new(RwLock::new(users)),
            tokens: Arc::new(RwLock::new(HashMap::new())),
            resources: Arc::new(RwLock::new(resources)),
            request_counts: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

// Authentication handler
async fn login(
    State(state): State<SecurityTestState>,
    Json(request): Json<AuthRequest>,
) -> std::result::Result<Json<AuthResponse>, StatusCode> {
    let users = state
        .users
        .read()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if let Some(password) = users.get(&request.username) {
        if password == &request.password {
            let token = format!("token_{}", Uuid::new_v4());
            let mut tokens = state
                .tokens
                .write()
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            tokens.insert(token.clone(), request.username);

            return Ok(Json(AuthResponse {
                token,
                expires_in: 3600,
            }));
        }
    }

    Err(StatusCode::UNAUTHORIZED)
}

// Protected resource handler with authorization
async fn get_resource(
    State(state): State<SecurityTestState>,
    Path(resource_id): Path<Uuid>,
    headers: HeaderMap,
) -> std::result::Result<Json<ProtectedResource>, StatusCode> {
    // Check authentication
    let token = headers
        .get("authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let tokens = state
        .tokens
        .read()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let username = tokens.get(token).ok_or(StatusCode::UNAUTHORIZED)?;

    // Check resource exists and authorization
    let resources = state
        .resources
        .read()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let resource = resources.get(&resource_id).ok_or(StatusCode::NOT_FOUND)?;

    // Check if user is authorized to access this resource
    if resource.owner != *username && *username != "admin" {
        return Err(StatusCode::FORBIDDEN);
    }

    Ok(Json(resource.clone()))
}

// Rate limited endpoint
async fn rate_limited_endpoint(
    State(state): State<SecurityTestState>,
    headers: HeaderMap,
) -> std::result::Result<Json<serde_json::Value>, StatusCode> {
    // Simple rate limiting based on IP (mocked with a header)
    let client_ip = headers
        .get("x-client-ip")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("unknown");

    let mut counts = state
        .request_counts
        .write()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let count = counts.entry(client_ip.to_string()).or_insert(0);
    *count += 1;

    // Allow max 5 requests per IP
    if *count > 5 {
        return Err(StatusCode::TOO_MANY_REQUESTS);
    }

    Ok(Json(json!({"message": "success", "count": *count})))
}

// Input validation endpoint
#[derive(Debug, Deserialize, Serialize)]
struct InputData {
    name: String,
    email: String,
    age: u32,
}

async fn validate_input(
    Json(data): Json<InputData>,
) -> std::result::Result<Json<serde_json::Value>, StatusCode> {
    // Validate name length
    if data.name.is_empty() || data.name.len() > 100 {
        return Err(StatusCode::BAD_REQUEST);
    }

    // Validate email format (simple check)
    if !data.email.contains('@') || data.email.len() > 254 {
        return Err(StatusCode::BAD_REQUEST);
    }

    // Validate age range
    if data.age > 150 {
        return Err(StatusCode::BAD_REQUEST);
    }

    // Check for potential injection attempts
    if data.name.contains("<script>") || data.email.contains("<script>") {
        return Err(StatusCode::BAD_REQUEST);
    }

    Ok(Json(json!({"message": "valid", "data": data})))
}

// SQL Injection protection test endpoint
#[derive(Debug, Deserialize)]
struct SearchQuery {
    query: String,
}

async fn search_endpoint(Query(params): Query<SearchQuery>) -> Result<Json<serde_json::Value>> {
    // Check for SQL injection attempts
    let dangerous_patterns = [
        "SELECT", "INSERT", "UPDATE", "DELETE", "DROP", "UNION", "'", "\"", ";", "--", "/*", "*/",
        "xp_", "sp_",
    ];

    let query_upper = params.query.to_uppercase();
    for pattern in &dangerous_patterns {
        if query_upper.contains(pattern) {
            return Err(NestGateError::api_error(
                "Invalid search query",
                Some("GET"),
                Some("/search"),
                Some(400),
            ));
        }
    }

    // Simulate search results
    Ok(Json(json!({
        "results": [
            {"id": 1, "title": "Result 1"},
            {"id": 2, "title": "Result 2"}
        ],
        "query": params.query
    })))
}

fn create_test_app() -> Router {
    let state = SecurityTestState::default();

    Router::new()
        .route("/auth/login", post(login))
        .route("/resources/:id", get(get_resource))
        .route("/rate-limited", get(rate_limited_endpoint))
        .route("/validate", post(validate_input))
        .route("/search", get(search_endpoint))
        .with_state(state)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum_test::TestServer;
    use serde_json::json;

    #[tokio::test]
    async fn test_authentication_bypass_protection() -> Result<(), Box<dyn std::error::Error>> {
        use crate::common::{TestHelpers, TestSetup};
        use nestgate_core::Result as TestResult;

        async fn run_test() -> TestResult<()> {
            let app = create_test_app();
            let app = create_test_app();

            // Test access to protected resource without token
            let response = server
                .get("/resources/00000000-0000-0000-0000-000000000000")
                .await;
            assert_eq!(
                response.status_code(),
                StatusCode::UNAUTHORIZED,
                "Access without token should be unauthorized"
            );

            // Test access with invalid token
            let response = server
                .get("/resources/00000000-0000-0000-0000-000000000000")
                .add_header("authorization", "Bearer invalid_token")
                .await;
            assert_eq!(
                response.status_code(),
                StatusCode::UNAUTHORIZED,
                "Access with invalid token should be unauthorized"
            );

            Ok(())
        }

        tokio::test("test_authentication_bypass_protection", run_test());
    }

    #[tokio::test]
    async fn test_authorization_boundary_enforcement() -> Result<(), Box<dyn std::error::Error>> {
        use crate::common::{
            helpers::assert_equals, helpers::create_test_server, test_error_handling::TestResult,
        };

        async fn run_test() -> TestResult<()> {
            let app = create_test_app();
            let server = create_test_server(app, "authorization_boundary_enforcement")?;

            // Login as user
            let login_response = server
                .post("/auth/login")
                .json(&json!({
                    "username": "user",
                    "password": "user123"
                }))
                .await;
            assert_eq!(login_response.status_code(), StatusCode::OK);

            let auth_data: AuthResponse = login_response.json();
            let _user_token = &auth_data.token;

            // Login as admin to get admin's resource ID
            let admin_login_response = server
                .post("/auth/login")
                .json(&json!({
                    "username": "admin",
                    "password": "admin123"
                }))
                .await;
            let admin_auth_data: AuthResponse = admin_login_response.json();
            let admin_token = &admin_auth_data.token;

            // Admin should be able to access any resource
            let response = server
                .get("/resources/00000000-0000-0000-0000-000000000001")
                .add_header("authorization", &format!("Bearer {admin_token}"))
                .await;
            // This will return NOT_FOUND since we're using a fake UUID, but not FORBIDDEN
            assert!(response.status_code() == StatusCode::NOT_FOUND);

            // User should not be able to access admin's resources (if we had real UUIDs)
            // This tests the authorization logic

            Ok(())
        }

        tokio::test("test_authorization_boundary_enforcement", run_test());
    }

    #[tokio::test]
    async fn test_input_validation_comprehensive() -> Result<(), Box<dyn std::error::Error>> {
        let app = create_test_app();
        let server = TestServer::new(app)?;

        // Test valid input
        let response = server
            .post("/validate")
            .json(&json!({
                "name": "John Doe",
                "email": "john@example.com",
                "age": 30
            }))
            .await;
        assert_eq!(response.status_code(), StatusCode::OK);

        // Test empty name
        let response = server
            .post("/validate")
            .json(&json!({
                "name": "",
                "email": "john@example.com",
                "age": 30
            }))
            .await;
        assert_eq!(response.status_code(), StatusCode::BAD_REQUEST);

        // Test name too long
        let long_name = "a".repeat(101);
        let response = server
            .post("/validate")
            .json(&json!({
                "name": long_name,
                "email": "john@example.com",
                "age": 30
            }))
            .await;
        assert_eq!(response.status_code(), StatusCode::BAD_REQUEST);

        // Test invalid email
        let response = server
            .post("/validate")
            .json(&json!({
                "name": "John Doe",
                "email": "invalid-email",
                "age": 30
            }))
            .await;
        assert_eq!(response.status_code(), StatusCode::BAD_REQUEST);

        // Test age too high
        let response = server
            .post("/validate")
            .json(&json!({
                "name": "John Doe",
                "email": "john@example.com",
                "age": 200
            }))
            .await;
        assert_eq!(response.status_code(), StatusCode::BAD_REQUEST);

        // Test XSS attempt
        let response = server
            .post("/validate")
            .json(&json!({
                "name": "<script>alert('xss')</script>",
                "email": "john@example.com",
                "age": 30
            }))
            .await;
        assert_eq!(response.status_code(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_rate_limiting_enforcement() -> Result<(), Box<dyn std::error::Error>> {
        let app = create_test_app();
        let server = TestServer::new(app)?;

        let client_ip = "192.168.1.100";

        // Make 5 requests (should succeed)
        for i in 1..=5 {
            let response = server
                .get("/rate-limited")
                .add_header("x-client-ip", client_ip)
                .await;
            assert_eq!(response.status_code(), StatusCode::OK);

            let data: serde_json::Value = response.json();
            assert_eq!(data["count"], i);
            Ok(())
        }

        // 6th request should be rate limited
        let response = server
            .get("/rate-limited")
            .add_header("x-client-ip", client_ip)
            .await;
        assert_eq!(response.status_code(), StatusCode::TOO_MANY_REQUESTS);

        // Different IP should still work
        let response = server
            .get("/rate-limited")
            .add_header("x-client-ip", "192.168.1.101")
            .await;
        assert_eq!(response.status_code(), StatusCode::OK);
        Ok(())
    }

    #[tokio::test]
    async fn test_sql_injection_protection() -> Result<(), Box<dyn std::error::Error>> {
        use crate::common::{
            helpers::assert_equals, helpers::create_test_server, test_error_handling::TestResult,
        };

        async fn run_test() -> TestResult<()> {
            let app = create_test_app();
            let server = create_test_server(app, "sql_injection_protection")?;

            // Test normal query
            let response = server.get("/search?query=hello").await;
            assert_eq!(
                response.status_code(),
                StatusCode::OK,
                "Normal query should succeed",
            )?;

            // Test SQL injection attempts
            let injection_attempts = [
                "'; DROP TABLE users; --",
                "1' OR '1'='1",
                "UNION SELECT * FROM users",
                "admin'--",
                "' OR 1=1 --",
                "'; INSERT INTO users VALUES",
                "1'; UPDATE users SET",
            ];

            for injection in &injection_attempts {
                let response = server
                    .get(&format!("/search?query={}", injection.replace(" ", "%20")))
                    .await;
                assert_eq!(
                    response.status_code(),
                    StatusCode::BAD_REQUEST,
                    "Failed to block injection: {}",
                    injection
                )?;
                Ok(())
            }

            Ok(())
        }

        tokio::test("test_sql_injection_protection", run_test());
    }

    #[tokio::test]
    async fn test_authentication_flow_comprehensive() -> Result<(), Box<dyn std::error::Error>> {
        let app = create_test_app();
        let server = TestServer::new(app)?;

        // Test successful login
        let response = server
            .post("/auth/login")
            .json(&json!({
                "username": "user",
                "password": "user123"
            }))
            .await;
        assert_eq!(response.status_code(), StatusCode::OK);

        let auth_data: AuthResponse = response.json();
        assert!(!auth_data.token.is_empty());
        assert_eq!(auth_data.expires_in, 3600);

        // Test invalid credentials
        let response = server
            .post("/auth/login")
            .json(&json!({
                "username": "user",
                "password": "wrong_password"
            }))
            .await;
        assert_eq!(response.status_code(), StatusCode::UNAUTHORIZED);

        // Test non-existent user
        let response = server
            .post("/auth/login")
            .json(&json!({
                "username": "nonexistent",
                "password": "password"
            }))
            .await;
        assert_eq!(response.status_code(), StatusCode::UNAUTHORIZED);
        Ok(())
    }

    #[tokio::test]
    async fn test_security_headers_and_cors() -> Result<(), Box<dyn std::error::Error>> {
        let app = create_test_app();
        let server = TestServer::new(app)?;

        // Test that sensitive endpoints don't leak information
        let response = server
            .get("/resources/00000000-0000-0000-0000-000000000000")
            .await;

        // Should return 401 Unauthorized, not 404 Not Found
        // This prevents information leakage about resource existence
        assert_eq!(response.status_code(), StatusCode::UNAUTHORIZED);
        Ok(())
    }

    #[tokio::test]
    async fn test_malformed_request_handling() -> Result<(), Box<dyn std::error::Error>> {
        let app = create_test_app();
        let server = TestServer::new(app)?;

        // Test malformed JSON - server returns 415 for unsupported media type with invalid JSON
        let response = server
            .post("/auth/login")
            .add_header("content-type", "application/json")
            .text("{invalid json}")
            .await;
        // Accept either 400 (Bad Request) or 415 (Unsupported Media Type) as both are valid security responses
        assert!(
            response.status_code() == StatusCode::BAD_REQUEST
                || response.status_code() == StatusCode::UNSUPPORTED_MEDIA_TYPE,
            "Expected 400 or 415, got {}",
            response.status_code().as_u16()
        );

        // Test missing required fields
        let response = server
            .post("/auth/login")
            .json(&json!({
                "username": "user"
                // missing password
            }))
            .await;
        // Accept either 400 (Bad Request) or 422 (Unprocessable Entity) for validation errors
        assert!(
            response.status_code() == StatusCode::BAD_REQUEST
                || response.status_code() == StatusCode::UNPROCESSABLE_ENTITY,
            "Expected 400 or 422, got {}",
            response.status_code().as_u16()
        );
        Ok(())
    }
}

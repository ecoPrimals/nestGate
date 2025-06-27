---
title: NestGate Security Implementation Plan
description: Detailed plan for addressing security issues in the NestGate API
version: 1.0.0
date: May 2024
---

# NestGate Security Implementation Plan

## Overview

This document provides a detailed implementation plan to address the security issues identified during testing of the NestGate API. It includes specific technical recommendations, code examples, and a phased approach for implementation.

## Phase 1: Authentication Enforcement

### 1.1 Apply Authentication Middleware

The existing authentication middleware needs to be properly applied to all API routes. The current issue is that invalid tokens are accepted.

**Current code in `src/http/api.rs`:**

```rust
let api_key_filter = warp::header::<String>("x-api-key")
    .and_then(move |api_key: String| {
        let auth = api_auth.clone();
        async move {
            match auth.validate_api_key(&api_key).await {
                true => Ok(()),
                false => Err(warp::reject::custom(ApiError(String::from("Invalid API key")))),
            }
        }
    });
```

**Implementation Steps:**

1. Update `ApiKeyStore` validation to properly check against stored keys:

```rust
// In src/http/middleware/api_key.rs
pub async fn validate_api_key(&self, api_key: &str) -> bool {
    // Current implementation just checks if the key exists
    self.keys.contains_key(api_key)
    
    // Change to actually validate the token properly
    // Add logic to check token expiration if applicable
    match self.keys.get(api_key) {
        Some(is_valid) => *is_valid,
        None => false
    }
}
```

2. Apply the middleware consistently to all route definitions:

```rust
// In src/http/api.rs
// Example for dataset endpoint
let datasets = warp::path("datasets")
    .and(warp::get())
    .and(with_zfs_engine(zfs_engine.clone()))
    .and(api_key_filter.clone())  // Apply authentication here
    .and_then(handle_list_datasets);
```

3. Add proper logging for authentication failures:

```rust
// In authentication error handling
async fn handle_authentication_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let (code, message) = if let Some(e) = err.find::<ApiError>() {
        error!("Authentication error: {}", e.0);
        (StatusCode::UNAUTHORIZED, e.0.clone())
    } else {
        // Handle other errors
        (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error".to_string())
    };
    
    Ok(warp::reply::with_status(
        warp::reply::json(&serde_json::json!({
            "status": "error",
            "message": message,
            "timestamp": chrono::Utc::now().to_rfc3339(),
        })),
        code,
    ))
}
```

### 1.2 Public vs. Protected Routes

Clearly separate public from protected routes to ensure consistent authentication.

```rust
// Define public routes that don't require authentication
let public_routes = health_route
    .or(some_other_public_route);

// Define protected routes that require authentication
let protected_routes = datasets_route
    .or(snapshots_route)
    .or(schedules_route)
    .and(api_key_filter.clone());  // Apply authentication to all protected routes

// Combine routes
let routes = public_routes.or(protected_routes);
```

## Phase 2: Authorization Implementation

### 2.1 Role-Based Access Control

Implement proper permission checking based on user roles.

1. Update `AuthManager` to check permissions properly:

```rust
// In src/security/auth.rs
pub fn has_permission(&self, context: &AuthContext, permission: &Permission) -> bool {
    match context.role {
        Role::Admin => true, // Admin has all permissions
        Role::Operator => {
            // Operator has all permissions except certain admin functions
            match permission {
                Permission::AdminConfig => false,
                Permission::UserManagement => false,
                _ => true,
            }
        },
        Role::ReadOnly => {
            // ReadOnly has only read permissions
            match permission {
                Permission::DatasetRead => true,
                Permission::SnapshotRead => true,
                Permission::ScheduleRead => true,
                Permission::PoolRead => true,
                _ => false,
            }
        }
    }
}
```

2. Use the `AuthFilterExt` trait to apply permission checks to routes:

```rust
// In src/http/api.rs
let datasets_read = warp::path("datasets")
    .and(warp::get())
    .and(with_zfs_engine(zfs_engine.clone()))
    .with_auth(auth_manager.clone(), Permission::DatasetRead)
    .and_then(handle_list_datasets);

let datasets_write = warp::path("datasets")
    .and(warp::post())
    .and(warp::body::json())
    .and(with_zfs_engine(zfs_engine.clone()))
    .with_auth(auth_manager.clone(), Permission::DatasetWrite)
    .and_then(handle_create_dataset);
```

3. Implement proper error handling for authorization failures:

```rust
async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let (code, message) = if let Some(e) = err.find::<ApiError>() {
        error!("API error: {}", e.0);
        (StatusCode::UNAUTHORIZED, e.0.clone())
    } else if let Some(_) = err.find::<AuthError>() {
        (StatusCode::FORBIDDEN, "Insufficient permissions".to_string())
    } else {
        // Handle other errors
        (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error".to_string())
    };
    
    Ok(warp::reply::with_status(
        warp::reply::json(&serde_json::json!({
            "status": "error",
            "message": message,
            "timestamp": chrono::Utc::now().to_rfc3339(),
        })),
        code,
    ))
}
```

## Phase 3: Input Validation

### 3.1 Dataset and Property Validation

Add validation for all user inputs to prevent injection attacks and ensure data quality.

1. Create a validation module:

```rust
// In src/validation/mod.rs
pub fn validate_dataset_name(name: &str) -> Result<(), String> {
    // Dataset name should be alphanumeric and contain only allowed characters
    let regex = regex::Regex::new(r"^[a-zA-Z0-9][a-zA-Z0-9_\-/.]+$").unwrap();
    if !regex.is_match(name) {
        return Err("Dataset name contains invalid characters".to_string());
    }
    
    // Prevent path traversal attacks
    if name.contains("..") {
        return Err("Dataset name cannot contain path traversal sequences".to_string());
    }
    
    // Length validation
    if name.len() > 255 {
        return Err("Dataset name is too long".to_string());
    }
    
    Ok(())
}

pub fn validate_property_value(property: &str, value: &str) -> Result<(), String> {
    // Validate based on property type
    match property {
        "compression" => {
            let valid_values = ["on", "off", "lz4", "gzip", "zstd", "lzjb"];
            if !valid_values.contains(&value) {
                return Err(format!("Invalid compression value: {}", value));
            }
        },
        "quota" => {
            // Check if it's a valid size string (e.g., "10G", "500M")
            let regex = regex::Regex::new(r"^\d+[KMGTP]?B?$").unwrap();
            if !regex.is_match(value) {
                return Err(format!("Invalid quota value: {}", value));
            }
        },
        // Add validation for other properties
        _ => {},
    }
    
    Ok(())
}
```

2. Apply validation in the API handlers:

```rust
async fn handle_create_dataset(
    request: CreateDatasetRequest,
    zfs_engine: Arc<ZfsEngine>,
    _: (),
) -> Result<impl Reply, Rejection> {
    // Validate dataset name
    if let Err(err) = validate_dataset_name(&request.name) {
        return Err(warp::reject::custom(ApiError(err)));
    }
    
    // Validate properties
    for (key, value) in &request.properties {
        if let Err(err) = validate_property_value(key, value) {
            return Err(warp::reject::custom(ApiError(err)));
        }
    }
    
    // Continue with dataset creation
    // ...
}
```

### 3.2 Request Size Limits

Add size limits to prevent denial of service attacks.

```rust
// In src/main.rs
let routes = warp::any()
    .and(warp::body::content_length_limit(1024 * 1024 * 10))  // 10MB limit
    .and(routes);
```

## Phase 4: Protection Against Injection Attacks

### 4.1 Command Injection Prevention

Ensure that all data passed to shell commands is properly sanitized.

```rust
// In src/libzfs/mod.rs
pub fn execute_command(&self, command: &str, args: &[&str]) -> Result<String, ZfsError> {
    // Instead of constructing shell commands with string interpolation,
    // use process APIs that handle argument passing safely
    let output = std::process::Command::new(command)
        .args(args)
        .output()
        .map_err(|e| ZfsError::IoError(e.to_string()))?;
    
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(ZfsError::CommandError(stderr.to_string()));
    }
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    Ok(stdout.to_string())
}
```

### 4.2 SQL/NoSQL Injection Prevention

If using a database, ensure proper parameterization of queries.

```rust
// Example with SQLite (if used)
pub async fn get_user_by_token(token: &str) -> Result<User, DbError> {
    let conn = self.pool.get().await?;
    
    // Use parameterized queries instead of string interpolation
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE api_token = ?")
        .bind(token)
        .fetch_optional(&conn)
        .await?;
    
    user.ok_or(DbError::NotFound)
}
```

### 4.3 XSS Prevention

Sanitize any user input that might be rendered in HTML.

```rust
// If returning HTML content that includes user input
fn sanitize_html(input: &str) -> String {
    ammonia::clean(input)
}
```

## Phase 5: HTTP Method Controls

### 5.1 Method Restriction

Ensure that endpoints only accept the appropriate HTTP methods.

```rust
// In src/http/api.rs
let datasets = warp::path("datasets")
    .and(warp::get().or(warp::post()).unify())
    .and_then(|method: Method| {
        if method == Method::GET {
            Ok(())
        } else if method == Method::POST {
            Ok(())
        } else {
            Err(warp::reject::custom(
                ApiError("Method not allowed".to_string())
            ))
        }
    })
    .and(with_zfs_engine(zfs_engine.clone()))
    .and(api_key_filter.clone())
    .and_then(|method, zfs, _| {
        if method == Method::GET {
            handle_list_datasets(zfs, ())
        } else {
            handle_create_dataset(zfs, ())
        }
    });
```

## Phase 6: Security Monitoring and Logging

### 6.1 Enhanced Logging

Implement comprehensive logging to track security events.

```rust
// In src/http/middleware/logging.rs
pub fn with_security_logging<T: Reply>(
    route_name: &'static str,
) -> impl Filter<Extract = (T,), Error = Rejection> + Clone {
    warp::header::optional::<String>("x-api-key")
        .map(move |api_key: Option<String>| {
            // Record API request with token info (partial token for security)
            let token_info = api_key
                .as_ref()
                .map(|t| format!("{}...{}", &t[0..4], &t[t.len()-4..]))
                .unwrap_or_else(|| "none".to_string());
            
            info!(
                target: "security",
                "API request: route={}, auth={}", 
                route_name, 
                token_info
            );
            
            // Record start time for performance monitoring
            let start = Instant::now();
            (start, route_name, api_key)
        })
        .untuple_one()
        .map(move |reply: T, (start, route_name, api_key): (Instant, &'static str, Option<String>)| {
            // Record completion time
            let duration = start.elapsed();
            let token_info = api_key
                .as_ref()
                .map(|t| format!("{}...{}", &t[0..4], &t[t.len()-4..]))
                .unwrap_or_else(|| "none".to_string());
            
            info!(
                target: "security",
                "API request completed: route={}, auth={}, duration={:?}", 
                route_name, 
                token_info,
                duration
            );
            
            reply
        })
}
```

### 6.2 Rate Limiting

Implement rate limiting to prevent abuse.

```rust
// In src/http/middleware/rate_limit.rs
pub fn with_rate_limit(
    rate_limiter: Arc<RateLimiter>,
) -> impl Filter<Extract = (), Error = Rejection> + Clone {
    warp::addr::remote()
        .and_then(move |addr: Option<SocketAddr>| {
            let rate_limiter = rate_limiter.clone();
            async move {
                match addr {
                    Some(addr) => {
                        let ip = addr.ip().to_string();
                        if rate_limiter.check_rate(&ip).await {
                            Ok(())
                        } else {
                            warn!(
                                target: "security",
                                "Rate limit exceeded for IP: {}", 
                                ip
                            );
                            Err(warp::reject::custom(ApiError("Rate limit exceeded".to_string())))
                        }
                    },
                    None => Ok(()),
                }
            }
        })
}
```

## Implementation Timeline

| Phase | Task | Priority | Estimated Effort |
|-------|------|----------|------------------|
| 1.1   | Apply Authentication Middleware | HIGH | 1 day |
| 1.2   | Public vs. Protected Routes | HIGH | 0.5 day |
| 2.1   | Role-Based Access Control | HIGH | 2 days |
| 3.1   | Dataset and Property Validation | MEDIUM | 2 days |
| 3.2   | Request Size Limits | MEDIUM | 0.5 day |
| 4.1   | Command Injection Prevention | HIGH | 1 day |
| 4.2   | SQL/NoSQL Injection Prevention | MEDIUM | 1 day |
| 4.3   | XSS Prevention | MEDIUM | 1 day |
| 5.1   | Method Restriction | LOW | 1 day |
| 6.1   | Enhanced Logging | MEDIUM | 1 day |
| 6.2   | Rate Limiting | LOW | 1 day |

Total estimated effort: 12 days

## Integration Guidance

1. Implement changes in a separate feature branch
2. Write unit tests for each security feature
3. Perform integration testing to ensure functionality is not broken
4. Update documentation to reflect changes
5. Perform security testing to verify issues are resolved

## Conclusion

This implementation plan provides a comprehensive approach to addressing the security issues identified in the NestGate API. By following this plan, the API will achieve a higher level of security while maintaining functionality and performance.

The most critical issues are addressed in Phases 1 and 2, which focus on authentication and authorization. These should be implemented first to provide a baseline of security before moving on to the other phases. 
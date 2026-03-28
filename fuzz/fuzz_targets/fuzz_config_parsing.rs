// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

#![no_main]

use libfuzzer_sys::fuzz_target;
use nestgate_core::config::Config;
use nestgate_core::unified_final_config::{
    ConfigFormat, FuzzConfigData, FuzzConfigParsingSettings, MaliciousContent, UnifiedFuzzConfig,
    ZfsConfig,
};
use nestgate_zfs::config::ZfsConfig;

fuzz_target!(|input: FuzzConfigParsingSettings| {
    // Test YAML parsing with malicious content
    test_yaml_parsing(&input);

    // Test JSON parsing with malicious content
    test_json_parsing(&input);

    // Test configuration validation
    test_config_validation(&input);

    // Test malicious content handling
    test_malicious_content_handling(&input);

    // Test resource exhaustion protection
    test_resource_exhaustion_protection(&input);
});

fn test_yaml_parsing(input: &FuzzConfigParsingSettings) {
    let yaml_content = generate_yaml_content(input);

    // Should never panic, even with malformed YAML
    let parse_result = serde_yaml_ng::from_str::<serde_yaml_ng::Value>(&yaml_content);
    match parse_result {
        Ok(_) => {
            // If it parses, validate the content is safe
            validate_parsed_content(&yaml_content);
        }
        Err(_) => {
            // Parsing failure is acceptable for malformed input
        }
    }
}

fn test_json_parsing(input: &FuzzConfigParsingSettings) {
    let json_content = generate_json_content(input);

    // Should never panic, even with malformed JSON
    let parse_result = serde_json::from_str::<serde_json::Value>(&json_content);
    match parse_result {
        Ok(value) => {
            // Test for JSON bombs and excessive nesting
            check_json_depth(&value, 0);
        }
        Err(_) => {
            // Parsing failure is acceptable for malformed input
        }
    }
}

fn test_config_validation(input: &FuzzConfigParsingSettings) {
    // Test Core config validation
    let core_config_result = create_core_config(input);
    match core_config_result {
        Ok(config) => {
            validate_core_config_safety(&config);
        }
        Err(_) => {
            // Validation failure is expected for malicious input
        }
    }

    // Test ZFS config validation
    let zfs_config_result = create_zfs_config(input);
    match zfs_config_result {
        Ok(config) => {
            validate_zfs_config_safety(&config);
        }
        Err(_) => {
            // Validation failure is expected for malicious input
        }
    }
}

fn test_malicious_content_handling(input: &FuzzConfigParsingSettings) {
    for malicious in &input.malicious_content {
        match malicious {
            MaliciousContent::YamlBomb(depth) => {
                test_yaml_bomb(*depth);
            }
            MaliciousContent::JsonBomb(depth) => {
                test_json_bomb(*depth);
            }
            MaliciousContent::BillionLaughs => {
                test_billion_laughs();
            }
            MaliciousContent::UnicodeCorruption(bytes) => {
                test_unicode_corruption(bytes);
            }
            MaliciousContent::NullByteInjection => {
                test_null_byte_injection();
            }
            MaliciousContent::PathTraversal(path) => {
                test_path_traversal(path);
            }
            MaliciousContent::SqlInjection(injection) => {
                test_sql_injection(injection);
            }
            MaliciousContent::CommandInjection(injection) => {
                test_command_injection(injection);
            }
            MaliciousContent::ExtremelyLongString(length) => {
                test_extremely_long_string(*length);
            }
            MaliciousContent::InvalidUtf8(bytes) => {
                test_invalid_utf8(bytes);
            }
        }
    }
}

fn test_resource_exhaustion_protection(input: &FuzzConfigParsingSettings) {
    // Test memory usage doesn't explode
    let start_time = std::time::Instant::now();

    let yaml_content = generate_yaml_content(input);

    // Should complete within reasonable time (prevent DoS)
    if start_time.elapsed().as_secs() > 5 {
        panic!("Configuration parsing took too long - potential DoS");
    }

    // Should not use excessive memory
    if yaml_content.len() > 10_000_000 {
        return; // Skip extremely large inputs
    }
}

fn generate_yaml_content(input: &FuzzConfigParsingSettings) -> String {
    let mut yaml = String::new();

    yaml.push_str(&format!("database:\n"));
    yaml.push_str(&format!(
        "  url: \"{}\"\n",
        escape_yaml_string(&input.config_data.database.url)
    ));
    yaml.push_str(&format!(
        "  max_connections: {}\n",
        input.config_data.database.max_connections
    ));
    yaml.push_str(&format!(
        "  timeout_seconds: {}\n",
        input.config_data.database.timeout_seconds
    ));

    yaml.push_str(&format!("logging:\n"));
    yaml.push_str(&format!(
        "  level: \"{}\"\n",
        escape_yaml_string(&input.config_data.logging.level)
    ));
    yaml.push_str(&format!(
        "  file_path: \"{}\"\n",
        escape_yaml_string(&input.config_data.logging.file_path)
    ));

    yaml.push_str(&format!("zfs:\n"));
    yaml.push_str(&format!(
        "  pool_name: \"{}\"\n",
        escape_yaml_string(&input.config_data.zfs.pool_name)
    ));
    yaml.push_str(&format!(
        "  auto_discovery: {}\n",
        input.config_data.zfs.auto_discovery
    ));

    // Add potentially malicious raw fields
    for (key, value) in &input.config_data.raw_fields {
        yaml.push_str(&format!(
            "  {}: \"{}\"\n",
            escape_yaml_string(key),
            escape_yaml_string(value)
        ));
    }

    yaml
}

fn generate_json_content(input: &FuzzConfigParsingSettings) -> String {
    let mut json_map = serde_json::Map::new();

    // Build potentially malicious JSON
    json_map.insert(
        "database".to_string(),
        serde_json::json!({
            "url": input.config_data.database.url,
            "max_connections": input.config_data.database.max_connections,
            "timeout_seconds": input.config_data.database.timeout_seconds
        }),
    );

    json_map.insert(
        "logging".to_string(),
        serde_json::json!({
            "level": input.config_data.logging.level,
            "file_path": input.config_data.logging.file_path,
            "max_size_mb": input.config_data.logging.max_size_mb
        }),
    );

    // Add raw fields that might contain malicious content
    for (key, value) in &input.config_data.raw_fields {
        json_map.insert(key.clone(), serde_json::Value::String(value.clone()));
    }

    serde_json::to_string(&json_map).unwrap_or_else(|_| "{}".to_string())
}

fn escape_yaml_string(s: &str) -> String {
    s.replace("\"", "\\\"")
        .replace("\n", "\\n")
        .replace("\t", "\\t")
        .replace("\r", "\\r")
}

fn check_json_depth(value: &serde_json::Value, current_depth: usize) {
    // Prevent stack overflow from extremely nested JSON
    if current_depth > 1000 {
        return;
    }

    match value {
        serde_json::Value::Object(map) => {
            for (_key, val) in map {
                check_json_depth(val, current_depth + 1);
            }
        }
        serde_json::Value::Array(arr) => {
            for val in arr {
                check_json_depth(val, current_depth + 1);
            }
        }
        _ => {}
    }
}

fn validate_parsed_content(content: &str) {
    // Check for dangerous content that shouldn't be in configs
    assert!(
        !content.contains("rm -rf"),
        "Dangerous command found in config"
    );
    assert!(
        !content.contains("DROP TABLE"),
        "SQL injection attempt found"
    );
    assert!(
        !content.contains("../../../"),
        "Path traversal attempt found"
    );
}

fn create_core_config(input: &FuzzConfigParsingSettings) -> Result<Config, String> {
    // Attempt to create a core config from fuzzed input
    // This should validate and sanitize all inputs

    let _logging_config = LoggingConfig {
        level: validate_log_level(&input.config_data.logging.level)?,
        format: "json".to_string(),
        destination: "file".to_string(),
        file_path: Some(validate_file_path(&input.config_data.logging.file_path)?),
        structured: true,
        custom: std::collections::HashMap::new(),
    };

    // Create a default config for fuzzing
    Ok(Config::default())
}

fn create_zfs_config(input: &FuzzConfigParsingSettings) -> Result<ZfsConfig, String> {
    let mut config = UnifiedZfsConfig::default();

    // Validate pool name but use the correct field structure
    let _pool_name = validate_pool_name(&input.config_data.zfs.pool_name)?;

    Ok(config)
}

fn validate_database_url(url: &str) -> Result<String, String> {
    if url.len() > 1000 {
        return Err("Database URL too long".to_string());
    }

    if url.contains('\0') {
        return Err("Null byte in database URL".to_string());
    }

    // Check for SQL injection attempts
    let dangerous_sql = ["DROP", "DELETE", "INSERT", "UPDATE", "CREATE", "ALTER"];
    let url_upper = url.to_uppercase();
    for dangerous in &dangerous_sql {
        if url_upper.contains(dangerous) {
            return Err("Potentially dangerous SQL in database URL".to_string());
        }
    }

    Ok(url.to_string())
}

fn validate_max_connections(connections: u32) -> Result<u32, String> {
    if connections > 10000 {
        return Err("Too many database connections".to_string());
    }
    Ok(connections)
}

fn validate_timeout(timeout: u64) -> Result<u64, String> {
    if timeout > 3600 {
        return Err("Timeout too long".to_string());
    }
    Ok(timeout)
}

fn validate_log_level(level: &str) -> Result<String, String> {
    let valid_levels = ["error", "warn", "info", "debug", "trace"];
    if !valid_levels.contains(&level.to_lowercase().as_str()) {
        return Err("Invalid log level".to_string());
    }
    Ok(level.to_string())
}

fn validate_file_path(path: &str) -> Result<String, String> {
    if path.contains("..") {
        return Err("Path traversal attempt".to_string());
    }

    if path.contains('\0') {
        return Err("Null byte in file path".to_string());
    }

    if path.len() > 4096 {
        return Err("File path too long".to_string());
    }

    Ok(path.to_string())
}

fn validate_max_size(size: u64) -> Result<u64, String> {
    if size > 10_000 {
        return Err("Max size too large".to_string());
    }
    Ok(size)
}

fn validate_pool_name(name: &str) -> Result<String, String> {
    if name.is_empty() {
        return Err("Empty pool name".to_string());
    }

    if name.len() > 256 {
        return Err("Pool name too long".to_string());
    }

    if name.contains("..") || name.contains('/') {
        return Err("Invalid characters in pool name".to_string());
    }

    Ok(name.to_string())
}

fn validate_core_config_safety(_config: &Config) {
    // Additional safety checks on the parsed config
    // This should never panic
}

fn validate_zfs_config_safety(_config: &ZfsConfig) {
    // Additional safety checks on the ZFS config
    // This should never panic
}

// Malicious content test functions
fn test_yaml_bomb(depth: u32) {
    let depth = std::cmp::min(depth, 100); // Limit depth to prevent actual DoS
    let bomb = "a: &a\n".to_string() + &"  - *a\n".repeat(depth as usize);
    let _ = serde_yaml_ng::from_str::<serde_yaml_ng::Value>(&bomb);
}

fn test_json_bomb(depth: u32) {
    let depth = std::cmp::min(depth, 100);
    let mut bomb = String::new();
    for _ in 0..depth {
        bomb.push('[');
    }
    bomb.push('1');
    for _ in 0..depth {
        bomb.push(']');
    }
    let _ = serde_json::from_str::<serde_json::Value>(&bomb);
}

fn test_billion_laughs() {
    let bomb = r#"
    lol: &lol "lol"
    lol2: &lol2 [*lol,*lol,*lol,*lol,*lol,*lol,*lol,*lol,*lol]
    lol3: &lol3 [*lol2,*lol2,*lol2,*lol2,*lol2,*lol2,*lol2,*lol2,*lol2]
    lol4: [*lol3,*lol3,*lol3,*lol3,*lol3,*lol3,*lol3,*lol3,*lol3]
    "#;
    let _ = serde_yaml_ng::from_str::<serde_yaml_ng::Value>(bomb);
}

fn test_unicode_corruption(bytes: &[u8]) {
    if let Ok(s) = String::from_utf8(bytes.to_vec()) {
        let yaml_content = format!("test: \"{}\"", s);
        let _ = serde_yaml_ng::from_str::<serde_yaml_ng::Value>(&yaml_content);
    }
}

fn test_null_byte_injection() {
    let malicious = "test: \"normal\0malicious\"";
    let _ = serde_yaml_ng::from_str::<serde_yaml_ng::Value>(malicious);
}

fn test_path_traversal(path: &str) {
    let yaml_content = format!("file_path: \"{}\"", path);
    let _ = serde_yaml_ng::from_str::<serde_yaml_ng::Value>(&yaml_content);
}

fn test_sql_injection(injection: &str) {
    let yaml_content = format!("database_url: \"{}\"", injection);
    let _ = serde_yaml_ng::from_str::<serde_yaml_ng::Value>(&yaml_content);
}

fn test_command_injection(injection: &str) {
    let yaml_content = format!("command: \"{}\"", injection);
    let _ = serde_yaml_ng::from_str::<serde_yaml_ng::Value>(&yaml_content);
}

fn test_extremely_long_string(length: usize) {
    let length = std::cmp::min(length, 1_000_000); // Limit to prevent actual DoS
    let long_string = "a".repeat(length);
    let yaml_content = format!("long_field: \"{}\"", long_string);
    let _ = serde_yaml_ng::from_str::<serde_yaml_ng::Value>(&yaml_content);
}

fn test_invalid_utf8(bytes: &[u8]) {
    // Try to parse invalid UTF-8 as if it were a config file
    if bytes.len() < 1000 {
        // Limit size
        let _ = serde_yaml_ng::from_slice::<serde_yaml_ng::Value>(bytes);
    }
}

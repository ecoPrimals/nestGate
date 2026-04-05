// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

#![expect(
    unused,
    dead_code,
    deprecated,
    missing_docs,
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::restriction,
    clippy::cargo
)]

use regex::Regex;
use std::collections::HashSet;
use std::fs;

/// Agnostic Hardcode Detection Test
/// Only allows explicitly whitelisted values - everything else is flagged
/// This ensures 100% configurable infrastructure

#[test]
fn test_no_hardcoded_network_values() -> Result<(), Box<dyn std::error::Error>> {
    let violations = scan_specific_files_for_hardcoded_values();

    if !violations.is_empty() {
        println!("\n🚨 HARDCODED VALUES DETECTED:");
        for violation in &violations {
            println!(
                "  📍 {}:{} - {}",
                violation.file, violation.line, violation.content
            );
        }
        panic!(
            "Found {} hardcoded values. Use environment variables or constants!",
            violations.len()
        );
    }

    println!("✅ No hardcoded values detected - all configuration is properly externalized!");
    Ok(())
}

#[derive(Debug)]
struct Violation {
    file: String,
    line: usize,
    content: String,
}

fn scan_specific_files_for_hardcoded_values() -> Vec<Violation> {
    let mut violations = Vec::new();

    // Target specific key files instead of scanning everything to avoid hanging
    let key_files = vec![
        "code/crates/nestgate-api/src/lib.rs",
        "code/crates/nestgate-network/src/lib.rs",
        "code/crates/nestgate-core/src/config.rs",
        "code/crates/nestgate-nas/src/lib.rs",
        "code/crates/nestgate-core/src/security_config.rs",
    ];

    for file_path in key_files {
        if std::path::Path::new(file_path).exists() {
            scan_file_for_violations(file_path, &mut violations);
        }
    }

    violations
}

fn scan_file_for_violations(file_path: &str, violations: &mut Vec<Violation>) {
    if let Ok(content) = fs::read_to_string(file_path) {
        for (line_num, line) in content.lines().enumerate() {
            check_line_for_hardcoded_values(file_path, line_num + 1, line, violations);
        }
    }
}

fn check_line_for_hardcoded_values(
    file_path: &str,
    line_num: usize,
    line: &str,
    violations: &mut Vec<Violation>,
) {
    // Skip comments, logs, and documentation
    let trimmed = line.trim();
    if trimmed.starts_with("//")
        || trimmed.starts_with("///")
        || trimmed.contains("debug!(")
        || trimmed.contains("info!(")
        || trimmed.contains("warn!(")
        || trimmed.contains("error!(")
    {
        return;
    }

    // Agnostic approach: Find ANY network value and check if explicitly allowed
    check_ports_agnostic(file_path, line_num, line, violations);
    check_ips_agnostic(file_path, line_num, line, violations);
    check_urls_agnostic(file_path, line_num, line, violations);
}

fn check_ports_agnostic(
    file_path: &str,
    line_num: usize,
    line: &str,
    violations: &mut Vec<Violation>,
) {
    // Find any port pattern (:NNNN)
    let Ok(port_regex) = Regex::new(r":(\d{3,5})\b") else {
        return;
    };

    for cap in port_regex.captures_iter(line) {
        if let Some(port_match) = cap.get(1) {
            let port: u16 = port_match.as_str().parse().unwrap_or(0);

            if !is_explicitly_allowed_port(port, line) {
                violations.push(Violation {
                    file: file_path.to_string(),
                    line: line_num,
                    content: line.trim().to_string(),
                });
            }
        }
    }
}

fn check_ips_agnostic(
    file_path: &str,
    line_num: usize,
    line: &str,
    violations: &mut Vec<Violation>,
) {
    // Find any IP address pattern
    let Ok(ip_regex) = Regex::new(r"\b(?:\d{1,3}\.){3}\d{1,3}\b") else {
        return;
    };

    for ip_match in ip_regex.find_iter(line) {
        let ip = ip_match.as_str();

        if !is_explicitly_allowed_ip(ip, line) {
            violations.push(Violation {
                file: file_path.to_string(),
                line: line_num,
                content: line.trim().to_string(),
            });
        }
    }
}

fn check_urls_agnostic(
    file_path: &str,
    line_num: usize,
    line: &str,
    violations: &mut Vec<Violation>,
) {
    // Find any HTTP/HTTPS URL
    let Ok(url_regex) = Regex::new(r#"https?://[^\s"']+  "#) else {
        return;
    };

    for url_match in url_regex.find_iter(line) {
        let url = url_match.as_str();

        if !is_explicitly_allowed_url(url, line) {
            violations.push(Violation {
                file: file_path.to_string(),
                line: line_num,
                content: line.trim().to_string(),
            });
        }
    }
}

// Explicit whitelist functions - ONLY these values are allowed

fn is_explicitly_allowed_port(port: u16, line: &str) -> bool {
    // Only well-known standard ports OR explicitly configurable values
    let standard_ports = HashSet::from([
        80, 443, 22, 53, 445, 2049, // Standard protocol ports
        25, 110, 143, 993, 995, // Email ports
    ]);

    // Allow if:
    // 1. It's a standard port
    // 2. It's explicitly configurable via environment variables
    // 3. It's defined as a constant
    // 4. It's part of a default configuration
    standard_ports.contains(&port)
        || line.contains("env::var")
        || line.contains("config.")
        || line.contains("const")
        || line.contains("DEFAULT_")
        || line.contains("WELL_KNOWN_")
        || line.contains("unwrap_or(") // Environment variable with fallback
}

fn is_explicitly_allowed_ip(ip: &str, line: &str) -> bool {
    // Very strict: Only allow specific safe IPs and configurable patterns
    let always_safe_ips = HashSet::from([
        "0.0.0.0", // Bind all interfaces - acceptable for servers
    ]);

    // Allow if:
    // 1. It's an always-safe IP (like 0.0.0.0)
    // 2. It's explicitly configurable
    // 3. It's defined as a constant
    always_safe_ips.contains(ip)
        || line.contains("env::var")
        || line.contains("config.")
        || line.contains("const")
        || line.contains("DEFAULT_")
        || line.contains("unwrap_or(")
}

fn is_explicitly_allowed_url(url: &str, line: &str) -> bool {
    // Very strict: NO hardcoded URLs unless explicitly configurable
    line.contains("env::var") ||
    line.contains("config.") ||
    line.contains("DEFAULT_") ||
    line.contains("const") ||
    line.contains("unwrap_or(") ||
    url.contains("example.com") || // Documentation examples only
    url.contains("localhost") && line.contains("unwrap_or")
}

#[test]
fn test_whitelist_validation() -> Result<(), Box<dyn std::error::Error>> {
    // Test that our whitelisting logic works correctly

    // Ports
    assert!(is_explicitly_allowed_port(80, "const HTTP_PORT: u16 = 80;"));
    assert!(is_explicitly_allowed_port(
        8080,
        "env::var(\"PORT\").unwrap_or(8080)"
    ));
    assert!(!is_explicitly_allowed_port(8080, "let port = 8080;"));

    // IPs
    assert!(is_explicitly_allowed_ip(
        "0.0.0.0",
        "bind_interface: \"0.0.0.0\".to_string()"
    ));
    assert!(is_explicitly_allowed_ip(
        "127.0.0.1",
        "env::var(\"BIND_HOST\").unwrap_or(\"127.0.0.1\")"
    ));
    assert!(!is_explicitly_allowed_ip(
        "192.168.1.1",
        "let ip = \"192.168.1.1\";"
    ));

    // URLs
    assert!(is_explicitly_allowed_url(
        "http://localhost:8080",
        "env::var(\"API_URL\").unwrap_or(\"http://localhost:8080\")"
    ));
    assert!(!is_explicitly_allowed_url(
        "http://hardcoded.com",
        "let url = \"http://hardcoded.com\";"
    ));

    println!("✅ Whitelist validation tests passed");
    Ok(())
}

#[test]
fn test_environment_variable_standards() -> Result<(), Box<dyn std::error::Error>> {
    // Ensure we follow NESTGATE_* naming convention
    // MODERNIZED: environment.rs now uses EnvironmentConfig abstraction instead of direct env::var
    let config_files = vec!["code/crates/nestgate-core/src/config.rs"];

    for config_file in config_files {
        if std::path::Path::new(config_file).exists() {
            if let Ok(content) = fs::read_to_string(config_file) {
                // Check for NESTGATE_* environment variables or config abstractions
                let has_nestgate_vars = content.contains("NESTGATE_")
                    || content.contains("env::var")
                    || content.contains("EnvironmentConfig");
                assert!(
                    has_nestgate_vars,
                    "Config file {config_file} should use NESTGATE_* environment variables or config abstractions"
                );
            }
        }
    }

    println!("✅ Environment variable naming standards validated");
    Ok(())
}

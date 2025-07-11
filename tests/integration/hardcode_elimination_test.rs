use std::collections::HashSet;
use std::fs;
use regex::Regex;

#[test]
fn test_no_hardcoded_network_values() {
    let violations = scan_specific_files_for_hardcoded_values();

    if !violations.is_empty() {
        println!("\n🚨 HARDCODED VALUES DETECTED:");
        for violation in &violations {
            println!("  📍 {}:{} - {}", violation.file, violation.line, violation.content);
        }
        panic!("Found {} hardcoded values. Use environment variables or constants!", violations.len());
    }

    println!("✅ No hardcoded values detected - all configuration is properly externalized!");
}

#[derive(Debug)]
struct Violation {
    file: String,
    line: usize,
    content: String,
}

fn scan_specific_files_for_hardcoded_values() -> Vec<Violation> {
    let mut violations = Vec::new();

    // Target specific key files instead of scanning everything
    let key_files = vec![
        "code/crates/nestgate-api/src/lib.rs",
        "code/crates/nestgate-network/src/lib.rs",
        "code/crates/nestgate-core/src/config.rs",
        "code/crates/nestgate-nas/src/lib.rs",
        "code/crates/nestgate-network/src/songbird.rs",
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

fn check_line_for_hardcoded_values(file_path: &str, line_num: usize, line: &str, violations: &mut Vec<Violation>) {
    // Skip comments and logs
    let trimmed = line.trim();
    if trimmed.starts_with("//") || trimmed.contains("debug!(") || trimmed.contains("info!(") {
        return;
    }

    // Agnostic approach: Find ANY port and check if explicitly allowed
    check_ports_agnostic(file_path, line_num, line, violations);
    check_ips_agnostic(file_path, line_num, line, violations);
    check_urls_agnostic(file_path, line_num, line, violations);
}

fn check_ports_agnostic(file_path: &str, line_num: usize, line: &str, violations: &mut Vec<Violation>) {
    let port_regex = Regex::new(r":(\d{3,5})\b").unwrap();

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

fn check_ips_agnostic(file_path: &str, line_num: usize, line: &str, violations: &mut Vec<Violation>) {
    let ip_regex = Regex::new(r"\b(?:\d{1,3}\.){3}\d{1,3}\b").unwrap();

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

fn check_urls_agnostic(file_path: &str, line_num: usize, line: &str, violations: &mut Vec<Violation>) {
    let url_regex = Regex::new(r"https?://[^\s\"']+").unwrap();

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

// Explicit whitelist - ONLY these are allowed

fn is_explicitly_allowed_port(port: u16, line: &str) -> bool {
    // Only well-known standard ports OR configurable values
    let standard_ports = HashSet::from([80, 443, 22, 53, 445, 2049]);

    standard_ports.contains(&port) ||
    line.contains("env::var") ||
    line.contains("config.") ||
    line.contains("const") ||
    line.contains("DEFAULT_")
}

fn is_explicitly_allowed_ip(ip: &str, line: &str) -> bool {
    // Only allow 0.0.0.0 and configurable IPs
    ip == "0.0.0.0" ||
    line.contains("env::var") ||
    line.contains("config.") ||
    line.contains("const") ||
    line.contains("DEFAULT_")
}

fn is_explicitly_allowed_url(url: &str, line: &str) -> bool {
    // NO hardcoded URLs unless configurable
    line.contains("env::var") ||
    line.contains("config.") ||
    line.contains("DEFAULT_") ||
    url.contains("example.com")
}

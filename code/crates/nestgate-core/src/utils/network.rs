/// Network Utilities
/// Network operations, IP validation, hostname checking, and related functions
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

use crate::error::{NestGateError, Result};

// ==================== IP ADDRESS VALIDATION ====================

/// Check if a string is a valid IP address (IPv4 or IPv6)
pub fn is_valid_ip(ip: &str) -> bool {
    ip.parse::<IpAddr>().is_ok()
}

/// Check if a string is a valid IPv4 address
pub fn is_valid_ipv4(ip: &str) -> bool {
    ip.parse::<Ipv4Addr>().is_ok()
}

/// Check if a string is a valid IPv6 address
pub fn is_valid_ipv6(ip: &str) -> bool {
    ip.parse::<Ipv6Addr>().is_ok()
}

/// Parse an IP address string to IpAddr
pub fn parse_ip(ip: &str) -> Result<IpAddr> {
    ip.parse::<IpAddr>().map_err(|_| NestGateError::Validation {
        field: "ip_address".to_string(),
        message: format!("Invalid IP address: {ip}"),
        current_value: Some(ip.to_string()),
        expected: Some("Valid IPv4 or IPv6 address".to_string()),
        user_error: true,
    })
}

/// Parse an IPv4 address string
pub fn parse_ipv4(ip: &str) -> Result<Ipv4Addr> {
    ip.parse::<Ipv4Addr>()
        .map_err(|_| NestGateError::Validation {
            field: "ipv4_address".to_string(),
            message: format!("Invalid IPv4 address: {ip}"),
            current_value: Some(ip.to_string()),
            expected: Some("Valid IPv4 address (e.g., 192.168.1.1)".to_string()),
            user_error: true,
        })
}

/// Parse an IPv6 address string
pub fn parse_ipv6(ip: &str) -> Result<Ipv6Addr> {
    ip.parse::<Ipv6Addr>()
        .map_err(|_| NestGateError::Validation {
            field: "ipv6_address".to_string(),
            message: format!("Invalid IPv6 address: {ip}"),
            current_value: Some(ip.to_string()),
            expected: Some("Valid IPv6 address (e.g., ::1)".to_string()),
            user_error: true,
        })
}

// ==================== CIDR VALIDATION ====================

/// Check if a string is a valid CIDR notation
pub fn is_valid_cidr(cidr: &str) -> bool {
    parse_cidr(cidr).is_ok()
}

/// Parse CIDR notation (e.g., "192.168.1.0/24")
pub fn parse_cidr(input: &str) -> Result<(IpAddr, u8)> {
    let parts: Vec<&str> = input.split('/').collect();
    if parts.len() != 2 {
        return Err(NestGateError::Validation {
            field: "cidr".to_string(),
            message: format!("Invalid CIDR format: {input}"),
            current_value: Some(input.to_string()),
            expected: Some("IP/prefix format (e.g., 192.168.1.0/24)".to_string()),
            user_error: true,
        });
    }

    let ip = parts[0]
        .parse::<IpAddr>()
        .map_err(|_| NestGateError::Validation {
            field: "ip".to_string(),
            message: format!("Invalid IP address: {}", parts[0]),
            current_value: Some(parts[0].to_string()),
            expected: Some("Valid IP address".to_string()),
            user_error: true,
        })?;

    let prefix = parts[1]
        .parse::<u8>()
        .map_err(|_| NestGateError::Validation {
            field: "prefix".to_string(),
            message: format!("Invalid prefix length: {}", parts[1]),
            current_value: Some(parts[1].to_string()),
            expected: Some("Number between 0-32 (IPv4) or 0-128 (IPv6)".to_string()),
            user_error: true,
        })?;

    // Validate prefix length based on IP address type
    match ip {
        IpAddr::V4(_) if prefix > 32 => {
            return Err(NestGateError::Validation {
                field: "prefix".to_string(),
                message: format!("Invalid IPv4 prefix length: {prefix}"),
                current_value: Some(prefix.to_string()),
                expected: Some("0-32".to_string()),
                user_error: true,
            });
        }
        IpAddr::V6(_) if prefix > 128 => {
            return Err(NestGateError::Validation {
                field: "prefix".to_string(),
                message: format!("Invalid IPv6 prefix length: {prefix}"),
                current_value: Some(prefix.to_string()),
                expected: Some("0-128".to_string()),
                user_error: true,
            });
        }
        _ => {}
    }

    Ok((ip, prefix))
}

// ==================== HOSTNAME VALIDATION ====================

/// Check if a hostname is valid
pub fn is_valid_hostname(hostname: &str) -> bool {
    if hostname.is_empty() || hostname.len() > 253 {
        return false;
    }

    // Split into labels
    let labels: Vec<&str> = hostname.split('.').collect();

    for label in labels {
        if !is_valid_hostname_label(label) {
            return false;
        }
    }

    true
}

/// Check if a single hostname label is valid
fn is_valid_hostname_label(label: &str) -> bool {
    if label.is_empty() || label.len() > 63 {
        return false;
    }

    if label.starts_with('-') || label.ends_with('-') {
        return false;
    }

    label.chars().all(|c| c.is_ascii_alphanumeric() || c == '-')
}

/// Check if a domain name is valid
pub fn is_valid_domain(domain: &str) -> bool {
    if domain.is_empty() || domain.len() > 253 {
        return false;
    }

    // Domain must contain at least one dot
    if !domain.contains('.') {
        return false;
    }

    is_valid_hostname(domain)
}

// ==================== PORT VALIDATION ====================

/// Check if a port number is valid (1-65535)
pub fn is_valid_port(port: u16) -> bool {
    port > 0
}

/// Check if a port is in the well-known range (1-1023)
pub fn is_well_known_port(port: u16) -> bool {
    port > 0 && port <= 1023
}

/// Check if a port is in the registered range (1024-49151)
pub fn is_registered_port(port: u16) -> bool {
    (1024..=49151).contains(&port)
}

/// Check if a port is in the dynamic/private range (49152-65535)
pub fn is_dynamic_port(port: u16) -> bool {
    (49152..=65535).contains(&port)
}

/// Check if a port is available by attempting to bind to it
pub async fn is_port_available(port: u16) -> bool {
    let addr = format!("127.0.0.1:{port}");
    tokio::net::TcpListener::bind(&addr).await.is_ok()
}

/// Find an available port starting from a given port
pub async fn find_available_port(start_port: u16) -> Option<u16> {
    for port in start_port..=65535 {
        if is_port_available(port).await {
            return Some(port);
        }
    }
    None
}

// ==================== URL VALIDATION ====================

/// Check if a string is a valid URL
pub fn is_valid_url(url: &str) -> bool {
    url::Url::parse(url).is_ok()
}

/// Check if a string is a valid HTTP/HTTPS URL
pub fn is_valid_http_url(url: &str) -> bool {
    if let Ok(parsed) = url::Url::parse(url) {
        matches!(parsed.scheme(), "http" | "https")
    } else {
        false
    }
}

/// Parse a URL and return its components
pub fn parse_url(url: &str) -> Result<url::Url> {
    url::Url::parse(url).map_err(|e| NestGateError::Validation {
        field: "url".to_string(),
        message: format!("Invalid URL: {e}"),
        current_value: Some(url.to_string()),
        expected: Some("Valid URL format".to_string()),
        user_error: true,
    })
}

// ==================== MAC ADDRESS VALIDATION ====================

/// Check if a string is a valid MAC address
pub fn is_valid_mac_address(mac: &str) -> bool {
    let parts: Vec<&str> = mac.split(':').collect();
    if parts.len() != 6 {
        return false;
    }

    parts
        .iter()
        .all(|part| part.len() == 2 && part.chars().all(|c| c.is_ascii_hexdigit()))
}

/// Normalize MAC address format (convert to lowercase with colons)
pub fn normalize_mac_address(mac: &str) -> Option<String> {
    let cleaned: String = mac.chars().filter(|c| c.is_ascii_hexdigit()).collect();

    if cleaned.len() != 12 {
        return None;
    }

    let normalized = cleaned
        .chars()
        .enumerate()
        .map(|(i, c)| {
            if i > 0 && i % 2 == 0 {
                format!(":{}", c.to_ascii_lowercase())
            } else {
                c.to_ascii_lowercase().to_string()
            }
        })
        .collect::<String>();

    Some(normalized)
}

// ==================== NETWORK UTILITIES ====================

/// Check if an IP address is in a private range
pub fn is_private_ip(ip: &IpAddr) -> bool {
    match ip {
        IpAddr::V4(ipv4) => {
            let octets = ipv4.octets();
            // 10.0.0.0/8
            octets[0] == 10
            // 172.16.0.0/12
            || (octets[0] == 172 && (octets[1] >= 16 && octets[1] <= 31))
            // 192.168.0.0/16
            || (octets[0] == 192 && octets[1] == 168)
        }
        IpAddr::V6(ipv6) => {
            // IPv6 unique local addresses (fc00::/7)
            let segments = ipv6.segments();
            (segments[0] & 0xfe00) == 0xfc00
        }
    }
}

/// Check if an IP address is a loopback address
pub fn is_loopback_ip(ip: &IpAddr) -> bool {
    match ip {
        IpAddr::V4(ipv4) => ipv4.is_loopback(),
        IpAddr::V6(ipv6) => ipv6.is_loopback(),
    }
}

/// Check if an IP address is a multicast address
pub fn is_multicast_ip(ip: &IpAddr) -> bool {
    match ip {
        IpAddr::V4(ipv4) => ipv4.is_multicast(),
        IpAddr::V6(ipv6) => ipv6.is_multicast(),
    }
}

/// Get the localhost IP address as a string
pub fn localhost() -> &'static str {
    "127.0.0.1"
}

/// Get the IPv6 localhost address as a string
pub fn localhost_ipv6() -> &'static str {
    "::1"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ip_validation() {
        // Valid IPs
        assert!(is_valid_ip("192.168.1.1"));
        assert!(is_valid_ip("::1"));
        assert!(is_valid_ipv4("192.168.1.1"));
        assert!(is_valid_ipv6("::1"));

        // Invalid IPs
        assert!(!is_valid_ip("256.256.256.256"));
        assert!(!is_valid_ip("invalid"));
        assert!(!is_valid_ipv4("::1"));
        assert!(!is_valid_ipv6("192.168.1.1"));
    }

    #[test]
    fn test_cidr_validation() {
        // Valid CIDR
        assert!(is_valid_cidr("192.168.1.0/24"));
        assert!(is_valid_cidr("10.0.0.0/8"));
        assert!(is_valid_cidr("::1/128"));

        // Invalid CIDR
        assert!(!is_valid_cidr("192.168.1.0"));
        assert!(!is_valid_cidr("192.168.1.0/33"));
        assert!(!is_valid_cidr("invalid/24"));
    }

    #[test]
    fn test_hostname_validation() {
        // Valid hostnames
        assert!(is_valid_hostname("example.com"));
        assert!(is_valid_hostname("test-host"));
        assert!(is_valid_hostname("sub.example.com"));

        // Invalid hostnames
        assert!(!is_valid_hostname(""));
        assert!(!is_valid_hostname("-example"));
        assert!(!is_valid_hostname("example-"));
        assert!(!is_valid_hostname("ex--ample"));
    }

    #[test]
    fn test_port_validation() {
        // Valid ports
        assert!(is_valid_port(80));
        assert!(is_valid_port(8080));
        assert!(is_valid_port(65535));

        // Invalid ports
        assert!(!is_valid_port(0));

        // Port ranges
        assert!(is_well_known_port(80));
        assert!(is_registered_port(8080));
        assert!(is_dynamic_port(50000));
    }

    #[test]
    fn test_url_validation() {
        // Valid URLs
        assert!(is_valid_url("https://example.com"));
        assert!(is_valid_url("http://localhost:8080"));
        assert!(is_valid_http_url("https://example.com"));

        // Invalid URLs
        assert!(!is_valid_url("not-a-url"));
        assert!(!is_valid_http_url("ftp://example.com"));
    }

    #[test]
    fn test_mac_address_validation() {
        // Valid MAC addresses
        assert!(is_valid_mac_address("00:11:22:33:44:55"));
        assert!(is_valid_mac_address("AB:CD:EF:12:34:56"));

        // Invalid MAC addresses
        assert!(!is_valid_mac_address("00:11:22:33:44"));
        assert!(!is_valid_mac_address("00-11-22-33-44-55"));
        assert!(!is_valid_mac_address("invalid"));

        // MAC address normalization
        assert_eq!(
            normalize_mac_address("00-11-22-33-44-55"),
            Some("00:11:22:33:44:55".to_string())
        );
        assert_eq!(
            normalize_mac_address("001122334455"),
            Some("00:11:22:33:44:55".to_string())
        );
    }

    #[test]
    fn test_private_ip_detection() {
        // Private IPv4 addresses
        assert!(is_private_ip(&"10.0.0.1".parse().unwrap()));
        assert!(is_private_ip(&"172.16.0.1".parse().unwrap()));
        assert!(is_private_ip(&"192.168.1.1".parse().unwrap()));

        // Public IPv4 addresses
        assert!(!is_private_ip(&"8.8.8.8".parse().unwrap()));
        assert!(!is_private_ip(&"1.1.1.1".parse().unwrap()));

        // Loopback addresses
        assert!(is_loopback_ip(&"127.0.0.1".parse().unwrap()));
        assert!(is_loopback_ip(&"::1".parse().unwrap()));
    }
}

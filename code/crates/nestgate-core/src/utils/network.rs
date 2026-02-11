/// Network Utilities
/// Network operations, IP validation, hostname checking, and related functions
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use crate::{NestGateError, Result};

// ==================== SECTION ====================

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
    ip.parse::<IpAddr>().map_err(|_| NestGateError::validation_error(
        &format!("Invalid IP address: '{}'", ip)
    ))
}
/// Parse an IPv4 address string
pub fn parse_ipv4(ip: &str) -> Result<Ipv4Addr> {
    ip.parse::<Ipv4Addr>()
        .map_err(|_| NestGateError::validation_error(
            &format!("Invalid IPv4 address: '{}' (expected format: 192.168.1.1)", ip)
        ))
}
/// Parse an IPv6 address string
pub fn parse_ipv6(ip: &str) -> Result<Ipv6Addr> {
    ip.parse::<Ipv6Addr>()
        .map_err(|_| NestGateError::validation_error(
            &format!("Invalid IPv6 address: '{}' (expected format: ::1)", ip)
        ))
}
// ==================== SECTION ====================

/// Check if a string is a valid CIDR notation
pub fn is_valid_cidr(cidr: &str) -> bool {
    parse_cidr(cidr).is_ok()
}
/// Parse CIDR notation (e.g., "192.168.1.0/24")
pub fn parse_cidr(input: &str) -> Result<(IpAddr, u8)> {
    let parts: Vec<&str> = input.split('/').collect();
    if parts.len() != 2 {
        return Err(NestGateError::validation_error(
            &format!("Invalid CIDR notation: '{}' (expected format: 192.168.1.0/24)", input)
        ));
    }
    let ip = parts[0]
        .parse::<IpAddr>()
        .map_err(|_| NestGateError::validation_error(
            &format!("Invalid IP address in CIDR: '{}'", parts[0])
        ))?;

    let prefix = parts[1]
        .parse::<u8>()
        .map_err(|_| NestGateError::validation_error(
            &format!("Invalid prefix length in CIDR: '{}'", parts[1])
        ))?;

    // Validate prefix length based on IP address type
    match ip {
        IpAddr::V4(_) if prefix > 32 => {
            return Err(NestGateError::validation_error(
                &format!("Invalid IPv4 prefix length: {} (max: 32)", prefix)
            ));
        }
        IpAddr::V6(_) if prefix > 128 => {
            return Err(NestGateError::validation_error(
                &format!("Invalid IPv6 prefix length: {} (max: 128)", prefix)
            ));
        }
        _ => {}
    }

    Ok((ip, prefix))
}

// ==================== SECTION ====================

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

// ==================== SECTION ====================

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
    use crate::constants::hardcoding::addresses;
    let addr = format!("{}:{}", addresses::LOCALHOST_IPV4, port);
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
// ==================== SECTION ====================

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
    url::Url::parse(url).map_err(|e| NestGateError::validation_error(
        &format!("Invalid URL: '{}' - {}", url, e)
    ))
}
// ==================== SECTION ====================

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
            let lower = c.to_ascii_lowercase();
            if i > 0 && i % 2 == 0 {
                format!(":{}", lower)
            } else {
                lower.to_string()
            }
        })
        .collect::<String>();

    Some(normalized)
}

// ==================== SECTION ====================

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
    use crate::constants::hardcoding::addresses;
    addresses::LOCALHOST_IPV4
}
/// Get the IPv6 localhost address as a string
pub fn localhost_ipv6() -> &'static str {
    "::1"  // IPv6 localhost - standard, not configurable
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
        assert!(is_valid_url("http://localhost:18080"));
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
        assert!(is_private_ip(&"10.0.0.1".parse().expect("Network operation failed")));
        assert!(is_private_ip(&"172.16.0.1".parse().expect("Network operation failed")));
        assert!(is_private_ip(&"192.168.1.1".parse().expect("Network operation failed")));

        // Public IPv4 addresses
        assert!(!is_private_ip(&"8.8.8.8".parse().expect("Network operation failed")));
        assert!(!is_private_ip(&"1.1.1.1".parse().expect("Network operation failed")));

        // Loopback addresses
        assert!(is_loopback_ip(&"127.0.0.1".parse().expect("Network operation failed")));
        assert!(is_loopback_ip(&"::1".parse().expect("Network operation failed")));
    }
    
    // **COMPREHENSIVE NETWORK UTILITIES TESTS** (Added Nov 3, 2025)
    
    #[test]
    fn test_ip_parsing_success() {
        // IPv4 parsing
        let ipv4 = parse_ipv4("192.168.1.100").expect("Should parse valid IPv4");
        assert_eq!(ipv4.to_string(), "192.168.1.100");
        
        // IPv6 parsing
        let ipv6 = parse_ipv6("2001:db8::1").expect("Should parse valid IPv6");
        assert_eq!(ipv6.to_string(), "2001:db8::1");
        
        // Generic IP parsing
        let ip_v4 = parse_ip("10.0.0.1").expect("Should parse IPv4 as IP");
        assert!(matches!(ip_v4, IpAddr::V4(_)));
        
        let ip_v6 = parse_ip("::1").expect("Should parse IPv6 as IP");
        assert!(matches!(ip_v6, IpAddr::V6(_)));
    }
    
    #[test]
    fn test_ip_parsing_errors() {
        // Invalid IPv4
        assert!(parse_ipv4("256.256.256.256").is_err());
        assert!(parse_ipv4("192.168.1").is_err());
        assert!(parse_ipv4("not-an-ip").is_err());
        assert!(parse_ipv4("::1").is_err()); // IPv6 as IPv4
        
        // Invalid IPv6
        assert!(parse_ipv6("192.168.1.1").is_err()); // IPv4 as IPv6
        assert!(parse_ipv6("gggg::1").is_err());
        assert!(parse_ipv6("not-an-ip").is_err());
        
        // Invalid generic IP
        assert!(parse_ip("invalid").is_err());
        assert!(parse_ip("999.999.999.999").is_err());
    }
    
    #[test]
    fn test_cidr_parsing_success() {
        // IPv4 CIDR
        let (ip, prefix) = parse_cidr("192.168.1.0/24").expect("Should parse IPv4 CIDR");
        assert_eq!(ip.to_string(), "192.168.1.0");
        assert_eq!(prefix, 24);
        
        // IPv6 CIDR
        let (ip6, prefix6) = parse_cidr("2001:db8::/32").expect("Should parse IPv6 CIDR");
        assert_eq!(ip6.to_string(), "2001:db8::");
        assert_eq!(prefix6, 32);
        
        // Edge cases - minimum and maximum prefix lengths
        assert!(parse_cidr("10.0.0.0/0").is_ok());
        assert!(parse_cidr("10.0.0.0/32").is_ok());
        assert!(parse_cidr("::1/0").is_ok());
        assert!(parse_cidr("::1/128").is_ok());
    }
    
    #[test]
    fn test_cidr_parsing_errors() {
        // Missing prefix
        assert!(parse_cidr("192.168.1.0").is_err());
        
        // Invalid prefix for IPv4
        assert!(parse_cidr("192.168.1.0/33").is_err());
        assert!(parse_cidr("192.168.1.0/256").is_err());
        
        // Invalid prefix for IPv6
        assert!(parse_cidr("::1/129").is_err());
        assert!(parse_cidr("::1/200").is_err());
        
        // Completely invalid
        assert!(parse_cidr("not-a-cidr/24").is_err());
        assert!(parse_cidr("192.168.1.0/invalid").is_err());
        
        // Multiple slashes
        assert!(parse_cidr("192.168.1.0/24/32").is_err());
    }
    
    #[test]
    fn test_hostname_label_validation() {
        // Valid labels
        assert!(is_valid_hostname("localhost"));
        assert!(is_valid_hostname("example"));
        assert!(is_valid_hostname("my-server"));
        assert!(is_valid_hostname("server123"));
        
        // Invalid - starts with hyphen
        assert!(!is_valid_hostname("-server"));
        
        // Invalid - ends with hyphen
        assert!(!is_valid_hostname("server-"));
        
        // Invalid - double hyphen (based on test at line 297)
        assert!(!is_valid_hostname("ex--ample"));
    }
    
    #[test]
    fn test_hostname_length_limits() {
        // Valid - max label length (63 characters)
        let max_label = "a".repeat(63);
        assert!(is_valid_hostname(&max_label));
        
        // Invalid - label too long (64 characters)
        let too_long_label = "a".repeat(64);
        assert!(!is_valid_hostname(&too_long_label));
        
        // Valid - max total length (253 characters)
        let max_hostname = format!("{}.{}.{}.com", "a".repeat(60), "b".repeat(60), "c".repeat(60));
        // Should be under 253
        if max_hostname.len() <= 253 {
            assert!(is_valid_hostname(&max_hostname));
        }
        
        // Invalid - empty hostname
        assert!(!is_valid_hostname(""));
    }
    
    #[test]
    fn test_domain_validation() {
        // Valid domains
        assert!(is_valid_domain("example.com"));
        assert!(is_valid_domain("sub.example.com"));
        assert!(is_valid_domain("deep.sub.example.com"));
        
        // Invalid - no dot
        assert!(!is_valid_domain("localhost"));
        assert!(!is_valid_domain("example"));
        
        // Invalid - empty
        assert!(!is_valid_domain(""));
        
        // Invalid - too long
        let too_long = "a".repeat(254);
        assert!(!is_valid_domain(&too_long));
    }
    
    #[test]
    fn test_port_ranges() {
        // Well-known ports (1-1023)
        assert!(is_well_known_port(1));
        assert!(is_well_known_port(80));
        assert!(is_well_known_port(443));
        assert!(is_well_known_port(1023));
        assert!(!is_well_known_port(0));
        assert!(!is_well_known_port(1024));
        
        // Registered ports (1024-49151)
        assert!(is_registered_port(1024));
        assert!(is_registered_port(8080));
        assert!(is_registered_port(49151));
        assert!(!is_registered_port(1023));
        assert!(!is_registered_port(49152));
        
        // Dynamic ports (49152-65535)
        assert!(is_dynamic_port(49152));
        assert!(is_dynamic_port(50000));
        assert!(is_dynamic_port(65535));
        assert!(!is_dynamic_port(49151));
    }
    
    #[test]
    fn test_url_parsing() {
        // Valid HTTP URLs
        let url1 = parse_url("https://example.com").expect("Should parse HTTPS URL");
        assert_eq!(url1.scheme(), "https");
        assert_eq!(url1.host_str(), Some("example.com"));
        
        // Valid URL with port
        let url2 = parse_url("http://localhost:18080").expect("Should parse URL with port");
        assert_eq!(url2.port(), Some(18080));
        
        // Valid URL with path
        let url3 = parse_url("https://example.com/path/to/resource").expect("Should parse URL with path");
        assert_eq!(url3.path(), "/path/to/resource");
        
        // Invalid URLs
        assert!(parse_url("not-a-url").is_err());
        assert!(parse_url("://invalid").is_err());
    }
    
    #[test]
    fn test_http_url_filtering() {
        // Valid HTTP/HTTPS
        assert!(is_valid_http_url("http://example.com"));
        assert!(is_valid_http_url("https://example.com"));
        
        // Invalid - other schemes
        assert!(!is_valid_http_url("ftp://example.com"));
        assert!(!is_valid_http_url("ws://example.com"));
        assert!(!is_valid_http_url("file:///path/to/file"));
        
        // Invalid - not a URL
        assert!(!is_valid_http_url("not-a-url"));
    }
    
    #[test]
    fn test_mac_address_edge_cases() {
        // Valid formats
        assert!(is_valid_mac_address("00:00:00:00:00:00"));
        assert!(is_valid_mac_address("FF:FF:FF:FF:FF:FF"));
        assert!(is_valid_mac_address("aA:bB:cC:dD:eE:fF")); // Mixed case
        
        // Invalid - wrong separator
        assert!(!is_valid_mac_address("00-11-22-33-44-55"));
        
        // Invalid - wrong length
        assert!(!is_valid_mac_address("00:11:22:33:44"));
        assert!(!is_valid_mac_address("00:11:22:33:44:55:66"));
        
        // Invalid - non-hex characters
        assert!(!is_valid_mac_address("GG:HH:II:JJ:KK:LL"));
        assert!(!is_valid_mac_address("00:11:22:33:44:ZZ"));
    }
    
    #[test]
    fn test_mac_address_normalization() {
        // Normalize with different separators
        assert_eq!(
            normalize_mac_address("00-11-22-33-44-55"),
            Some("00:11:22:33:44:55".to_string())
        );
        
        // Normalize without separators
        assert_eq!(
            normalize_mac_address("001122334455"),
            Some("00:11:22:33:44:55".to_string())
        );
        
        // Normalize with mixed case
        assert_eq!(
            normalize_mac_address("AA:BB:CC:DD:EE:FF"),
            Some("aa:bb:cc:dd:ee:ff".to_string())
        );
        
        // Invalid - too short
        assert_eq!(normalize_mac_address("0011223344"), None);
        
        // Invalid - too long
        assert_eq!(normalize_mac_address("00112233445566"), None);
        
        // Invalid - non-hex
        assert_eq!(normalize_mac_address("GHIJKLMNOPQR"), None);
    }
    
    #[test]
    fn test_private_ip_ranges_comprehensive() {
        // All private IPv4 ranges
        // 10.0.0.0/8
        assert!(is_private_ip(&parse_ip("10.0.0.0").expect("Valid IP")));
        assert!(is_private_ip(&parse_ip("10.255.255.255").expect("Valid IP")));
        
        // 172.16.0.0/12
        assert!(is_private_ip(&parse_ip("172.16.0.0").expect("Valid IP")));
        assert!(is_private_ip(&parse_ip("172.31.255.255").expect("Valid IP")));
        assert!(!is_private_ip(&parse_ip("172.15.0.0").expect("Valid IP"))); // Just outside range
        assert!(!is_private_ip(&parse_ip("172.32.0.0").expect("Valid IP"))); // Just outside range
        
        // 192.168.0.0/16
        assert!(is_private_ip(&parse_ip("192.168.0.0").expect("Valid IP")));
        assert!(is_private_ip(&parse_ip("192.168.255.255").expect("Valid IP")));
        
        // Public IPs
        assert!(!is_private_ip(&parse_ip("8.8.8.8").expect("Valid IP")));
        assert!(!is_private_ip(&parse_ip("1.1.1.1").expect("Valid IP")));
        assert!(!is_private_ip(&parse_ip("74.125.224.72").expect("Valid IP"))); // Google
    }
    
    #[test]
    fn test_loopback_addresses() {
        // IPv4 loopback (127.0.0.0/8)
        assert!(is_loopback_ip(&parse_ip("127.0.0.1").expect("Valid loopback IP")));
        assert!(is_loopback_ip(&parse_ip("127.0.0.2").expect("Valid loopback IP")));
        assert!(is_loopback_ip(&parse_ip("127.255.255.255").expect("Valid loopback IP")));
        
        // IPv6 loopback (::1)
        assert!(is_loopback_ip(&parse_ip("::1").expect("Valid IPv6 loopback")));
        
        // Not loopback
        assert!(!is_loopback_ip(&parse_ip("192.168.1.1").expect("Valid IP")));
        assert!(!is_loopback_ip(&parse_ip("8.8.8.8").expect("Valid public IP")));
    }
    
    #[test]
    fn test_multicast_addresses() {
        // IPv4 multicast (224.0.0.0/4)
        assert!(is_multicast_ip(&parse_ip("224.0.0.1").expect("Valid multicast IP")));
        assert!(is_multicast_ip(&parse_ip("239.255.255.255").expect("Valid multicast IP")));
        
        // IPv6 multicast (ff00::/8)
        assert!(is_multicast_ip(&parse_ip("ff02::1").expect("Valid IPv6 multicast")));
        
        // Not multicast
        assert!(!is_multicast_ip(&parse_ip("192.168.1.1").expect("Valid IP")));
        assert!(!is_multicast_ip(&parse_ip("8.8.8.8").expect("Valid public IP")));
        assert!(!is_multicast_ip(&parse_ip("::1").expect("Valid loopback")));
    }
    
    #[test]
    fn test_localhost_helper() {
        let localhost = localhost();
        assert_eq!(localhost, "127.0.0.1");
        assert!(is_valid_ipv4(localhost));
        assert!(is_loopback_ip(&parse_ip(localhost).expect("Localhost should be valid IP")));
    }
    
    #[test]
    fn test_edge_case_ips() {
        // All zeros
        assert!(is_valid_ipv4("0.0.0.0"));
        assert!(is_valid_ipv6("::"));
        
        // All ones (IPv4 broadcast)
        assert!(is_valid_ipv4("255.255.255.255"));
        
        // IPv6 with compressed zeros
        assert!(is_valid_ipv6("2001:db8::"));
        assert!(is_valid_ipv6("::1"));
        assert!(is_valid_ipv6("fe80::"));
        
        // IPv6 full form
        assert!(is_valid_ipv6("2001:0db8:0000:0000:0000:0000:0000:0001"));
    }
    
    #[test]
    fn test_port_zero() {
        // Port 0 is technically valid for binding (OS chooses port)
        // but our is_valid_port requires > 0
        assert!(!is_valid_port(0));
        assert!(is_valid_port(1));
        assert!(is_valid_port(65535));
    }
    
    #[test]
    fn test_hostname_special_cases() {
        // Single character labels
        assert!(is_valid_hostname("a"));
        assert!(is_valid_hostname("a.b.c"));
        
        // Numbers in hostnames
        assert!(is_valid_hostname("server1"));
        assert!(is_valid_hostname("web-01"));
        assert!(is_valid_hostname("123"));
        
        // Mixed alphanumeric
        assert!(is_valid_hostname("web1-app2-db3"));
        
        // Special characters not allowed
        assert!(!is_valid_hostname("server_01")); // underscore
        assert!(!is_valid_hostname("web.server!")); // exclamation
    }
}

//! Additional error path tests for network utilities
//!
//! These tests ensure proper error handling for invalid inputs and edge cases.

#[cfg(test)]
mod network_error_tests {
    use crate::utils::network::*;

    #[test]
    fn test_parse_invalid_ipv4() {
        // Invalid IPv4 addresses should return errors
        assert!(parse_ipv4("256.1.1.1").is_err());
        assert!(parse_ipv4("192.168.1.").is_err());
        assert!(parse_ipv4("192.168").is_err());
        assert!(parse_ipv4("not-an-ip").is_err());
        assert!(parse_ipv4("").is_err());
        assert!(parse_ipv4("192.168.1.1.1").is_err());
    }

    #[test]
    fn test_parse_invalid_ipv6() {
        // Invalid IPv6 addresses should return errors
        assert!(parse_ipv6("gggg::1").is_err());
        assert!(parse_ipv6("2001:db8:::1").is_err());
        assert!(parse_ipv6("not-ipv6").is_err());
        assert!(parse_ipv6("").is_err());
        assert!(parse_ipv6("192.168.1.1").is_err()); // IPv4 is not IPv6
    }

    #[test]
    fn test_parse_invalid_cidr() {
        // Invalid CIDR notations should return errors
        assert!(parse_cidr("192.168.1.0/33").is_err()); // Prefix too large
        assert!(parse_cidr("192.168.1.0").is_err()); // Missing prefix
        assert!(parse_cidr("192.168.1.0/").is_err()); // Empty prefix
        assert!(parse_cidr("invalid/24").is_err()); // Invalid IP
        assert!(parse_cidr("").is_err()); // Empty string
    }

    #[test]
    fn test_parse_invalid_url() {
        // Invalid URLs should return errors
        assert!(parse_url("not a url").is_err());
        assert!(parse_url("").is_err());
        assert!(parse_url("ht!tp://invalid").is_err());
        assert!(parse_url("://missing-scheme").is_err());
    }

    #[test]
    fn test_boundary_conditions() {
        // Test boundary IP addresses
        let zero = parse_ipv4("0.0.0.0");
        assert!(zero.is_ok());
        
        let max = parse_ipv4("255.255.255.255");
        assert!(max.is_ok());
        
        // Just over boundary should fail
        assert!(parse_ipv4("256.0.0.0").is_err());
        assert!(parse_ipv4("0.256.0.0").is_err());
        assert!(parse_ipv4("0.0.256.0").is_err());
        assert!(parse_ipv4("0.0.0.256").is_err());
    }

    #[test]
    fn test_cidr_prefix_bounds() {
        // Valid CIDR prefixes (0-32 for IPv4, 0-128 for IPv6)
        assert!(parse_cidr("192.168.1.0/0").is_ok());
        assert!(parse_cidr("192.168.1.0/32").is_ok());
        assert!(parse_cidr("2001:db8::/0").is_ok());
        assert!(parse_cidr("2001:db8::/128").is_ok());
        
        // Invalid prefixes
        assert!(parse_cidr("192.168.1.0/33").is_err());
        assert!(parse_cidr("2001:db8::/129").is_err());
    }

    #[test]
    fn test_special_ip_addresses() {
        // Test special-purpose IP addresses
        let localhost = parse_ip("127.0.0.1").unwrap();
        assert!(is_loopback_ip(&localhost));
        assert!(!is_private_ip(&localhost));
        
        let localhost_v6 = parse_ip("::1").unwrap();
        assert!(is_loopback_ip(&localhost_v6));
        
        // Link-local addresses
        let link_local = parse_ip("169.254.1.1").unwrap();
        assert!(!is_loopback_ip(&link_local));
    }

    #[test]
    fn test_url_components() {
        // Test URL parsing with various components
        let url_with_port = parse_url("https://example.com:443/path").unwrap();
        assert!(url_with_port.to_string().contains("example.com"));
        
        let url_with_query = parse_url("https://example.com/path?key=value").unwrap();
        assert!(url_with_query.to_string().contains("example.com"));
        
        let url_with_fragment = parse_url("https://example.com/path#section").unwrap();
        assert!(url_with_fragment.to_string().contains("example.com"));
    }

    #[test]
    fn test_normalize_ip_strings() {
        // Test that IP parsing normalizes string representations
        let ip1 = parse_ipv4("192.168.001.001").unwrap();
        let ip2 = parse_ipv4("192.168.1.1").unwrap();
        assert_eq!(ip1, ip2);
    }

    #[test]
    fn test_ipv6_compressed_formats() {
        // Test various IPv6 compressed representations
        assert!(parse_ipv6("::1").is_ok());
        assert!(parse_ipv6("::").is_ok());
        assert!(parse_ipv6("2001:db8::1").is_ok());
        assert!(parse_ipv6("2001:db8::").is_ok());
        assert!(parse_ipv6("::ffff:192.168.1.1").is_ok()); // IPv4-mapped IPv6
    }
}


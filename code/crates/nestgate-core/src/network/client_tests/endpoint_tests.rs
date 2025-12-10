//! Endpoint Tests
//!
//! Auto-generated from smart refactoring of client_tests.rs
//! Sections: ENDPOINT TESTS

use super::super::client::*;

// ==================== ENDPOINT TESTS ====================
#[test]
fn test_endpoint_http() {
    use crate::constants::hardcoding::{addresses, ports};
    let port = Port::new(ports::HTTP_DEFAULT).expect("Network operation failed");
    let endpoint = Endpoint::http(addresses::LOCALHOST_NAME.to_string(), port);

    assert_eq!(endpoint.host, addresses::LOCALHOST_NAME);
    assert_eq!(endpoint.port.get(), ports::HTTP_DEFAULT);
    assert_eq!(endpoint.scheme, Scheme::Http);
}

#[test]
fn test_endpoint_https() {
    let port = Port::new(443).expect("Network operation failed");
    let endpoint = Endpoint::https("example.com".to_string(), port);

    assert_eq!(endpoint.host, "example.com");
    assert_eq!(endpoint.port.get(), 443);
    assert_eq!(endpoint.scheme, Scheme::Https);
}

#[test]
fn test_endpoint_url_http() {
    use crate::constants::hardcoding::{addresses, ports};
    let port = Port::new(ports::HTTP_DEFAULT).expect("Network operation failed");
    let endpoint = Endpoint::http(addresses::LOCALHOST_NAME.to_string(), port);
    let expected_url = format!(
        "http://{}:{}",
        addresses::LOCALHOST_NAME,
        ports::HTTP_DEFAULT
    );

    assert_eq!(endpoint.url(), expected_url);
}

#[test]
fn test_endpoint_url_https() {
    let port = Port::new(443).expect("Network operation failed");
    let endpoint = Endpoint::https("example.com".to_string(), port);

    assert_eq!(endpoint.url(), "https://example.com:443");
}

#[test]
fn test_endpoint_equality() {
    use crate::constants::hardcoding::{addresses, ports};
    let port1 = Port::new(ports::HTTP_DEFAULT).expect("Network operation failed");
    let port2 = Port::new(ports::HTTP_DEFAULT).expect("Network operation failed");
    let endpoint1 = Endpoint::http(addresses::LOCALHOST_NAME.to_string(), port1);
    let endpoint2 = Endpoint::http(addresses::LOCALHOST_NAME.to_string(), port2);

    assert_eq!(endpoint1, endpoint2);
}

#[test]
fn test_endpoint_serialization() {
    use crate::constants::hardcoding::{addresses, ports};
    let port = Port::new(ports::HTTP_DEFAULT).expect("Network operation failed");
    let endpoint = Endpoint::http(addresses::LOCALHOST_NAME.to_string(), port);
    let json = serde_json::to_string(&endpoint);
    assert!(json.is_ok());
}

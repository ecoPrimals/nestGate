//! Port type tests

use super::super::client::*;

#[test]
fn test_port_new_valid() {
    let port = Port::new(8080);
    assert!(port.is_ok());
    assert_eq!(port.expect("Network operation failed").get(), 8080);
}

#[test]
fn test_port_new_zero_invalid() {
    let port = Port::new(0);
    assert!(port.is_err());
}

#[test]
fn test_port_new_max_valid() {
    let port = Port::new(65535);
    assert!(port.is_ok());
    assert_eq!(port.expect("Network operation failed").get(), 65535);
}

#[test]
fn test_port_get() {
    let port = Port::new(3000).expect("Network operation failed");
    assert_eq!(port.get(), 3000);
}

#[test]
fn test_port_equality() {
    let port1 = Port::new(8080).expect("Network operation failed");
    let port2 = Port::new(8080).expect("Network operation failed");
    let port3 = Port::new(8081).expect("Network operation failed");

    assert_eq!(port1, port2);
    assert_ne!(port1, port3);
}

#[test]
fn test_port_serialization() {
    let port = Port::new(8080).expect("Network operation failed");
    let json = serde_json::to_string(&port);
    assert!(json.is_ok());
}

#[test]
fn test_port_edge_cases() {
    // Port 1 is valid
    assert!(Port::new(1).is_ok());
    
    // Port 0 is invalid
    assert!(Port::new(0).is_err());
    
    // Port 65535 is valid (max)
    assert!(Port::new(65535).is_ok());
    
    // Common ports
    assert!(Port::new(80).is_ok());
    assert!(Port::new(443).is_ok());
    assert!(Port::new(8080).is_ok());
    assert!(Port::new(3000).is_ok());
}

#[test]
fn test_port_comprehensive_coverage() {
    // Test boundary values
    assert!(Port::new(0).is_err());
    assert!(Port::new(1).is_ok());
    assert!(Port::new(1024).is_ok());
    assert!(Port::new(49152).is_ok());
    assert!(Port::new(65534).is_ok());
    assert!(Port::new(65535).is_ok());
}


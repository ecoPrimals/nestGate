// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Port Tests
//!
//! Auto-generated from smart refactoring of client_tests.rs
//! Sections: PORT TESTS

use super::super::client::*;

// ==================== PORT TESTS ====================
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

// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Port Edge Cases Tests
//!
//! Auto-generated from smart refactoring of client_tests.rs
//! Sections: PORT EDGE CASES

use super::super::client::*;

// ==================== PORT EDGE CASES ====================
#[test]
fn test_port_boundary_values() {
    assert!(Port::new(0).is_err());
    assert!(Port::new(1).is_ok());
    assert!(Port::new(1024).is_ok());
    assert!(Port::new(8080).is_ok());
    assert!(Port::new(65534).is_ok());
    assert!(Port::new(65535).is_ok());
}

#[test]
fn test_port_common_values() {
    let common_ports = [
        20, 21, 22, 23, 25, 53, 80, 110, 143, 443, 3306, 5432, 6379, 8080, 9090,
    ];

    for port_num in &common_ports {
        let port = Port::new(*port_num);
        assert!(port.is_ok());
        assert_eq!(port.expect("Network operation failed").get(), *port_num);
    }
}

//! Port Coverage Tests
//!
//! Auto-generated from smart refactoring of client_tests.rs
//! Sections: PORT COMPREHENSIVE COVERAGE

use super::super::client::*;

// ==================== PORT COMPREHENSIVE COVERAGE ====================
#[test]
fn test_port_clone() {
    let original = Port::new(8080).expect("Network operation failed");
    let cloned = original;

    assert_eq!(original, cloned);
    assert_eq!(original.get(), cloned.get());
}

#[test]
fn test_port_hash_consistency() {
    use std::collections::HashSet;

    let port1 = Port::new(8080).expect("Network operation failed");
    let port2 = Port::new(8080).expect("Network operation failed");
    let port3 = Port::new(9090).expect("Network operation failed");

    let mut set = HashSet::new();
    set.insert(port1);
    set.insert(port2); // Should not add (same as port1)
    set.insert(port3);

    assert_eq!(set.len(), 2); // Only 8080 and 9090
}

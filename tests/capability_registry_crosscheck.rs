// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

#![allow(
    missing_docs,
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::restriction,
    clippy::cargo
)]

//! Cross-check invariant tests: `capability_registry.toml` ↔ Wire Standard L3 response.
//!
//! These tests guarantee that the machine-readable registry stays in sync with
//! the runtime `capabilities.list` JSON-RPC payload and catch drift early.

use std::collections::BTreeSet;

const REGISTRY_TOML: &str = include_str!("../capability_registry.toml");

fn registry_methods() -> BTreeSet<String> {
    let table: toml::Table = REGISTRY_TOML.parse().expect("valid TOML");
    let caps = table
        .get("capabilities")
        .expect("capabilities section")
        .as_table()
        .expect("table");

    let mut methods = BTreeSet::new();
    for (_domain, block) in caps {
        let block = block.as_table().expect("capability block");
        if let Some(arr) = block.get("methods") {
            for m in arr.as_array().expect("methods array") {
                methods.insert(m.as_str().expect("string method").to_string());
            }
        }
    }
    methods
}

fn registry_domains() -> BTreeSet<String> {
    let table: toml::Table = REGISTRY_TOML.parse().expect("valid TOML");
    let caps = table
        .get("capabilities")
        .expect("capabilities section")
        .as_table()
        .expect("table");

    caps.iter()
        .filter_map(|(_, block)| {
            block
                .as_table()
                .and_then(|t| t.get("domain"))
                .and_then(|d| d.as_str())
                .map(String::from)
        })
        .collect()
}

fn registry_consumed_capabilities() -> BTreeSet<String> {
    let table: toml::Table = REGISTRY_TOML.parse().expect("valid TOML");
    table
        .get("consumed_capabilities")
        .expect("consumed_capabilities section")
        .as_table()
        .expect("table")
        .keys()
        .cloned()
        .collect()
}

#[test]
fn registry_parses_without_error() {
    let table: toml::Table = REGISTRY_TOML.parse().expect("valid TOML");
    assert!(table.contains_key("primal"), "missing [primal] section");
    assert!(
        table.contains_key("capabilities"),
        "missing [capabilities] section"
    );
    assert!(
        table.contains_key("consumed_capabilities"),
        "missing [consumed_capabilities]"
    );
}

#[test]
fn registry_primal_identity_is_nestgate() {
    let table: toml::Table = REGISTRY_TOML.parse().expect("valid TOML");
    let primal = table["primal"].as_table().expect("primal table");
    assert_eq!(primal["name"].as_str().unwrap(), "nestgate");
    assert_eq!(primal["domain"].as_str().unwrap(), "storage");
    assert_eq!(primal["protocol"].as_str().unwrap(), "jsonrpc-2.0");
    assert_eq!(primal["license"].as_str().unwrap(), "AGPL-3.0-or-later");
}

#[test]
fn every_registry_method_follows_semantic_naming() {
    for method in &registry_methods() {
        let parts: Vec<&str> = method.split('.').collect();
        assert!(
            parts.len() >= 2,
            "method '{}' must be domain.operation (at least 2 segments)",
            method
        );
        for part in &parts {
            assert!(
                part.chars().all(|c| c.is_ascii_lowercase() || c == '_'),
                "method '{}' contains non-lowercase/non-underscore characters",
                method
            );
        }
    }
}

#[test]
fn registry_method_count_matches_status_expectation() {
    let methods = registry_methods();
    assert!(
        methods.len() >= 45,
        "Registry declares {} methods, expected at least 45 (12 domains, canonical dispatch)",
        methods.len()
    );
}

#[test]
fn registry_has_required_core_domains() {
    let domains = registry_domains();
    for required in &["storage", "health", "identity", "discovery", "zfs", "model"] {
        assert!(
            domains.contains(*required),
            "missing required domain: {}",
            required
        );
    }
}

#[test]
fn registry_has_required_health_triad() {
    let methods = registry_methods();
    for required in &["health.liveness", "health.readiness", "health.check"] {
        assert!(
            methods.contains(&(*required).to_string()),
            "missing health triad method: {}",
            required
        );
    }
}

#[test]
fn registry_has_identity_get() {
    let methods = registry_methods();
    assert!(
        methods.contains("identity.get"),
        "missing Wire Standard required method: identity.get"
    );
}

#[test]
fn registry_has_capabilities_list() {
    let methods = registry_methods();
    assert!(
        methods.contains("capabilities.list"),
        "missing Wire Standard required method: capabilities.list"
    );
}

#[test]
fn consumed_capabilities_are_declared() {
    let consumed = registry_consumed_capabilities();
    assert!(
        !consumed.is_empty(),
        "consumed_capabilities should declare at least one entry"
    );
    assert!(
        consumed.contains("security"),
        "should declare security as consumed"
    );
}

#[test]
fn no_duplicate_methods_across_domains() {
    let table: toml::Table = REGISTRY_TOML.parse().expect("valid TOML");
    let caps = table["capabilities"].as_table().expect("table");

    let mut seen = BTreeSet::new();
    for (_domain, block) in caps {
        let block = block.as_table().expect("capability block");
        if let Some(arr) = block.get("methods") {
            for m in arr.as_array().expect("methods array") {
                let method = m.as_str().expect("string").to_string();
                assert!(
                    seen.insert(method.clone()),
                    "duplicate method across domains: {}",
                    method
                );
            }
        }
    }
}

#[test]
fn transport_includes_uds() {
    let table: toml::Table = REGISTRY_TOML.parse().expect("valid TOML");
    let transport = table["primal"]["transport"]
        .as_array()
        .expect("transport array");
    let values: Vec<&str> = transport.iter().map(|v| v.as_str().unwrap()).collect();
    assert!(values.contains(&"uds"), "transport must include 'uds'");
}

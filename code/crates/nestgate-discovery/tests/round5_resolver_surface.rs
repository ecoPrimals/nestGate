// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Round 5: `ResolvedService` URL helpers and unified capability mapping smoke tests.

use nestgate_discovery::capability_resolver::ResolvedService;
use nestgate_discovery::unified_capabilities::UnifiedCapability;

#[test]
fn resolved_service_url_https() {
    let s = ResolvedService {
        id: "1".into(),
        host: "h".into(),
        port: 443,
        protocol: "https".into(),
        capabilities: vec![UnifiedCapability::HttpApi],
        is_healthy: true,
    };
    assert_eq!(s.url(), "https://h:443");
}

#[test]
fn resolved_service_url_http() {
    let s = ResolvedService {
        id: "1".into(),
        host: "127.0.0.1".into(),
        port: 8080,
        protocol: "http".into(),
        capabilities: vec![],
        is_healthy: false,
    };
    assert_eq!(s.url(), "http://127.0.0.1:8080");
}

#[test]
fn resolved_service_endpoint_no_protocol() {
    let s = ResolvedService {
        id: "1".into(),
        host: "a".into(),
        port: 1,
        protocol: "grpc".into(),
        capabilities: vec![],
        is_healthy: true,
    };
    assert_eq!(s.endpoint(), "a:1");
}

#[test]
fn unified_capability_display_or_debug_non_empty() {
    let c = UnifiedCapability::ZfsManagement;
    let t = format!("{c:?}");
    assert!(!t.is_empty());
}

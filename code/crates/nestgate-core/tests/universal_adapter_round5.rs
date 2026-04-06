// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective
#![allow(
    dead_code,
    missing_docs,
    unused_imports,
    unused_variables,
    clippy::all,
    clippy::cargo,
    clippy::nursery,
    clippy::pedantic,
    clippy::restriction
)]

//! Round 5: `UniversalAdapter::route_capability_request` ok/err branches.

use nestgate_core::universal_adapter::{CapabilityInfo, UniversalAdapter};
use std::collections::HashMap;
use std::time::SystemTime;

fn sample_capability(key: &str) -> CapabilityInfo {
    CapabilityInfo {
        category: key.to_string(),
        provider: "p".into(),
        endpoint: format!("http://example/{key}"),
        performance_tier: "standard".into(),
        availability: 99.0,
        metadata: HashMap::new(),
        discovered_at: SystemTime::UNIX_EPOCH,
    }
}

#[test]
fn route_capability_request_unknown_returns_not_found() {
    let adapter = UniversalAdapter::new("http://local".into());
    let req = nestgate_core::universal_adapter::CapabilityRequest::new("missing", "m");
    let err = adapter
        .route_capability_request(&req)
        .expect_err("unknown capability");
    assert!(err.to_string().contains("No capability found"));
}

#[test]
fn route_capability_request_hits_storage_entry() {
    let mut adapter = UniversalAdapter::new("http://local".into());
    adapter
        .capabilities
        .insert("storage".into(), sample_capability("storage"));
    let req = nestgate_core::universal_adapter::CapabilityRequest::new("storage", "get");
    let v = adapter.route_capability_request(&req).expect("routed");
    assert_eq!(v["status"], "routed");
    assert_eq!(v["operation"], "get");
}

#[test]
fn get_capability_miss_not_in_map() {
    let adapter = UniversalAdapter::new("http://local".into());
    let err = adapter.get_capability("nope").expect_err("miss");
    assert!(err.contains("not found"));
}

#[test]
fn query_capability_filters_by_category_substring() {
    let mut adapter = UniversalAdapter::new("http://local".into());
    adapter
        .capabilities
        .insert("alpha".into(), sample_capability("alpha"));
    let q = nestgate_core::universal_adapter::types::CapabilityQuery::new("alp");
    let hits = adapter.query_capability(&q).expect("q");
    assert!(!hits.is_empty());
}

#[test]
fn query_capability_no_match_empty() {
    let adapter = UniversalAdapter::new("http://local".into());
    let q = nestgate_core::universal_adapter::types::CapabilityQuery::new("zzz");
    let hits = adapter.query_capability(&q).expect("q");
    assert!(hits.is_empty());
}

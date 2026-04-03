// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Integration tests for the middleware crate’s public configuration surface.

#![allow(clippy::expect_used)]

use nestgate_middleware::{MiddlewareConfig, create_default_config};
use serde::{Deserialize, Serialize};

fn assert_json_idempotent_roundtrip<T>(value: &T)
where
    T: Serialize + for<'de> Deserialize<'de>,
{
    let json = serde_json::to_string(value).expect("serialize config to JSON");
    let back: T = serde_json::from_str(&json).expect("deserialize config from JSON");
    let json_again = serde_json::to_string(&back).expect("re-serialize after roundtrip");
    assert_eq!(
        json, json_again,
        "JSON roundtrip should be stable (no semantic drift on re-serialize)"
    );
}

#[test]
fn create_default_config_round_trips_through_json() {
    let cfg = create_default_config();
    assert_json_idempotent_roundtrip(&cfg);
}

#[test]
fn middleware_config_default_matches_factory() {
    let from_fn = create_default_config();
    let from_default = MiddlewareConfig::default();
    assert_eq!(
        from_fn.system.instance_name,
        from_default.system.instance_name
    );
}

#[test]
fn clone_preserves_instance_name() {
    let a = create_default_config();
    let b = a.clone();
    assert_eq!(a.system.instance_name, b.system.instance_name);
}

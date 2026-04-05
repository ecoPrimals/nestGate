// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

#![no_main]

use arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;

#[derive(Arbitrary, Debug)]
struct FuzzAdapter {
    service_name: String,
    capabilities: Vec<String>,
    endpoint: String,
    metadata: std::collections::HashMap<String, String>,
}

fuzz_target!(|input: FuzzAdapter| {
    // Test universal adapter input validation
    if input.service_name.len() > 256 {
        return; // Skip overly long service names
    }

    if input.endpoint.len() > 2048 {
        return; // Skip overly long endpoints
    }

    // Test basic URL-like validation
    if input.endpoint.starts_with("http://") || input.endpoint.starts_with("https://") {
        assert!(!input.endpoint.contains(".."));
        assert!(!input.endpoint.contains("//"));
    }

    // Validate service name doesn't contain dangerous patterns
    assert!(!input.service_name.contains('/'));
    assert!(!input.service_name.contains('\\'));
    assert!(!input.service_name.contains('\0'));

    // Test capability validation
    for capability in &input.capabilities {
        assert!(!capability.contains("../"));
        assert!(!capability.contains("javascript:"));
    }

    // Test metadata validation
    for (key, value) in &input.metadata {
        assert!(!key.contains('\0'));
        assert!(!value.contains('\0'));
    }
});

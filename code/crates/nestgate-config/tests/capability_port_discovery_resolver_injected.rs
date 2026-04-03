// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

#![allow(clippy::unwrap_used)]

//! With a registered resolver, [`discover_*_port`] uses resolved ports before env/defaults.

use nestgate_config::constants::capability_port_discovery::{
    CapabilityPortResolver, capability_ids, discover_api_port, discover_metrics_port,
    discover_storage_port, register_capability_resolver,
};
use temp_env::with_vars;

struct FullResolver;

impl CapabilityPortResolver for FullResolver {
    fn resolve_service_port(&self, capability: &str) -> Option<u16> {
        match capability {
            capability_ids::API_GATEWAY => Some(6100),
            capability_ids::OBSERVABILITY_METRICS => Some(6200),
            capability_ids::STORAGE_ZFS => Some(6300),
            _ => None,
        }
    }
}

#[test]
fn capability_port_discovery_injected_resolver_overrides_defaults() {
    register_capability_resolver(Box::new(FullResolver));

    with_vars(
        vec![
            ("NESTGATE_API_PORT", None::<&str>),
            ("NESTGATE_METRICS_PORT", None::<&str>),
            ("NESTGATE_STORAGE_PORT", None::<&str>),
        ],
        || {
            assert_eq!(discover_api_port().unwrap(), 6100);
            assert_eq!(discover_metrics_port().unwrap(), 6200);
            assert_eq!(discover_storage_port().unwrap(), 6300);
        },
    );
}

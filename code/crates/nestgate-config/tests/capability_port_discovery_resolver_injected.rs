// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

#![expect(clippy::unwrap_used)]

//! With a registered resolver, [`discover_*_port`] uses resolved ports before env/defaults.

use nestgate_config::constants::capability_port_discovery::{
    CapabilityPortResolver, capability_ids, discover_api_port_from_env_source,
    discover_metrics_port_from_env_source, discover_storage_port_from_env_source,
    register_capability_resolver,
};
use nestgate_types::MapEnv;

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

    let env = MapEnv::new();
    assert_eq!(discover_api_port_from_env_source(&env).unwrap(), 6100);
    assert_eq!(discover_metrics_port_from_env_source(&env).unwrap(), 6200);
    assert_eq!(discover_storage_port_from_env_source(&env).unwrap(), 6300);
}

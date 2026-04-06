// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

#![expect(clippy::unwrap_used)]

//! When the resolver returns [`None`] for a capability, discovery falls through to env/defaults.

use nestgate_config::constants::capability_port_discovery::{
    CapabilityPortResolver, capability_ids, discover_api_port_from_env_source,
    discover_metrics_port_from_env_source, discover_storage_port_from_env_source,
    register_capability_resolver,
};
use nestgate_types::MapEnv;

struct PartialResolver;

impl CapabilityPortResolver for PartialResolver {
    fn resolve_service_port(&self, capability: &str) -> Option<u16> {
        match capability {
            capability_ids::API_GATEWAY => Some(7100),
            capability_ids::STORAGE_ZFS => Some(7300),
            // `OBSERVABILITY_METRICS` and unknown capabilities fall through to env/defaults.
            _ => None,
        }
    }
}

#[test]
fn capability_port_discovery_resolver_none_falls_through_to_env() {
    register_capability_resolver(Box::new(PartialResolver));

    let env = MapEnv::from([("NESTGATE_METRICS_PORT", "7777")]);
    assert_eq!(discover_api_port_from_env_source(&env).unwrap(), 7100);
    assert_eq!(discover_metrics_port_from_env_source(&env).unwrap(), 7777);
    assert_eq!(discover_storage_port_from_env_source(&env).unwrap(), 7300);
}

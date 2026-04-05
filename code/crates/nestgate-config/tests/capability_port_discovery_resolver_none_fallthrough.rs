// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

#![expect(clippy::unwrap_used)]

//! When the resolver returns [`None`] for a capability, discovery falls through to env/defaults.

use nestgate_config::constants::capability_port_discovery::{
    CapabilityPortResolver, capability_ids, discover_api_port, discover_metrics_port,
    discover_storage_port, register_capability_resolver,
};
use temp_env::with_vars;

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

    with_vars(
        vec![
            ("NESTGATE_API_PORT", None::<&str>),
            ("NESTGATE_METRICS_PORT", Some("7777")),
            ("NESTGATE_STORAGE_PORT", None::<&str>),
        ],
        || {
            assert_eq!(discover_api_port().unwrap(), 7100);
            assert_eq!(discover_metrics_port().unwrap(), 7777);
            assert_eq!(discover_storage_port().unwrap(), 7300);
        },
    );
}

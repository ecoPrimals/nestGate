// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

#![expect(clippy::unwrap_used)]

//! When no [`CapabilityPortResolver`] is registered, discovery falls through to env/defaults.

use nestgate_config::constants::capability_port_discovery::{
    discover_api_port_from_env_source, discover_metrics_port_from_env_source,
    discover_storage_port_from_env_source,
};
use nestgate_types::MapEnv;

#[test]
fn capability_port_discovery_no_resolver_uses_env_or_defaults() {
    let env = MapEnv::new();
    assert_eq!(discover_api_port_from_env_source(&env).unwrap(), 8080);
    assert_eq!(discover_metrics_port_from_env_source(&env).unwrap(), 9090);
    assert_eq!(discover_storage_port_from_env_source(&env).unwrap(), 8083);
}

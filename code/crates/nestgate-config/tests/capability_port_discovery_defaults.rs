// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

#![allow(clippy::unwrap_used)]

//! When no [`CapabilityPortResolver`] is registered, discovery falls through to env/defaults.

use nestgate_config::constants::capability_port_discovery::{
    discover_api_port, discover_metrics_port, discover_storage_port,
};
use temp_env::with_vars;

#[test]
fn capability_port_discovery_no_resolver_uses_env_or_defaults() {
    with_vars(
        vec![
            ("NESTGATE_API_PORT", None::<&str>),
            ("NESTGATE_METRICS_PORT", None::<&str>),
            ("NESTGATE_STORAGE_PORT", None::<&str>),
        ],
        || {
            assert_eq!(discover_api_port().unwrap(), 8080);
            assert_eq!(discover_metrics_port().unwrap(), 9090);
            assert_eq!(discover_storage_port().unwrap(), 8083);
        },
    );
}

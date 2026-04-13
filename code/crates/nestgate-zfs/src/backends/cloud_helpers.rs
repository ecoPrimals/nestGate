// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Shared helpers for cloud object-storage backends (`GCS`, Azure Blob, etc.):
//! naming, configuration provenance, and capability-discovery stubs.

use nestgate_core::NestGateError;

/// Tracks where backend configuration came from (capability discovery vs env vs explicit).
#[derive(Debug, Clone)]
pub enum CloudConfigSource {
    /// Discovered via NestGate capability system (preferred).
    CapabilityDiscovered {
        /// Service identifier from discovery.
        service_id: String,
    },
    /// Loaded from process environment variables.
    Environment,
    /// Explicit / injected configuration (tests or future static wiring).
    ///
    /// `Some(x)` is serialized as `explicit:{x}`; `None` as `explicit`.
    Explicit {
        /// Optional qualifier (e.g. project id for GCS).
        detail: Option<String>,
    },
}

/// Value stored under `config_source` in pool `custom` metadata.
pub fn config_source_custom_value(source: &CloudConfigSource) -> String {
    match source {
        CloudConfigSource::CapabilityDiscovered { service_id } => {
            format!("capability:{service_id}")
        }
        CloudConfigSource::Environment => "environment".to_string(),
        CloudConfigSource::Explicit { detail } => match detail {
            Some(d) => format!("explicit:{d}"),
            None => "explicit".to_string(),
        },
    }
}

/// Pool-level resource name: `{prefix}-{pool}`, lowercased, underscores → hyphens.
#[must_use]
pub fn prefixed_pool_resource_name(prefix: &str, pool_name: &str) -> String {
    format!("{prefix}-{pool_name}")
        .to_lowercase()
        .replace('_', "-")
}

/// Dataset key prefix inside a pool: `{pool}/{dataset}`.
#[must_use]
pub fn dataset_path_prefix(pool_name: &str, dataset_name: &str) -> String {
    format!("{pool_name}/{dataset_name}")
}

/// Placeholder error until capability discovery returns real cloud config.
pub fn pending_capability_discovery(provider: &str) -> NestGateError {
    NestGateError::not_found(format!(
        "{provider} capability discovery integration pending"
    ))
}

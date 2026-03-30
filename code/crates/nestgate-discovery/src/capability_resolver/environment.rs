// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Environment-variable based resolution (fallback when no registry is available).

use crate::unified_capabilities::{CapabilityMapper, UnifiedCapability};
use nestgate_types::error::{NestGateError, Result};

use super::types::{CapabilityResolver, ResolvedService};
use std::future::{Future, ready};
use std::pin::Pin;
use std::sync::Arc;

/// Environment-based capability resolver (fallback)
///
/// Resolves capabilities using environment variables only.
/// Used when no registry is available.
pub struct EnvironmentResolver;

impl EnvironmentResolver {
    /// Create new environment resolver
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl Default for EnvironmentResolver {
    fn default() -> Self {
        Self::new()
    }
}

/// Shared env parsing for [`EnvironmentResolver`] — avoids nested `resolve_capability().await`
/// inside `resolve_capability_all` (one fewer `dyn Future` layer on that path).
fn resolve_capability_from_env(capability: UnifiedCapability) -> Result<ResolvedService> {
    let env_var = CapabilityMapper::env_var_name(&capability);
    let value = std::env::var(&env_var).map_err(|_| {
        NestGateError::internal_error(
            format!(
                "Capability '{capability}' not configured. Set {env_var} environment variable."
            ),
            "environment_resolver",
        )
    })?;

    if let Ok(url) = value.parse::<url::Url>() {
        let host = url
            .host_str()
            .ok_or_else(|| {
                NestGateError::configuration_error(
                    "capability_endpoint_host",
                    format!("Environment variable {env_var} has URL without host: {value}"),
                )
            })?
            .to_string();

        let port = url.port().or_else(|| {
            match url.scheme() {
                "https" => Some(443),
                "http" | "ws" | "wss" => Some(80),
                "grpc" => Some(9090),
                _ => None,
            }
        }).ok_or_else(|| NestGateError::configuration_error(
            "capability_endpoint_port",
            format!("Environment variable {} has URL without port and no default for scheme: {}", env_var, url.scheme())
        ))?;

        Ok(ResolvedService {
            id: Arc::from("env-configured"),
            host,
            port,
            protocol: Arc::from(url.scheme()),
            capabilities: vec![capability],
            is_healthy: true,
        })
    } else if let Some((host, port_str)) = value.split_once(':') {
        let port = port_str.parse().map_err(|_| {
            NestGateError::internal_error(
                format!("Invalid port in {env_var}: {port_str}"),
                "environment_resolver",
            )
        })?;
        Ok(ResolvedService {
            id: Arc::from("env-configured"),
            host: host.to_string(),
            port,
            protocol: Arc::from("http"),
            capabilities: vec![capability],
            is_healthy: true,
        })
    } else {
        Err(NestGateError::internal_error(
            format!("Invalid endpoint format in {env_var}: {value}. Expected URL or host:port"),
            "environment_resolver",
        ))
    }
}

impl CapabilityResolver for EnvironmentResolver {
    fn resolve_capability(
        &self,
        capability: &UnifiedCapability,
    ) -> Pin<Box<dyn Future<Output = Result<ResolvedService>> + Send + '_>> {
        let capability = capability.clone();
        Box::pin(ready(resolve_capability_from_env(capability)))
    }

    fn resolve_capability_all(
        &self,
        capability: &UnifiedCapability,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ResolvedService>>> + Send + '_>> {
        let capability = capability.clone();
        Box::pin(ready(
            resolve_capability_from_env(capability).map(|service| vec![service]),
        ))
    }

    fn has_capability(
        &self,
        capability: &UnifiedCapability,
    ) -> Pin<Box<dyn Future<Output = bool> + Send + '_>> {
        let capability = capability.clone();
        Box::pin(async move {
            let env_var = CapabilityMapper::env_var_name(&capability);
            std::env::var(&env_var).is_ok()
        })
    }
}

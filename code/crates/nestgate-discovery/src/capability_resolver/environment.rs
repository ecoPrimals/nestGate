// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Environment-variable based resolution (fallback when no registry is available).

use crate::unified_capabilities::{CapabilityMapper, UnifiedCapability};
use nestgate_types::error::{NestGateError, Result};
use nestgate_types::{EnvSource, ProcessEnv};

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
    resolve_capability_from_env_source(&ProcessEnv, capability)
}

fn resolve_capability_from_env_source(
    env: &(impl EnvSource + ?Sized),
    capability: UnifiedCapability,
) -> Result<ResolvedService> {
    let env_var = CapabilityMapper::env_var_name(&capability);
    let value = env.get(&env_var).ok_or_else(|| {
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

#[cfg(test)]
mod tests {
    use super::*;
    use nestgate_types::MapEnv;

    fn env_with(pairs: &[(&str, &str)]) -> MapEnv {
        let map = pairs.iter().map(|(k, v)| ((*k).to_string(), (*v).to_string())).collect();
        MapEnv(map)
    }

    #[test]
    fn resolves_https_url_with_explicit_port() {
        let env = env_with(&[("NESTGATE_CAPABILITY_STORAGE_ENDPOINT", "https://storage.local:8443")]);
        let svc = resolve_capability_from_env_source(&env, UnifiedCapability::Storage).unwrap();
        assert_eq!(svc.host, "storage.local");
        assert_eq!(svc.port, 8443);
        assert_eq!(&*svc.protocol, "https");
        assert!(svc.capabilities.contains(&UnifiedCapability::Storage));
    }

    #[test]
    fn resolves_https_url_with_default_port() {
        let env = env_with(&[("NESTGATE_CAPABILITY_STORAGE_ENDPOINT", "https://storage.local")]);
        let svc = resolve_capability_from_env_source(&env, UnifiedCapability::Storage).unwrap();
        assert_eq!(svc.port, 443);
    }

    #[test]
    fn resolves_http_url_with_default_port() {
        let env = env_with(&[("NESTGATE_CAPABILITY_STORAGE_ENDPOINT", "http://storage.local")]);
        let svc = resolve_capability_from_env_source(&env, UnifiedCapability::Storage).unwrap();
        assert_eq!(svc.port, 80);
    }

    #[test]
    fn resolves_grpc_url_with_default_port() {
        let env = env_with(&[("NESTGATE_CAPABILITY_STORAGE_ENDPOINT", "grpc://storage.local")]);
        let svc = resolve_capability_from_env_source(&env, UnifiedCapability::Storage).unwrap();
        assert_eq!(svc.port, 9090);
        assert_eq!(&*svc.protocol, "grpc");
    }

    #[test]
    fn resolves_host_colon_port_fallback() {
        let env = env_with(&[("NESTGATE_CAPABILITY_STORAGE_ENDPOINT", "192.168.1.1:3000")]);
        let svc = resolve_capability_from_env_source(&env, UnifiedCapability::Storage).unwrap();
        assert_eq!(svc.host, "192.168.1.1");
        assert_eq!(svc.port, 3000);
        assert_eq!(&*svc.protocol, "http");
    }

    #[test]
    fn errors_on_missing_env_var() {
        let env = MapEnv::new();
        let err = resolve_capability_from_env_source(&env, UnifiedCapability::Storage).unwrap_err();
        assert!(err.to_string().contains("not configured"));
    }

    #[test]
    fn errors_on_invalid_format() {
        let env = env_with(&[("NESTGATE_CAPABILITY_STORAGE_ENDPOINT", "not-a-url")]);
        let err = resolve_capability_from_env_source(&env, UnifiedCapability::Storage).unwrap_err();
        assert!(err.to_string().contains("Invalid"));
    }

    #[test]
    fn errors_on_invalid_port_in_host_colon_port() {
        let env = env_with(&[("NESTGATE_CAPABILITY_STORAGE_ENDPOINT", "host:abc")]);
        let err = resolve_capability_from_env_source(&env, UnifiedCapability::Storage).unwrap_err();
        assert!(err.to_string().contains("port"));
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
            ProcessEnv.get(&env_var).is_some()
        })
    }
}

// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Kubernetes-based discovery (orchestrated environments)
//!
//! Provides Kubernetes service discovery via the k8s REST API using the
//! pure-Rust bootstrap HTTP client.
//! Requires the `kubernetes` feature flag.
//!
//! ## Connectivity
//!
//! This module connects to the k8s API server over **HTTP** (not HTTPS).
//! In production k8s clusters, use one of:
//! - `kubectl proxy` sidecar (exposes HTTP on localhost)
//! - Service mesh (Istio/Linkerd) with mTLS handled at the mesh layer
//! - `KUBERNETES_API_PROXY` env var pointing to an HTTP proxy
//!
//! Direct HTTPS to the k8s API server requires TLS, which is outside
//! the scope of the discovery bootstrap client.

use super::http::DiscoveryHttpClient;
use super::{Capability, DiscoveryBuilder, DiscoveryMechanism, ServiceInfo};
use crate::self_knowledge::SelfKnowledge;
use nestgate_types::error::{NestGateError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::time::Duration;

/// Kubernetes service metadata
#[derive(Debug, Serialize, Deserialize)]
struct K8sMetadata {
    name: String,
    namespace: String,
    labels: HashMap<String, String>,
    annotations: HashMap<String, String>,
}

/// Kubernetes service spec
#[derive(Debug, Serialize, Deserialize)]
struct K8sServiceSpec {
    ports: Vec<K8sServicePort>,
    #[serde(rename = "clusterIP")]
    cluster_ip: Option<String>,
}

/// Kubernetes service port
#[derive(Debug, Serialize, Deserialize)]
struct K8sServicePort {
    port: i32,
    #[serde(rename = "targetPort", skip_serializing_if = "Option::is_none")]
    target_port: Option<i32>,
    protocol: String,
}

/// Kubernetes service
#[derive(Debug, Serialize, Deserialize)]
struct K8sService {
    metadata: K8sMetadata,
    spec: K8sServiceSpec,
}

/// Kubernetes service list
#[derive(Debug, Deserialize)]
struct K8sServiceList {
    items: Vec<K8sService>,
}

/// Resolve the Kubernetes API HTTP base URL (no implicit remote peers).
///
/// Order: `KUBERNETES_API_PROXY`, `NESTGATE_KUBERNETES_API_PROXY`,
/// `NESTGATE_CAPABILITY_KUBERNETES_API_ENDPOINT`, in-cluster
/// `KUBERNETES_SERVICE_HOST` + `KUBERNETES_SERVICE_PORT` (both required), then
/// `NESTGATE_K8S_LOCAL_PROXY_URL`. If none apply, returns a configuration error
/// (no loopback default).
fn resolve_kubernetes_api_server_url() -> Result<String> {
    if let Ok(url) = std::env::var("KUBERNETES_API_PROXY") {
        return Ok(url);
    }
    if let Ok(url) = std::env::var("NESTGATE_KUBERNETES_API_PROXY") {
        return Ok(url);
    }
    if let Ok(url) = std::env::var("NESTGATE_CAPABILITY_KUBERNETES_API_ENDPOINT") {
        return Ok(url);
    }
    if let Ok(host) = std::env::var("KUBERNETES_SERVICE_HOST") {
        let port = std::env::var("KUBERNETES_SERVICE_PORT").map_err(|_| {
            NestGateError::configuration_error(
                "KUBERNETES_SERVICE_PORT",
                "KUBERNETES_SERVICE_PORT not set — K8s discovery requires in-cluster environment",
            )
        })?;
        return Ok(format!("http://{host}:{port}"));
    }
    if let Ok(url) = std::env::var("NESTGATE_K8S_LOCAL_PROXY_URL") {
        return Ok(url);
    }
    Err(NestGateError::configuration_error(
        "KUBERNETES_SERVICE_HOST",
        "KUBERNETES_SERVICE_HOST not set — K8s discovery requires in-cluster environment",
    ))
}

/// Kubernetes discovery mechanism (pure-Rust HTTP via kubectl proxy or mesh)
pub struct KubernetesDiscovery {
    _timeout: Duration,
    _cache_duration: Duration,
    namespace: String,
    client: DiscoveryHttpClient,
    api_server: String,
}

impl KubernetesDiscovery {
    /// Create a new Kubernetes discovery instance.
    ///
    /// Resolves the API URL via `resolve_kubernetes_api_server_url` (see that function for
    /// `NESTGATE_*` / capability env vars and in-cluster requirements).
    ///
    /// # Errors
    ///
    /// Returns an error if the client cannot be constructed.
    pub fn new(builder: &DiscoveryBuilder) -> Result<Self> {
        let namespace = std::env::var("NAMESPACE")
            .or_else(|_| std::env::var("POD_NAMESPACE"))
            .unwrap_or_else(|_| "default".to_string());

        let api_server = resolve_kubernetes_api_server_url()?;

        let token =
            std::fs::read_to_string("/var/run/secrets/kubernetes.io/serviceaccount/token").ok();

        let mut client = DiscoveryHttpClient::new(builder.timeout);
        if let Some(t) = token {
            client = client.with_header("Authorization", format!("Bearer {t}"));
        }

        Ok(Self {
            _timeout: builder.timeout,
            _cache_duration: builder.cache_duration,
            namespace,
            client,
            api_server,
        })
    }

    fn service_to_info(svc: K8sService) -> Option<ServiceInfo> {
        let port = svc.spec.ports.first()?.port;
        let ip = svc.spec.cluster_ip.as_ref()?;

        let capabilities: Vec<String> = svc
            .metadata
            .labels
            .get("capabilities")
            .map(|s| s.split(',').map(|s| s.trim().to_string()).collect())
            .unwrap_or_default();

        Some(ServiceInfo {
            id: format!("{}.{}", svc.metadata.name, svc.metadata.namespace),
            name: svc.metadata.name.clone(),
            capabilities,
            endpoint: format!("http://{ip}:{port}"),
            metadata: svc.metadata.annotations,
            health_endpoint: None,
        })
    }
}

impl DiscoveryMechanism for KubernetesDiscovery {
    fn announce(
        &self,
        self_knowledge: &SelfKnowledge,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + Send + '_>> {
        let name = self_knowledge.name.clone();
        let caps_joined = self_knowledge.capabilities.join(",");
        Box::pin(async move {
            tracing::info!("k8s announce: {}", name);

            // In Kubernetes, services are defined via manifests.
            // Announce is informational — the pod's labels/annotations should
            // already declare its capabilities in the deployment spec.
            tracing::info!(
                "Kubernetes services should be defined via manifests with labels: capabilities={}",
                caps_joined
            );

            Ok(())
        })
    }

    fn find_by_capability(
        &self,
        capability: Capability,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ServiceInfo>>> + Send + '_>> {
        let client = self.client.clone();
        let api_server = self.api_server.clone();
        let namespace = self.namespace.clone();
        Box::pin(async move {
            tracing::debug!("k8s query for capability: {:?}", capability);

            let url = format!(
                "{api_server}/api/v1/namespaces/{namespace}/services?labelSelector=capabilities={capability}"
            );

            let response = client.get(&url).await.map_err(|e| {
                nestgate_types::error::NestGateError::api_error(format!("k8s query failed: {e}"))
            })?;

            if !response.is_success() {
                return Ok(vec![]);
            }

            let service_list: K8sServiceList = response.json().map_err(|e| {
                nestgate_types::error::NestGateError::api_error(format!(
                    "Failed to parse k8s response: {e}"
                ))
            })?;

            Ok(service_list
                .items
                .into_iter()
                .filter_map(Self::service_to_info)
                .collect())
        })
    }

    fn find_by_id(
        &self,
        id: &str,
    ) -> Pin<Box<dyn Future<Output = Result<Option<ServiceInfo>>> + Send + '_>> {
        let client = self.client.clone();
        let api_server = self.api_server.clone();
        let namespace = self.namespace.clone();
        let id = id.to_string();
        Box::pin(async move {
            tracing::debug!("k8s lookup service: {}", id);

            let (name, ns) = if let Some((n, ns)) = id.split_once('.') {
                (n.to_string(), ns.to_string())
            } else {
                (id.clone(), namespace)
            };

            let url = format!("{api_server}/api/v1/namespaces/{ns}/services/{name}");

            let response = client.get(&url).await.map_err(|e| {
                nestgate_types::error::NestGateError::api_error(format!("k8s lookup failed: {e}"))
            })?;

            if !response.is_success() {
                return Ok(None);
            }

            let service: K8sService = response.json().map_err(|e| {
                nestgate_types::error::NestGateError::api_error(format!(
                    "Failed to parse k8s response: {e}"
                ))
            })?;

            Ok(Self::service_to_info(service))
        })
    }

    fn health_check(
        &self,
        service_id: &str,
    ) -> Pin<Box<dyn Future<Output = Result<bool>> + Send + '_>> {
        let client = self.client.clone();
        let api_server = self.api_server.clone();
        let namespace = self.namespace.clone();
        let service_id = service_id.to_string();
        Box::pin(async move {
            tracing::debug!("k8s health check: {}", service_id);

            let (name, ns) = if let Some((n, ns)) = service_id.split_once('.') {
                (n.to_string(), ns.to_string())
            } else {
                (service_id.clone(), namespace)
            };

            let url = format!("{api_server}/api/v1/namespaces/{ns}/endpoints/{name}");

            let response = client.get(&url).await.map_err(|e| {
                nestgate_types::error::NestGateError::api_error(format!(
                    "k8s health check failed: {e}"
                ))
            })?;

            Ok(response.is_success())
        })
    }

    fn deregister(
        &self,
        service_id: &str,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + Send + '_>> {
        let service_id = service_id.to_string();
        Box::pin(async move {
            tracing::info!("k8s deregister: {}", service_id);
            // In Kubernetes, services are managed by the control plane.
            // Deregistration happens when the pod terminates.
            tracing::info!("Kubernetes services are managed by k8s control plane");
            Ok(())
        })
    }

    fn mechanism_name(&self) -> &'static str {
        "kubernetes"
    }
}

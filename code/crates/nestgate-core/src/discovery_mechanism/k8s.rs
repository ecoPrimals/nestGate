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
use crate::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
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
    /// Prefers `KUBERNETES_API_PROXY` (e.g. `http://localhost:8001` from
    /// `kubectl proxy`). Falls back to constructing from `KUBERNETES_SERVICE_HOST`
    /// and `KUBERNETES_SERVICE_PORT` over HTTP.
    ///
    /// # Errors
    ///
    /// Returns an error if the client cannot be constructed.
    pub async fn new(builder: DiscoveryBuilder) -> Result<Self> {
        let namespace = std::env::var("NAMESPACE")
            .or_else(|_| std::env::var("POD_NAMESPACE"))
            .unwrap_or_else(|_| "default".to_string());

        let api_server = std::env::var("KUBERNETES_API_PROXY").unwrap_or_else(|_| {
            std::env::var("KUBERNETES_SERVICE_HOST")
                .map(|host| {
                    let port = std::env::var("KUBERNETES_SERVICE_PORT")
                        .unwrap_or_else(|_| "8001".to_string());
                    format!("http://{host}:{port}")
                })
                .unwrap_or_else(|_| "http://localhost:8001".to_string())
        });

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

    fn service_to_info(&self, svc: K8sService) -> Option<ServiceInfo> {
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

#[async_trait::async_trait]
impl DiscoveryMechanism for KubernetesDiscovery {
    async fn announce(&self, self_knowledge: &SelfKnowledge) -> Result<()> {
        tracing::info!("k8s announce: {}", self_knowledge.name);

        // In Kubernetes, services are defined via manifests.
        // Announce is informational — the pod's labels/annotations should
        // already declare its capabilities in the deployment spec.
        tracing::info!(
            "Kubernetes services should be defined via manifests with labels: capabilities={}",
            self_knowledge.capabilities.join(",")
        );

        Ok(())
    }

    async fn find_by_capability(&self, capability: Capability) -> Result<Vec<ServiceInfo>> {
        tracing::debug!("k8s query for capability: {:?}", capability);

        let url = format!(
            "{}/api/v1/namespaces/{}/services?labelSelector=capabilities={}",
            self.api_server, self.namespace, capability
        );

        let response = self.client.get(&url).await.map_err(|e| {
            crate::error::NestGateError::api_error(&format!("k8s query failed: {e}"))
        })?;

        if !response.is_success() {
            return Ok(vec![]);
        }

        let service_list: K8sServiceList = response.json().map_err(|e| {
            crate::error::NestGateError::api_error(&format!("Failed to parse k8s response: {e}"))
        })?;

        Ok(service_list
            .items
            .into_iter()
            .filter_map(|svc| self.service_to_info(svc))
            .collect())
    }

    async fn find_by_id(&self, id: &str) -> Result<Option<ServiceInfo>> {
        tracing::debug!("k8s lookup service: {}", id);

        let (name, ns) = if let Some((n, ns)) = id.split_once('.') {
            (n, ns.to_string())
        } else {
            (id, self.namespace.clone())
        };

        let url = format!(
            "{}/api/v1/namespaces/{}/services/{}",
            self.api_server, ns, name
        );

        let response = self.client.get(&url).await.map_err(|e| {
            crate::error::NestGateError::api_error(&format!("k8s lookup failed: {e}"))
        })?;

        if !response.is_success() {
            return Ok(None);
        }

        let service: K8sService = response.json().map_err(|e| {
            crate::error::NestGateError::api_error(&format!("Failed to parse k8s response: {e}"))
        })?;

        Ok(self.service_to_info(service))
    }

    async fn health_check(&self, service_id: &str) -> Result<bool> {
        tracing::debug!("k8s health check: {}", service_id);

        let (name, ns) = if let Some((n, ns)) = service_id.split_once('.') {
            (n, ns.to_string())
        } else {
            (service_id, self.namespace.clone())
        };

        let url = format!(
            "{}/api/v1/namespaces/{}/endpoints/{}",
            self.api_server, ns, name
        );

        let response = self.client.get(&url).await.map_err(|e| {
            crate::error::NestGateError::api_error(&format!("k8s health check failed: {e}"))
        })?;

        Ok(response.is_success())
    }

    async fn deregister(&self, service_id: &str) -> Result<()> {
        tracing::info!("k8s deregister: {}", service_id);
        // In Kubernetes, services are managed by the control plane.
        // Deregistration happens when the pod terminates.
        tracing::info!("Kubernetes services are managed by k8s control plane");
        Ok(())
    }

    fn mechanism_name(&self) -> &'static str {
        "kubernetes"
    }
}

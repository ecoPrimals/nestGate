//! Kubernetes-based discovery (orchestrated)
//!
//! This module provides Kubernetes service discovery integration.
//! Requires the `kubernetes` feature flag.

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

/// Kubernetes discovery mechanism (uses k8s REST API via reqwest)
pub struct KubernetesDiscovery {
    timeout: Duration,
    cache_duration: Duration,
    namespace: String,
    client: reqwest::Client,
    api_server: String,
    token: Option<String>,
}

impl KubernetesDiscovery {
    /// Create new Kubernetes discovery
    pub async fn new(builder: DiscoveryBuilder) -> Result<Self> {
        let namespace = std::env::var("NAMESPACE")
            .or_else(|_| std::env::var("POD_NAMESPACE"))
            .unwrap_or_else(|_| "default".to_string());

        // Get k8s API server address
        let api_server = std::env::var("KUBERNETES_SERVICE_HOST")
            .map(|host| {
                let port = std::env::var("KUBERNETES_SERVICE_PORT")
                    .unwrap_or_else(|_| "443".to_string());
                format!("https://{}:{}", host, port)
            })
            .unwrap_or_else(|_| "https://kubernetes.default.svc".to_string());

        // Get service account token (if running in pod)
        let token =
            std::fs::read_to_string("/var/run/secrets/kubernetes.io/serviceaccount/token").ok();

        let client = reqwest::Client::builder()
            .timeout(builder.timeout)
            .danger_accept_invalid_certs(true) // In-cluster certs are self-signed
            .build()
            .map_err(|e| {
                crate::error::NestGateError::config(&format!(
                    "Failed to create HTTP client: {}",
                    e
                ))
            })?;

        Ok(Self {
            timeout: builder.timeout,
            cache_duration: builder.cache_duration,
            namespace,
            client,
            api_server,
            token,
        })
    }

    /// Create authorization header
    fn auth_header(&self) -> Option<String> {
        self.token.as_ref().map(|token| format!("Bearer {}", token))
    }

    /// Convert k8s service to ServiceInfo
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
            endpoint: format!("http://{}:{}", ip, port),
            metadata: svc.metadata.annotations,
            health_endpoint: None,
        })
    }
}

#[async_trait::async_trait]
impl DiscoveryMechanism for KubernetesDiscovery {
    async fn announce(&self, self_knowledge: &SelfKnowledge) -> Result<()> {
        tracing::info!("k8s announce: {}", self_knowledge.name);

        // In Kubernetes, services are typically pre-created via manifests
        // This would update labels/annotations on an existing service
        // For now, we log that the service should be defined in k8s manifests

        tracing::info!(
            "Kubernetes services should be defined via manifests with labels: capabilities={}",
            self_knowledge.capabilities.join(",")
        );

        // In a full implementation, this could update the service's labels
        // via PATCH /api/v1/namespaces/{namespace}/services/{name}

        Ok(())
    }

    async fn find_by_capability(&self, capability: Capability) -> Result<Vec<ServiceInfo>> {
        tracing::debug!("k8s query for capability: {:?}", capability);

        let url = format!(
            "{}/api/v1/namespaces/{}/services?labelSelector=capabilities={}",
            self.api_server, self.namespace, capability
        );

        let mut req = self.client.get(&url);
        if let Some(auth) = self.auth_header() {
            req = req.header("Authorization", auth);
        }

        let response = req.send().await.map_err(|e| {
            crate::error::NestGateError::api_error(&format!("k8s query failed: {}", e))
        })?;

        if !response.status().is_success() {
            return Ok(vec![]);
        }

        let service_list: K8sServiceList = response.json().await.map_err(|e| {
            crate::error::NestGateError::api_error(&format!(
                "Failed to parse k8s response: {}",
                e
            ))
        })?;

        Ok(service_list
            .items
            .into_iter()
            .filter_map(|svc| self.service_to_info(svc))
            .collect())
    }

    async fn find_by_id(&self, id: &str) -> Result<Option<ServiceInfo>> {
        tracing::debug!("k8s lookup service: {}", id);

        // ID format: service-name.namespace
        let (name, ns) = if let Some((n, ns)) = id.split_once('.') {
            (n, ns.to_string())
        } else {
            (id, self.namespace.clone())
        };

        let url = format!(
            "{}/api/v1/namespaces/{}/services/{}",
            self.api_server, ns, name
        );

        let mut req = self.client.get(&url);
        if let Some(auth) = self.auth_header() {
            req = req.header("Authorization", auth);
        }

        let response = req.send().await.map_err(|e| {
            crate::error::NestGateError::api_error(&format!("k8s lookup failed: {}", e))
        })?;

        if !response.status().is_success() {
            return Ok(None);
        }

        let service: K8sService = response.json().await.map_err(|e| {
            crate::error::NestGateError::api_error(&format!(
                "Failed to parse k8s response: {}",
                e
            ))
        })?;

        Ok(self.service_to_info(service))
    }

    async fn health_check(&self, service_id: &str) -> Result<bool> {
        tracing::debug!("k8s health check: {}", service_id);

        // Check if service exists and has endpoints
        let (name, ns) = if let Some((n, ns)) = service_id.split_once('.') {
            (n, ns.to_string())
        } else {
            (service_id, self.namespace.clone())
        };

        let url = format!(
            "{}/api/v1/namespaces/{}/endpoints/{}",
            self.api_server, ns, name
        );

        let mut req = self.client.get(&url);
        if let Some(auth) = self.auth_header() {
            req = req.header("Authorization", auth);
        }

        let response = req.send().await.map_err(|e| {
            crate::error::NestGateError::api_error(&format!("k8s health check failed: {}", e))
        })?;

        Ok(response.status().is_success())
    }

    async fn deregister(&self, service_id: &str) -> Result<()> {
        tracing::info!("k8s deregister: {}", service_id);

        // In Kubernetes, services persist and are managed by k8s
        // Deregistration typically means the pod terminates and k8s removes it from endpoints
        // We just log this action

        tracing::info!("Kubernetes services are managed by k8s control plane");
        Ok(())
    }

    fn mechanism_name(&self) -> &'static str {
        "kubernetes"
    }
}

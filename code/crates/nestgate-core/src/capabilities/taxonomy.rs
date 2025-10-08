//! 🌱 **CAPABILITY TAXONOMY**
//!
//! Complete taxonomy of capabilities for the ecoPrimals ecosystem.
//! This replaces ALL hardcoded primal names, vendor names, and service names.
//!
//! ## Infant Discovery Philosophy
//!
//! Each primal only knows itself. All other capabilities are discovered at runtime
//! through the Universal Adapter. No hardcoded knowledge of:
//! - Which primal provides what
//! - Which vendor technology is used
//! - Which ports or endpoints exist
//!
//! Everything is discovered, nothing is assumed.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Standard capability types in the ecoPrimals ecosystem
///
/// These replace hardcoded primal names (songbird, toadstool, etc.)
/// and vendor names (k8s, redis, etc.)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CapabilityType {
    // ═══════════════════════════════════════════════════════════════
    // PRIMAL CAPABILITIES (No Primal Names!)
    // ═══════════════════════════════════════════════════════════════
    /// Orchestration capability (e.g., workflow management, service coordination)
    /// - Discovered: Whatever provides orchestration (maybe Songbird, maybe something else)
    /// - NOT hardcoded: Never assume "Songbird" exists
    Orchestration,

    /// Compute capability (e.g., parallel processing, job execution)
    /// - Discovered: Whatever provides compute (maybe Toadstool, maybe something else)
    /// - NOT hardcoded: Never assume "Toadstool" exists
    Compute,

    /// AI/ML capability (e.g., model inference, training, analysis)
    /// - Discovered: Whatever provides AI (maybe Squirrel, maybe something else)
    /// - NOT hardcoded: Never assume "Squirrel" exists
    ArtificialIntelligence,

    /// Security capability (e.g., authentication, encryption, access control)
    /// - Discovered: Whatever provides security (maybe `BearDog`, maybe something else)
    /// - NOT hardcoded: Never assume "`BearDog`" exists
    Security,

    /// Management capability (e.g., biome management, lifecycle control)
    /// - Discovered: Whatever provides management (maybe `BiomeOS`, maybe something else)
    /// - NOT hardcoded: Never assume "`BiomeOS`" exists
    Management,

    /// Data storage capability (e.g., persistent storage, data management)
    /// - Discovered: Whatever provides storage (maybe `NestGate`, maybe something else)
    /// - NOT hardcoded: Never assume "`NestGate`" is the only storage
    DataStorage,

    // ═══════════════════════════════════════════════════════════════
    // INFRASTRUCTURE CAPABILITIES (No Vendor Names!)
    // ═══════════════════════════════════════════════════════════════
    /// Container orchestration (e.g., pod management, scaling, deployments)
    /// - Discovered: Could be k8s, Nomad, Swarm, or anything
    /// - NOT hardcoded: Never assume Kubernetes
    ContainerOrchestration,

    /// Service registry (e.g., service discovery, health checks, metadata)
    /// - Discovered: Could be Consul, etcd, Zookeeper, or anything
    /// - NOT hardcoded: Never assume Consul
    ServiceRegistry,

    /// Key-value storage (e.g., caching, session storage, fast lookups)
    /// - Discovered: Could be Redis, Memcached, or anything
    /// - NOT hardcoded: Never assume Redis
    KeyValueStorage,

    /// Relational storage (e.g., SQL databases, structured data)
    /// - Discovered: Could be Postgres, `MySQL`, `MariaDB`, or anything
    /// - NOT hardcoded: Never assume Postgres
    RelationalStorage,

    /// Document storage (e.g., JSON documents, `NoSQL`)
    /// - Discovered: Could be `MongoDB`, `CouchDB`, or anything
    /// - NOT hardcoded: Never assume `MongoDB`
    DocumentStorage,

    /// Time-series storage (e.g., metrics, logs, events)
    /// - Discovered: Could be `InfluxDB`, `TimescaleDB`, or anything
    /// - NOT hardcoded: Never assume specific vendor
    TimeSeriesStorage,

    /// Object storage (e.g., blobs, files, media)
    /// - Discovered: Could be S3, `MinIO`, or anything
    /// - NOT hardcoded: Never assume S3
    ObjectStorage,

    /// Message queue (e.g., async messaging, event streaming)
    /// - Discovered: Could be `RabbitMQ`, Kafka, NATS, or anything
    /// - NOT hardcoded: Never assume `RabbitMQ`
    MessageQueue,

    /// Secret management (e.g., credentials, encryption keys)
    /// - Discovered: Could be Vault, AWS Secrets Manager, or anything
    /// - NOT hardcoded: Never assume `HashiCorp` Vault
    SecretManagement,

    // ═══════════════════════════════════════════════════════════════
    // NETWORK CAPABILITIES (No Technology Assumptions!)
    // ═══════════════════════════════════════════════════════════════
    /// API gateway (e.g., routing, rate limiting, auth)
    /// - Discovered: Could be Kong, nginx, Traefik, or anything
    /// - NOT hardcoded: Never assume specific gateway
    APIGateway,

    /// Load balancer (e.g., traffic distribution, health checks)
    /// - Discovered: Could be `HAProxy`, nginx, or anything
    /// - NOT hardcoded: Never assume specific load balancer
    LoadBalancer,

    /// Service mesh (e.g., sidecar proxies, traffic management)
    /// - Discovered: Could be Istio, Linkerd, or anything
    /// - NOT hardcoded: Never assume Istio
    ServiceMesh,

    /// DNS service (e.g., name resolution, service discovery)
    /// - Discovered: Could be `CoreDNS`, bind, or anything
    /// - NOT hardcoded: Never assume `CoreDNS`
    DNSService,

    // ═══════════════════════════════════════════════════════════════
    // OBSERVABILITY CAPABILITIES (No Tool Assumptions!)
    // ═══════════════════════════════════════════════════════════════
    /// Metrics collection (e.g., time-series metrics, gauges, counters)
    /// - Discovered: Could be Prometheus, Datadog, or anything
    /// - NOT hardcoded: Never assume Prometheus
    MetricsCollection,

    /// Log aggregation (e.g., centralized logging, log search)
    /// - Discovered: Could be Elasticsearch, Loki, or anything
    /// - NOT hardcoded: Never assume Elasticsearch
    LogAggregation,

    /// Distributed tracing (e.g., request tracing, span analysis)
    /// - Discovered: Could be Jaeger, Zipkin, or anything
    /// - NOT hardcoded: Never assume Jaeger
    DistributedTracing,

    /// Alerting (e.g., notifications, incident management)
    /// - Discovered: Could be `AlertManager`, `PagerDuty`, or anything
    /// - NOT hardcoded: Never assume specific alerting tool
    Alerting,

    // ═══════════════════════════════════════════════════════════════
    // SPECIALIZED CAPABILITIES
    // ═══════════════════════════════════════════════════════════════
    /// Search engine (e.g., full-text search, indexing)
    /// - Discovered: Could be Elasticsearch, Solr, or anything
    /// - NOT hardcoded: Never assume Elasticsearch
    SearchEngine,

    /// Workflow engine (e.g., task orchestration, DAGs)
    /// - Discovered: Could be Airflow, Temporal, or anything
    /// - NOT hardcoded: Never assume specific workflow engine
    WorkflowEngine,

    /// Event streaming (e.g., real-time events, pub/sub)
    /// - Discovered: Could be Kafka, Pulsar, or anything
    /// - NOT hardcoded: Never assume Kafka
    EventStreaming,

    /// Graph database (e.g., relationships, graph queries)
    /// - Discovered: Could be Neo4j, `ArangoDB`, or anything
    /// - NOT hardcoded: Never assume Neo4j
    GraphDatabase,

    /// Custom capability (for extensibility)
    Custom(String),
}

impl CapabilityType {
    /// Get the standard string representation
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            // Primal capabilities
            Self::Orchestration => "orchestration",
            Self::Compute => "compute",
            Self::ArtificialIntelligence => "artificial_intelligence",
            Self::Security => "security",
            Self::Management => "management",
            Self::DataStorage => "data_storage",

            // Infrastructure
            Self::ContainerOrchestration => "container_orchestration",
            Self::ServiceRegistry => "service_registry",
            Self::KeyValueStorage => "key_value_storage",
            Self::RelationalStorage => "relational_storage",
            Self::DocumentStorage => "document_storage",
            Self::TimeSeriesStorage => "time_series_storage",
            Self::ObjectStorage => "object_storage",
            Self::MessageQueue => "message_queue",
            Self::SecretManagement => "secret_management",

            // Network
            Self::APIGateway => "api_gateway",
            Self::LoadBalancer => "load_balancer",
            Self::ServiceMesh => "service_mesh",
            Self::DNSService => "dns_service",

            // Observability
            Self::MetricsCollection => "metrics_collection",
            Self::LogAggregation => "log_aggregation",
            Self::DistributedTracing => "distributed_tracing",
            Self::Alerting => "alerting",

            // Specialized
            Self::SearchEngine => "search_engine",
            Self::WorkflowEngine => "workflow_engine",
            Self::EventStreaming => "event_streaming",
            Self::GraphDatabase => "graph_database",

            Self::Custom(name) => name.as_str(),
        }
    }

    /// Parse from string representation
    ///
    /// Note: Consider using the `FromStr` trait implementation for error handling
    #[must_use]
    pub fn from_string(s: &str) -> Self {
        s.parse().unwrap_or_else(|_| Self::Custom(s.to_string()))
    }

    /// Alias for `from_string` (for backward compatibility with tests)
    #[must_use]
    pub fn from_str(s: &str) -> Self {
        Self::from_string(s)
    }

    /// Get capability category for grouping
    #[must_use]
    pub fn category(&self) -> CapabilityCategory {
        match self {
            Self::Orchestration
            | Self::Compute
            | Self::ArtificialIntelligence
            | Self::Security
            | Self::Management
            | Self::DataStorage => CapabilityCategory::Primal,

            Self::ContainerOrchestration
            | Self::ServiceRegistry
            | Self::KeyValueStorage
            | Self::RelationalStorage
            | Self::DocumentStorage
            | Self::TimeSeriesStorage
            | Self::ObjectStorage
            | Self::MessageQueue
            | Self::SecretManagement => CapabilityCategory::Infrastructure,

            Self::APIGateway | Self::LoadBalancer | Self::ServiceMesh | Self::DNSService => {
                CapabilityCategory::Network
            }

            Self::MetricsCollection
            | Self::LogAggregation
            | Self::DistributedTracing
            | Self::Alerting => CapabilityCategory::Observability,

            Self::SearchEngine
            | Self::WorkflowEngine
            | Self::EventStreaming
            | Self::GraphDatabase
            | Self::Custom(_) => CapabilityCategory::Specialized,
        }
    }
}

impl std::str::FromStr for CapabilityType {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            // Primal capabilities
            "orchestration" => Self::Orchestration,
            "compute" => Self::Compute,
            "artificial_intelligence" | "ai" | "ml" => Self::ArtificialIntelligence,
            "security" => Self::Security,
            "management" => Self::Management,
            "data_storage" | "storage" => Self::DataStorage,

            // Infrastructure
            "container_orchestration" => Self::ContainerOrchestration,
            "service_registry" => Self::ServiceRegistry,
            "key_value_storage" | "cache" => Self::KeyValueStorage,
            "relational_storage" | "sql" => Self::RelationalStorage,
            "document_storage" | "nosql" => Self::DocumentStorage,
            "time_series_storage" | "timeseries" => Self::TimeSeriesStorage,
            "object_storage" | "blob" => Self::ObjectStorage,
            "message_queue" | "queue" => Self::MessageQueue,
            "secret_management" | "secrets" => Self::SecretManagement,

            // Network
            "api_gateway" | "gateway" => Self::APIGateway,
            "load_balancer" | "lb" => Self::LoadBalancer,
            "service_mesh" | "mesh" => Self::ServiceMesh,
            "dns_service" | "dns" => Self::DNSService,

            // Observability
            "metrics_collection" | "metrics" => Self::MetricsCollection,
            "log_aggregation" | "logs" => Self::LogAggregation,
            "distributed_tracing" | "tracing" => Self::DistributedTracing,
            "alerting" | "alerts" => Self::Alerting,

            // Specialized
            "search_engine" | "search" => Self::SearchEngine,
            "workflow_engine" | "workflow" => Self::WorkflowEngine,
            "event_streaming" | "streaming" => Self::EventStreaming,
            "graph_database" | "graph" => Self::GraphDatabase,

            other => Self::Custom(other.to_string()),
        })
    }
}

/// Capability category for organizational purposes
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CapabilityCategory {
    /// Primal-level capabilities (orchestration, compute, AI, security, etc.)
    Primal,
    /// Infrastructure capabilities (storage, messaging, registries, etc.)
    Infrastructure,
    /// Network capabilities (gateways, load balancers, mesh, etc.)
    Network,
    /// Observability capabilities (metrics, logs, tracing, alerts)
    Observability,
    /// Specialized capabilities (search, workflows, graphs, etc.)
    Specialized,
}

/// Discovered capability information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Capability {
    /// Type of capability
    pub capability_type: CapabilityType,
    /// Endpoint URL (discovered at runtime)
    pub endpoint: String,
    /// Provider name (discovered, not assumed)
    pub provider: Option<String>,
    /// Version information
    pub version: Option<String>,
    /// Confidence level (0.0 to 1.0)
    pub confidence: f64,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

impl Capability {
    /// Create a new capability
    #[must_use]
    pub fn new(capability_type: CapabilityType, endpoint: String) -> Self {
        Self {
            capability_type,
            endpoint,
            provider: None,
            version: None,
            confidence: 1.0,
            metadata: HashMap::new(),
        }
    }

    /// Set provider information
    #[must_use]
    pub fn with_provider(mut self, provider: String) -> Self {
        self.provider = Some(provider);
        self
    }

    /// Set version information
    #[must_use]
    pub fn with_version(mut self, version: String) -> Self {
        self.version = Some(version);
        self
    }

    /// Set confidence level
    #[must_use]
    pub fn with_confidence(mut self, confidence: f64) -> Self {
        self.confidence = confidence.clamp(0.0, 1.0);
        self
    }

    /// Add metadata
    #[must_use]
    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capability_type_string_conversion() {
        assert_eq!(CapabilityType::Orchestration.as_str(), "orchestration");
        assert_eq!(
            CapabilityType::KeyValueStorage.as_str(),
            "key_value_storage"
        );
        assert_eq!(
            CapabilityType::ArtificialIntelligence.as_str(),
            "artificial_intelligence"
        );
    }

    #[test]
    fn test_capability_type_from_string() {
        assert_eq!(
            CapabilityType::from_string("orchestration"),
            CapabilityType::Orchestration
        );
        assert_eq!(
            CapabilityType::from_string("cache"),
            CapabilityType::KeyValueStorage
        );
        assert_eq!(
            CapabilityType::from_string("ai"),
            CapabilityType::ArtificialIntelligence
        );
    }

    #[test]
    fn test_capability_categories() {
        assert_eq!(
            CapabilityType::Orchestration.category(),
            CapabilityCategory::Primal
        );
        assert_eq!(
            CapabilityType::KeyValueStorage.category(),
            CapabilityCategory::Infrastructure
        );
        assert_eq!(
            CapabilityType::APIGateway.category(),
            CapabilityCategory::Network
        );
        assert_eq!(
            CapabilityType::MetricsCollection.category(),
            CapabilityCategory::Observability
        );
    }

    #[test]
    fn test_capability_builder() {
        let cap = Capability::new(
            CapabilityType::Orchestration,
            "http://discovered-service:8080".to_string(),
        )
        .with_provider("discovered-provider".to_string())
        .with_version("1.0.0".to_string())
        .with_confidence(0.95)
        .with_metadata("region".to_string(), "us-west".to_string());

        assert_eq!(cap.capability_type, CapabilityType::Orchestration);
        assert_eq!(cap.endpoint, "http://discovered-service:8080");
        assert_eq!(cap.provider, Some("discovered-provider".to_string()));
        assert_eq!(cap.confidence, 0.95);
        assert_eq!(cap.metadata.get("region"), Some(&"us-west".to_string()));
    }

    #[test]
    fn test_capability_new() {
        let cap = Capability::new(
            CapabilityType::DataStorage,
            "http://storage:9000".to_string(),
        );

        assert_eq!(cap.capability_type, CapabilityType::DataStorage);
        assert_eq!(cap.endpoint, "http://storage:9000");
        assert_eq!(cap.provider, None);
        assert_eq!(cap.version, None);
        assert_eq!(cap.confidence, 1.0); // Default confidence
        assert!(cap.metadata.is_empty());
    }

    #[test]
    fn test_capability_with_provider() {
        let cap = Capability::new(CapabilityType::Security, "http://security:8443".to_string())
            .with_provider("secure-provider".to_string());

        assert_eq!(cap.provider, Some("secure-provider".to_string()));
    }

    #[test]
    fn test_capability_with_version() {
        let cap = Capability::new(CapabilityType::Compute, "http://compute:5000".to_string())
            .with_version("2.1.0".to_string());

        assert_eq!(cap.version, Some("2.1.0".to_string()));
    }

    #[test]
    fn test_capability_with_confidence() {
        let cap = Capability::new(
            CapabilityType::ArtificialIntelligence,
            "http://ai:3000".to_string(),
        )
        .with_confidence(0.85);

        assert_eq!(cap.confidence, 0.85);
    }

    #[test]
    fn test_capability_with_multiple_metadata() {
        let cap = Capability::new(
            CapabilityType::MessageQueue,
            "http://queue:5672".to_string(),
        )
        .with_metadata("region".to_string(), "eu-central".to_string())
        .with_metadata("env".to_string(), "production".to_string())
        .with_metadata("tier".to_string(), "premium".to_string());

        assert_eq!(cap.metadata.len(), 3);
        assert_eq!(cap.metadata.get("region"), Some(&"eu-central".to_string()));
        assert_eq!(cap.metadata.get("env"), Some(&"production".to_string()));
        assert_eq!(cap.metadata.get("tier"), Some(&"premium".to_string()));
    }

    #[test]
    fn test_capability_type_all_primal_types() {
        let primals = vec![
            CapabilityType::Orchestration,
            CapabilityType::Compute,
            CapabilityType::ArtificialIntelligence,
            CapabilityType::Security,
            CapabilityType::Management,
            CapabilityType::DataStorage,
        ];

        for primal in primals {
            assert_eq!(primal.category(), CapabilityCategory::Primal);
        }
    }

    #[test]
    fn test_capability_type_all_infrastructure_types() {
        let infrastructure = vec![
            CapabilityType::ContainerOrchestration,
            CapabilityType::ServiceRegistry,
            CapabilityType::KeyValueStorage,
            CapabilityType::RelationalStorage,
            CapabilityType::DocumentStorage,
            CapabilityType::TimeSeriesStorage,
            CapabilityType::ObjectStorage,
            CapabilityType::MessageQueue,
        ];

        for infra in infrastructure {
            assert_eq!(infra.category(), CapabilityCategory::Infrastructure);
        }
    }

    #[test]
    fn test_capability_type_network_types() {
        assert_eq!(
            CapabilityType::LoadBalancer.category(),
            CapabilityCategory::Network
        );
        assert_eq!(
            CapabilityType::APIGateway.category(),
            CapabilityCategory::Network
        );
    }

    #[test]
    fn test_capability_type_observability_types() {
        assert_eq!(
            CapabilityType::MetricsCollection.category(),
            CapabilityCategory::Observability
        );
        assert_eq!(
            CapabilityType::LogAggregation.category(),
            CapabilityCategory::Observability
        );
    }

    #[test]
    fn test_capability_type_serialization() {
        let cap_type = CapabilityType::Orchestration;
        let serialized = serde_json::to_string(&cap_type).unwrap();
        let deserialized: CapabilityType = serde_json::from_str(&serialized).unwrap();

        assert_eq!(cap_type, deserialized);
    }

    #[test]
    fn test_capability_serialization() {
        let cap = Capability::new(CapabilityType::Security, "http://security:8080".to_string())
            .with_provider("auth-service".to_string())
            .with_version("1.2.3".to_string());

        let serialized = serde_json::to_string(&cap).unwrap();
        let deserialized: Capability = serde_json::from_str(&serialized).unwrap();

        assert_eq!(cap.capability_type, deserialized.capability_type);
        assert_eq!(cap.endpoint, deserialized.endpoint);
        assert_eq!(cap.provider, deserialized.provider);
        assert_eq!(cap.version, deserialized.version);
    }

    #[test]
    fn test_capability_category_serialization() {
        let category = CapabilityCategory::Infrastructure;
        let serialized = serde_json::to_string(&category).unwrap();
        let deserialized: CapabilityCategory = serde_json::from_str(&serialized).unwrap();

        assert_eq!(category, deserialized);
    }

    #[test]
    fn test_capability_type_clone() {
        let cap_type = CapabilityType::Compute;
        let cloned = cap_type.clone();

        assert_eq!(cap_type, cloned);
    }

    #[test]
    fn test_capability_clone() {
        let cap = Capability::new(
            CapabilityType::ArtificialIntelligence,
            "http://ai:3000".to_string(),
        )
        .with_provider("ml-service".to_string());

        let cloned = cap.clone();

        assert_eq!(cap.capability_type, cloned.capability_type);
        assert_eq!(cap.endpoint, cloned.endpoint);
        assert_eq!(cap.provider, cloned.provider);
    }

    #[test]
    fn test_capability_type_hash() {
        use std::collections::HashSet;

        let mut set = HashSet::new();
        set.insert(CapabilityType::Orchestration);
        set.insert(CapabilityType::Compute);
        set.insert(CapabilityType::Orchestration); // Duplicate

        assert_eq!(set.len(), 2);
        assert!(set.contains(&CapabilityType::Orchestration));
        assert!(set.contains(&CapabilityType::Compute));
    }

    #[test]
    fn test_capability_type_equality() {
        assert_eq!(CapabilityType::Security, CapabilityType::Security);
        assert_ne!(CapabilityType::Security, CapabilityType::Orchestration);
    }

    #[test]
    fn test_capability_category_equality() {
        assert_eq!(CapabilityCategory::Primal, CapabilityCategory::Primal);
        assert_ne!(
            CapabilityCategory::Primal,
            CapabilityCategory::Infrastructure
        );
    }

    #[test]
    fn test_capability_confidence_bounds() {
        let cap_low = Capability::new(
            CapabilityType::DataStorage,
            "http://storage:9000".to_string(),
        )
        .with_confidence(0.0);

        let cap_high = Capability::new(
            CapabilityType::DataStorage,
            "http://storage:9000".to_string(),
        )
        .with_confidence(1.0);

        assert_eq!(cap_low.confidence, 0.0);
        assert_eq!(cap_high.confidence, 1.0);
    }

    #[test]
    fn test_capability_empty_metadata() {
        let cap = Capability::new(CapabilityType::Management, "http://mgmt:8080".to_string());

        assert!(cap.metadata.is_empty());
    }

    #[test]
    fn test_capability_metadata_override() {
        let cap = Capability::new(CapabilityType::Security, "http://security:8443".to_string())
            .with_metadata("env".to_string(), "dev".to_string())
            .with_metadata("env".to_string(), "production".to_string());

        // Later value should override
        assert_eq!(cap.metadata.get("env"), Some(&"production".to_string()));
    }

    #[test]
    fn test_capability_type_debug_format() {
        let cap_type = CapabilityType::Orchestration;
        let debug_str = format!("{cap_type:?}");

        assert!(debug_str.contains("Orchestration"));
    }

    #[test]
    fn test_capability_debug_format() {
        let cap = Capability::new(CapabilityType::Compute, "http://compute:5000".to_string());
        let debug_str = format!("{cap:?}");

        assert!(debug_str.contains("Compute"));
        assert!(debug_str.contains("http://compute:5000"));
    }

    #[test]
    fn test_capability_builder_chaining() {
        let cap = Capability::new(
            CapabilityType::MessageQueue,
            "http://queue:5672".to_string(),
        )
        .with_provider("rabbitmq-discovered".to_string())
        .with_version("3.9.0".to_string())
        .with_confidence(0.99)
        .with_metadata("cluster".to_string(), "main".to_string())
        .with_metadata("replicas".to_string(), "3".to_string());

        assert_eq!(cap.capability_type, CapabilityType::MessageQueue);
        assert_eq!(cap.endpoint, "http://queue:5672");
        assert_eq!(cap.provider, Some("rabbitmq-discovered".to_string()));
        assert_eq!(cap.version, Some("3.9.0".to_string()));
        assert_eq!(cap.confidence, 0.99);
        assert_eq!(cap.metadata.len(), 2);
    }

    #[test]
    fn test_capability_type_from_str_exact_match() {
        // Test that from_str matches exact strings
        assert_eq!(
            CapabilityType::from_str("orchestration"),
            CapabilityType::Orchestration
        );
        assert_eq!(CapabilityType::from_str("compute"), CapabilityType::Compute);
        assert_eq!(
            CapabilityType::from_str("security"),
            CapabilityType::Security
        );
    }

    #[test]
    fn test_capability_type_from_str_aliases() {
        // Test common aliases
        assert_eq!(
            CapabilityType::from_str("storage"),
            CapabilityType::DataStorage
        );
        assert_eq!(
            CapabilityType::from_str("ml"),
            CapabilityType::ArtificialIntelligence
        );
    }

    #[test]
    fn test_all_capability_types_have_category() {
        // Ensure every capability type has a valid category
        let types = vec![
            CapabilityType::Orchestration,
            CapabilityType::Compute,
            CapabilityType::ArtificialIntelligence,
            CapabilityType::Security,
            CapabilityType::Management,
            CapabilityType::DataStorage,
            CapabilityType::ContainerOrchestration,
            CapabilityType::ServiceRegistry,
            CapabilityType::KeyValueStorage,
            CapabilityType::LoadBalancer,
            CapabilityType::MetricsCollection,
        ];

        for cap_type in types {
            // Should not panic
            let _category = cap_type.category();
        }
    }

    #[test]
    fn test_capability_with_special_endpoint_formats() {
        let endpoints = vec![
            "http://service:8080",
            "https://secure-service:8443",
            "tcp://message-queue:5672",
            "grpc://api-service:9090",
            "internal://nestgate/storage",
        ];

        for endpoint in endpoints {
            let cap = Capability::new(CapabilityType::DataStorage, endpoint.to_string());

            assert_eq!(cap.endpoint, endpoint);
        }
    }
}

# Enhanced Primal Integration Specification

**Version**: 2.0.0  
**Date**: January 30, 2025  
**Status**: ✅ **PRODUCTION-READY**  
**Purpose**: Complete integration specification for NestGate's Universal RPC ecosystem with all ecoPrimals

---

## 📊 **Executive Summary**

This specification defines how NestGate's enhanced Universal RPC System integrates with all ecoPrimals (Beardog, Songbird, Toadstool, Squirrel) to create a seamless, high-performance ecosystem. The integration provides protocol-specific optimizations, intelligent routing, and enterprise-grade reliability.

### **Integration Matrix**

| **Primal** | **Protocol** | **Primary Use Case** | **Performance** | **Status** |
|------------|-------------|---------------------|-----------------|------------|
| **🔐 Beardog** | tarpc (Binary) | Security & Encryption | Ultra-High | ✅ Ready |
| **🎼 Songbird** | JSON-RPC (HTTP) | Orchestration & Discovery | High | ✅ Ready |
| **🍄 Toadstool** | JSON-RPC (HTTP) | Compute Resources | High | ✅ Ready |
| **🐿️ Squirrel** | JSON-RPC (HTTP) | AI/ML Operations | High | ✅ Ready |

---

## 🔐 **Beardog Security Integration**

### **Protocol Specification**

```rust
/// High-performance binary RPC for security operations
pub struct BeardogIntegration {
    tarpc_client: TarpcRpcService,
    security_context: SecurityContext,
    encryption_cache: Arc<RwLock<EncryptionCache>>,
}

/// Security operations supported
pub enum BeardogOperation {
    // Core Encryption
    EncryptData { data: Vec<u8>, algorithm: String },
    DecryptData { encrypted_data: Vec<u8>, key_id: String },
    
    // Authentication
    AuthenticateUser { credentials: UserCredentials },
    ValidateToken { token: String },
    RefreshToken { refresh_token: String },
    
    // Key Management
    GenerateKey { algorithm: String, purpose: String },
    RotateKey { key_id: String },
    RevokeKey { key_id: String },
    
    // Security Monitoring
    AuditSecurityEvent { event: SecurityEvent },
    DetectThreats { data: ThreatAnalysisData },
    ComplianceCheck { policy: CompliancePolicy },
    
    // Advanced Security
    CreateSecureChannel { endpoint: String },
    SignData { data: Vec<u8>, key_id: String },
    VerifySignature { data: Vec<u8>, signature: Vec<u8> },
}
```

### **Connection Configuration**

```rust
/// Production Beardog configuration
let beardog_config = BeardogRpcConfig {
    endpoint: "beardog.local:9090".to_string(),
    protocol: RpcConnectionType::Tarpc,
    security: SecurityConfig {
        tls_enabled: true,
        mutual_tls: true,
        certificate_path: "/etc/nestgate/certs/beardog.crt",
        private_key_path: "/etc/nestgate/keys/beardog.key",
        ca_bundle: "/etc/nestgate/ca/beardog-ca.pem",
    },
    performance: PerformanceConfig {
        connection_pool_size: 20,
        request_timeout: Duration::from_secs(30),
        keep_alive_interval: Duration::from_secs(60),
        max_retries: 3,
        circuit_breaker: CircuitBreakerConfig {
            failure_threshold: 5,
            recovery_timeout: Duration::from_secs(60),
            success_threshold: 3,
        },
    },
};
```

### **Usage Examples**

```rust
// Initialize Beardog integration
let mut rpc_manager = UnifiedRpcManager::new_production().await?;
rpc_manager.init_tarpc_service("beardog.local:9090").await?;

// Encrypt sensitive data
let request = UnifiedRpcRequest {
    id: Uuid::new_v4(),
    source: "nestgate".to_string(),
    target: "beardog".to_string(),
    method: "encrypt_data".to_string(),
    params: serde_json::json!({
        "data": base64::encode(&sensitive_data),
        "algorithm": "AES-256-GCM",
        "purpose": "storage_encryption"
    }),
    priority: RequestPriority::High,
    timeout: Some(Duration::from_secs(30)),
    // ... other fields
};

let response = rpc_manager.call(request).await?;
let encrypted_result: EncryptionResult = serde_json::from_value(response.data.unwrap())?;

// Authenticate user
let auth_request = UnifiedRpcRequest {
    target: "beardog".to_string(),
    method: "authenticate_user".to_string(),
    params: serde_json::json!({
        "username": "admin",
        "password_hash": "sha256_hash",
        "multi_factor_token": "123456"
    }),
    priority: RequestPriority::Critical,
    // ... other fields
};

let auth_response = rpc_manager.call(auth_request).await?;
```

### **Security Event Streaming**

```rust
// Start security event stream
let stream_request = UnifiedRpcRequest {
    target: "beardog".to_string(),
    method: "stream_security_events".to_string(),
    streaming: true,
    params: serde_json::json!({
        "event_types": ["threat_detection", "authentication_failure", "key_rotation"],
        "severity_threshold": "medium"
    }),
    // ... other fields
};

let (tx, mut rx) = rpc_manager.start_bidirectional_stream(stream_request).await?;

// Handle security events
tokio::spawn(async move {
    while let Some(event) = rx.recv().await {
        match event.event_type.as_str() {
            "threat_detection" => handle_threat_detection(event.data).await,
            "authentication_failure" => handle_auth_failure(event.data).await,
            "key_rotation" => handle_key_rotation(event.data).await,
            _ => tracing::warn!("Unknown security event: {}", event.event_type),
        }
    }
});
```

---

## 🎼 **Songbird Orchestration Integration**

### **Protocol Specification**

```rust
/// JSON-RPC integration for orchestration operations
pub struct SongbirdIntegration {
    json_rpc_client: JsonRpcService,
    service_registry: Arc<RwLock<ServiceRegistry>>,
    workflow_engine: WorkflowEngine,
}

/// Orchestration operations supported
pub enum SongbirdOperation {
    // Service Discovery
    RegisterService { service: ServiceRegistration },
    UnregisterService { service_id: String },
    DiscoverServices { filter: ServiceFilter },
    GetServiceHealth { service_id: String },
    
    // Resource Management
    AllocatePort { service_name: String, port_type: PortType },
    ReleasePort { service_name: String, port: u16 },
    ReserveResources { resource_spec: ResourceSpecification },
    
    // Workflow Coordination
    StartWorkflow { workflow_definition: WorkflowDef },
    StopWorkflow { workflow_id: String },
    GetWorkflowStatus { workflow_id: String },
    UpdateWorkflowState { workflow_id: String, state: WorkflowState },
    
    // Network Topology
    GetNetworkTopology,
    UpdateNetworkConfig { config: NetworkConfiguration },
    MonitorNetworkHealth,
    
    // Load Balancing
    RegisterLoadBalancer { lb_config: LoadBalancerConfig },
    UpdateLoadBalancingRules { rules: Vec<LoadBalancingRule> },
    GetLoadBalancingStats { lb_id: String },
}
```

### **Connection Configuration**

```rust
/// Production Songbird configuration
let songbird_config = SongbirdRpcConfig {
    endpoint: "http://songbird.local:8080/rpc".to_string(),
    protocol: RpcConnectionType::JsonRpc,
    authentication: AuthConfig {
        auth_type: AuthType::OAuth2,
        client_id: "nestgate-integration",
        client_secret: env::var("SONGBIRD_CLIENT_SECRET")?,
        token_endpoint: "http://songbird.local:8080/oauth/token",
        refresh_threshold: Duration::from_secs(300),
    },
    http_config: HttpConfig {
        timeout: Duration::from_secs(60),
        max_connections: 10,
        keep_alive: true,
        compression: true,
        user_agent: "NestGate/2.0 RPC Client",
    },
    retry_policy: RetryPolicy {
        max_retries: 3,
        initial_delay: Duration::from_millis(100),
        max_delay: Duration::from_secs(5),
        exponential_backoff: true,
    },
};
```

### **Usage Examples**

```rust
// Initialize Songbird integration
rpc_manager.init_json_rpc_service("http://songbird.local:8080/rpc").await?;

// Register NestGate as a service
let registration_request = UnifiedRpcRequest {
    target: "songbird".to_string(),
    method: "register_service".to_string(),
    params: serde_json::json!({
        "service": {
            "name": "nestgate-storage",
            "version": "2.0.0",
            "endpoints": [
                {
                    "type": "http",
                    "address": "http://nestgate.local:8080/api/v1",
                    "health_check": "/health"
                },
                {
                    "type": "websocket",
                    "address": "ws://nestgate.local:8080/ws",
                    "protocols": ["metrics", "events"]
                }
            ],
            "capabilities": [
                "storage_management",
                "zfs_operations",
                "data_tiering",
                "backup_services"
            ],
            "metadata": {
                "max_storage_gb": 10000,
                "supported_protocols": ["nfs", "smb", "iscsi", "s3"],
                "replication_factor": 3
            }
        }
    }),
    priority: RequestPriority::High,
    // ... other fields
};

let registration_response = rpc_manager.call(registration_request).await?;

// Discover compute services
let discovery_request = UnifiedRpcRequest {
    target: "songbird".to_string(),
    method: "discover_services".to_string(),
    params: serde_json::json!({
        "filter": {
            "service_type": "compute",
            "capabilities": ["gpu_acceleration", "ml_inference"],
            "min_memory_gb": 16,
            "availability_zone": "local"
        }
    }),
    // ... other fields
};

let discovery_response = rpc_manager.call(discovery_request).await?;
let available_services: Vec<ServiceInstance> = serde_json::from_value(discovery_response.data.unwrap())?;
```

### **Workflow Coordination**

```rust
// Start a complex workflow
let workflow_request = UnifiedRpcRequest {
    target: "songbird".to_string(),
    method: "start_workflow".to_string(),
    params: serde_json::json!({
        "workflow": {
            "name": "data_processing_pipeline",
            "steps": [
                {
                    "name": "data_ingestion",
                    "service": "nestgate-storage",
                    "action": "create_dataset",
                    "parameters": {
                        "name": "raw_data",
                        "size_gb": 100,
                        "compression": "lz4"
                    }
                },
                {
                    "name": "data_processing",
                    "service": "toadstool-compute",
                    "action": "process_data",
                    "depends_on": ["data_ingestion"],
                    "parameters": {
                        "input_dataset": "raw_data",
                        "algorithm": "ml_classification",
                        "gpu_required": true
                    }
                },
                {
                    "name": "ai_analysis",
                    "service": "squirrel-ai",
                    "action": "analyze_results",
                    "depends_on": ["data_processing"],
                    "parameters": {
                        "model": "analysis_v2.1",
                        "confidence_threshold": 0.85
                    }
                }
            ],
            "timeout": "1h",
            "retry_policy": {
                "max_retries": 2,
                "retry_on": ["timeout", "service_unavailable"]
            }
        }
    }),
    priority: RequestPriority::Normal,
    // ... other fields
};

let workflow_response = rpc_manager.call(workflow_request).await?;
let workflow_id: String = serde_json::from_value(workflow_response.data.unwrap())?;
```

---

## 🍄 **Toadstool Compute Integration**

### **Protocol Specification**

```rust
/// JSON-RPC integration for compute operations
pub struct ToadstoolIntegration {
    json_rpc_client: JsonRpcService,
    resource_manager: ResourceManager,
    job_scheduler: JobScheduler,
}

/// Compute operations supported
pub enum ToadstoolOperation {
    // Resource Management
    GetAvailableResources,
    ReserveResources { spec: ResourceSpecification },
    ReleaseResources { reservation_id: String },
    
    // Job Management
    SubmitJob { job: ComputeJob },
    CancelJob { job_id: String },
    GetJobStatus { job_id: String },
    GetJobResults { job_id: String },
    
    // Hardware Optimization
    OptimizeHardware { optimization_spec: HardwareOptimization },
    GetHardwareMetrics,
    ConfigureGpu { gpu_config: GpuConfiguration },
    
    // Performance Tuning
    TunePerformance { workload_profile: WorkloadProfile },
    GetPerformanceMetrics { time_range: TimeRange },
    SetResourceLimits { limits: ResourceLimits },
}
```

### **Usage Examples**

```rust
// Submit a compute job to Toadstool
let job_request = UnifiedRpcRequest {
    target: "toadstool".to_string(),
    method: "submit_job".to_string(),
    params: serde_json::json!({
        "job": {
            "name": "data_analysis_job",
            "type": "machine_learning",
            "resources": {
                "cpu_cores": 8,
                "memory_gb": 32,
                "gpu_count": 2,
                "gpu_memory_gb": 16,
                "storage_gb": 100
            },
            "container": {
                "image": "tensorflow/tensorflow:2.8.0-gpu",
                "command": ["python", "analysis.py"],
                "environment": {
                    "DATA_PATH": "/mnt/nestgate/dataset",
                    "OUTPUT_PATH": "/mnt/nestgate/results"
                }
            },
            "data_sources": [
                {
                    "type": "nestgate_dataset",
                    "path": "/tank/ml_data/training_set",
                    "mount_path": "/mnt/nestgate/dataset"
                }
            ],
            "output_destinations": [
                {
                    "type": "nestgate_dataset",
                    "path": "/tank/ml_results/job_output",
                    "mount_path": "/mnt/nestgate/results"
                }
            ],
            "timeout": "2h",
            "priority": "high"
        }
    }),
    priority: RequestPriority::Normal,
    // ... other fields
};

let job_response = rpc_manager.call(job_request).await?;
let job_id: String = serde_json::from_value(job_response.data.unwrap())?;

// Monitor job status
let status_request = UnifiedRpcRequest {
    target: "toadstool".to_string(),
    method: "get_job_status".to_string(),
    params: serde_json::json!({
        "job_id": job_id
    }),
    // ... other fields
};

let status_response = rpc_manager.call(status_request).await?;
let job_status: JobStatus = serde_json::from_value(status_response.data.unwrap())?;
```

---

## 🐿️ **Squirrel AI Integration**

### **Protocol Specification**

```rust
/// JSON-RPC integration for AI/ML operations
pub struct SquirrelIntegration {
    json_rpc_client: JsonRpcService,
    model_registry: ModelRegistry,
    inference_engine: InferenceEngine,
}

/// AI/ML operations supported
pub enum SquirrelOperation {
    // Model Management
    LoadModel { model_id: String, version: String },
    UnloadModel { model_id: String },
    ListModels { filter: ModelFilter },
    GetModelInfo { model_id: String },
    
    // Inference
    RunInference { model_id: String, input_data: InferenceInput },
    BatchInference { model_id: String, batch_data: Vec<InferenceInput> },
    StreamInference { model_id: String, stream_config: StreamConfig },
    
    // Training
    StartTraining { training_config: TrainingConfiguration },
    StopTraining { training_id: String },
    GetTrainingStatus { training_id: String },
    
    // Data Analysis
    AnalyzeData { data_source: DataSource, analysis_type: AnalysisType },
    GenerateInsights { dataset: String, insight_config: InsightConfig },
    PredictTrends { historical_data: String, prediction_horizon: Duration },
}
```

### **Usage Examples**

```rust
// Load an AI model for data analysis
let model_request = UnifiedRpcRequest {
    target: "squirrel".to_string(),
    method: "load_model".to_string(),
    params: serde_json::json!({
        "model_id": "data_classifier_v2",
        "version": "2.1.0",
        "optimization": {
            "precision": "fp16",
            "batch_size": 32,
            "gpu_acceleration": true
        }
    }),
    priority: RequestPriority::Normal,
    // ... other fields
};

let model_response = rpc_manager.call(model_request).await?;

// Run inference on storage data
let inference_request = UnifiedRpcRequest {
    target: "squirrel".to_string(),
    method: "run_inference".to_string(),
    params: serde_json::json!({
        "model_id": "data_classifier_v2",
        "input_data": {
            "type": "nestgate_dataset",
            "dataset_path": "/tank/unclassified_data",
            "format": "parquet",
            "columns": ["feature1", "feature2", "feature3"]
        },
        "output_config": {
            "format": "json",
            "include_confidence": true,
            "threshold": 0.8
        }
    }),
    priority: RequestPriority::Normal,
    // ... other fields
};

let inference_response = rpc_manager.call(inference_request).await?;
let classification_results: ClassificationResults = serde_json::from_value(inference_response.data.unwrap())?;

// Analyze storage patterns for optimization
let analysis_request = UnifiedRpcRequest {
    target: "squirrel".to_string(),
    method: "analyze_data".to_string(),
    params: serde_json::json!({
        "data_source": {
            "type": "nestgate_metrics",
            "metrics": ["storage_usage", "access_patterns", "performance_data"],
            "time_range": "30d"
        },
        "analysis_type": "storage_optimization",
        "parameters": {
            "predict_future_usage": true,
            "identify_hot_data": true,
            "recommend_tiering": true
        }
    }),
    // ... other fields
};

let analysis_response = rpc_manager.call(analysis_request).await?;
let optimization_recommendations: StorageOptimization = serde_json::from_value(analysis_response.data.unwrap())?;
```

---

## 🔄 **Cross-Primal Workflows**

### **Complex Integration Example**

```rust
/// Example: Secure AI-powered storage optimization workflow
async fn secure_ai_storage_optimization(
    rpc_manager: &UnifiedRpcManager
) -> Result<OptimizationResults, IntegrationError> {
    
    // Step 1: Encrypt sensitive data with Beardog
    let encryption_request = UnifiedRpcRequest {
        target: "beardog".to_string(),
        method: "encrypt_data".to_string(),
        params: serde_json::json!({
            "data": storage_metrics,
            "algorithm": "AES-256-GCM",
            "purpose": "ai_analysis"
        }),
        priority: RequestPriority::High,
        // ... other fields
    };
    
    let encrypted_data = rpc_manager.call(encryption_request).await?;
    
    // Step 2: Register workflow with Songbird
    let workflow_request = UnifiedRpcRequest {
        target: "songbird".to_string(),
        method: "start_workflow".to_string(),
        params: serde_json::json!({
            "workflow": {
                "name": "secure_storage_optimization",
                "participants": ["nestgate", "beardog", "toadstool", "squirrel"],
                "security_level": "high",
                "data_classification": "sensitive"
            }
        }),
        // ... other fields
    };
    
    let workflow_id = rpc_manager.call(workflow_request).await?;
    
    // Step 3: Submit compute job to Toadstool
    let compute_request = UnifiedRpcRequest {
        target: "toadstool".to_string(),
        method: "submit_job".to_string(),
        params: serde_json::json!({
            "job": {
                "name": "storage_data_preprocessing",
                "type": "data_processing",
                "security_context": workflow_id,
                "encrypted_input": encrypted_data,
                "resources": {
                    "cpu_cores": 4,
                    "memory_gb": 16,
                    "storage_gb": 50
                }
            }
        }),
        // ... other fields
    };
    
    let job_result = rpc_manager.call(compute_request).await?;
    
    // Step 4: Run AI analysis with Squirrel
    let ai_request = UnifiedRpcRequest {
        target: "squirrel".to_string(),
        method: "analyze_data".to_string(),
        params: serde_json::json!({
            "data_source": job_result,
            "analysis_type": "storage_optimization",
            "security_context": workflow_id,
            "model": "storage_optimizer_v3"
        }),
        // ... other fields
    };
    
    let optimization_results = rpc_manager.call(ai_request).await?;
    
    // Step 5: Decrypt results with Beardog
    let decryption_request = UnifiedRpcRequest {
        target: "beardog".to_string(),
        method: "decrypt_data".to_string(),
        params: serde_json::json!({
            "encrypted_data": optimization_results,
            "security_context": workflow_id
        }),
        // ... other fields
    };
    
    let final_results = rpc_manager.call(decryption_request).await?;
    
    // Step 6: Complete workflow with Songbird
    let completion_request = UnifiedRpcRequest {
        target: "songbird".to_string(),
        method: "complete_workflow".to_string(),
        params: serde_json::json!({
            "workflow_id": workflow_id,
            "status": "success",
            "results": final_results
        }),
        // ... other fields
    };
    
    rpc_manager.call(completion_request).await?;
    
    Ok(serde_json::from_value(final_results)?)
}
```

---

## 📊 **Performance Optimization**

### **Protocol-Specific Optimizations**

```rust
/// Optimization strategies per protocol
pub struct ProtocolOptimizations {
    tarpc: TarpcOptimizations,
    json_rpc: JsonRpcOptimizations,
    websocket: WebSocketOptimizations,
}

pub struct TarpcOptimizations {
    /// Use connection pooling for high-frequency security operations
    pub connection_pool_size: usize,
    /// Enable binary compression for large payloads
    pub enable_compression: bool,
    /// Use persistent connections for streaming
    pub persistent_connections: bool,
    /// Batch small requests to reduce overhead
    pub request_batching: bool,
}

pub struct JsonRpcOptimizations {
    /// Use HTTP/2 for multiplexing
    pub http2_enabled: bool,
    /// Enable gzip compression
    pub compression_enabled: bool,
    /// Connection keep-alive
    pub keep_alive: bool,
    /// Request pipelining
    pub pipelining: bool,
}

pub struct WebSocketOptimizations {
    /// Message compression
    pub per_message_deflate: bool,
    /// Binary frames for efficiency
    pub binary_frames: bool,
    /// Automatic reconnection
    pub auto_reconnect: bool,
    /// Heartbeat interval
    pub heartbeat_interval: Duration,
}
```

### **Caching Strategies**

```rust
/// Multi-level caching for improved performance
pub struct IntegrationCache {
    /// Service discovery cache
    service_cache: Arc<RwLock<HashMap<String, CachedServiceInfo>>>,
    /// Authentication token cache
    auth_cache: Arc<RwLock<HashMap<String, CachedToken>>>,
    /// Response cache for idempotent operations
    response_cache: Arc<RwLock<LruCache<String, CachedResponse>>>,
    /// Connection pool cache
    connection_cache: Arc<RwLock<HashMap<String, PooledConnection>>>,
}

impl IntegrationCache {
    /// Cache service discovery results
    pub async fn cache_service_info(&self, service: &str, info: ServiceInfo) {
        let mut cache = self.service_cache.write().await;
        cache.insert(service.to_string(), CachedServiceInfo {
            info,
            cached_at: Instant::now(),
            ttl: Duration::from_secs(300), // 5 minutes
        });
    }
    
    /// Get cached authentication token
    pub async fn get_cached_token(&self, service: &str) -> Option<String> {
        let cache = self.auth_cache.read().await;
        if let Some(cached_token) = cache.get(service) {
            if cached_token.expires_at > Instant::now() {
                return Some(cached_token.token.clone());
            }
        }
        None
    }
}
```

---

## 🛡️ **Security Considerations**

### **End-to-End Security**

```rust
/// Comprehensive security configuration
pub struct IntegrationSecurity {
    /// TLS configuration for all connections
    pub tls_config: TlsConfiguration,
    /// Authentication configuration per service
    pub auth_configs: HashMap<String, AuthConfiguration>,
    /// Encryption for sensitive data
    pub encryption_config: EncryptionConfiguration,
    /// Audit logging configuration
    pub audit_config: AuditConfiguration,
}

pub struct TlsConfiguration {
    /// Minimum TLS version
    pub min_version: TlsVersion,
    /// Certificate validation
    pub verify_certificates: bool,
    /// Cipher suites
    pub cipher_suites: Vec<CipherSuite>,
    /// Certificate pinning
    pub certificate_pins: HashMap<String, String>,
}

pub struct AuthConfiguration {
    /// Authentication method
    pub auth_method: AuthMethod,
    /// Token refresh configuration
    pub token_refresh: TokenRefreshConfig,
    /// Multi-factor authentication
    pub mfa_required: bool,
    /// Session timeout
    pub session_timeout: Duration,
}
```

### **Data Classification**

```rust
/// Data classification for proper handling
pub enum DataClassification {
    Public,
    Internal,
    Confidential,
    Secret,
}

pub struct SecureDataHandler {
    classification: DataClassification,
    encryption_required: bool,
    audit_required: bool,
    retention_policy: RetentionPolicy,
}

impl SecureDataHandler {
    pub async fn process_request(&self, request: &mut UnifiedRpcRequest) -> Result<(), SecurityError> {
        // Apply classification-based security measures
        match self.classification {
            DataClassification::Secret => {
                request.metadata.insert("encryption".to_string(), "required".to_string());
                request.metadata.insert("audit".to_string(), "full".to_string());
                request.priority = RequestPriority::Critical;
            },
            DataClassification::Confidential => {
                request.metadata.insert("encryption".to_string(), "preferred".to_string());
                request.metadata.insert("audit".to_string(), "standard".to_string());
            },
            _ => {
                // Standard processing
            }
        }
        
        Ok(())
    }
}
```

---

## 🧪 **Testing & Validation**

### **Integration Test Suite**

```rust
#[cfg(test)]
mod integration_tests {
    use super::*;
    
    #[tokio::test]
    async fn test_beardog_security_integration() {
        let manager = setup_test_rpc_manager().await;
        
        // Test encryption round-trip
        let plaintext = b"sensitive test data";
        let encrypt_request = create_encryption_request(plaintext);
        let encrypt_response = manager.call(encrypt_request).await.unwrap();
        
        let decrypt_request = create_decryption_request(&encrypt_response);
        let decrypt_response = manager.call(decrypt_request).await.unwrap();
        
        assert_eq!(decrypt_response.data.unwrap(), base64::encode(plaintext));
    }
    
    #[tokio::test]
    async fn test_songbird_orchestration_integration() {
        let manager = setup_test_rpc_manager().await;
        
        // Test service registration
        let registration = create_service_registration();
        let response = manager.call(registration).await.unwrap();
        assert!(response.success);
        
        // Test service discovery
        let discovery = create_service_discovery_request();
        let services = manager.call(discovery).await.unwrap();
        assert!(!services.data.unwrap().as_array().unwrap().is_empty());
    }
    
    #[tokio::test]
    async fn test_cross_primal_workflow() {
        let manager = setup_test_rpc_manager().await;
        
        // Execute complex cross-primal workflow
        let results = secure_ai_storage_optimization(&manager).await.unwrap();
        assert!(results.optimization_score > 0.8);
    }
    
    #[tokio::test]
    async fn test_performance_requirements() {
        let manager = setup_production_rpc_manager().await;
        
        // Test throughput
        let start = Instant::now();
        let mut tasks = Vec::new();
        
        for _ in 0..1000 {
            let manager = manager.clone();
            tasks.push(tokio::spawn(async move {
                let request = create_health_check_request();
                manager.call(request).await
            }));
        }
        
        let results: Vec<_> = futures::future::join_all(tasks).await;
        let duration = start.elapsed();
        
        let successful = results.iter().filter(|r| r.is_ok()).count();
        let rps = successful as f64 / duration.as_secs_f64();
        
        assert!(rps > 500.0, "Should achieve >500 RPS, got {}", rps);
        
        // Test latency
        let latencies: Vec<_> = results.iter()
            .filter_map(|r| r.as_ref().ok())
            .filter_map(|r| r.as_ref().ok())
            .map(|r| r.metrics.processing_time_ms)
            .collect();
        
        let p95_latency = percentile(&latencies, 95.0);
        assert!(p95_latency < 100.0, "P95 latency should be <100ms, got {}ms", p95_latency);
    }
}
```

---

## 📚 **Migration Guide**

### **From Legacy Integrations**

```rust
// OLD: Direct client integrations
let toadstool_client = ToadstoolComputeClient::new(hardcoded_config);
let result = toadstool_client.optimize_hardware().await?;

let beardog_client = BeardogSecurityClient::new(hardcoded_config);
let encrypted = beardog_client.encrypt(data).await?;

// NEW: Unified RPC integration
let rpc_manager = UnifiedRpcManager::new_production().await?;

let toadstool_request = UnifiedRpcRequest {
    target: "toadstool".to_string(),
    method: "optimize_hardware".to_string(),
    // ... unified configuration
};
let result = rpc_manager.call(toadstool_request).await?;

let beardog_request = UnifiedRpcRequest {
    target: "beardog".to_string(),
    method: "encrypt_data".to_string(),
    // ... unified configuration
};
let encrypted = rpc_manager.call(beardog_request).await?;
```

### **Configuration Migration**

```yaml
# OLD: Service-specific configurations
toadstool:
  endpoint: "http://toadstool.local:8080"
  api_key: "secret"
  
beardog:
  endpoint: "beardog.local:9090"
  certificate: "/path/to/cert"

# NEW: Unified RPC configuration
rpc:
  services:
    toadstool:
      protocol: json_rpc
      endpoints:
        - address: "http://toadstool.local:8080"
          weight: 100
      authentication:
        type: api_key
        key: "${TOADSTOOL_API_KEY}"
      
    beardog:
      protocol: tarpc
      endpoints:
        - address: "beardog.local:9090"
          weight: 100
      security:
        tls_enabled: true
        certificate_path: "/path/to/cert"
```

---

## 🎯 **Future Enhancements**

### **Planned Features**

1. **Distributed Tracing**: Full request tracing across all primals
2. **Service Mesh Integration**: Istio/Linkerd compatibility
3. **Advanced Analytics**: AI-powered performance optimization
4. **Auto-scaling**: Dynamic resource allocation based on load
5. **Multi-tenancy**: Isolated environments for different users
6. **Federation**: Cross-cluster primal communication

### **Roadmap**

| **Phase** | **Features** | **Timeline** | **Status** |
|-----------|-------------|--------------|------------|
| **Phase 1** | Core Integration | Q1 2025 | ✅ Complete |
| **Phase 2** | Advanced Security | Q2 2025 | 🔄 In Progress |
| **Phase 3** | AI Integration | Q3 2025 | 📋 Planned |
| **Phase 4** | Federation | Q4 2025 | 📋 Planned |

---

**This specification represents the complete integration framework for NestGate's Universal RPC System with all ecoPrimals, providing enterprise-grade capabilities that enable seamless, secure, and high-performance inter-service communication across the entire ecosystem.** 